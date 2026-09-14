//! Tests for the pure core.
//!
//! None of these need a bus, a mock, or `async`. Where the input domain is
//! small enough to walk, they walk it: exhaustion is both cheaper and stronger
//! than sampling. Where it is not, they state properties.
//!
//! Concrete vectors are taken from the datasheet rather than from this
//! implementation. A test transcribed from the code it tests proves only that
//! the code is self-consistent.

use proptest::prelude::*;

use ina4230::AlertSlot;
use ina4230::convert::{
    channel_bit, decode_bus_voltage, decode_current, decode_energy, decode_power, decode_shunt_voltage, set_channel_bit,
};
use ina4230::units::{
    AdcRange, AddrPinState, Address, AddressPins, BusVoltage, Calibration, CalibrationError, Channel, Current,
    CurrentLsb, Energy, Power, ShuntCal, ShuntResistance, ShuntVoltage,
};

// ── Helpers ───────────────────────────────────────────────────────────────────

/// The calibration from the datasheet's worked design example (§8.2):
/// `CURRENT_LSB` = 500 µA, `R_SHUNT` = 8.0 mΩ, ADCRANGE = 0.
fn datasheet_example_calibration() -> Calibration {
    Calibration::new(
        CurrentLsb::from_nanoamps(500_000).unwrap(),
        ShuntResistance::from_microohms(8_000).unwrap(),
        AdcRange::Range0,
    )
    .unwrap()
}

// ── I²C address ───────────────────────────────────────────────────────────────

/// Datasheet Table 6-1, transcribed as `(A1, A0, address)`.
///
/// Note the column order: the table lists A1 first. Transposing these two
/// columns yields a plausible-looking table that is wrong for ten of the
/// sixteen strappings.
const DATASHEET_TABLE_6_1: [(AddrPinState, AddrPinState, u8); 16] = [
    (AddrPinState::Gnd, AddrPinState::Gnd, 0x40),
    (AddrPinState::Gnd, AddrPinState::Vs, 0x41),
    (AddrPinState::Gnd, AddrPinState::Sda, 0x42),
    (AddrPinState::Gnd, AddrPinState::Scl, 0x43),
    (AddrPinState::Vs, AddrPinState::Gnd, 0x44),
    (AddrPinState::Vs, AddrPinState::Vs, 0x45),
    (AddrPinState::Vs, AddrPinState::Sda, 0x46),
    (AddrPinState::Vs, AddrPinState::Scl, 0x47),
    (AddrPinState::Sda, AddrPinState::Gnd, 0x48),
    (AddrPinState::Sda, AddrPinState::Vs, 0x49),
    (AddrPinState::Sda, AddrPinState::Sda, 0x4A),
    (AddrPinState::Sda, AddrPinState::Scl, 0x4B),
    (AddrPinState::Scl, AddrPinState::Gnd, 0x4C),
    (AddrPinState::Scl, AddrPinState::Vs, 0x4D),
    (AddrPinState::Scl, AddrPinState::Sda, 0x4E),
    (AddrPinState::Scl, AddrPinState::Scl, 0x4F),
];

#[test]
fn address_matches_datasheet_table_6_1() {
    for (a1, a0, expected) in DATASHEET_TABLE_6_1 {
        let got = Address::from_pins(AddressPins { a0, a1 }).as_u8();
        assert_eq!(
            got, expected,
            "A1={a1:?} A0={a0:?}: expected {expected:#04X}, got {got:#04X}"
        );
    }
}

#[test]
fn address_pin_order_is_not_symmetric() {
    // Guards the transposition specifically: swapping A0 and A1 must change
    // the address for every strapping where the two pins differ.
    for (a1, a0, _) in DATASHEET_TABLE_6_1 {
        if a0 == a1 {
            continue;
        }
        let straight = Address::from_pins(AddressPins { a0, a1 });
        let swapped = Address::from_pins(AddressPins { a0: a1, a1: a0 });
        assert_ne!(straight, swapped, "A1={a1:?} A0={a0:?} survived transposition");
    }
}

#[test]
fn address_covers_exactly_the_reachable_range() {
    // All sixteen strappings, and nothing outside 0x40..=0x4F.
    let mut seen = [false; 16];
    for a0 in AddrPinState::ALL {
        for a1 in AddrPinState::ALL {
            let addr = Address::from_pins(AddressPins { a0, a1 }).as_u8();
            assert!((Address::MIN..=Address::MAX).contains(&addr));
            seen[usize::from(addr - Address::MIN)] = true;
        }
    }
    assert!(seen.iter().all(|&s| s), "not every address in 0x40..=0x4F is reachable");
}

// ── Channel masks ─────────────────────────────────────────────────────────────

#[test]
fn channel_mask_matches_datasheet_bit_layout() {
    // CONFIG1.ACTIVE_CHANNEL and CONFIG2.RANGE both put channel 1 at bit 0
    // (datasheet Tables 7-3 and 7-4).
    assert_eq!(Channel::Ch1.mask(), 0b0001);
    assert_eq!(Channel::Ch2.mask(), 0b0010);
    assert_eq!(Channel::Ch3.mask(), 0b0100);
    assert_eq!(Channel::Ch4.mask(), 0b1000);
}

#[test]
fn channel_index_matches_register_stride() {
    for (i, ch) in Channel::ALL.iter().enumerate() {
        assert_eq!(ch.index(), i);
    }
}

#[test]
fn set_channel_bit_exhaustive() {
    // 256 masks x 4 channels x {set, clear}: walk all 2048.
    for mask in 0u8..=255 {
        for ch in Channel::ALL {
            let set = set_channel_bit(mask, ch, true);
            let cleared = set_channel_bit(mask, ch, false);

            assert!(channel_bit(set, ch));
            assert!(!channel_bit(cleared, ch));

            // Only the target bit may move.
            assert_eq!(set & !ch.mask(), mask & !ch.mask());
            assert_eq!(cleared & !ch.mask(), mask & !ch.mask());

            // Setting then clearing returns to a canonical state.
            assert_eq!(set_channel_bit(set, ch, false), cleared);
            assert_eq!(set_channel_bit(cleared, ch, true), set);
        }
    }
}

// ── Calibration ───────────────────────────────────────────────────────────────

#[test]
fn calibration_matches_datasheet_worked_example() {
    // Datasheet §8.2.2.3: CURRENT_LSB = 500 µA, R_SHUNT = 8.0 mΩ
    // gives SHUNT_CAL = 1280d (0x500).
    let cal = datasheet_example_calibration();
    assert_eq!(cal.shunt_cal().as_u16(), 1280);
}

#[test]
fn calibration_matches_known_vectors() {
    // Vectors that the pre-refactor mock tests asserted on the wire.
    let cases = [
        (100_000u32, 10_000u32, AdcRange::Range0, 5120u16),
        (200_000, 20_000, AdcRange::Range0, 1280),
        (50_000, 5_000, AdcRange::Range0, 20480),
    ];
    for (lsb_na, r_uohm, range, expected) in cases {
        let cal = Calibration::new(
            CurrentLsb::from_nanoamps(lsb_na).unwrap(),
            ShuntResistance::from_microohms(r_uohm).unwrap(),
            range,
        )
        .unwrap();
        assert_eq!(cal.shunt_cal().as_u16(), expected, "lsb={lsb_na}nA r={r_uohm}uOhm");
    }
}

#[test]
fn calibration_range1_divides_by_four() {
    // Datasheet §8.1.2: SHUNT_CAL is divided by 4 for ADCRANGE = 1.
    let lsb = CurrentLsb::from_nanoamps(100_000).unwrap();
    let shunt = ShuntResistance::from_microohms(10_000).unwrap();
    let r0 = Calibration::new(lsb, shunt, AdcRange::Range0).unwrap();
    let r1 = Calibration::new(lsb, shunt, AdcRange::Range1).unwrap();
    assert_eq!(r0.shunt_cal().as_u16(), 5120);
    assert_eq!(r1.shunt_cal().as_u16(), 1280);
}

#[test]
fn calibration_rejects_degenerate_inputs() {
    assert_eq!(
        CurrentLsb::from_nanoamps(0),
        Err(CalibrationError::CurrentLsbOutOfRange)
    );
    assert_eq!(
        CurrentLsb::from_nanoamps(CurrentLsb::MAX_NA + 1),
        Err(CalibrationError::CurrentLsbOutOfRange)
    );
    assert_eq!(
        ShuntResistance::from_microohms(0),
        Err(CalibrationError::ShuntResistanceZero)
    );
}

#[test]
fn calibration_reports_out_of_range_instead_of_clamping() {
    let big_shunt = ShuntResistance::from_microohms(1_000_000).unwrap();
    let small_shunt = ShuntResistance::from_microohms(1).unwrap();

    // Overflow: tiny LSB with a small shunt wants SHUNT_CAL far above 32767.
    assert_eq!(
        Calibration::new(CurrentLsb::from_nanoamps(1).unwrap(), small_shunt, AdcRange::Range0),
        Err(CalibrationError::ShuntCalOverflow)
    );

    // Underflow: a huge LSB with a large shunt rounds SHUNT_CAL to zero, which
    // would make the device report zero current forever.
    assert_eq!(
        Calibration::new(
            CurrentLsb::from_nanoamps(CurrentLsb::MAX_NA).unwrap(),
            big_shunt,
            AdcRange::Range0
        ),
        Err(CalibrationError::ShuntCalUnderflow)
    );
}

#[test]
fn current_lsb_min_for_max_current_matches_datasheet() {
    // Datasheet §8.2.2.3: IMAX = 10 A gives CURRENT_LSB(min) = 305.17578 µA.
    // Rounded up to whole nanoamps: 305_176 nA.
    let lsb = CurrentLsb::min_for_max_current(10_000_000_000).unwrap();
    assert_eq!(lsb.as_nanoamps(), 305_176);

    // Equation 2 divides by 2^15, but the CURRENT register saturates at 32767,
    // so the datasheet minimum lands just under the requested full scale. This
    // pins that gap so it stays documented rather than surprising.
    let reachable = i64::from(lsb.as_nanoamps()) * 32_767;
    assert!(reachable < 10_000_000_000);
    assert!(reachable > 9_999_000_000, "gap should be well under 0.1%");

    // Rounding up to a convenient value, as the datasheet example does,
    // comfortably covers the full scale.
    let chosen = CurrentLsb::from_nanoamps(500_000).unwrap();
    assert!(i64::from(chosen.as_nanoamps()) * 32_767 > 10_000_000_000);
}

// ── Decoding: datasheet vectors ───────────────────────────────────────────────

#[test]
fn decode_matches_datasheet_table_8_3() {
    // Datasheet Table 8-3, the register contents and calculated values for the
    // worked design example.
    let cal = datasheet_example_calibration();

    // Shunt_Voltage_CH1 = 19200d, LSB 2.5 µV -> 0.048 V
    let shunt = decode_shunt_voltage(19_200, AdcRange::Range0);
    assert_eq!(shunt.as_nanovolts(), 48_000_000);
    assert!((shunt.to_millivolts() - 48.0).abs() < 1e-3);

    // Bus_Voltage_CH1 = 7500d, LSB 1.6 mV -> 12 V
    let bus = decode_bus_voltage(7_500);
    assert_eq!(bus.as_microvolts(), 12_000_000);
    assert!((bus.to_volts() - 12.0).abs() < 1e-3);

    // Current_CH1 = 12000d, LSB 500 µA -> 6 A
    let current = decode_current(12_000, cal);
    assert_eq!(current.as_nanoamps(), 6_000_000_000);
    assert!((current.to_amps() - 6.0).abs() < 1e-3);

    // Power_CH1 = 4500d, LSB 16 mW -> 72 W
    let power = decode_power(4_500, cal);
    assert_eq!(power.as_nanowatts(), 72_000_000_000);
    assert!((power.to_watts() - 72.0).abs() < 1e-2);

    // Energy_CH1 = 16200000d, LSB 16 mJ -> 259.2 kJ
    let energy = decode_energy(16_200_000, cal);
    assert_eq!(energy.as_nanojoules(), 259_200_000_000_000);
}

#[test]
fn decode_shunt_voltage_range1_lsb() {
    // Datasheet §7.4: 625 nV/LSB on the ±20.48 mV range.
    assert_eq!(decode_shunt_voltage(1, AdcRange::Range1).as_nanovolts(), 625);
    assert_eq!(decode_shunt_voltage(1, AdcRange::Range0).as_nanovolts(), 2_500);
}

// ── Decoding: exhaustive ──────────────────────────────────────────────────────

#[test]
fn decode_bus_voltage_exhaustive() {
    // All 65,536 register values.
    let mut previous = None;
    for raw in u16::MIN..=u16::MAX {
        let uv = decode_bus_voltage(raw).as_microvolts();

        // Exact multiple of the 1.6 mV LSB.
        assert_eq!(uv % 1_600, 0);
        assert_eq!(uv / 1_600, u32::from(raw));

        // Strictly increasing, and therefore never wrapping.
        if let Some(prev) = previous {
            assert!(uv > prev);
        }
        previous = Some(uv);
    }

    // The extremes match the datasheet's usable range: 0 V up past 52.4 V.
    assert_eq!(decode_bus_voltage(0).as_microvolts(), 0);
    assert_eq!(decode_bus_voltage(u16::MAX).as_microvolts(), 104_856_000);
}

#[test]
fn decode_shunt_voltage_exhaustive() {
    // All 65,536 register values on both ranges: 131,072 cases.
    for range in AdcRange::ALL {
        let lsb = i64::from(range.shunt_lsb_nv());
        let mut previous = None;
        for raw in i16::MIN..=i16::MAX {
            let nv = decode_shunt_voltage(raw, range).as_nanovolts();

            // Exact, and sign-preserving.
            assert_eq!(i64::from(nv), i64::from(raw) * lsb);
            assert_eq!(nv.signum(), raw.signum().into());

            if let Some(prev) = previous {
                assert!(nv > prev);
            }
            previous = Some(nv);
        }
    }

    // Two's complement is lopsided: one more negative code than positive.
    assert_eq!(
        decode_shunt_voltage(i16::MIN, AdcRange::Range0).as_nanovolts(),
        -81_920_000
    );
    assert_eq!(
        decode_shunt_voltage(i16::MAX, AdcRange::Range0).as_nanovolts(),
        81_917_500
    );
}

// ── Properties ────────────────────────────────────────────────────────────────

prop_compose! {
    fn any_calibration()(
        lsb in 1u32..=CurrentLsb::MAX_NA,
        shunt in 1u32..=1_000_000u32,
        range in prop::sample::select(&AdcRange::ALL[..]),
    ) -> Result<Calibration, CalibrationError> {
        Calibration::new(
            CurrentLsb::from_nanoamps(lsb).unwrap(),
            ShuntResistance::from_microohms(shunt).unwrap(),
            range,
        )
    }
}

proptest! {
    /// Calibration either succeeds or reports why. It never panics, never
    /// divides by zero, and never silently clamps.
    #[test]
    fn calibration_never_panics(cal in any_calibration()) {
        let _ = cal;
    }

    /// When calibration succeeds, `SHUNT_CAL` is inside the usable range of
    /// the 15-bit field. Zero is excluded because the device reports zero
    /// current for a zero `SHUNT_CAL`.
    #[test]
    fn shunt_cal_is_always_usable(cal in any_calibration()) {
        if let Ok(cal) = cal {
            let v = cal.shunt_cal().as_u16();
            prop_assert!(v >= 1);
            prop_assert!(v <= ShuntCal::MAX);
        }
    }

    /// The only failures are the two range conditions. Any future edit that
    /// makes calibration reject something else breaks this immediately.
    #[test]
    fn calibration_fails_only_on_range(
        lsb in 1u32..=CurrentLsb::MAX_NA,
        shunt in 1u32..=1_000_000u32,
        range in prop::sample::select(&AdcRange::ALL[..]),
    ) {
        const NUMERATOR: u128 = 5_120_000_000_000;
        let divisor = u128::from(lsb) * u128::from(shunt) * u128::from(range.shunt_cal_divisor());
        let ideal = (NUMERATOR + divisor / 2) / divisor;

        let got = Calibration::new(
            CurrentLsb::from_nanoamps(lsb).unwrap(),
            ShuntResistance::from_microohms(shunt).unwrap(),
            range,
        );

        match got {
            Ok(cal) => {
                prop_assert_eq!(u128::from(cal.shunt_cal().as_u16()), ideal);
            }
            Err(CalibrationError::ShuntCalUnderflow) => prop_assert_eq!(ideal, 0),
            Err(CalibrationError::ShuntCalOverflow) => {
                prop_assert!(ideal > u128::from(ShuntCal::MAX));
            }
            Err(other) => prop_assert!(false, "unexpected error {:?}", other),
        }
    }

    /// Rounding is to nearest, so the programmed value is never more than half
    /// an LSB from the ideal ratio. Truncation would fail this on one side.
    #[test]
    fn shunt_cal_rounds_to_nearest(
        lsb in 1u32..=CurrentLsb::MAX_NA,
        shunt in 1u32..=1_000_000u32,
    ) {
        let got = Calibration::new(
            CurrentLsb::from_nanoamps(lsb).unwrap(),
            ShuntResistance::from_microohms(shunt).unwrap(),
            AdcRange::Range0,
        );
        if let Ok(cal) = got {
            let divisor = u128::from(lsb) * u128::from(shunt);
            let scaled = u128::from(cal.shunt_cal().as_u16()) * divisor;
            let ideal = 5_120_000_000_000u128;
            let err = scaled.abs_diff(ideal);
            prop_assert!(err * 2 <= divisor, "rounding error exceeds half an LSB");
        }
    }

    /// Decoding never panics for any register value the part can return.
    #[test]
    fn decode_never_panics(
        shunt_raw: i16,
        bus_raw: u16,
        current_raw: i16,
        power_raw: u16,
        energy_raw: u32,
        range in prop::sample::select(&AdcRange::ALL[..]),
    ) {
        let cal = datasheet_example_calibration();
        let _ = decode_shunt_voltage(shunt_raw, range);
        let _ = decode_bus_voltage(bus_raw);
        let _ = decode_current(current_raw, cal);
        let _ = decode_power(power_raw, cal);
        let _ = decode_energy(energy_raw, cal);
    }

    /// Current decoding preserves sign and magnitude ordering.
    #[test]
    fn decode_current_is_monotonic(a: i16, b: i16) {
        let cal = datasheet_example_calibration();
        let da = decode_current(a, cal).as_nanoamps();
        let db = decode_current(b, cal).as_nanoamps();
        prop_assert_eq!(a.cmp(&b), da.cmp(&db));
    }

    /// Power and energy are unsigned and scale linearly with the raw value.
    #[test]
    fn decode_power_energy_scale_linearly(raw: u16, energy_raw: u32) {
        let cal = datasheet_example_calibration();
        let lsb = u64::from(cal.current_lsb().as_nanoamps());
        prop_assert_eq!(decode_power(raw, cal).as_nanowatts(), u64::from(raw) * 32 * lsb);
        prop_assert_eq!(
            decode_energy(energy_raw, cal).as_nanojoules(),
            u64::from(energy_raw) * 32 * lsb
        );
    }

    /// Every strapping produces an address in the reachable range, and the
    /// mapping is injective.
    #[test]
    fn address_is_injective(
        a0 in prop::sample::select(&AddrPinState::ALL[..]),
        a1 in prop::sample::select(&AddrPinState::ALL[..]),
        b0 in prop::sample::select(&AddrPinState::ALL[..]),
        b1 in prop::sample::select(&AddrPinState::ALL[..]),
    ) {
        let a = Address::from_pins(AddressPins { a0, a1 });
        let b = Address::from_pins(AddressPins { a0: b0, a1: b1 });
        prop_assert_eq!(a == b, (a0, a1) == (b0, b1));
    }
}

// ── Alert slots ───────────────────────────────────────────────────────────────

#[test]
fn alert_slot_indexes_are_zero_based_and_ordered() {
    assert_eq!(AlertSlot::ALL.len(), 4);
    for (i, slot) in AlertSlot::ALL.into_iter().enumerate() {
        assert_eq!(slot.index(), i);
    }
    assert_eq!(AlertSlot::One.index(), 0);
    assert_eq!(AlertSlot::Four.index(), 3);
}

// ── Measurement construction ──────────────────────────────────────────────────

#[test]
fn measurement_types_are_constructible_by_callers() {
    assert_eq!(ShuntVoltage::from_nanovolts(-80_000_000).as_nanovolts(), -80_000_000);
    assert_eq!(BusVoltage::from_microvolts(12_000_000).as_microvolts(), 12_000_000);
    assert_eq!(Current::from_nanoamps(6_000_000_000).as_nanoamps(), 6_000_000_000);
    assert_eq!(Power::from_nanowatts(72_000_000_000).as_nanowatts(), 72_000_000_000);
    assert_eq!(
        Energy::from_nanojoules(259_200_000_000_000).as_nanojoules(),
        259_200_000_000_000
    );
}
