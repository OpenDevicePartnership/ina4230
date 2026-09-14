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
    AdcRange, AddrPinState, AddressPins, Alert, AlertLatch, AlertPinConfig, AlertPolarity, AlertSlot, Averaging,
    BusConversionTime, BusVoltage, Calibration, Channel, ConversionTiming, CurrentLsb, CurrentSensor, EnergySensor,
    Ina4230, Ina4230Error, OperatingMode, Power, PowerSensor, ShuntConversionTime, ShuntResistance, ShuntVoltage,
    VoltageSensor,
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
async fn a_failed_reset_keeps_the_alert_cache_so_a_later_calibrate_still_disarms() {
    // The alert cache resolves the ambiguity the other way from the
    // calibration cache. If the RST write did not land, the device still has
    // slots armed; dropping the cache would leave the next calibrate with
    // nothing to disarm, and CONFIG2.RANGE would move out from under a live
    // shunt threshold.
    let cal = example_cal();
    let mut dev = sensor(&[
        // calibrate Ch1
        Transaction::write_read(ADDR, vec![0x21], vec![0x00, 0x00]),
        Transaction::write(ADDR, vec![0x21, 0x00, 0x00]),
        Transaction::write(ADDR, vec![0x05, 0x05, 0x00]),
        // arm slot 1 with a shunt alert on Ch1: 1 mV / 2.5 uV = 400 = 0x0190
        Transaction::write(ADDR, vec![0x06, 0x01, 0x90]),
        Transaction::write(ADDR, vec![0x07, 0x00, 0x01]),
        // the reset fails
        Transaction::write(ADDR, vec![0x21, 0x80, 0x00]).with_error(ErrorKind::Other),
        // recalibrating must still disarm slot 1 BEFORE touching CONFIG2
        Transaction::write(ADDR, vec![0x07, 0x00, 0x00]),
        Transaction::write_read(ADDR, vec![0x21], vec![0x00, 0x00]),
        Transaction::write(ADDR, vec![0x21, 0x00, 0x00]),
        Transaction::write(ADDR, vec![0x05, 0x05, 0x00]),
    ]);

    dev.calibrate(Channel::Ch1, cal).await.unwrap();
    let shunt = Alert::ShuntOver(ShuntVoltage::from_nanovolts(1_000_000));
    dev.set_alert(AlertSlot::One, Channel::Ch1, shunt).await.unwrap();

    assert!(dev.reset().await.is_err());

    assert_eq!(
        dev.alert(AlertSlot::One),
        Some((Channel::Ch1, shunt)),
        "the reset may not have landed, so the slot may still be armed"
    );

    dev.calibrate(Channel::Ch1, cal).await.unwrap();
    assert_eq!(dev.alert(AlertSlot::One), None);
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

#[tokio::test]
async fn alert_slots_start_empty_and_cost_no_bus_traffic_to_read() {
    let dev = sensor(&[]);
    for slot in AlertSlot::ALL {
        assert_eq!(dev.alert(slot), None);
    }
    dev.release().done();
}

#[tokio::test]
async fn set_alert_addresses_every_slot_and_encodes_the_config() {
    // ALERT_LIMIT 0x06/0x0E/0x16/0x1E, ALERT_CONFIG 0x07/0x0F/0x17/0x1F.
    // 12 V / 1.6 mV = 7500 = 0x1D4C. Watching Ch3 => CHANNEL = 0b10,
    // BusOver => ALERT_MASK = 3, so ALERT_CONFIG = 0b10_011 = 0x13.
    let expectations = vec![
        Transaction::write(ADDR, vec![0x06, 0x1D, 0x4C]),
        Transaction::write(ADDR, vec![0x07, 0x00, 0x13]),
        Transaction::write(ADDR, vec![0x0E, 0x1D, 0x4C]),
        Transaction::write(ADDR, vec![0x0F, 0x00, 0x13]),
        Transaction::write(ADDR, vec![0x16, 0x1D, 0x4C]),
        Transaction::write(ADDR, vec![0x17, 0x00, 0x13]),
        Transaction::write(ADDR, vec![0x1E, 0x1D, 0x4C]),
        Transaction::write(ADDR, vec![0x1F, 0x00, 0x13]),
    ];
    let mut dev = sensor(&expectations);
    let alert = Alert::BusOver(BusVoltage::from_microvolts(12_000_000));

    for slot in AlertSlot::ALL {
        dev.set_alert(slot, Channel::Ch3, alert).await.unwrap();
        assert_eq!(dev.alert(slot), Some((Channel::Ch3, alert)));
    }
    dev.release().done();
}

#[tokio::test]
async fn alert_config_encodes_each_target_channel() {
    // Same slot, four different target channels. CHANNEL occupies bits 4:3.
    // Each iteration after the first reprograms an armed slot, so it is
    // disarmed first rather than left holding a limit against the old channel.
    let expectations = vec![
        Transaction::write(ADDR, vec![0x06, 0x1D, 0x4C]),
        Transaction::write(ADDR, vec![0x07, 0x00, 0x03]), // Ch1 => 0b00_011
        Transaction::write(ADDR, vec![0x07, 0x00, 0x00]),
        Transaction::write(ADDR, vec![0x06, 0x1D, 0x4C]),
        Transaction::write(ADDR, vec![0x07, 0x00, 0x0B]), // Ch2 => 0b01_011
        Transaction::write(ADDR, vec![0x07, 0x00, 0x00]),
        Transaction::write(ADDR, vec![0x06, 0x1D, 0x4C]),
        Transaction::write(ADDR, vec![0x07, 0x00, 0x13]), // Ch3 => 0b10_011
        Transaction::write(ADDR, vec![0x07, 0x00, 0x00]),
        Transaction::write(ADDR, vec![0x06, 0x1D, 0x4C]),
        Transaction::write(ADDR, vec![0x07, 0x00, 0x1B]), // Ch4 => 0b11_011
    ];
    let mut dev = sensor(&expectations);
    let alert = Alert::BusOver(BusVoltage::from_microvolts(12_000_000));

    for ch in Channel::ALL {
        dev.set_alert(AlertSlot::One, ch, alert).await.unwrap();
    }
    dev.release().done();
}

#[tokio::test]
async fn shunt_and_power_alerts_on_an_uncalibrated_channel_do_not_touch_the_bus() {
    // No expectations: any transaction panics.
    let mut dev = sensor(&[]);
    let ch = Channel::Ch1;
    let v = ShuntVoltage::from_nanovolts(1_000_000);
    let p = Power::from_nanowatts(1_000_000_000);

    assert_eq!(
        dev.set_alert(AlertSlot::One, ch, Alert::ShuntOver(v)).await,
        Err(Ina4230Error::NotCalibrated(ch))
    );
    assert_eq!(
        dev.set_alert(AlertSlot::One, ch, Alert::PowerOver(p)).await,
        Err(Ina4230Error::NotCalibrated(ch))
    );
    assert_eq!(dev.alert(AlertSlot::One), None);
    dev.release().done();
}

#[tokio::test]
async fn an_unrepresentable_threshold_does_not_touch_the_bus() {
    let mut dev = sensor(&[]);
    let too_big = BusVoltage::from_microvolts(60_000_000); // > 52.4272 V
    assert_eq!(
        dev.set_alert(AlertSlot::Four, Channel::Ch1, Alert::BusOver(too_big))
            .await,
        Err(Ina4230Error::LimitOutOfRange(AlertSlot::Four))
    );
    assert_eq!(dev.alert(AlertSlot::Four), None);
    dev.release().done();
}

#[tokio::test]
async fn clear_alert_zeroes_the_mask_and_drops_the_cache_entry() {
    let mut dev = sensor(&[
        Transaction::write(ADDR, vec![0x0E, 0x1D, 0x4C]),
        Transaction::write(ADDR, vec![0x0F, 0x00, 0x13]),
        Transaction::write(ADDR, vec![0x0F, 0x00, 0x00]),
    ]);
    let alert = Alert::BusOver(BusVoltage::from_microvolts(12_000_000));
    dev.set_alert(AlertSlot::Two, Channel::Ch3, alert).await.unwrap();
    dev.clear_alert(AlertSlot::Two).await.unwrap();
    assert_eq!(dev.alert(AlertSlot::Two), None);
    dev.release().done();
}

#[tokio::test]
async fn reprogramming_an_armed_slot_disarms_it_first() {
    // Slot 1 is armed as ShuntOver on Ch1 (limit 400, mask 1) and is
    // reprogrammed to BusOver (limit 7500, mask 3). If only the limit landed,
    // the device would read 7500 as a *shunt* threshold: 7500 * 2.5 uV =
    // 18.75 mV, 18x weaker than the 1 mV the caller asked for. So the mask is
    // cleared before the new limit goes in.
    let cal = example_cal();
    let mut dev = sensor(&[
        // calibrate Ch1
        Transaction::write_read(ADDR, vec![0x21], vec![0x00, 0x00]),
        Transaction::write(ADDR, vec![0x21, 0x00, 0x00]),
        Transaction::write(ADDR, vec![0x05, 0x05, 0x00]),
        // arm slot 1: 1 mV / 2.5 uV = 400 = 0x0190, Ch1 + SOL => 0b00_001
        Transaction::write(ADDR, vec![0x06, 0x01, 0x90]),
        Transaction::write(ADDR, vec![0x07, 0x00, 0x01]),
        // reprogram to BusOver: the disarm comes first
        Transaction::write(ADDR, vec![0x07, 0x00, 0x00]),
        // 12 V / 1.6 mV = 7500 = 0x1D4C
        Transaction::write(ADDR, vec![0x06, 0x1D, 0x4C]),
        // and the new config write fails
        Transaction::write(ADDR, vec![0x07, 0x00, 0x03]).with_error(ErrorKind::Other),
    ]);

    dev.calibrate(Channel::Ch1, cal).await.unwrap();
    let shunt = Alert::ShuntOver(ShuntVoltage::from_nanovolts(1_000_000));
    dev.set_alert(AlertSlot::One, Channel::Ch1, shunt).await.unwrap();

    let bus = Alert::BusOver(BusVoltage::from_microvolts(12_000_000));
    assert!(dev.set_alert(AlertSlot::One, Channel::Ch1, bus).await.is_err());

    assert_eq!(
        dev.alert(AlertSlot::One),
        None,
        "the slot was disarmed and never re-armed, so no stale configuration may remain"
    );
    dev.release().done();
}

#[tokio::test]
async fn reprogramming_a_disarmed_slot_costs_no_extra_write() {
    // The disarm is paid for only when the slot is actually armed.
    let mut dev = sensor(&[
        Transaction::write(ADDR, vec![0x06, 0x1D, 0x4C]),
        Transaction::write(ADDR, vec![0x07, 0x00, 0x03]),
        Transaction::write(ADDR, vec![0x07, 0x00, 0x00]),
        Transaction::write(ADDR, vec![0x06, 0x1D, 0x4C]),
        Transaction::write(ADDR, vec![0x07, 0x00, 0x03]),
    ]);
    let bus = Alert::BusOver(BusVoltage::from_microvolts(12_000_000));
    dev.set_alert(AlertSlot::One, Channel::Ch1, bus).await.unwrap();
    dev.clear_alert(AlertSlot::One).await.unwrap();
    dev.set_alert(AlertSlot::One, Channel::Ch1, bus).await.unwrap();
    dev.release().done();
}

#[tokio::test]
async fn recalibration_disarms_shunt_alerts_and_leaves_bus_alerts_armed() {
    let cal = example_cal();
    let mut dev = sensor(&[
        // calibrate Ch1 the first time
        Transaction::write_read(ADDR, vec![0x21], vec![0x00, 0x00]),
        Transaction::write(ADDR, vec![0x21, 0x00, 0x00]),
        Transaction::write(ADDR, vec![0x05, 0x05, 0x00]),
        // arm slot 1 with a shunt alert on Ch1: 1 mV / 2.5 uV = 400 = 0x0190
        Transaction::write(ADDR, vec![0x06, 0x01, 0x90]),
        Transaction::write(ADDR, vec![0x07, 0x00, 0x01]),
        // arm slot 2 with a bus alert on Ch1: CHANNEL = 0b00, BOL = 3
        Transaction::write(ADDR, vec![0x0E, 0x1D, 0x4C]),
        Transaction::write(ADDR, vec![0x0F, 0x00, 0x03]),
        // recalibrate Ch1: slot 1 is disarmed FIRST, slot 2 is not touched
        Transaction::write(ADDR, vec![0x07, 0x00, 0x00]),
        Transaction::write_read(ADDR, vec![0x21], vec![0x00, 0x00]),
        Transaction::write(ADDR, vec![0x21, 0x00, 0x00]),
        Transaction::write(ADDR, vec![0x05, 0x05, 0x00]),
    ]);

    dev.calibrate(Channel::Ch1, cal).await.unwrap();

    let shunt = Alert::ShuntOver(ShuntVoltage::from_nanovolts(1_000_000));
    let bus = Alert::BusOver(BusVoltage::from_microvolts(12_000_000));
    dev.set_alert(AlertSlot::One, Channel::Ch1, shunt).await.unwrap();
    dev.set_alert(AlertSlot::Two, Channel::Ch1, bus).await.unwrap();

    dev.calibrate(Channel::Ch1, cal).await.unwrap();

    assert_eq!(dev.alert(AlertSlot::One), None, "shunt alert must be disarmed");
    assert_eq!(
        dev.alert(AlertSlot::Two),
        Some((Channel::Ch1, bus)),
        "bus alert is absolutely scaled and must survive"
    );
    dev.release().done();
}

#[tokio::test]
async fn recalibration_with_no_alerts_emits_no_extra_writes() {
    let cal = example_cal();
    // Exactly the three transactions calibrate has always issued.
    let mut dev = sensor(&[
        Transaction::write_read(ADDR, vec![0x21], vec![0x00, 0x00]),
        Transaction::write(ADDR, vec![0x21, 0x00, 0x00]),
        Transaction::write(ADDR, vec![0x05, 0x05, 0x00]),
    ]);
    dev.calibrate(Channel::Ch1, cal).await.unwrap();
    dev.release().done();
}

#[tokio::test]
async fn alerts_on_other_channels_survive_recalibration() {
    let cal = example_cal();
    let mut dev = sensor(&[
        // calibrate Ch1 and Ch2
        Transaction::write_read(ADDR, vec![0x21], vec![0x00, 0x00]),
        Transaction::write(ADDR, vec![0x21, 0x00, 0x00]),
        Transaction::write(ADDR, vec![0x05, 0x05, 0x00]),
        Transaction::write_read(ADDR, vec![0x21], vec![0x00, 0x00]),
        Transaction::write(ADDR, vec![0x21, 0x00, 0x00]),
        Transaction::write(ADDR, vec![0x0D, 0x05, 0x00]),
        // arm slot 3 with a shunt alert on Ch2
        Transaction::write(ADDR, vec![0x16, 0x01, 0x90]),
        Transaction::write(ADDR, vec![0x17, 0x00, 0x09]), // Ch2 => 0b01_001
        // recalibrate Ch1: slot 3 targets Ch2, so it must not be touched
        Transaction::write_read(ADDR, vec![0x21], vec![0x00, 0x00]),
        Transaction::write(ADDR, vec![0x21, 0x00, 0x00]),
        Transaction::write(ADDR, vec![0x05, 0x05, 0x00]),
    ]);

    dev.calibrate(Channel::Ch1, cal).await.unwrap();
    dev.calibrate(Channel::Ch2, cal).await.unwrap();
    let shunt = Alert::ShuntOver(ShuntVoltage::from_nanovolts(1_000_000));
    dev.set_alert(AlertSlot::Three, Channel::Ch2, shunt).await.unwrap();

    dev.calibrate(Channel::Ch1, cal).await.unwrap();

    assert_eq!(dev.alert(AlertSlot::Three), Some((Channel::Ch2, shunt)));
    dev.release().done();
}

#[tokio::test]
async fn a_failed_disarm_aborts_before_the_range_is_rewritten() {
    let cal = example_cal();
    let mut dev = sensor(&[
        Transaction::write_read(ADDR, vec![0x21], vec![0x00, 0x00]),
        Transaction::write(ADDR, vec![0x21, 0x00, 0x00]),
        Transaction::write(ADDR, vec![0x05, 0x05, 0x00]),
        Transaction::write(ADDR, vec![0x06, 0x01, 0x90]),
        Transaction::write(ADDR, vec![0x07, 0x00, 0x01]),
        // the disarm fails; NO CONFIG2 write may follow
        Transaction::write(ADDR, vec![0x07, 0x00, 0x00]).with_error(ErrorKind::Other),
    ]);

    dev.calibrate(Channel::Ch1, cal).await.unwrap();
    let shunt = Alert::ShuntOver(ShuntVoltage::from_nanovolts(1_000_000));
    dev.set_alert(AlertSlot::One, Channel::Ch1, shunt).await.unwrap();

    assert!(dev.calibrate(Channel::Ch1, cal).await.is_err());
    assert_eq!(
        dev.alert(AlertSlot::One),
        Some((Channel::Ch1, shunt)),
        "the disarm did not land, so the cache must still show it armed"
    );
    dev.release().done();
}

#[tokio::test]
async fn limit_alert_is_indexed_by_slot() {
    // FLAGS 0x22, bit 13 = LIMIT2_ALERT.
    let mut dev = sensor(&[Transaction::write_read(ADDR, vec![0x22], vec![0x20, 0x00])]);
    let flags = dev.read_flags().await.unwrap();
    assert!(!flags.limit_alert(AlertSlot::One));
    assert!(flags.limit_alert(AlertSlot::Two));
    assert!(!flags.limit_alert(AlertSlot::Three));
    assert!(!flags.limit_alert(AlertSlot::Four));
    assert_eq!(flags.limit_alerts(), [false, true, false, false]);
    dev.release().done();
}

// ── Operating mode ────────────────────────────────────────────────────────────

// CONFIG1 is 0x20, reset 0xF127, and MODE is bits 2:0. Every setter below is
// therefore a read of 0xF127 followed by a write of 0xF120 | MODE. These
// vectors pin the MODE encodings only: the reset value has AVG = 0, so it
// cannot witness field preservation. See
// `set_mode_preserves_other_config1_fields` for that.
fn set_mode_expectations(low: u8) -> Vec<Transaction> {
    vec![
        Transaction::write_read(ADDR, vec![0x20], vec![0xF1, 0x27]),
        Transaction::write(ADDR, vec![0x20, 0xF1, low]),
    ]
}

#[tokio::test]
async fn set_mode_writes_shutdown_encoding_000() {
    let mut dev = sensor(&set_mode_expectations(0x20));
    dev.set_mode(OperatingMode::Shutdown).await.unwrap();
    dev.release().done();
}

#[tokio::test]
async fn set_mode_writes_shunt_triggered_encoding_001() {
    let mut dev = sensor(&set_mode_expectations(0x21));
    dev.set_mode(OperatingMode::ShuntTriggered).await.unwrap();
    dev.release().done();
}

#[tokio::test]
async fn set_mode_writes_bus_triggered_encoding_010() {
    let mut dev = sensor(&set_mode_expectations(0x22));
    dev.set_mode(OperatingMode::BusTriggered).await.unwrap();
    dev.release().done();
}

#[tokio::test]
async fn set_mode_writes_shunt_and_bus_triggered_encoding_011() {
    let mut dev = sensor(&set_mode_expectations(0x23));
    dev.set_mode(OperatingMode::ShuntAndBusTriggered).await.unwrap();
    dev.release().done();
}

#[tokio::test]
async fn set_mode_writes_shutdown_encoding_100() {
    // The second shutdown encoding is preserved rather than canonicalised.
    let mut dev = sensor(&set_mode_expectations(0x24));
    dev.set_mode(OperatingMode::ShutdownAlternate).await.unwrap();
    dev.release().done();
}

#[tokio::test]
async fn set_mode_writes_continuous_shunt_encoding_101() {
    let mut dev = sensor(&set_mode_expectations(0x25));
    dev.set_mode(OperatingMode::ContinuousShunt).await.unwrap();
    dev.release().done();
}

#[tokio::test]
async fn set_mode_writes_continuous_bus_encoding_110() {
    let mut dev = sensor(&set_mode_expectations(0x26));
    dev.set_mode(OperatingMode::ContinuousBus).await.unwrap();
    dev.release().done();
}

#[tokio::test]
async fn set_mode_writes_continuous_shunt_and_bus_encoding_111() {
    let mut dev = sensor(&set_mode_expectations(0x27));
    dev.set_mode(OperatingMode::ContinuousShuntAndBus).await.unwrap();
    dev.release().done();
}

#[tokio::test]
async fn set_mode_preserves_other_config1_fields() {
    // A deliberately non-default CONFIG1 in which every field is non-zero and
    // distinct, so that dropping any one of them is visible:
    //
    //   ACTIVE_CHANNEL 15:12 = 0b1010 = 0xA << 12 = 0xA000
    //   AVG            11:9  = 0b011  = 3   << 9  = 0x0600
    //   VBUSCT          8:6  = 0b110  = 6   << 6  = 0x0180
    //   VSHCT           5:3  = 0b001  = 1   << 3  = 0x0008
    //   MODE            2:0  = 0b100  = 4         = 0x0004
    //                                            -> 0xA78C
    //
    // Setting MODE to 0b111 must move bits 2:0 only: 0xA78C & !0x7 | 0x7 =
    // 0xA78F. Writing a freshly defaulted CONFIG1 instead would yield 0xF12F.
    let mut dev = sensor(&[
        Transaction::write_read(ADDR, vec![0x20], vec![0xA7, 0x8C]),
        Transaction::write(ADDR, vec![0x20, 0xA7, 0x8F]),
    ]);
    dev.set_mode(OperatingMode::ContinuousShuntAndBus).await.unwrap();
    dev.release().done();
}

#[tokio::test]
async fn mode_reads_every_encoding() {
    // Both shutdown encodings must come back distinct: the getter reports what
    // the device holds, it does not canonicalise.
    let expected = [
        (0x20u8, OperatingMode::Shutdown),
        (0x21, OperatingMode::ShuntTriggered),
        (0x22, OperatingMode::BusTriggered),
        (0x23, OperatingMode::ShuntAndBusTriggered),
        (0x24, OperatingMode::ShutdownAlternate),
        (0x25, OperatingMode::ContinuousShunt),
        (0x26, OperatingMode::ContinuousBus),
        (0x27, OperatingMode::ContinuousShuntAndBus),
    ];

    let expectations: Vec<_> = expected
        .iter()
        .map(|&(low, _)| Transaction::write_read(ADDR, vec![0x20], vec![0xF1, low]))
        .collect();

    let mut dev = sensor(&expectations);
    for (low, mode) in expected {
        assert_eq!(dev.mode().await.unwrap(), mode, "encoding 0x{low:02X}");
    }
    dev.release().done();
}

#[tokio::test]
async fn setting_the_same_triggered_mode_still_writes_config1() {
    // Triggering happens on the write, not on a change of value, so an
    // equality short-circuit would silently stop retriggering.
    let expectations = vec![
        Transaction::write_read(ADDR, vec![0x20], vec![0xF1, 0x21]),
        Transaction::write(ADDR, vec![0x20, 0xF1, 0x21]),
    ];
    let mut dev = sensor(&expectations);
    dev.set_mode(OperatingMode::ShuntTriggered).await.unwrap();
    dev.release().done();
}

#[tokio::test]
async fn shutdown_preserves_cached_calibration() {
    // Shutdown does not reset SHUNT_CAL or CONFIG2.RANGE, so the cache stays
    // valid across it.
    let expectations = vec![
        // calibrate CH1
        Transaction::write_read(ADDR, vec![0x21], vec![0x00, 0x00]),
        Transaction::write(ADDR, vec![0x21, 0x00, 0x00]),
        Transaction::write(ADDR, vec![0x05, 0x05, 0x00]),
        // set_mode(Shutdown)
        Transaction::write_read(ADDR, vec![0x20], vec![0xF1, 0x27]),
        Transaction::write(ADDR, vec![0x20, 0xF1, 0x20]),
    ];
    let mut dev = sensor(&expectations);
    dev.calibrate(Channel::Ch1, example_cal()).await.unwrap();
    dev.set_mode(OperatingMode::Shutdown).await.unwrap();

    assert_eq!(dev.calibration(Channel::Ch1), Some(example_cal()));
    dev.release().done();
}

// ── Conversion timing ─────────────────────────────────────────────────────────

// CONFIG1 is 0x20, reset 0xF127, and the timing fields are AVG 11:9, VBUSCT 8:6
// and VSHCT 5:3:
//
//   0xF127 = ACTIVE_CHANNEL 0b1111, AVG 0b000, VBUSCT 0b100, VSHCT 0b100,
//            MODE 0b111
//
// Each setter below is therefore a read of 0xF127 followed by a write of the
// reset word with one field replaced. The reset value cannot witness
// preservation of AVG, which is 0, so see the `preserves_other_config1_fields`
// tests for that.
fn config1_expectations(high: u8, low: u8) -> Vec<Transaction> {
    vec![
        Transaction::write_read(ADDR, vec![0x20], vec![0xF1, 0x27]),
        Transaction::write(ADDR, vec![0x20, high, low]),
    ]
}

#[tokio::test]
async fn conversion_timing_defaults_match_power_on() {
    // AVG = 0b000, VBUSCT = VSHCT = 0b100, the 0xF127 reset encoding.
    let timing = ConversionTiming::default();
    assert_eq!(timing.averaging, Averaging::Samples1);
    assert_eq!(timing.bus_conversion_time, BusConversionTime::Microseconds1100);
    assert_eq!(timing.shunt_conversion_time, ShuntConversionTime::Microseconds1100);
}

#[tokio::test]
async fn set_conversion_timing_writes_every_averaging_encoding() {
    // Bus and shunt stay at the reset 1100 µs, so the word is
    // (0xF127 & !0x0E00) | n << 9 = 0xF127 | n << 9.
    let cases = [
        (Averaging::Samples1, 0xF1u8),
        (Averaging::Samples4, 0xF3),
        (Averaging::Samples16, 0xF5),
        (Averaging::Samples64, 0xF7),
        (Averaging::Samples128, 0xF9),
        (Averaging::Samples256, 0xFB),
        (Averaging::Samples512, 0xFD),
        (Averaging::Samples1024, 0xFF),
    ];

    for (averaging, high) in cases {
        // Encoding 0 must still write: every CONFIG1 write clears CVRF and
        // retriggers a triggered mode, so it is never suppressed as redundant.
        let mut dev = sensor(&config1_expectations(high, 0x27));
        dev.set_conversion_timing(ConversionTiming {
            averaging,
            ..ConversionTiming::default()
        })
        .await
        .unwrap();
        dev.release().done();
    }
}

#[tokio::test]
async fn set_conversion_timing_writes_every_bus_time_encoding() {
    // AVG and shunt stay at their defaults, so the word is
    // (0xF127 & !0x01C0) | n << 6 = 0xF027 | n << 6.
    let cases = [
        (BusConversionTime::Microseconds140, 0xF0u8, 0x27u8),
        (BusConversionTime::Microseconds204, 0xF0, 0x67),
        (BusConversionTime::Microseconds332, 0xF0, 0xA7),
        (BusConversionTime::Microseconds588, 0xF0, 0xE7),
        (BusConversionTime::Microseconds1100, 0xF1, 0x27),
        (BusConversionTime::Microseconds2116, 0xF1, 0x67),
        (BusConversionTime::Microseconds4156, 0xF1, 0xA7),
        (BusConversionTime::Microseconds8244, 0xF1, 0xE7),
    ];

    for (bus_conversion_time, high, low) in cases {
        let mut dev = sensor(&config1_expectations(high, low));
        dev.set_conversion_timing(ConversionTiming {
            bus_conversion_time,
            ..ConversionTiming::default()
        })
        .await
        .unwrap();
        dev.release().done();
    }
}

#[tokio::test]
async fn set_conversion_timing_writes_every_shunt_time_encoding() {
    // AVG and bus stay at their defaults, so the word is
    // (0xF127 & !0x0038) | n << 3 = 0xF107 | n << 3.
    let cases = [
        (ShuntConversionTime::Microseconds140, 0x07u8),
        (ShuntConversionTime::Microseconds204, 0x0F),
        (ShuntConversionTime::Microseconds332, 0x17),
        (ShuntConversionTime::Microseconds588, 0x1F),
        (ShuntConversionTime::Microseconds1100, 0x27),
        (ShuntConversionTime::Microseconds2116, 0x2F),
        (ShuntConversionTime::Microseconds4156, 0x37),
        (ShuntConversionTime::Microseconds8244, 0x3F),
    ];

    for (shunt_conversion_time, low) in cases {
        let mut dev = sensor(&config1_expectations(0xF1, low));
        dev.set_conversion_timing(ConversionTiming {
            shunt_conversion_time,
            ..ConversionTiming::default()
        })
        .await
        .unwrap();
        dev.release().done();
    }
}

#[tokio::test]
async fn set_conversion_timing_preserves_other_config1_fields() {
    // The same deliberately non-default CONFIG1 the operating-mode tests use,
    // in which every field is non-zero and distinct:
    //
    //   ACTIVE_CHANNEL 15:12 = 0b1010 = 0xA << 12 = 0xA000
    //   AVG            11:9  = 0b011  = 3   << 9  = 0x0600
    //   VBUSCT          8:6  = 0b110  = 6   << 6  = 0x0180
    //   VSHCT           5:3  = 0b001  = 1   << 3  = 0x0008
    //   MODE            2:0  = 0b100  = 4         = 0x0004
    //                                            -> 0xA78C
    //
    // Setting AVG = 5, VBUSCT = 2 and VSHCT = 7 must move bits 11:3 only:
    // preserved 0xA004, plus 0x0A00 | 0x0080 | 0x0038 = 0xAABC. Writing a
    // freshly defaulted CONFIG1 instead would yield 0xFABC.
    let mut dev = sensor(&[
        Transaction::write_read(ADDR, vec![0x20], vec![0xA7, 0x8C]),
        Transaction::write(ADDR, vec![0x20, 0xAA, 0xBC]),
    ]);
    dev.set_conversion_timing(ConversionTiming {
        averaging: Averaging::Samples256,
        bus_conversion_time: BusConversionTime::Microseconds332,
        shunt_conversion_time: ShuntConversionTime::Microseconds8244,
    })
    .await
    .unwrap();
    dev.release().done();
}

#[tokio::test]
async fn conversion_timing_reads_every_encoding() {
    // All three fields set to the same encoding n, so the word is
    // 0xF007 + n * (0x0200 + 0x0040 + 0x0008) = 0xF007 + n * 0x0248.
    let expected = [
        (
            [0xF0u8, 0x07u8],
            Averaging::Samples1,
            BusConversionTime::Microseconds140,
            ShuntConversionTime::Microseconds140,
        ),
        (
            [0xF2, 0x4F],
            Averaging::Samples4,
            BusConversionTime::Microseconds204,
            ShuntConversionTime::Microseconds204,
        ),
        (
            [0xF4, 0x97],
            Averaging::Samples16,
            BusConversionTime::Microseconds332,
            ShuntConversionTime::Microseconds332,
        ),
        (
            [0xF6, 0xDF],
            Averaging::Samples64,
            BusConversionTime::Microseconds588,
            ShuntConversionTime::Microseconds588,
        ),
        (
            [0xF9, 0x27],
            Averaging::Samples128,
            BusConversionTime::Microseconds1100,
            ShuntConversionTime::Microseconds1100,
        ),
        (
            [0xFB, 0x6F],
            Averaging::Samples256,
            BusConversionTime::Microseconds2116,
            ShuntConversionTime::Microseconds2116,
        ),
        (
            [0xFD, 0xB7],
            Averaging::Samples512,
            BusConversionTime::Microseconds4156,
            ShuntConversionTime::Microseconds4156,
        ),
        (
            [0xFF, 0xFF],
            Averaging::Samples1024,
            BusConversionTime::Microseconds8244,
            ShuntConversionTime::Microseconds8244,
        ),
    ];

    let expectations: Vec<_> = expected
        .iter()
        .map(|&(word, ..)| Transaction::write_read(ADDR, vec![0x20], word.to_vec()))
        .collect();

    let mut dev = sensor(&expectations);
    for (word, averaging, bus_conversion_time, shunt_conversion_time) in expected {
        assert_eq!(
            dev.conversion_timing().await.unwrap(),
            ConversionTiming {
                averaging,
                bus_conversion_time,
                shunt_conversion_time,
            },
            "word 0x{:02X}{:02X}",
            word[0],
            word[1]
        );
    }
    dev.release().done();
}

#[tokio::test]
async fn set_averaging_writes_every_encoding() {
    // AVG is bits 11:9, and the reset word already holds 0, so the write is
    // (0xF127 & !0x0E00) | n << 9 = 0xF127 | n << 9.
    let cases = [
        (Averaging::Samples1, 0xF1u8),
        (Averaging::Samples4, 0xF3),
        (Averaging::Samples16, 0xF5),
        (Averaging::Samples64, 0xF7),
        (Averaging::Samples128, 0xF9),
        (Averaging::Samples256, 0xFB),
        (Averaging::Samples512, 0xFD),
        (Averaging::Samples1024, 0xFF),
    ];

    for (averaging, high) in cases {
        // Encoding 0 is the unchanged-value case and must still write.
        let mut dev = sensor(&config1_expectations(high, 0x27));
        dev.set_averaging(averaging).await.unwrap();
        dev.release().done();
    }
}

#[tokio::test]
async fn set_bus_conversion_time_writes_every_encoding() {
    // VBUSCT is bits 8:6, so the write is
    // (0xF127 & !0x01C0) | n << 6 = 0xF027 | n << 6.
    let cases = [
        (BusConversionTime::Microseconds140, 0xF0u8, 0x27u8),
        (BusConversionTime::Microseconds204, 0xF0, 0x67),
        (BusConversionTime::Microseconds332, 0xF0, 0xA7),
        (BusConversionTime::Microseconds588, 0xF0, 0xE7),
        (BusConversionTime::Microseconds1100, 0xF1, 0x27),
        (BusConversionTime::Microseconds2116, 0xF1, 0x67),
        (BusConversionTime::Microseconds4156, 0xF1, 0xA7),
        (BusConversionTime::Microseconds8244, 0xF1, 0xE7),
    ];

    for (conversion_time, high, low) in cases {
        // Encoding 4 is the unchanged-value case and must still write.
        let mut dev = sensor(&config1_expectations(high, low));
        dev.set_bus_conversion_time(conversion_time).await.unwrap();
        dev.release().done();
    }
}

#[tokio::test]
async fn set_shunt_conversion_time_writes_every_encoding() {
    // VSHCT is bits 5:3, so the write is
    // (0xF127 & !0x0038) | n << 3 = 0xF107 | n << 3.
    let cases = [
        (ShuntConversionTime::Microseconds140, 0x07u8),
        (ShuntConversionTime::Microseconds204, 0x0F),
        (ShuntConversionTime::Microseconds332, 0x17),
        (ShuntConversionTime::Microseconds588, 0x1F),
        (ShuntConversionTime::Microseconds1100, 0x27),
        (ShuntConversionTime::Microseconds2116, 0x2F),
        (ShuntConversionTime::Microseconds4156, 0x37),
        (ShuntConversionTime::Microseconds8244, 0x3F),
    ];

    for (conversion_time, low) in cases {
        // Encoding 4 is the unchanged-value case and must still write.
        let mut dev = sensor(&config1_expectations(0xF1, low));
        dev.set_shunt_conversion_time(conversion_time).await.unwrap();
        dev.release().done();
    }
}

#[tokio::test]
async fn set_averaging_preserves_other_config1_fields_and_round_trips() {
    // From the distinctive 0xA78C (ACTIVE 10, AVG 3, VBUSCT 6, VSHCT 1,
    // MODE 4), setting AVG = 5 must move bits 11:9 only:
    // (0xA78C & !0x0E00) | 5 << 9 = 0xA18C | 0x0A00 = 0xAB8C. The unchanged
    // ACTIVE, VBUSCT, VSHCT and MODE make neighbouring-field clobbering
    // observable.
    let mut dev = sensor(&[
        Transaction::write_read(ADDR, vec![0x20], vec![0xA7, 0x8C]),
        Transaction::write(ADDR, vec![0x20, 0xAB, 0x8C]),
        Transaction::write_read(ADDR, vec![0x20], vec![0xAB, 0x8C]),
    ]);
    dev.set_averaging(Averaging::Samples256).await.unwrap();
    assert_eq!(dev.averaging().await.unwrap(), Averaging::Samples256);
    dev.release().done();
}

#[tokio::test]
async fn set_bus_conversion_time_preserves_other_config1_fields_and_round_trips() {
    // From the same 0xA78C, setting VBUSCT = 2 must move bits 8:6 only:
    // (0xA78C & !0x01C0) | 2 << 6 = 0xA60C | 0x0080 = 0xA68C. The unchanged
    // ACTIVE, AVG, VSHCT and MODE make neighbouring-field clobbering
    // observable.
    let mut dev = sensor(&[
        Transaction::write_read(ADDR, vec![0x20], vec![0xA7, 0x8C]),
        Transaction::write(ADDR, vec![0x20, 0xA6, 0x8C]),
        Transaction::write_read(ADDR, vec![0x20], vec![0xA6, 0x8C]),
    ]);
    dev.set_bus_conversion_time(BusConversionTime::Microseconds332)
        .await
        .unwrap();
    assert_eq!(
        dev.bus_conversion_time().await.unwrap(),
        BusConversionTime::Microseconds332
    );
    dev.release().done();
}

#[tokio::test]
async fn set_shunt_conversion_time_preserves_other_config1_fields_and_round_trips() {
    // From the same 0xA78C, setting VSHCT = 7 must move bits 5:3 only:
    // (0xA78C & !0x0038) | 7 << 3 = 0xA784 | 0x0038 = 0xA7BC. The unchanged
    // ACTIVE, AVG, VBUSCT and MODE make neighbouring-field clobbering
    // observable.
    let mut dev = sensor(&[
        Transaction::write_read(ADDR, vec![0x20], vec![0xA7, 0x8C]),
        Transaction::write(ADDR, vec![0x20, 0xA7, 0xBC]),
        Transaction::write_read(ADDR, vec![0x20], vec![0xA7, 0xBC]),
    ]);
    dev.set_shunt_conversion_time(ShuntConversionTime::Microseconds8244)
        .await
        .unwrap();
    assert_eq!(
        dev.shunt_conversion_time().await.unwrap(),
        ShuntConversionTime::Microseconds8244
    );
    dev.release().done();
}

// ── ALERT pin configuration ───────────────────────────────────────────────────

/// Decompose a target nibble `n` = `CNVR ENOF LATCH POL` into the public
/// configuration it stands for. CONFIG2 bits 7:4, so the register value is
/// `n << 4`.
fn alert_pin_config_for(n: u8) -> AlertPinConfig {
    AlertPinConfig {
        on_conversion_ready: n & 0b1000 != 0,
        on_energy_overflow: n & 0b0100 != 0,
        latch: if n & 0b0010 != 0 {
            AlertLatch::Latched
        } else {
            AlertLatch::Transparent
        },
        polarity: if n & 0b0001 != 0 {
            AlertPolarity::ActiveHigh
        } else {
            AlertPolarity::ActiveLow
        },
    }
}

#[test]
fn alert_pin_config_defaults_match_power_on() {
    // CONFIG2 resets to 0x0000, so the semantic default must be exactly this
    // and not merely whatever the derived defaults happen to be.
    assert_eq!(
        AlertPinConfig::default(),
        AlertPinConfig {
            polarity: AlertPolarity::ActiveLow,
            latch: AlertLatch::Transparent,
            on_conversion_ready: false,
            on_energy_overflow: false,
        }
    );
}

#[tokio::test]
async fn set_alert_pin_config_writes_all_sixteen_encodings() {
    // CNVR = 0x80, ENOF = 0x40, LATCH = 0x20, POL = 0x10, so the four-bit
    // public combination lands in CONFIG2 bits 7:4 as n << 4. Four independent
    // one-bit fields have exactly 16 combinations, so this is exhaustive.
    for n in 0x0u8..=0xF {
        let mut dev = sensor(&[
            Transaction::write_read(ADDR, vec![0x21], vec![0x00, 0x00]),
            Transaction::write(ADDR, vec![0x21, 0x00, n << 4]),
        ]);
        dev.set_alert_pin_config(alert_pin_config_for(n)).await.unwrap();
        dev.release().done();
    }
}

#[tokio::test]
async fn alert_pin_config_reads_all_sixteen_encodings() {
    // Full round-trip decode coverage: catches swapped fields as well as
    // inverted enum meanings.
    let expectations: Vec<_> = (0x0u8..=0xF)
        .map(|n| Transaction::write_read(ADDR, vec![0x21], vec![0x00, n << 4]))
        .collect();

    let mut dev = sensor(&expectations);
    for n in 0x0u8..=0xF {
        assert_eq!(
            dev.alert_pin_config().await.unwrap(),
            alert_pin_config_for(n),
            "n = {n:#X}"
        );
    }
    dev.release().done();
}

#[tokio::test]
async fn set_alert_pin_config_preserves_range() {
    // RANGE lives in CONFIG2 bits 3:0 and belongs to calibrate(); clobbering it
    // would silently rescale every current and power reading. The read-back
    // nibble 0b1010 has both zeroes and both ones, so losing any one of the
    // four bits is visible: 0x000A & !0x00F0 | 0x00F0 = 0x00FA. Writing a
    // freshly defaulted CONFIG2 instead would yield [0x00, 0xF0].
    let mut dev = sensor(&[
        Transaction::write_read(ADDR, vec![0x21], vec![0x00, 0x0A]),
        Transaction::write(ADDR, vec![0x21, 0x00, 0xFA]),
    ]);
    dev.set_alert_pin_config(AlertPinConfig {
        polarity: AlertPolarity::ActiveHigh,
        latch: AlertLatch::Latched,
        on_conversion_ready: true,
        on_energy_overflow: true,
    })
    .await
    .unwrap();
    dev.release().done();
}

#[tokio::test]
async fn calibrate_preserves_alert_pin_config() {
    // The reciprocal of set_alert_pin_config_preserves_range: calibrate() owns
    // RANGE and must leave bits 7:4 alone. The read-back value has all four
    // ALERT controls set, so it is distinguishable from the reset value.
    //
    //   (0x00F0 & !0x0001) | 0x0001 = 0x00F1   // Ch1 RANGE bit 0, Range1
    //
    // Range1 with 500 µA/LSB and 8 mΩ gives SHUNT_CAL = 1280 / 4 = 320 =
    // 0x0140.
    let cal = Calibration::new(
        CurrentLsb::from_nanoamps(500_000).unwrap(),
        ShuntResistance::from_microohms(8_000).unwrap(),
        AdcRange::Range1,
    )
    .unwrap();

    let mut dev = sensor(&[
        Transaction::write_read(ADDR, vec![0x21], vec![0x00, 0xF0]),
        Transaction::write(ADDR, vec![0x21, 0x00, 0xF1]),
        Transaction::write(ADDR, vec![0x05, 0x01, 0x40]),
    ]);
    dev.calibrate(Channel::Ch1, cal).await.unwrap();

    assert_eq!(dev.calibration(Channel::Ch1), Some(cal));
    dev.release().done();
}
