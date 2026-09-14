//! Conversions between raw register values and physical quantities.
//!
//! Every decode function in this module is pure and *total*: given a value of
//! the input type it always produces an output, with no failure case and no
//! panic. The encode functions are pure but *not* total — a caller-supplied
//! threshold may not be representable in a 16-bit register at the channel's
//! configured scale, so they return [`Option`]. Purity is the property that
//! matters: it is what keeps this module testable by walking its input domain
//! on the host, with no bus involved.
//!
//! Because nothing here touches a bus, these functions are testable by walking
//! their entire input domain on the host. [`decode_bus_voltage`] has 65,536
//! inputs; [`decode_shunt_voltage`] has 131,072. Both are exhausted in the
//! test suite in well under a millisecond.

use crate::units::{AdcRange, BusVoltage, Calibration, Channel, Current, Energy, Power, ShuntVoltage};

/// Bus voltage LSB, in microvolts (1.6 mV, datasheet §7.1.7).
const BUS_LSB_UV: u32 = 1_600;

/// Multiplier relating `POWER`/`ENERGY` LSB to `CURRENT_LSB` (datasheet
/// Equations 4 and 5).
const POWER_LSB_MULTIPLIER: u64 = 32;

/// Decode a `SHUNT_VOLTAGE` register value.
///
/// `Value [V] = shunt_lsb × raw`, where the LSB is 2.5 µV on
/// [`AdcRange::Range0`] and 625 nV on [`AdcRange::Range1`]. Both are whole
/// nanovolts, so the result is exact.
///
/// Total: the widest product, `-32768 × 2500`, is well inside `i32`.
#[must_use]
pub const fn decode_shunt_voltage(raw: i16, range: AdcRange) -> ShuntVoltage {
    ShuntVoltage::from_nanovolts(raw as i32 * range.shunt_lsb_nv())
}

/// Decode a `BUS_VOLTAGE` register value.
///
/// `Value [V] = 1.6 mV × raw`. The register is always positive
/// (datasheet Table 7-13), so the raw value is read unsigned.
///
/// Total: the widest product, `65535 × 1600`, is well inside `u32`.
#[must_use]
pub const fn decode_bus_voltage(raw: u16) -> BusVoltage {
    BusVoltage::from_microvolts(raw as u32 * BUS_LSB_UV)
}

/// Decode a `CURRENT` register value.
///
/// `Value [A] = CURRENT_LSB × raw` (datasheet Equation 3).
///
/// Total: `CurrentLsb` is bounded at construction so the product always fits
/// `i64`.
#[must_use]
pub const fn decode_current(raw: i16, cal: Calibration) -> Current {
    Current::from_nanoamps(raw as i64 * cal.current_lsb().as_nanoamps() as i64)
}

/// Decode a `POWER` register value.
///
/// `Value [W] = 32 × CURRENT_LSB × raw` (datasheet Equation 4). The register
/// is unsigned.
///
/// Total: `CurrentLsb` is bounded at construction so the product always fits
/// `u64`.
#[must_use]
pub const fn decode_power(raw: u16, cal: Calibration) -> Power {
    Power::from_nanowatts(raw as u64 * POWER_LSB_MULTIPLIER * cal.current_lsb().as_nanoamps() as u64)
}

/// Decode an `ENERGY` register value.
///
/// `Value [J] = 32 × CURRENT_LSB × raw` (datasheet Equation 5). The register
/// is an unsigned 32-bit accumulator (datasheet Table 7-18).
///
/// Total: `CurrentLsb` is bounded at construction so the product always fits
/// `u64`.
#[must_use]
pub const fn decode_energy(raw: u32, cal: Calibration) -> Energy {
    Energy::from_nanojoules(raw as u64 * POWER_LSB_MULTIPLIER * cal.current_lsb().as_nanoamps() as u64)
}

/// Set or clear a channel's bit in a four-bit channel mask.
///
/// Used for both `CONFIG1.ACTIVE_CHANNEL` and `CONFIG2.RANGE`, which share a
/// layout in which bit 0 is channel 1.
#[must_use]
pub const fn set_channel_bit(mask: u8, channel: Channel, set: bool) -> u8 {
    if set {
        mask | channel.mask()
    } else {
        mask & !channel.mask()
    }
}

/// Test a channel's bit in a four-bit channel mask.
#[must_use]
pub const fn channel_bit(mask: u8, channel: Channel) -> bool {
    mask & channel.mask() != 0
}

/// Divide, rounding to nearest, with halves rounding away from zero.
///
/// Matches the rounding `Calibration::new` already uses for `SHUNT_CAL`.
///
/// `d` is always a non-zero LSB constant, so this cannot trap. Callers pass
/// `n` widened from `i32`, so `n + d / 2` cannot overflow `i64`.
const fn div_round_nearest_i64(n: i64, d: i64) -> i64 {
    if (n < 0) == (d < 0) {
        (n + d / 2) / d
    } else {
        (n - d / 2) / d
    }
}

/// Unsigned counterpart of [`div_round_nearest_i64`].
///
/// Written with an explicit remainder rather than `(n + d / 2) / d`, because
/// `encode_power_limit` passes a full-range `u64` and that form overflows near
/// `u64::MAX`. `r < d` and `d` is at most `32 * u32::MAX`, so `r * 2` stays
/// well inside `u64`.
const fn div_round_nearest_u64(n: u64, d: u64) -> u64 {
    let q = n / d;
    let r = n % d;
    if r * 2 >= d { q + 1 } else { q }
}

/// Encode a shunt-voltage threshold for `ALERT_LIMIT`.
///
/// Inverse of [`decode_shunt_voltage`]. Rounds to nearest, matching
/// `Calibration::new`.
///
/// Returns [`None`] if the threshold does not fit the signed 16-bit register
/// at `range`: -81.92 mV to 81.9175 mV on [`AdcRange::Range0`], -20.48 mV to
/// 20.479375 mV on [`AdcRange::Range1`].
#[must_use]
pub const fn encode_shunt_limit(v: ShuntVoltage, range: AdcRange) -> Option<i16> {
    let raw = div_round_nearest_i64(v.as_nanovolts() as i64, range.shunt_lsb_nv() as i64);
    if raw < i16::MIN as i64 || raw > i16::MAX as i64 {
        return None;
    }
    #[allow(clippy::cast_possible_truncation)]
    Some(raw as i16)
}

/// Highest `ALERT_LIMIT` code for a bus-voltage threshold.
///
/// Datasheet §7.1.5 specifies bus limits as unsigned *15*-bit. That is not an
/// inconsistency against the 16-bit result register: `32767 × 1.6 mV =
/// 52.4272 V`, exactly the bus measurement range in §8.1.1, so reserved bit 15
/// covers codes the ADC never produces.
const BUS_LIMIT_MAX: u64 = 0x7FFF;

/// Encode a bus-voltage threshold for `ALERT_LIMIT`.
///
/// Inverse of [`decode_bus_voltage`]. Rounds to nearest.
///
/// Returns [`None`] above 52.4272 V. Needs no calibration: the bus LSB is a
/// fixed 1.6 mV.
#[must_use]
pub const fn encode_bus_limit(v: BusVoltage) -> Option<u16> {
    let raw = div_round_nearest_u64(v.as_microvolts() as u64, BUS_LSB_UV as u64);
    if raw > BUS_LIMIT_MAX {
        return None;
    }
    #[allow(clippy::cast_possible_truncation)]
    Some(raw as u16)
}

/// Encode a power threshold for `ALERT_LIMIT`.
///
/// Inverse of [`decode_power`]. Rounds to nearest.
///
/// Returns [`None`] above `65535 × 32 × CURRENT_LSB`. The scale comes from the
/// target channel's calibration, so the same threshold may be representable on
/// one channel and not another.
#[must_use]
pub const fn encode_power_limit(p: Power, cal: Calibration) -> Option<u16> {
    let lsb = POWER_LSB_MULTIPLIER * cal.current_lsb().as_nanoamps() as u64;
    let raw = div_round_nearest_u64(p.as_nanowatts(), lsb);
    if raw > u16::MAX as u64 {
        return None;
    }
    #[allow(clippy::cast_possible_truncation)]
    Some(raw as u16)
}
