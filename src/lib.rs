#![doc = include_str!("../README.md")]
#![cfg_attr(not(test), no_std)]
#![allow(async_fn_in_trait)]

use device_driver::FieldsetMetadata;
use embedded_sensors_hal_async::sensor;

#[allow(clippy::all)]
#[allow(clippy::pedantic)]
#[allow(unsafe_code)]
#[allow(missing_docs)]
mod device;

pub mod alert;
pub mod convert;
pub mod units;

pub use crate::alert::{Alert, AlertSlot};
pub use crate::units::{
    AdcRange, AddrPinState, Address, AddressPins, BusVoltage, Calibration, CalibrationError, Channel, Current,
    CurrentLsb, Energy, Power, ShuntCal, ShuntResistance, ShuntVoltage,
};

/// Maximum register data size in bytes (energy registers are 32-bit = 4 bytes).
const LARGEST_REG_SIZE_BYTES: usize = 4;

// ── Error type ────────────────────────────────────────────────────────────────

/// INA4230 driver error.
///
/// Overflow conditions are deliberately *not* errors. They live in [`Flags`],
/// which reports every condition at once; collapsing them into a single error
/// variant would discard the others, and reading the register clears them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[non_exhaustive]
pub enum Ina4230Error<I2cError> {
    /// An error occurred on the I²C bus.
    Bus(I2cError),
    /// A shunt-voltage, current, power, or energy reading was requested on a
    /// channel that has not been calibrated.
    ///
    /// Shunt voltage is included because its scale depends on the channel's
    /// configured [`AdcRange`], which is part of the calibration.
    ///
    /// Detected before any bus traffic is generated.
    NotCalibrated(Channel),
    /// An alert threshold does not fit `ALERT_LIMIT` at the target channel's
    /// scale.
    ///
    /// The representable range depends on the alert kind: ±81.92 mV or
    /// ±20.48 mV for shunt thresholds depending on [`AdcRange`], 0 to
    /// 52.4272 V for bus thresholds, and `65535 × 32 × CURRENT_LSB` for power
    /// thresholds.
    ///
    /// Detected before any bus traffic is generated.
    LimitOutOfRange(AlertSlot),
}

impl<E: embedded_hal_async::i2c::Error> sensor::Error for Ina4230Error<E> {
    fn kind(&self) -> sensor::ErrorKind {
        match self {
            Self::Bus(_) => sensor::ErrorKind::Peripheral,
            Self::NotCalibrated(_) => sensor::ErrorKind::NotReady,
            Self::LimitOutOfRange(_) => sensor::ErrorKind::InvalidInput,
        }
    }
}

// ── DeviceInterface ───────────────────────────────────────────────────────────

/// Async I²C interface adapter for the INA4230.
struct DeviceInterface<I2c: embedded_hal_async::i2c::I2c> {
    i2c: I2c,
    address: u8,
}

impl<I2c: embedded_hal_async::i2c::I2c> device_driver::RegisterInterfaceBase for DeviceInterface<I2c> {
    type Error = Ina4230Error<I2c::Error>;
    type AddressType = u8;
}

impl<I2c: embedded_hal_async::i2c::I2c> device_driver::AsyncRegisterInterface for DeviceInterface<I2c> {
    async fn write_register(
        &mut self,
        address: Self::AddressType,
        data: &mut [u8],
        _meta_data: &FieldsetMetadata,
    ) -> Result<(), Self::Error> {
        debug_assert!(data.len() <= LARGEST_REG_SIZE_BYTES, "Register data too large");
        let mut buf = [0u8; 1 + LARGEST_REG_SIZE_BYTES];
        buf[0] = address;
        buf[1..=data.len()].copy_from_slice(data);
        self.i2c
            .write(self.address, &buf[..=data.len()])
            .await
            .map_err(Ina4230Error::Bus)
    }

    async fn read_register(
        &mut self,
        address: Self::AddressType,
        data: &mut [u8],
        _meta_data: &FieldsetMetadata,
    ) -> Result<(), Self::Error> {
        self.i2c
            .write_read(self.address, &[address], data)
            .await
            .map_err(Ina4230Error::Bus)
    }
}

// ── Flags ─────────────────────────────────────────────────────────────────────

/// A snapshot of the `FLAGS` register.
///
/// Every condition the register can report is preserved here. That matters
/// because reading `FLAGS` clears the conversion-ready and latched alert bits
/// (datasheet Table 7-20), so a discarded flag is lost for good.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Flags {
    conversion_ready: bool,
    math_overflow: bool,
    energy_overflow: [bool; 4],
    limit_alerts: [bool; 4],
}

impl Flags {
    /// All conversions and averaging are complete.
    #[must_use]
    pub const fn conversion_ready(self) -> bool {
        self.conversion_ready
    }

    /// An arithmetic operation overflowed; current and power data may be
    /// invalid.
    #[must_use]
    pub const fn math_overflow(self) -> bool {
        self.math_overflow
    }

    /// The energy accumulator for `channel` has overflowed.
    ///
    /// # This crate cannot clear it
    ///
    /// The bit is not read-to-clear. The datasheet clears it only through
    /// `CONFIG2.ACC_RST` (Table 7-4, bits 11-8), which this crate does not
    /// expose yet, so the flag stays set and the accumulator keeps returning
    /// a wrapped value for as long as the driver lives.
    ///
    /// The only recovery available here is [`Ina4230::reset`], which restores
    /// every register to its default. That zeroes `SHUNT_CAL`, so the channel
    /// must be recalibrated afterwards or it reports zero current forever
    /// (datasheet 8.1.2). Treat an energy overflow as "reset and recalibrate",
    /// not as something a subsequent read clears.
    #[must_use]
    pub const fn energy_overflow(self, channel: Channel) -> bool {
        self.energy_overflow[channel.index()]
    }

    /// Any channel's energy accumulator has overflowed.
    ///
    /// See [`Flags::energy_overflow`] for why this condition is sticky and
    /// what clearing it costs.
    #[must_use]
    pub fn any_energy_overflow(self) -> bool {
        self.energy_overflow.iter().any(|&v| v)
    }

    /// Whether `slot`'s alert limit has been exceeded.
    ///
    /// The channel this refers to is whichever one the slot's `ALERT_CONFIG`
    /// selects (datasheet Table 7-8), not one derived from the slot number —
    /// Table 7-20 describes these flags as independent of channel.
    ///
    /// See [`Flags::limit_alerts`] for why a limit flag need not agree with the
    /// averaged measurement registers.
    #[must_use]
    pub const fn limit_alert(self, slot: AlertSlot) -> bool {
        self.limit_alerts[slot.index()]
    }

    /// The four alert limit flags, ordered `[LIMIT1, LIMIT2, LIMIT3, LIMIT4]`.
    ///
    /// Each flag belongs to an `ALERT_CONFIG` register rather than to a fixed
    /// channel, so the channel a flag refers to is whatever that register's
    /// `CHANNEL` field selects (datasheet Table 7-8).
    ///
    /// # These do not correspond to the averaged readings
    ///
    /// The device compares the alert limit against *every* conversion, not
    /// against the averaged result that reaches the output registers
    /// (datasheet 6.3.5). With averaging enabled a limit flag can therefore
    /// report an excursion that appears in no value [`Ina4230`] can read back,
    /// and the flag disagreeing with the measurement registers is correct
    /// behaviour rather than a fault.
    ///
    /// This crate does not configure `AVG` and the power-on default is a
    /// single sample, so the two agree unless something else on the bus has
    /// programmed `CONFIG1`.
    #[must_use]
    pub const fn limit_alerts(self) -> [bool; 4] {
        self.limit_alerts
    }
}

// ── Sensor traits ─────────────────────────────────────────────────────────────

/// Async voltage sensor — reads bus or shunt voltage per channel.
pub trait VoltageSensor: sensor::ErrorType {
    /// Read the bus voltage for the given channel.
    async fn bus_voltage(&mut self, channel: Channel) -> Result<BusVoltage, Self::Error>;
    /// Read the shunt voltage for the given channel.
    ///
    /// The scale depends on the channel's configured [`AdcRange`], so this
    /// requires the channel to have been calibrated.
    async fn shunt_voltage(&mut self, channel: Channel) -> Result<ShuntVoltage, Self::Error>;
}

impl<T: VoltageSensor + ?Sized> VoltageSensor for &mut T {
    async fn bus_voltage(&mut self, channel: Channel) -> Result<BusVoltage, Self::Error> {
        T::bus_voltage(self, channel).await
    }
    async fn shunt_voltage(&mut self, channel: Channel) -> Result<ShuntVoltage, Self::Error> {
        T::shunt_voltage(self, channel).await
    }
}

/// Async current sensor — reads calculated current per channel.
pub trait CurrentSensor: sensor::ErrorType {
    /// Read the calculated current for the given channel.
    /// Requires [`Ina4230::calibrate`] to have been called first.
    async fn current(&mut self, channel: Channel) -> Result<Current, Self::Error>;
}

impl<T: CurrentSensor + ?Sized> CurrentSensor for &mut T {
    async fn current(&mut self, channel: Channel) -> Result<Current, Self::Error> {
        T::current(self, channel).await
    }
}

/// Async power sensor — reads calculated power per channel.
pub trait PowerSensor: sensor::ErrorType {
    /// Read the calculated power for the given channel.
    /// Requires [`Ina4230::calibrate`] to have been called first.
    async fn power(&mut self, channel: Channel) -> Result<Power, Self::Error>;
}

impl<T: PowerSensor + ?Sized> PowerSensor for &mut T {
    async fn power(&mut self, channel: Channel) -> Result<Power, Self::Error> {
        T::power(self, channel).await
    }
}

/// Async energy sensor — reads accumulated energy per channel.
pub trait EnergySensor: sensor::ErrorType {
    /// Read the accumulated energy for the given channel.
    /// Requires [`Ina4230::calibrate`] to have been called first.
    async fn energy(&mut self, channel: Channel) -> Result<Energy, Self::Error>;
}

impl<T: EnergySensor + ?Sized> EnergySensor for &mut T {
    async fn energy(&mut self, channel: Channel) -> Result<Energy, Self::Error> {
        T::energy(self, channel).await
    }
}

// ── Ina4230 driver struct ─────────────────────────────────────────────────────

/// High-level driver for the INA4230 quad-channel power and energy monitor.
///
/// The driver holds no logic of its own: it moves bytes to and from the device
/// and hands them to the pure functions in [`convert`]. Everything that can be
/// reasoned about lives there.
pub struct Ina4230<I2c: embedded_hal_async::i2c::I2c> {
    device: device::Device<DeviceInterface<I2c>>,
    /// The address this instance talks to, kept for introspection.
    address: Address,
    /// Per-channel calibration, as programmed through this driver.
    ///
    /// This is the driver's record of its own successful writes, not a
    /// reflection of the device. It cannot observe a power cycle, an EN-pin
    /// toggle, a General Call reset, or writes by another bus controller.
    calibration: [Option<Calibration>; 4],
    /// Per-slot alert configuration, as programmed through this driver.
    ///
    /// Carries the same caveat as [`Ina4230::calibration`]: a record of this
    /// driver's own successful writes, not a reflection of the device.
    ///
    /// Cached because invalidation needs it. When a channel is recalibrated,
    /// [`Ina4230::calibrate`] has to know which slots target that channel and
    /// which of those are shunt or power alerts. Without this, that question
    /// would cost four register reads on every calibration.
    alerts: [Option<(Channel, Alert)>; 4],
}

impl<I2c: embedded_hal_async::i2c::I2c> Ina4230<I2c> {
    /// Create a new driver instance.
    ///
    /// `pins` describes how A0 and A1 are strapped on the board; the fields are
    /// named so the two cannot be transposed.
    pub fn new(i2c: I2c, pins: AddressPins) -> Self {
        let address = Address::from_pins(pins);
        Self {
            device: device::Device::new(DeviceInterface {
                i2c,
                address: address.as_u8(),
            }),
            address,
            calibration: [None; 4],
            alerts: [None; 4],
        }
    }

    /// The I²C address this instance talks to.
    #[must_use]
    pub const fn address(&self) -> Address {
        self.address
    }

    /// Release the underlying I²C bus.
    pub fn release(self) -> I2c {
        self.device.free().i2c
    }

    // ── Device management ─────────────────────────────────────────────────────

    /// Issue a full device reset (`CONFIG2.RST = 1`).
    ///
    /// All registers return to power-on defaults and the bit self-clears. That
    /// includes the calibration registers and `CONFIG2.RANGE`, so the cached
    /// per-channel calibration is discarded here too: leaving it in place would
    /// make the driver confidently scale readings with settings the device no
    /// longer has.
    ///
    /// It also includes every `ALERT_CONFIG`, which returns to a reserved
    /// no-effect `ALERT_MASK`, so the cached per-slot alert configuration is
    /// discarded once the write lands. [`Ina4230::alert`] reports every slot as
    /// disarmed afterwards, which is what the device now is.
    ///
    /// Call [`Ina4230::calibrate`] again before reading shunt voltage, current,
    /// power, or energy.
    ///
    /// # The calibration cache is cleared before the write, the alert cache after
    ///
    /// I²C gives no way to learn whether a device acted on a transaction that
    /// failed partway, and a future dropped at an `await` point may still have
    /// put the write on the wire. So after a failure the device may or may not
    /// have reset, and the two caches want opposite answers to that.
    ///
    /// A stale *calibration* is the dangerous direction: a reset zeroes
    /// `SHUNT_CAL`, so the part reports a current of exactly zero (datasheet
    /// §8.1.2) — a plausible reading rather than an obvious fault. Clearing
    /// first makes the cache never outlive the device's calibration. The cost
    /// of clearing unnecessarily is one redundant calibration; the cost of not
    /// clearing is silently wrong measurements.
    ///
    /// A cleared *alert* cache is the dangerous direction. If the reset did not
    /// land, slots are still armed on the device while the driver believes they
    /// are not, so the next [`Ina4230::calibrate`] finds nothing to disarm and
    /// moves `CONFIG2.RANGE` under a live shunt or power threshold — the
    /// factor-of-four error the disarming exists to prevent, with no record
    /// left anywhere. The cache is therefore cleared only once the write
    /// succeeds. Over-reporting costs at most a few redundant `ALERT_MASK` = 0
    /// writes to already-cleared registers.
    ///
    /// # Errors
    ///
    /// Returns [`Ina4230Error::Bus`] if an I²C bus error occurs. A failed reset
    /// always leaves the driver in the loud [`Ina4230Error::NotCalibrated`]
    /// state rather than a quiet wrong one, and leaves every slot it had armed
    /// still marked as armed.
    pub async fn reset(&mut self) -> Result<(), Ina4230Error<I2c::Error>> {
        self.calibration = [None; 4];
        self.device.config_2().write_async(|w| w.set_rst(true)).await?;
        self.alerts = [None; 4];
        Ok(())
    }

    /// Read the manufacturer ID register.
    ///
    /// # Errors
    ///
    /// Returns [`Ina4230Error::Bus`] if an I²C bus error occurs.
    pub async fn manufacturer_id(&mut self) -> Result<u16, Ina4230Error<I2c::Error>> {
        Ok(self.device.manufacturer_id().read_async().await?.id())
    }

    /// Check that the manufacturer ID reads back the expected value.
    ///
    /// # Errors
    ///
    /// Returns [`Ina4230Error::Bus`] if an I²C bus error occurs.
    pub async fn is_present(&mut self) -> Result<bool, Ina4230Error<I2c::Error>> {
        Ok(self.manufacturer_id().await? == Self::MANUFACTURER_ID)
    }

    /// Value the manufacturer ID register reads back on a healthy device:
    /// `"TI"` in ASCII.
    pub const MANUFACTURER_ID: u16 = 0x5449;

    /// Read the `FLAGS` register.
    ///
    /// # This read has side effects
    ///
    /// Reading `FLAGS` clears the conversion-ready flag and any latched alert
    /// flags (datasheet Table 7-20, and `CONFIG2.ALERT_LATCH`). There is no
    /// way to poll `CVRF` without reading the other flags at the same time,
    /// which is why this returns the whole register rather than offering
    /// per-bit accessors that would discard the rest of the snapshot.
    ///
    /// The datasheet does not specify the math-overflow or energy-overflow
    /// bits as read-to-clear; energy overflow is cleared through
    /// `CONFIG2.ACC_RST`. Those conditions therefore persist in the device
    /// across reads, but each returned [`Flags`] value is still the only
    /// record the caller gets of that particular snapshot.
    ///
    /// `CONFIG2.ACC_RST` is not exposed by this crate, so an energy overflow
    /// cannot be cleared short of [`Ina4230::reset`] and a recalibration. See
    /// [`Flags::energy_overflow`].
    ///
    /// To poll for conversion completion:
    ///
    /// ```rust,ignore
    /// while !sensor.read_flags().await?.conversion_ready() {}
    /// ```
    ///
    /// Inspect every returned [`Flags`] if alert information matters: a
    /// latched alert observed by an intermediate read of that loop is cleared
    /// in the device and will not appear again.
    ///
    /// # Errors
    ///
    /// Returns [`Ina4230Error::Bus`] if an I²C bus error occurs.
    pub async fn read_flags(&mut self) -> Result<Flags, Ina4230Error<I2c::Error>> {
        let f = self.device.flags().read_async().await?;
        Ok(Flags {
            conversion_ready: f.cvrf(),
            math_overflow: f.ovf(),
            energy_overflow: [f.energyof_ch1(), f.energyof_ch2(), f.energyof_ch3(), f.energyof_ch4()],
            limit_alerts: [f.limit1_alert(), f.limit2_alert(), f.limit3_alert(), f.limit4_alert()],
        })
    }

    /// Enable or disable a channel in `CONFIG1.ACTIVE_CHANNEL`.
    ///
    /// Disabled channels are skipped in the round-robin conversion cycle.
    ///
    /// Note that writing `CONFIG1` clears the conversion-ready flag
    /// (datasheet Table 7-20), so a poll in progress will wait for the next
    /// full conversion.
    ///
    /// # Errors
    ///
    /// Returns [`Ina4230Error::Bus`] if an I²C bus error occurs.
    pub async fn set_channel_active(&mut self, channel: Channel, active: bool) -> Result<(), Ina4230Error<I2c::Error>> {
        self.device
            .config_1()
            .modify_async(|w| {
                w.set_active_channel(convert::set_channel_bit(w.active_channel(), channel, active));
            })
            .await
    }

    // ── Calibration ───────────────────────────────────────────────────────────

    /// Program the calibration for a single channel.
    ///
    /// Writes `CONFIG2.RANGE` for the channel and the channel's `SHUNT_CAL`
    /// register, then caches the calibration for scaling subsequent readings.
    ///
    /// Build the [`Calibration`] with [`Calibration::new`]; all validation
    /// happens there, so this cannot fail for anything but a bus error.
    ///
    /// # Errors
    ///
    /// Returns [`Ina4230Error::Bus`] if an I²C bus error occurs.
    ///
    /// This operation is not transactional: it writes `CONFIG2.RANGE` and then
    /// `SHUNT_CAL`. A bus error, or a dropped future, can leave the range
    /// updated while the calibration register is not. The channel's cache
    /// entry is therefore invalidated *before* the first write and only
    /// repopulated once both succeed, so a partial update surfaces as
    /// [`Ina4230Error::NotCalibrated`] rather than as readings scaled with a
    /// range the device no longer uses — which for the two ADC ranges would be
    /// a silent factor-of-four error.
    ///
    /// # Alert slots scaled by this channel are disarmed
    ///
    /// `ALERT_LIMIT` holds raw counts whose meaning comes from the target
    /// channel's calibration, and nothing in the device couples the two. Every
    /// slot holding a shunt or power threshold for `channel` is therefore
    /// disarmed — `ALERT_MASK` written to 0 — before `CONFIG2.RANGE` moves, so
    /// that no threshold is ever enforced against a scale it was not programmed
    /// for. Re-arm them with [`Ina4230::set_alert`] after recalibrating.
    ///
    /// Bus thresholds have a fixed 1.6 mV LSB, do not depend on any
    /// calibration, and are left armed.
    ///
    /// A bus error during the disarming returns with `CONFIG2.RANGE` untouched,
    /// so any slot still armed is still armed against the scale it was given.
    pub async fn calibrate(
        &mut self,
        channel: Channel,
        calibration: Calibration,
    ) -> Result<(), Ina4230Error<I2c::Error>> {
        // Disarm anything whose scale is about to move, before it moves. A
        // failure here returns with CONFIG2.RANGE untouched, so whatever is
        // still armed is still armed against the scale it was given.
        self.disarm_scaled_alerts(channel).await?;

        // Invalidate first: from here until both writes land, the device's
        // calibration state is not something this driver can vouch for.
        self.calibration[channel.index()] = None;

        self.device
            .config_2()
            .modify_async(|w| {
                let set = matches!(calibration.adc_range(), AdcRange::Range1);
                w.set_range(convert::set_channel_bit(w.range(), channel, set));
            })
            .await?;

        self.device
            .channel_regs(channel.into())
            .calibration()
            .write_async(|w| w.set_shunt_cal(calibration.shunt_cal().as_u16()))
            .await?;

        self.calibration[channel.index()] = Some(calibration);
        Ok(())
    }

    /// Program the calibration for all four channels.
    ///
    /// `calibrations` is ordered `[Ch1, Ch2, Ch3, Ch4]`. This performs a single
    /// read-modify-write of `CONFIG2` rather than one per channel, since all
    /// four range bits are known up front.
    ///
    /// # Errors
    ///
    /// Returns [`Ina4230Error::Bus`] if an I²C bus error occurs.
    ///
    /// This operation is not transactional. It writes `CONFIG2` once — setting
    /// all four range bits together — and then each channel's `SHUNT_CAL` in
    /// turn, returning on the first failure. Every cache entry is therefore
    /// invalidated before the `CONFIG2` write, and each channel is repopulated
    /// only once its own `SHUNT_CAL` lands. A partial update leaves the
    /// channels it did not reach reporting
    /// [`Ina4230Error::NotCalibrated`]; inspect [`Ina4230::calibration`] to see
    /// how far it got.
    ///
    /// # Alert slots scaled by any channel are disarmed
    ///
    /// The single `CONFIG2` write moves all four range bits, so every shunt and
    /// power threshold in every slot is about to mean something else. All of
    /// them are disarmed first, for the reasons given on [`Ina4230::calibrate`].
    /// Bus thresholds are absolutely scaled and are left armed. If disarming
    /// fails partway through, inspect [`Ina4230::alert`] to see which slots
    /// remain armed.
    pub async fn calibrate_all(&mut self, calibrations: [Calibration; 4]) -> Result<(), Ina4230Error<I2c::Error>> {
        // One CONFIG2 write moves all four range bits, so every channel's
        // scaled alerts are about to become wrong.
        for channel in Channel::ALL {
            self.disarm_scaled_alerts(channel).await?;
        }

        // The CONFIG2 write moves all four range bits at once, so every
        // channel's cached calibration is suspect from here until its own
        // SHUNT_CAL is back in place.
        self.calibration = [None; 4];

        self.device
            .config_2()
            .modify_async(|w| {
                let mut range = w.range();
                for (ch, cal) in Channel::ALL.iter().zip(calibrations.iter()) {
                    let set = matches!(cal.adc_range(), AdcRange::Range1);
                    range = convert::set_channel_bit(range, *ch, set);
                }
                w.set_range(range);
            })
            .await?;

        for (ch, cal) in Channel::ALL.iter().zip(calibrations.iter()) {
            self.device
                .channel_regs((*ch).into())
                .calibration()
                .write_async(|w| w.set_shunt_cal(cal.shunt_cal().as_u16()))
                .await?;
            self.calibration[ch.index()] = Some(*cal);
        }
        Ok(())
    }

    /// The calibration this driver has cached for `channel`, if any.
    ///
    /// This reflects only successful writes made through this instance. It
    /// cannot detect an external reset, a power cycle, an EN-pin toggle, or
    /// writes performed by another bus controller. After any such event,
    /// recalibrate before requesting scaled measurements.
    #[must_use]
    pub fn calibration(&self, channel: Channel) -> Option<Calibration> {
        self.calibration[channel.index()]
    }

    /// Fetch the calibration for a channel, or fail before touching the bus.
    fn require_calibration(&self, channel: Channel) -> Result<Calibration, Ina4230Error<I2c::Error>> {
        self.calibration[channel.index()].ok_or(Ina4230Error::NotCalibrated(channel))
    }

    // ── Alerts ────────────────────────────────────────────────────────────────

    /// The alert configured for `slot`, as programmed through this driver.
    ///
    /// [`None`] means the slot is disarmed — `ALERT_MASK` is one of the
    /// reserved no-effect encodings.
    ///
    /// Reads the driver's cache, so it costs no bus traffic and cannot see a
    /// power cycle, an EN-pin toggle, a General Call reset, or another
    /// controller on the bus.
    #[must_use]
    pub const fn alert(&self, slot: AlertSlot) -> Option<(Channel, Alert)> {
        self.alerts[slot.index()]
    }

    /// Arm `slot` to watch `channel` for `alert`.
    ///
    /// Writes `ALERT_LIMIT` and then `ALERT_CONFIG`, so the threshold is in
    /// place before the condition is enabled.
    ///
    /// # Reprogramming an armed slot disarms it first
    ///
    /// `ALERT_LIMIT` and `ALERT_CONFIG` are separate registers, so a slot that
    /// is already armed would spend the gap between the two writes holding the
    /// *new* threshold against the *old* mask and channel. For a slot armed as
    /// a 1 mV shunt-over alert (limit 400, mask 1) and reprogrammed to a 12 V
    /// bus-over alert, a failure in between would leave the device enforcing
    /// 7500 counts as a shunt threshold — 18.75 mV, eighteen times weaker than
    /// asked for — while the cache still claimed 1 mV.
    ///
    /// So when the cache shows `slot` armed, `ALERT_MASK` is cleared and the
    /// cache entry dropped before the new pair goes in. Any failure then leaves
    /// the slot disarmed, or loaded with a new threshold and a clear mask,
    /// which raises nothing. A slot the cache shows as disarmed still costs
    /// exactly the two writes.
    ///
    /// # Errors
    ///
    /// Returns [`Ina4230Error::NotCalibrated`] if `alert` is a shunt or power
    /// condition and `channel` has no calibration — those thresholds scale with
    /// [`AdcRange`] and `CURRENT_LSB`. Bus conditions need no calibration.
    ///
    /// Returns [`Ina4230Error::LimitOutOfRange`] if the threshold does not fit
    /// `ALERT_LIMIT` at that scale.
    ///
    /// Both are detected before any bus traffic is generated.
    ///
    /// Returns [`Ina4230Error::Bus`] if an I²C bus error occurs. This operation
    /// is not transactional: if the `ALERT_LIMIT` write lands and the
    /// `ALERT_CONFIG` write does not, the slot is left disarmed with a new
    /// threshold loaded, and the cache is not updated.
    pub async fn set_alert(
        &mut self,
        slot: AlertSlot,
        channel: Channel,
        alert: Alert,
    ) -> Result<(), Ina4230Error<I2c::Error>> {
        let raw = self.encode_alert(slot, channel, alert)?;

        // Never let the new limit sit against the old mask and channel.
        if self.alerts[slot.index()].is_some() {
            self.clear_alert(slot).await?;
        }

        self.device
            .alert_regs(slot.into())
            .alert_limit()
            .write_async(|w| w.set_limit(raw))
            .await?;

        self.device
            .alert_regs(slot.into())
            .alert_config()
            .write_async(|w| {
                w.set_channel(channel.into());
                w.set_alert_mask(alert.alert_function());
            })
            .await?;

        self.alerts[slot.index()] = Some((channel, alert));
        Ok(())
    }

    /// Disarm `slot`.
    ///
    /// Writes `ALERT_MASK` = 0, one of the reserved no-effect encodings
    /// (datasheet Table 7-8). `ALERT_LIMIT` is left alone; it has no effect
    /// while the mask is clear.
    ///
    /// # Errors
    ///
    /// Returns [`Ina4230Error::Bus`] if an I²C bus error occurs. The cache
    /// entry is dropped only once the write lands.
    pub async fn clear_alert(&mut self, slot: AlertSlot) -> Result<(), Ina4230Error<I2c::Error>> {
        self.device
            .alert_regs(slot.into())
            .alert_config()
            .write_async(|w| w.set_alert_mask(device::AlertFunction::NoEffect))
            .await?;

        self.alerts[slot.index()] = None;
        Ok(())
    }

    /// Disarm every slot whose threshold is scaled by `channel`'s calibration.
    ///
    /// Shunt and power thresholds are stored as raw counts scaled by
    /// [`AdcRange`] and `CURRENT_LSB`. Recalibrating moves that scale, so a
    /// slot left armed would enforce a different threshold than the one it was
    /// given — for the two ADC ranges, a factor of four. Bus thresholds are
    /// absolute and are left alone.
    ///
    /// Called before `CONFIG2.RANGE` moves, so that a failure here leaves every
    /// still-armed slot armed against the scale it was programmed for.
    async fn disarm_scaled_alerts(&mut self, channel: Channel) -> Result<(), Ina4230Error<I2c::Error>> {
        for slot in AlertSlot::ALL {
            match self.alerts[slot.index()] {
                Some((ch, alert)) if ch == channel && alert.needs_calibration() => {
                    self.clear_alert(slot).await?;
                }
                _ => {}
            }
        }
        Ok(())
    }

    /// Encode an alert threshold, failing before any bus traffic.
    fn encode_alert(&self, slot: AlertSlot, channel: Channel, alert: Alert) -> Result<u16, Ina4230Error<I2c::Error>> {
        let encoded = match alert {
            Alert::ShuntOver(v) | Alert::ShuntUnder(v) => {
                let cal = self.require_calibration(channel)?;
                convert::encode_shunt_limit(v, cal.adc_range()).map(i16::cast_unsigned)
            }
            Alert::BusOver(v) | Alert::BusUnder(v) => convert::encode_bus_limit(v),
            Alert::PowerOver(p) => {
                let cal = self.require_calibration(channel)?;
                convert::encode_power_limit(p, cal)
            }
        };
        encoded.ok_or(Ina4230Error::LimitOutOfRange(slot))
    }
}

impl From<Channel> for device::Channel {
    fn from(channel: Channel) -> Self {
        match channel {
            Channel::Ch1 => Self::Ch1,
            Channel::Ch2 => Self::Ch2,
            Channel::Ch3 => Self::Ch3,
            Channel::Ch4 => Self::Ch4,
        }
    }
}

// ── Trait implementations ─────────────────────────────────────────────────────

impl<I2c: embedded_hal_async::i2c::I2c> sensor::ErrorType for Ina4230<I2c> {
    type Error = Ina4230Error<I2c::Error>;
}

impl<I2c: embedded_hal_async::i2c::I2c> VoltageSensor for Ina4230<I2c> {
    async fn bus_voltage(&mut self, channel: Channel) -> Result<BusVoltage, Self::Error> {
        let raw = self
            .device
            .channel_regs(channel.into())
            .bus_voltage()
            .read_async()
            .await?
            .vbus();
        Ok(convert::decode_bus_voltage(raw))
    }

    async fn shunt_voltage(&mut self, channel: Channel) -> Result<ShuntVoltage, Self::Error> {
        let cal = self.require_calibration(channel)?;
        let raw = self
            .device
            .channel_regs(channel.into())
            .shunt_voltage()
            .read_async()
            .await?
            .vshunt();
        Ok(convert::decode_shunt_voltage(raw, cal.adc_range()))
    }
}

impl<I2c: embedded_hal_async::i2c::I2c> CurrentSensor for Ina4230<I2c> {
    async fn current(&mut self, channel: Channel) -> Result<Current, Self::Error> {
        let cal = self.require_calibration(channel)?;
        let raw = self
            .device
            .channel_regs(channel.into())
            .current()
            .read_async()
            .await?
            .current();
        Ok(convert::decode_current(raw, cal))
    }
}

impl<I2c: embedded_hal_async::i2c::I2c> PowerSensor for Ina4230<I2c> {
    async fn power(&mut self, channel: Channel) -> Result<Power, Self::Error> {
        let cal = self.require_calibration(channel)?;
        let raw = self
            .device
            .channel_regs(channel.into())
            .power()
            .read_async()
            .await?
            .power();
        Ok(convert::decode_power(raw, cal))
    }
}

impl<I2c: embedded_hal_async::i2c::I2c> EnergySensor for Ina4230<I2c> {
    async fn energy(&mut self, channel: Channel) -> Result<Energy, Self::Error> {
        let cal = self.require_calibration(channel)?;
        let raw = self
            .device
            .channel_regs(channel.into())
            .energy()
            .read_async()
            .await?
            .energy();
        Ok(convert::decode_energy(raw, cal))
    }
}
