//! Physical quantities and validated configuration values.
//!
//! Everything in this module is pure: no bus, no `async`, no HAL. Values
//! arrive as raw register words and leave as types that cannot be nonsensical,
//! so the rest of the driver never has to re-check them.
//!
//! Calibration and every stored measurement use integer arithmetic. Floating
//! point appears only in the display-oriented `to_*` methods.
//!
//! # Units
//!
//! Each measurement is stored in the finest unit that keeps the conversion
//! from the raw register value *exact*, so no precision is lost between the
//! device and the caller:
//!
//! | Type           | Stored as | Why                                    |
//! | -------------- | --------- | -------------------------------------- |
//! | [`ShuntVoltage`] | nV (`i32`) | Finest shunt LSB is 625 nV           |
//! | [`BusVoltage`]   | µV (`u32`) | Bus LSB is 1.6 mV = 1600 µV          |
//! | [`Current`]      | nA (`i64`) | `CURRENT_LSB` is caller-chosen       |
//! | [`Power`]        | nW (`u64`) | 32 × `CURRENT_LSB`                   |
//! | [`Energy`]       | nJ (`u64`) | 32 × `CURRENT_LSB`, accumulated      |
//!
//! Floating point appears only in the `to_*` rendering methods, which are for
//! display and logging. For arithmetic, prefer each type's base-unit accessor
//! — `as_nanovolts`, `as_microvolts`, `as_nanoamps`, `as_nanowatts`,
//! `as_nanojoules` — which are lossless. The coarser convenience accessors
//! (`as_microamps`, `as_microwatts`, `as_microjoules`) divide and so truncate
//! toward zero.

use core::num::NonZeroU32;

// ── I²C address ───────────────────────────────────────────────────────────────

/// Logic level of an I²C address pin (A0 or A1).
///
/// The discriminants are the two-bit codes used by the address encoding in
/// datasheet Table 6-1.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum AddrPinState {
    /// Address pin tied to GND (default).
    #[default]
    Gnd = 0,
    /// Address pin tied to VS.
    Vs = 1,
    /// Address pin tied to SDA.
    Sda = 2,
    /// Address pin tied to SCL.
    Scl = 3,
}

impl AddrPinState {
    /// Every pin state, in encoding order.
    pub const ALL: [Self; 4] = [Self::Gnd, Self::Vs, Self::Sda, Self::Scl];
}

/// How the A0 and A1 pins are strapped on the board.
///
/// The fields are named rather than positional precisely because both are
/// [`AddrPinState`]: a positional pair can be transposed silently, and a
/// transposed pair yields a valid-looking but wrong address.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct AddressPins {
    /// Strapping of the A0 pin.
    pub a0: AddrPinState,
    /// Strapping of the A1 pin.
    pub a1: AddrPinState,
}

/// A 7-bit I²C target address for an INA4230.
///
/// Only the sixteen addresses reachable by pin strapping can be constructed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Address(u8);

impl Address {
    /// Lowest reachable address (A1 = A0 = GND).
    pub const MIN: u8 = 0x40;
    /// Highest reachable address (A1 = A0 = SCL).
    pub const MAX: u8 = 0x4F;

    /// Derive the target address from the A0/A1 pin strapping.
    ///
    /// Datasheet Table 6-1: the address byte is `100_A3A2A1A0`, where the
    /// A1 strapping supplies the upper two bits and A0 the lower two:
    ///
    /// ```text
    /// address = 0x40 | (A1 << 2) | A0
    /// ```
    #[must_use]
    pub const fn from_pins(pins: AddressPins) -> Self {
        Self(Self::MIN | ((pins.a1 as u8) << 2) | (pins.a0 as u8))
    }

    /// The 7-bit address as consumed by an I²C driver.
    #[must_use]
    pub const fn as_u8(self) -> u8 {
        self.0
    }
}

impl From<AddressPins> for Address {
    fn from(pins: AddressPins) -> Self {
        Self::from_pins(pins)
    }
}

// ── Channels ──────────────────────────────────────────────────────────────────

/// One of the four measurement channels on the INA4230.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum Channel {
    /// Channel 1.
    Ch1 = 0,
    /// Channel 2.
    Ch2 = 1,
    /// Channel 3.
    Ch3 = 2,
    /// Channel 4.
    Ch4 = 3,
}

impl Channel {
    /// Every channel, in index order.
    pub const ALL: [Self; 4] = [Self::Ch1, Self::Ch2, Self::Ch3, Self::Ch4];

    /// Zero-based index, usable for array lookup and register striding.
    #[must_use]
    pub const fn index(self) -> usize {
        self as usize
    }

    /// This channel's bit within `CONFIG1.ACTIVE_CHANNEL` and `CONFIG2.RANGE`.
    ///
    /// Both fields use the same layout: bit 0 is channel 1 (datasheet
    /// Tables 7-3 and 7-4).
    #[must_use]
    pub const fn mask(self) -> u8 {
        1 << (self as u8)
    }
}

// ── ADC range ─────────────────────────────────────────────────────────────────

/// ADC full-scale input range for shunt voltage measurement.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AdcRange {
    /// ±81.92 mV full scale, LSB = 2.5 µV (default).
    #[default]
    Range0,
    /// ±20.48 mV full scale, LSB = 625 nV. `SHUNT_CAL` is divided by 4.
    Range1,
}

impl AdcRange {
    /// Both ranges.
    pub const ALL: [Self; 2] = [Self::Range0, Self::Range1];

    /// Shunt voltage LSB in nanovolts. Both values are exact integers.
    #[must_use]
    pub const fn shunt_lsb_nv(self) -> i32 {
        match self {
            Self::Range0 => 2_500,
            Self::Range1 => 625,
        }
    }

    /// Divisor applied to `SHUNT_CAL` for this range (datasheet §8.1.2).
    #[must_use]
    pub const fn shunt_cal_divisor(self) -> u64 {
        match self {
            Self::Range0 => 1,
            Self::Range1 => 4,
        }
    }
}

// ── Calibration inputs ────────────────────────────────────────────────────────

/// Why a [`Calibration`] could not be built.
///
/// These are the only ways calibration can fail. Once a [`Calibration`]
/// exists, every conversion that uses it is total.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CalibrationError {
    /// `CURRENT_LSB` was zero or above [`CurrentLsb::MAX_NA`].
    CurrentLsbOutOfRange,
    /// The shunt resistance was zero.
    ShuntResistanceZero,
    /// The resulting `SHUNT_CAL` would be zero.
    ///
    /// The device reports a current of zero when `SHUNT_CAL` is zero
    /// (datasheet §8.1.2), so this combination can never produce a reading.
    /// Choose a smaller `CURRENT_LSB` or a smaller shunt.
    ShuntCalUnderflow,
    /// The resulting `SHUNT_CAL` would exceed the 15-bit register field.
    ///
    /// Choose a larger `CURRENT_LSB` or a larger shunt.
    ShuntCalOverflow,
}

/// Current resolution, in nanoamperes per LSB of the `CURRENT` register.
///
/// Datasheet Equation 2 gives the minimum useful value as
/// `max_expected_current / 2^15`; the selected value must also stay strictly
/// below eight times that minimum to avoid losing resolution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct CurrentLsb(NonZeroU32);

impl CurrentLsb {
    /// Largest accepted value, 100 mA/LSB.
    ///
    /// This corresponds to a full-scale current of about ±3.3 kA, far beyond
    /// anything this part is used for. The bound exists so that [`Energy`],
    /// the widest derived quantity, provably cannot overflow its `u64`.
    pub const MAX_NA: u32 = 100_000_000;

    /// Build a current LSB from nanoamperes per LSB.
    ///
    /// # Errors
    ///
    /// Returns [`CalibrationError::CurrentLsbOutOfRange`] if `na_per_lsb` is
    /// zero or greater than [`Self::MAX_NA`].
    pub const fn from_nanoamps(na_per_lsb: u32) -> Result<Self, CalibrationError> {
        if na_per_lsb == 0 || na_per_lsb > Self::MAX_NA {
            return Err(CalibrationError::CurrentLsbOutOfRange);
        }
        match NonZeroU32::new(na_per_lsb) {
            Some(v) => Ok(Self(v)),
            None => Err(CalibrationError::CurrentLsbOutOfRange),
        }
    }

    /// The datasheet's minimum `CURRENT_LSB` for a given full-scale current.
    ///
    /// Datasheet Equation 2: `CURRENT_LSB(min) = MAX_CURRENT / 2^15`. The
    /// result is rounded up, so it is never below the datasheet value.
    ///
    /// # This is a floor, not a recommendation
    ///
    /// Equation 2 divides by `2^15` = 32768, but the `CURRENT` register is
    /// signed 16-bit, so its largest positive code is 32767. The returned LSB
    /// therefore reaches `32767 × lsb`, marginally *below* `max_current_na`
    /// (about 0.003% short). This is the datasheet's own formula, kept as-is
    /// so the value cross-checks against TI's worked example.
    ///
    /// The datasheet expects you to round up from here to a convenient number
    /// — its example takes a 305.17578 µA minimum for 10 A and uses 500 µA.
    /// The selected value must stay *strictly below* eight times the minimum
    /// to avoid losing resolution (datasheet §8.1.2). Rounding up also
    /// resolves the shortfall.
    ///
    /// # Errors
    ///
    /// Returns [`CalibrationError::CurrentLsbOutOfRange`] if the result is
    /// zero or exceeds [`Self::MAX_NA`].
    pub const fn min_for_max_current(max_current_na: u64) -> Result<Self, CalibrationError> {
        let lsb = max_current_na.div_ceil(1 << 15);
        if lsb == 0 || lsb > Self::MAX_NA as u64 {
            return Err(CalibrationError::CurrentLsbOutOfRange);
        }
        #[allow(clippy::cast_possible_truncation)]
        Self::from_nanoamps(lsb as u32)
    }

    /// Nanoamperes per LSB.
    #[must_use]
    pub const fn as_nanoamps(self) -> u32 {
        self.0.get()
    }
}

/// Shunt resistance, in microohms.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ShuntResistance(NonZeroU32);

impl ShuntResistance {
    /// Build a shunt resistance from microohms.
    ///
    /// # Errors
    ///
    /// Returns [`CalibrationError::ShuntResistanceZero`] if `microohms` is zero.
    pub const fn from_microohms(microohms: u32) -> Result<Self, CalibrationError> {
        match NonZeroU32::new(microohms) {
            Some(v) => Ok(Self(v)),
            None => Err(CalibrationError::ShuntResistanceZero),
        }
    }

    /// Microohms.
    #[must_use]
    pub const fn as_microohms(self) -> u32 {
        self.0.get()
    }
}

/// A validated `SHUNT_CAL` register value.
///
/// Guaranteed to be in `1..=32767`: non-zero, because a zero `SHUNT_CAL` makes
/// the device report zero current, and within the 15-bit register field.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ShuntCal(u16);

impl ShuntCal {
    /// Largest value the 15-bit `SHUNT_CAL` field can hold.
    pub const MAX: u16 = 0x7FFF;

    /// The register value.
    #[must_use]
    pub const fn as_u16(self) -> u16 {
        self.0
    }

    pub(crate) const fn new_unchecked(value: u16) -> Self {
        Self(value)
    }
}

/// A complete, validated per-channel calibration.
///
/// The existence of this value is the evidence that the calibration inputs
/// were sound, so every conversion taking a `Calibration` is total.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Calibration {
    current_lsb: CurrentLsb,
    shunt_cal: ShuntCal,
    adc_range: AdcRange,
}

impl Calibration {
    /// Compute the calibration for a channel.
    ///
    /// Datasheet Equation 1, `SHUNT_CAL = 0.00512 / (CURRENT_LSB × R_SHUNT)`,
    /// with `SHUNT_CAL` divided by four when `adc_range` is
    /// [`AdcRange::Range1`]. Expressed in the integer units used here, with
    /// `CURRENT_LSB` in nA and `R_SHUNT` in µΩ, that constant becomes
    /// `5.12e12`, so the whole calculation is exact integer arithmetic:
    ///
    /// ```text
    /// SHUNT_CAL = 5_120_000_000_000 / (current_lsb_na × shunt_uohm × divisor)
    /// ```
    ///
    /// The division rounds to nearest rather than truncating, which would
    /// otherwise introduce a systematic one-sided gain error.
    ///
    /// Because both operands are non-zero by construction, the division
    /// cannot divide by zero.
    ///
    /// # Errors
    ///
    /// Returns [`CalibrationError::ShuntCalUnderflow`] or
    /// [`CalibrationError::ShuntCalOverflow`] if the resulting `SHUNT_CAL`
    /// does not fit the register's usable `1..=32767` range.
    pub const fn new(
        current_lsb: CurrentLsb,
        shunt: ShuntResistance,
        adc_range: AdcRange,
    ) -> Result<Self, CalibrationError> {
        /// `0.00512` scaled by `1e15` to convert nA × µΩ into A × Ω.
        const NUMERATOR: u64 = 5_120_000_000_000;

        let divisor = current_lsb.as_nanoamps() as u64 * shunt.as_microohms() as u64 * adc_range.shunt_cal_divisor();

        // Round to nearest. `divisor` is non-zero, so this cannot trap.
        let value = (NUMERATOR + divisor / 2) / divisor;

        if value == 0 {
            return Err(CalibrationError::ShuntCalUnderflow);
        }
        if value > ShuntCal::MAX as u64 {
            return Err(CalibrationError::ShuntCalOverflow);
        }

        #[allow(clippy::cast_possible_truncation)]
        Ok(Self {
            current_lsb,
            shunt_cal: ShuntCal::new_unchecked(value as u16),
            adc_range,
        })
    }

    /// The `SHUNT_CAL` value to write to the device.
    #[must_use]
    pub const fn shunt_cal(self) -> ShuntCal {
        self.shunt_cal
    }

    /// The current resolution this calibration was built for.
    #[must_use]
    pub const fn current_lsb(self) -> CurrentLsb {
        self.current_lsb
    }

    /// The ADC range this calibration was built for.
    #[must_use]
    pub const fn adc_range(self) -> AdcRange {
        self.adc_range
    }
}

// ── Measurements ──────────────────────────────────────────────────────────────

/// A shunt voltage reading, in nanovolts.
///
/// Exact: both ADC range LSBs (2.5 µV and 625 nV) are whole nanovolts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ShuntVoltage(i32);

impl ShuntVoltage {
    /// Construct from a raw value in nanovolts.
    ///
    /// Values outside the device's measurable range are accepted here and
    /// rejected later, when a threshold is encoded against a specific
    /// calibration — the representable range depends on `AdcRange` or
    /// `CURRENT_LSB`, neither of which is known at this point.
    #[must_use]
    pub const fn from_nanovolts(nv: i32) -> Self {
        Self(nv)
    }

    /// Nanovolts.
    #[must_use]
    pub const fn as_nanovolts(self) -> i32 {
        self.0
    }

    /// Millivolts, for display.
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn to_millivolts(self) -> f32 {
        self.0 as f32 / 1_000_000.0
    }
}

/// A bus voltage reading, in microvolts.
///
/// Exact: the bus LSB is 1.6 mV, or 1600 whole microvolts. Always positive
/// (datasheet Table 7-13).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct BusVoltage(u32);

impl BusVoltage {
    /// Construct from a raw value in microvolts.
    ///
    /// Values outside the device's measurable range are accepted here and
    /// rejected later, when a threshold is encoded against a specific
    /// calibration — the representable range depends on `AdcRange` or
    /// `CURRENT_LSB`, neither of which is known at this point.
    #[must_use]
    pub const fn from_microvolts(uv: u32) -> Self {
        Self(uv)
    }

    /// Microvolts.
    #[must_use]
    pub const fn as_microvolts(self) -> u32 {
        self.0
    }

    /// Millivolts, for display.
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn to_millivolts(self) -> f32 {
        self.0 as f32 / 1_000.0
    }

    /// Volts, for display.
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn to_volts(self) -> f32 {
        self.0 as f32 / 1_000_000.0
    }
}

/// A current reading, in nanoamperes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Current(i64);

impl Current {
    /// Construct from a raw value in nanoamperes.
    ///
    /// Values outside the device's measurable range are accepted here and
    /// rejected later, when a threshold is encoded against a specific
    /// calibration — the representable range depends on `AdcRange` or
    /// `CURRENT_LSB`, neither of which is known at this point.
    #[must_use]
    pub const fn from_nanoamps(na: i64) -> Self {
        Self(na)
    }

    /// Nanoamperes.
    #[must_use]
    pub const fn as_nanoamps(self) -> i64 {
        self.0
    }

    /// Microamperes, truncated toward zero.
    #[must_use]
    pub const fn as_microamps(self) -> i64 {
        self.0 / 1_000
    }

    /// Milliamperes, for display.
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn to_milliamps(self) -> f32 {
        self.0 as f32 / 1_000_000.0
    }

    /// Amperes, for display.
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn to_amps(self) -> f32 {
        self.0 as f32 / 1_000_000_000.0
    }
}

/// A power reading, in nanowatts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Power(u64);

impl Power {
    /// Construct from a raw value in nanowatts.
    ///
    /// Values outside the device's measurable range are accepted here and
    /// rejected later, when a threshold is encoded against a specific
    /// calibration — the representable range depends on `AdcRange` or
    /// `CURRENT_LSB`, neither of which is known at this point.
    #[must_use]
    pub const fn from_nanowatts(nw: u64) -> Self {
        Self(nw)
    }

    /// Nanowatts.
    #[must_use]
    pub const fn as_nanowatts(self) -> u64 {
        self.0
    }

    /// Microwatts, truncated.
    #[must_use]
    pub const fn as_microwatts(self) -> u64 {
        self.0 / 1_000
    }

    /// Milliwatts, for display.
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn to_milliwatts(self) -> f32 {
        self.0 as f32 / 1_000_000.0
    }

    /// Watts, for display.
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn to_watts(self) -> f32 {
        self.0 as f32 / 1_000_000_000.0
    }
}

/// An accumulated energy reading, in nanojoules.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Energy(u64);

impl Energy {
    /// Construct from a raw value in nanojoules.
    ///
    /// Values outside the device's measurable range are accepted here and
    /// rejected later, when a threshold is encoded against a specific
    /// calibration — the representable range depends on `AdcRange` or
    /// `CURRENT_LSB`, neither of which is known at this point.
    #[must_use]
    pub const fn from_nanojoules(nj: u64) -> Self {
        Self(nj)
    }

    /// Nanojoules.
    #[must_use]
    pub const fn as_nanojoules(self) -> u64 {
        self.0
    }

    /// Microjoules, truncated.
    #[must_use]
    pub const fn as_microjoules(self) -> u64 {
        self.0 / 1_000
    }

    /// Millijoules, for display.
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn to_millijoules(self) -> f32 {
        self.0 as f32 / 1_000_000.0
    }

    /// Joules, for display.
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn to_joules(self) -> f32 {
        self.0 as f32 / 1_000_000_000.0
    }
}
