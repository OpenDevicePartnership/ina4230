//! Tests for the imperative shell.
//!
//! The shell has no arithmetic in it, so these do not re-test conversions —
//! that is what `core_tests` is for, and it does it without a bus. What is
//! worth checking here is the wiring: that the right register is addressed on
//! the right channel, and that the driver's cached state stays consistent with
//! the device.

use embedded_hal::i2c::ErrorKind;
use embedded_hal_mock::eh1::i2c::{Mock, Transaction};

use ina4230::{
    AdcRange, AddrPinState, AddressPins, AlertSlot, Calibration, Channel, CurrentLsb, CurrentSensor, EnergySensor,
    Ina4230, Ina4230Error, PowerSensor, ShuntResistance, VoltageSensor,
};

/// Address for the default strapping, A0 = A1 = GND.
const ADDR: u8 = 0x40;

fn pins() -> AddressPins {
    AddressPins {
        a0: AddrPinState::Gnd,
        a1: AddrPinState::Gnd,
    }
}

/// The calibration from the datasheet's worked example: 500 µA/LSB, 8 mΩ,
/// giving `SHUNT_CAL` = 1280.
fn example_cal() -> Calibration {
    Calibration::new(
        CurrentLsb::from_nanoamps(500_000).unwrap(),
        ShuntResistance::from_microohms(8_000).unwrap(),
        AdcRange::Range0,
    )
    .unwrap()
}

fn sensor(expectations: &[Transaction]) -> Ina4230<Mock> {
    Ina4230::new(Mock::new(expectations), pins())
}

// ── Addressing ────────────────────────────────────────────────────────────────

#[test]
fn new_uses_the_strapped_address() {
    let dev = Ina4230::new(Mock::new(&[]), pins());
    assert_eq!(dev.address().as_u8(), ADDR);
    dev.release().done();

    // A transposition-sensitive strapping: A1=SDA, A0=GND is 0x48, while the
    // transposed A1=GND, A0=SDA is 0x42.
    let dev = Ina4230::new(
        Mock::new(&[]),
        AddressPins {
            a0: AddrPinState::Gnd,
            a1: AddrPinState::Sda,
        },
    );
    assert_eq!(dev.address().as_u8(), 0x48);
    dev.release().done();
}

#[tokio::test]
async fn manufacturer_id_reads_expected_register() {
    let mut dev = sensor(&[Transaction::write_read(ADDR, vec![0x7E], vec![0x54, 0x49])]);
    assert_eq!(dev.manufacturer_id().await.unwrap(), Ina4230::<Mock>::MANUFACTURER_ID);
    dev.release().done();
}

// ── Per-channel register striding ─────────────────────────────────────────────

#[tokio::test]
async fn bus_voltage_addresses_every_channel() {
    // Datasheet Table 7-1: 0x01, 0x09, 0x11, 0x19.
    let raw: u16 = 7_500; // 12 V in the worked example
    let [hi, lo] = raw.to_be_bytes();
    let expectations: Vec<_> = [0x01u8, 0x09, 0x11, 0x19]
        .iter()
        .map(|&reg| Transaction::write_read(ADDR, vec![reg], vec![hi, lo]))
        .collect();

    let mut dev = sensor(&expectations);
    for ch in Channel::ALL {
        let v = dev.bus_voltage(ch).await.unwrap();
        assert_eq!(v.as_microvolts(), 12_000_000);
    }
    dev.release().done();
}

#[tokio::test]
async fn measurement_registers_use_the_right_offsets() {
    // Channel 3 bank starts at 0x10: shunt 0x10, current 0x12, power 0x13,
    // energy 0x14.
    let cal = example_cal();
    let expectations = vec![
        // calibrate: CONFIG2 read-modify-write, then SHUNT_CAL at 0x15
        Transaction::write_read(ADDR, vec![0x21], vec![0x00, 0x00]),
        Transaction::write(ADDR, vec![0x21, 0x00, 0x00]),
        Transaction::write(ADDR, vec![0x15, 0x05, 0x00]),
        // shunt voltage 0x10
        Transaction::write_read(ADDR, vec![0x10], vec![0x4B, 0x00]),
        // current 0x12
        Transaction::write_read(ADDR, vec![0x12], vec![0x2E, 0xE0]),
        // power 0x13
        Transaction::write_read(ADDR, vec![0x13], vec![0x11, 0x94]),
        // energy 0x14
        Transaction::write_read(ADDR, vec![0x14], vec![0x00, 0xF7, 0x31, 0x40]),
    ];

    let mut dev = sensor(&expectations);
    dev.calibrate(Channel::Ch3, cal).await.unwrap();

    // Values are the datasheet Table 8-3 vectors.
    assert_eq!(
        dev.shunt_voltage(Channel::Ch3).await.unwrap().as_nanovolts(),
        48_000_000
    );
    assert_eq!(dev.current(Channel::Ch3).await.unwrap().as_nanoamps(), 6_000_000_000);
    assert_eq!(dev.power(Channel::Ch3).await.unwrap().as_nanowatts(), 72_000_000_000);
    assert_eq!(
        dev.energy(Channel::Ch3).await.unwrap().as_nanojoules(),
        259_200_000_000_000
    );
    dev.release().done();
}

// ── Calibration state ─────────────────────────────────────────────────────────

#[tokio::test]
async fn uncalibrated_reads_do_not_touch_the_bus() {
    // The mock is given no expectations at all: if the driver issued a
    // transaction before noticing the channel is uncalibrated, this panics.
    let mut dev = sensor(&[]);

    for ch in Channel::ALL {
        assert_eq!(dev.current(ch).await, Err(Ina4230Error::NotCalibrated(ch)));
        assert_eq!(dev.power(ch).await, Err(Ina4230Error::NotCalibrated(ch)));
        assert_eq!(dev.energy(ch).await, Err(Ina4230Error::NotCalibrated(ch)));
        assert_eq!(dev.shunt_voltage(ch).await, Err(Ina4230Error::NotCalibrated(ch)));
    }
    dev.release().done();
}

#[tokio::test]
async fn calibration_is_per_channel() {
    let expectations = vec![
        Transaction::write_read(ADDR, vec![0x21], vec![0x00, 0x00]),
        Transaction::write(ADDR, vec![0x21, 0x00, 0x00]),
        Transaction::write(ADDR, vec![0x05, 0x05, 0x00]),
    ];
    let mut dev = sensor(&expectations);
    dev.calibrate(Channel::Ch1, example_cal()).await.unwrap();

    assert_eq!(dev.calibration(Channel::Ch1), Some(example_cal()));
    for ch in [Channel::Ch2, Channel::Ch3, Channel::Ch4] {
        assert_eq!(dev.calibration(ch), None);
        assert_eq!(dev.current(ch).await, Err(Ina4230Error::NotCalibrated(ch)));
    }
    dev.release().done();
}

#[tokio::test]
async fn reset_discards_cached_calibration() {
    // Reset returns the calibration registers and CONFIG2.RANGE to defaults,
    // so keeping the cached calibration would make the driver scale readings
    // with settings the device no longer has.
    let expectations = vec![
        // calibrate CH1
        Transaction::write_read(ADDR, vec![0x21], vec![0x00, 0x00]),
        Transaction::write(ADDR, vec![0x21, 0x00, 0x00]),
        Transaction::write(ADDR, vec![0x05, 0x05, 0x00]),
        // reset: CONFIG2 with RST set
        Transaction::write(ADDR, vec![0x21, 0x80, 0x00]),
    ];
    let mut dev = sensor(&expectations);

    dev.calibrate(Channel::Ch1, example_cal()).await.unwrap();
    assert!(dev.calibration(Channel::Ch1).is_some());

    dev.reset().await.unwrap();

    assert_eq!(dev.calibration(Channel::Ch1), None);
    // And a subsequent read fails loudly instead of returning a wrong number.
    assert_eq!(
        dev.current(Channel::Ch1).await,
        Err(Ina4230Error::NotCalibrated(Channel::Ch1))
    );
    dev.release().done();
}

// ── State after a failed write ────────────────────────────────────────────────
//
// I2C cannot tell you whether a device acted on a transaction that failed
// partway through, so after any failure the device's calibration state is
// unknown. The driver resolves that ambiguity towards a loud NotCalibrated
// rather than a quiet wrong number.

#[tokio::test]
async fn reset_clears_cache_even_when_the_write_fails() {
    // The device may well have reset before reporting the error. Keeping the
    // cache would leave the driver scaling against a SHUNT_CAL of zero, which
    // makes the part report exactly 0 A: a plausible reading, not an obvious
    // fault.
    let expectations = vec![
        Transaction::write_read(ADDR, vec![0x21], vec![0x00, 0x00]),
        Transaction::write(ADDR, vec![0x21, 0x00, 0x00]),
        Transaction::write(ADDR, vec![0x05, 0x05, 0x00]),
        Transaction::write(ADDR, vec![0x21, 0x80, 0x00]).with_error(ErrorKind::Other),
    ];
    let mut dev = sensor(&expectations);

    dev.calibrate(Channel::Ch1, example_cal()).await.unwrap();
    assert!(dev.calibration(Channel::Ch1).is_some());

    assert!(dev.reset().await.is_err());

    assert_eq!(dev.calibration(Channel::Ch1), None);
    assert_eq!(
        dev.current(Channel::Ch1).await,
        Err(Ina4230Error::NotCalibrated(Channel::Ch1))
    );
    dev.release().done();
}

#[tokio::test]
async fn calibrate_invalidates_the_channel_when_shunt_cal_fails() {
    // CONFIG2.RANGE lands but SHUNT_CAL does not. Retaining the previous
    // calibration would pair the device's new range with the cache's old one,
    // and the two ADC ranges differ by a factor of four.
    let range1_cal = Calibration::new(
        CurrentLsb::from_nanoamps(500_000).unwrap(),
        ShuntResistance::from_microohms(8_000).unwrap(),
        AdcRange::Range1,
    )
    .unwrap();

    let expectations = vec![
        // first calibration, Range0, succeeds
        Transaction::write_read(ADDR, vec![0x21], vec![0x00, 0x00]),
        Transaction::write(ADDR, vec![0x21, 0x00, 0x00]),
        Transaction::write(ADDR, vec![0x05, 0x05, 0x00]),
        // second calibration, Range1: the range write lands...
        Transaction::write_read(ADDR, vec![0x21], vec![0x00, 0x00]),
        Transaction::write(ADDR, vec![0x21, 0x00, 0x01]),
        // ...and SHUNT_CAL (1280 / 4 = 320 = 0x0140) fails
        Transaction::write(ADDR, vec![0x05, 0x01, 0x40]).with_error(ErrorKind::Other),
    ];
    let mut dev = sensor(&expectations);

    dev.calibrate(Channel::Ch1, example_cal()).await.unwrap();
    assert_eq!(dev.calibration(Channel::Ch1), Some(example_cal()));

    assert!(dev.calibrate(Channel::Ch1, range1_cal).await.is_err());

    // Not the old Range0 calibration, and not the new one either.
    assert_eq!(dev.calibration(Channel::Ch1), None);
    assert_eq!(
        dev.shunt_voltage(Channel::Ch1).await,
        Err(Ina4230Error::NotCalibrated(Channel::Ch1))
    );
    dev.release().done();
}

#[tokio::test]
async fn calibrate_all_invalidates_the_channels_it_did_not_reach() {
    // CONFIG2 moves all four range bits together, so a failure partway through
    // the SHUNT_CAL writes leaves the remaining channels with a device range
    // the cache knows nothing about.
    //
    // The hazard is specifically a *stale* entry surviving, so this calibrates
    // successfully first and then fails a second pass: channels 3 and 4 must
    // end up invalidated rather than holding their earlier values.
    let cal = example_cal();
    let expectations = vec![
        // first pass: all four succeed
        Transaction::write_read(ADDR, vec![0x21], vec![0x00, 0x00]),
        Transaction::write(ADDR, vec![0x21, 0x00, 0x00]),
        Transaction::write(ADDR, vec![0x05, 0x05, 0x00]),
        Transaction::write(ADDR, vec![0x0D, 0x05, 0x00]),
        Transaction::write(ADDR, vec![0x15, 0x05, 0x00]),
        Transaction::write(ADDR, vec![0x1D, 0x05, 0x00]),
        // second pass: CONFIG2 and channels 1-2 land, channel 3 fails,
        // channel 4 is never attempted
        Transaction::write_read(ADDR, vec![0x21], vec![0x00, 0x00]),
        Transaction::write(ADDR, vec![0x21, 0x00, 0x00]),
        Transaction::write(ADDR, vec![0x05, 0x05, 0x00]),
        Transaction::write(ADDR, vec![0x0D, 0x05, 0x00]),
        Transaction::write(ADDR, vec![0x15, 0x05, 0x00]).with_error(ErrorKind::Other),
    ];
    let mut dev = sensor(&expectations);

    dev.calibrate_all([cal; 4]).await.unwrap();
    for ch in Channel::ALL {
        assert_eq!(dev.calibration(ch), Some(cal));
    }

    assert!(dev.calibrate_all([cal; 4]).await.is_err());

    // The two that completed are usable; the two that did not must have been
    // invalidated, not left holding the first pass's values.
    assert_eq!(dev.calibration(Channel::Ch1), Some(cal));
    assert_eq!(dev.calibration(Channel::Ch2), Some(cal));
    assert_eq!(dev.calibration(Channel::Ch3), None);
    assert_eq!(dev.calibration(Channel::Ch4), None);

    assert_eq!(
        dev.current(Channel::Ch3).await,
        Err(Ina4230Error::NotCalibrated(Channel::Ch3))
    );
    dev.release().done();
}

#[tokio::test]
async fn range1_sets_the_channel_range_bit() {
    let cal = Calibration::new(
        CurrentLsb::from_nanoamps(500_000).unwrap(),
        ShuntResistance::from_microohms(8_000).unwrap(),
        AdcRange::Range1,
    )
    .unwrap();
    // SHUNT_CAL is divided by 4 for Range1: 1280 / 4 = 320 = 0x0140.
    assert_eq!(cal.shunt_cal().as_u16(), 320);

    let expectations = vec![
        Transaction::write_read(ADDR, vec![0x21], vec![0x00, 0x00]),
        // CONFIG2.RANGE bit 1 set for channel 2
        Transaction::write(ADDR, vec![0x21, 0x00, 0x02]),
        Transaction::write(ADDR, vec![0x0D, 0x01, 0x40]),
    ];
    let mut dev = sensor(&expectations);
    dev.calibrate(Channel::Ch2, cal).await.unwrap();

    // The shunt LSB follows the calibrated range: 625 nV rather than 2500 nV.
    dev.release().done();
}

#[tokio::test]
async fn calibrate_all_writes_config2_once() {
    // Four separate read-modify-writes would be four times the traffic and
    // three extra non-atomic windows; all four range bits are known up front.
    let cal = example_cal();
    let expectations = vec![
        Transaction::write_read(ADDR, vec![0x21], vec![0x00, 0x00]),
        Transaction::write(ADDR, vec![0x21, 0x00, 0x00]),
        Transaction::write(ADDR, vec![0x05, 0x05, 0x00]),
        Transaction::write(ADDR, vec![0x0D, 0x05, 0x00]),
        Transaction::write(ADDR, vec![0x15, 0x05, 0x00]),
        Transaction::write(ADDR, vec![0x1D, 0x05, 0x00]),
    ];
    let mut dev = sensor(&expectations);
    dev.calibrate_all([cal; 4]).await.unwrap();

    for ch in Channel::ALL {
        assert_eq!(dev.calibration(ch), Some(cal));
    }
    dev.release().done();
}

// ── Channel enable ────────────────────────────────────────────────────────────

#[tokio::test]
async fn set_channel_active_touches_only_its_own_bit() {
    // CONFIG1 reset is 0xF127: all four channels active.
    let expectations = vec![
        Transaction::write_read(ADDR, vec![0x20], vec![0xF1, 0x27]),
        // Clearing channel 3 clears bit 14 -> 0xB127
        Transaction::write(ADDR, vec![0x20, 0xB1, 0x27]),
    ];
    let mut dev = sensor(&expectations);
    dev.set_channel_active(Channel::Ch3, false).await.unwrap();
    dev.release().done();
}

// ── Flags ─────────────────────────────────────────────────────────────────────

#[tokio::test]
async fn read_flags_reports_every_condition_at_once() {
    // OVF (bit 6), CVRF (bit 7), ENERGYOF_CH2 (bit 9), LIMIT1 (bit 12).
    let bits: u16 = (1 << 6) | (1 << 7) | (1 << 9) | (1 << 12);
    let [hi, lo] = bits.to_be_bytes();
    let mut dev = sensor(&[Transaction::write_read(ADDR, vec![0x22], vec![hi, lo])]);

    let flags = dev.read_flags().await.unwrap();

    // The old check_flags() returned only the first condition it found, and
    // since the read clears the register the rest were lost for good.
    assert!(flags.math_overflow());
    assert!(flags.conversion_ready());
    assert!(flags.energy_overflow(Channel::Ch2));
    assert!(!flags.energy_overflow(Channel::Ch1));
    assert!(flags.any_energy_overflow());
    assert_eq!(flags.limit_alerts(), [true, false, false, false]);

    dev.release().done();
}

#[test]
fn limit_out_of_range_reports_the_slot() {
    let e: Ina4230Error<ErrorKind> = Ina4230Error::LimitOutOfRange(AlertSlot::Two);
    assert_eq!(e, Ina4230Error::LimitOutOfRange(AlertSlot::Two));
    assert_ne!(e, Ina4230Error::LimitOutOfRange(AlertSlot::Three));
}
