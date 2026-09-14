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

// ── Operating mode ────────────────────────────────────────────────────────────

/// The operating mode selected by `CONFIG1.MODE` (datasheet Table 7-3).
///
/// The mode picks which measurements the ADC performs and whether it performs
/// them once or forever. A *triggered* mode runs a single conversion sequence
/// after each write of `CONFIG1` and then stops; a *continuous* mode repeats
/// the conversion sequence indefinitely.
///
/// The mode also decides which conversion terms occur in a round-robin cycle:
/// see [`ConversionTiming`] for how long each of them takes.
///
/// [`Default`] is [`OperatingMode::ContinuousShuntAndBus`], the power-on value.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum OperatingMode {
    /// `000` — shutdown.
    ///
    /// Conversions stop and the device draws under 4 µA, typically 2.5 µA in
    /// standby, recovering in about 40 µs (datasheet 6.4.2 and 3). Registers,
    /// including `SHUNT_CAL` and `CONFIG2.RANGE`, keep their values.
    Shutdown = 0,
    /// `001` — one triggered shunt-voltage conversion sequence.
    ///
    /// Bus voltage is removed from the round-robin cycle (datasheet 6.3.2).
    ShuntTriggered = 1,
    /// `010` — one triggered bus-voltage conversion sequence.
    ///
    /// Shunt voltage is removed from the round-robin cycle (datasheet 6.3.2).
    BusTriggered = 2,
    /// `011` — one triggered shunt-and-bus conversion sequence.
    ShuntAndBusTriggered = 3,
    /// `100` — shutdown, behaviourally identical to [`OperatingMode::Shutdown`].
    ///
    /// The field has two encodings for the same state. Prefer
    /// [`OperatingMode::Shutdown`] in new code; this variant exists so that an
    /// encoding observed on the device can be reported and written back
    /// faithfully.
    ShutdownAlternate = 4,
    /// `101` — continuous shunt-voltage conversion only.
    ///
    /// Bus voltage is removed from the round-robin cycle (datasheet 6.3.2).
    ContinuousShunt = 5,
    /// `110` — continuous bus-voltage conversion only.
    ///
    /// Shunt voltage is removed from the round-robin cycle (datasheet 6.3.2).
    ContinuousBus = 6,
    /// `111` — continuous shunt-and-bus conversion. The power-on default.
    #[default]
    ContinuousShuntAndBus = 7,
}

impl From<OperatingMode> for device::Mode {
    fn from(mode: OperatingMode) -> Self {
        match mode {
            OperatingMode::Shutdown => Self::Shutdown,
            OperatingMode::ShuntTriggered => Self::ShuntTriggered,
            OperatingMode::BusTriggered => Self::BusTriggered,
            OperatingMode::ShuntAndBusTriggered => Self::ShuntAndBusTriggered,
            OperatingMode::ShutdownAlternate => Self::Shutdown2,
            OperatingMode::ContinuousShunt => Self::ContinuousShunt,
            OperatingMode::ContinuousBus => Self::ContinuousBus,
            OperatingMode::ContinuousShuntAndBus => Self::ContinuousShuntAndBus,
        }
    }
}

impl From<device::Mode> for OperatingMode {
    fn from(mode: device::Mode) -> Self {
        match mode {
            device::Mode::Shutdown => Self::Shutdown,
            device::Mode::ShuntTriggered => Self::ShuntTriggered,
            device::Mode::BusTriggered => Self::BusTriggered,
            device::Mode::ShuntAndBusTriggered => Self::ShuntAndBusTriggered,
            device::Mode::Shutdown2 => Self::ShutdownAlternate,
            device::Mode::ContinuousShunt => Self::ContinuousShunt,
            device::Mode::ContinuousBus => Self::ContinuousBus,
            device::Mode::ContinuousShuntAndBus => Self::ContinuousShuntAndBus,
        }
    }
}

// ── Conversion timing ─────────────────────────────────────────────────────────

/// The averaging count selected by `CONFIG1.AVG` (datasheet Table 7-3).
///
/// The device averages this many conversions before updating the output
/// registers and setting `CVRF`, so a larger count trades a slower output
/// update for a quieter reading. Limit alerts are unaffected: the device
/// compares every conversion rather than the average, as
/// [`Flags::limit_alerts`] describes.
///
/// This is device-global, not per-channel.
///
/// [`Default`] is [`Averaging::Samples1`], the power-on value.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum Averaging {
    /// `000` — 1 sample; no averaging. The power-on default.
    #[default]
    Samples1 = 0,
    /// `001` — 4 samples.
    Samples4 = 1,
    /// `010` — 16 samples.
    Samples16 = 2,
    /// `011` — 64 samples.
    Samples64 = 3,
    /// `100` — 128 samples.
    Samples128 = 4,
    /// `101` — 256 samples.
    Samples256 = 5,
    /// `110` — 512 samples.
    Samples512 = 6,
    /// `111` — 1024 samples.
    Samples1024 = 7,
}

/// The bus-voltage conversion time selected by `CONFIG1.VBUSCT` (datasheet
/// Table 7-3).
///
/// This is the time one bus-voltage conversion takes. It applies only when the
/// selected [`OperatingMode`] measures bus voltage.
///
/// This is device-global, not per-channel.
///
/// [`Default`] is [`BusConversionTime::Microseconds1100`], the power-on value.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum BusConversionTime {
    /// `000` — 140 µs.
    Microseconds140 = 0,
    /// `001` — 204 µs.
    Microseconds204 = 1,
    /// `010` — 332 µs.
    Microseconds332 = 2,
    /// `011` — 588 µs.
    Microseconds588 = 3,
    /// `100` — 1100 µs. The power-on default.
    #[default]
    Microseconds1100 = 4,
    /// `101` — 2116 µs.
    Microseconds2116 = 5,
    /// `110` — 4156 µs.
    Microseconds4156 = 6,
    /// `111` — 8244 µs.
    Microseconds8244 = 7,
}

/// The shunt-voltage conversion time selected by `CONFIG1.VSHCT` (datasheet
/// Table 7-3).
///
/// This is the time one shunt-voltage conversion takes. It applies only when
/// the selected [`OperatingMode`] measures shunt voltage.
///
/// This is device-global, not per-channel.
///
/// [`Default`] is [`ShuntConversionTime::Microseconds1100`], the power-on
/// value.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum ShuntConversionTime {
    /// `000` — 140 µs.
    Microseconds140 = 0,
    /// `001` — 204 µs.
    Microseconds204 = 1,
    /// `010` — 332 µs.
    Microseconds332 = 2,
    /// `011` — 588 µs.
    Microseconds588 = 3,
    /// `100` — 1100 µs. The power-on default.
    #[default]
    Microseconds1100 = 4,
    /// `101` — 2116 µs.
    Microseconds2116 = 5,
    /// `110` — 4156 µs.
    Microseconds4156 = 6,
    /// `111` — 8244 µs.
    Microseconds8244 = 7,
}

/// The `CONFIG1` conversion timing: `AVG`, `VBUSCT`, and `VSHCT` (datasheet
/// Table 7-3 and 6.4.4).
///
/// All three fields are device-global, not per-channel, and together they set
/// how long a round-robin cycle takes and how much the readings are filtered.
/// Which conversion terms actually occur is decided by [`OperatingMode`].
///
/// # Cycle time
///
/// For `N` active channels and an averaging count of `A`, one complete
/// round-robin cycle takes `A × N × (T_shunt + T_bus)` when the selected
/// [`OperatingMode`] measures both. Omit `T_shunt` or `T_bus` when the mode
/// disables that measurement; disabled channels are also omitted. Current,
/// power, and energy calculations run in the background and add no conversion
/// time (datasheet 6.3.2 and 6.4.4). For example, with all four channels
/// active, 16-sample averaging, a 588 µs shunt conversion, and a 204 µs bus
/// conversion, a complete cycle takes `16 × 4 × (588 µs + 204 µs) = 50,688 µs
/// = 50.688 ms`. A triggered conversion completes after that cycle; in
/// continuous mode `CVRF` is set after each such cycle. Polling should allow
/// at least this duration plus device and bus scheduling margin.
///
/// [`Default`] is the power-on timing: one sample and 1100 µs for each
/// conversion.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ConversionTiming {
    /// The averaging count, `CONFIG1.AVG`.
    pub averaging: Averaging,
    /// The bus-voltage conversion time, `CONFIG1.VBUSCT`.
    pub bus_conversion_time: BusConversionTime,
    /// The shunt-voltage conversion time, `CONFIG1.VSHCT`.
    pub shunt_conversion_time: ShuntConversionTime,
}

impl From<Averaging> for device::Averaging {
    fn from(averaging: Averaging) -> Self {
        match averaging {
            Averaging::Samples1 => Self::Num1,
            Averaging::Samples4 => Self::Num4,
            Averaging::Samples16 => Self::Num16,
            Averaging::Samples64 => Self::Num64,
            Averaging::Samples128 => Self::Num128,
            Averaging::Samples256 => Self::Num256,
            Averaging::Samples512 => Self::Num512,
            Averaging::Samples1024 => Self::Num1024,
        }
    }
}

impl From<device::Averaging> for Averaging {
    fn from(averaging: device::Averaging) -> Self {
        match averaging {
            device::Averaging::Num1 => Self::Samples1,
            device::Averaging::Num4 => Self::Samples4,
            device::Averaging::Num16 => Self::Samples16,
            device::Averaging::Num64 => Self::Samples64,
            device::Averaging::Num128 => Self::Samples128,
            device::Averaging::Num256 => Self::Samples256,
            device::Averaging::Num512 => Self::Samples512,
            device::Averaging::Num1024 => Self::Samples1024,
        }
    }
}

impl From<BusConversionTime> for device::BusConversionTime {
    fn from(time: BusConversionTime) -> Self {
        match time {
            BusConversionTime::Microseconds140 => Self::Us140,
            BusConversionTime::Microseconds204 => Self::Us204,
            BusConversionTime::Microseconds332 => Self::Us332,
            BusConversionTime::Microseconds588 => Self::Us588,
            BusConversionTime::Microseconds1100 => Self::Us1100,
            BusConversionTime::Microseconds2116 => Self::Us2116,
            BusConversionTime::Microseconds4156 => Self::Us4156,
            BusConversionTime::Microseconds8244 => Self::Us8244,
        }
    }
}

impl From<device::BusConversionTime> for BusConversionTime {
    fn from(time: device::BusConversionTime) -> Self {
        match time {
            device::BusConversionTime::Us140 => Self::Microseconds140,
            device::BusConversionTime::Us204 => Self::Microseconds204,
            device::BusConversionTime::Us332 => Self::Microseconds332,
            device::BusConversionTime::Us588 => Self::Microseconds588,
            device::BusConversionTime::Us1100 => Self::Microseconds1100,
            device::BusConversionTime::Us2116 => Self::Microseconds2116,
            device::BusConversionTime::Us4156 => Self::Microseconds4156,
            device::BusConversionTime::Us8244 => Self::Microseconds8244,
        }
    }
}

impl From<ShuntConversionTime> for device::ShuntConversionTime {
    fn from(time: ShuntConversionTime) -> Self {
        match time {
            ShuntConversionTime::Microseconds140 => Self::Us140,
            ShuntConversionTime::Microseconds204 => Self::Us204,
            ShuntConversionTime::Microseconds332 => Self::Us332,
            ShuntConversionTime::Microseconds588 => Self::Us588,
            ShuntConversionTime::Microseconds1100 => Self::Us1100,
            ShuntConversionTime::Microseconds2116 => Self::Us2116,
            ShuntConversionTime::Microseconds4156 => Self::Us4156,
            ShuntConversionTime::Microseconds8244 => Self::Us8244,
        }
    }
}

impl From<device::ShuntConversionTime> for ShuntConversionTime {
    fn from(time: device::ShuntConversionTime) -> Self {
        match time {
            device::ShuntConversionTime::Us140 => Self::Microseconds140,
            device::ShuntConversionTime::Us204 => Self::Microseconds204,
            device::ShuntConversionTime::Us332 => Self::Microseconds332,
            device::ShuntConversionTime::Us588 => Self::Microseconds588,
            device::ShuntConversionTime::Us1100 => Self::Microseconds1100,
            device::ShuntConversionTime::Us2116 => Self::Microseconds2116,
            device::ShuntConversionTime::Us4156 => Self::Microseconds4156,
            device::ShuntConversionTime::Us8244 => Self::Microseconds8244,
        }
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
    ///
    /// This is the completion signal for the triggered modes of
    /// [`OperatingMode`]: after [`Ina4230::set_mode`] starts a sequence, this
    /// is what says the results in the measurement registers belong to it.
    ///
    /// [`ConversionTiming`] sets how long that takes: the flag is set once per
    /// averaged round-robin cycle, and any `CONFIG1` write — including
    /// [`Ina4230::set_conversion_timing`] — clears it again.
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
    /// The device compares each alert limit against *every* conversion, not against
    /// the averaged result that reaches the output registers (datasheet 6.3.2 and
    /// 6.3.5). When [`Averaging`] is greater than one sample, a limit flag can
    /// therefore report an excursion that appears in no value [`Ina4230`] can read
    /// back. This disagreement is correct device behaviour, not a fault. See
    /// [`ConversionTiming`] for the related response-time and noise trade-off, and
    /// [`Ina4230::read_flags`] for the read-to-clear behavior of latched alerts.
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
    /// discarded once the write lands. On success [`Ina4230::alert`] reports
    /// every slot as disarmed, which is what the device now is; on failure the
    /// cache is left alone, for the reasons below.
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
    /// To poll for conversion completion — which is also how to wait for a
    /// conversion started by
    /// [`set_mode(OperatingMode::*Triggered)`](Ina4230::set_mode):
    ///
    /// ```rust,no_run
    /// # use embedded_hal_mock::eh1::i2c::Mock;
    /// # use ina4230::{AddrPinState, AddressPins, Ina4230};
    /// # #[derive(Debug)]
    /// # struct DocError;
    /// # impl<E: core::fmt::Debug> From<ina4230::Ina4230Error<E>> for DocError {
    /// #     fn from(_: ina4230::Ina4230Error<E>) -> Self { DocError }
    /// # }
    /// # async fn example() -> Result<(), DocError> {
    /// # let i2c = Mock::new(&[]);
    /// # let mut sensor = Ina4230::new(i2c, AddressPins {
    /// #     a0: AddrPinState::Gnd,
    /// #     a1: AddrPinState::Gnd,
    /// # });
    /// while !sensor.read_flags().await?.conversion_ready() {}
    /// # Ok(())
    /// # }
    /// # fn main() { tokio::runtime::Runtime::new().unwrap().block_on(example()).unwrap(); }
    /// ```
    ///
    /// Inspect every returned [`Flags`] if alert information matters: a
    /// latched alert observed by an intermediate read of that loop is cleared
    /// in the device and will not appear again.
    ///
    /// [`ConversionTiming`] sets how long such a loop must wait, and any
    /// `CONFIG1` write — including [`Ina4230::set_conversion_timing`] — clears
    /// `CVRF` and pushes the next completion out by a further cycle.
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

    /// Read `CONFIG1.MODE`.
    ///
    /// This reads the device rather than a cache, so it reflects a power
    /// cycle, an EN-pin toggle, a General Call reset, or a write by another
    /// bus controller.
    ///
    /// # Errors
    ///
    /// Returns [`Ina4230Error::Bus`] if an I²C bus error occurs.
    pub async fn mode(&mut self) -> Result<OperatingMode, Ina4230Error<I2c::Error>> {
        Ok(self.device.config_1().read_async().await?.mode().into())
    }

    /// Set `CONFIG1.MODE`.
    ///
    /// Performs a read-modify-write of `CONFIG1`, so only bits 2:0 move and
    /// every other field — including `ACTIVE_CHANNEL` — keeps the value read
    /// back from the device.
    ///
    /// # Writing clears conversion ready
    ///
    /// Every call writes `CONFIG1`, and that clears `CVRF` (datasheet 6.4.1
    /// and Table 7-20). A poll in progress through [`Ina4230::read_flags`]
    /// will therefore wait for the next conversion to complete; see
    /// [`Flags::conversion_ready`].
    ///
    /// # The same triggered mode retriggers
    ///
    /// Triggering happens on the write, not on a change of value. Calling this
    /// with a triggered mode that is already selected still issues the
    /// read-modify-write and still starts another conversion sequence; the
    /// write is never suppressed as redundant. Poll
    /// `read_flags().await?.conversion_ready()` before reading the results.
    /// How long that takes is set by [`ConversionTiming`].
    ///
    /// # Shutdown preserves calibration
    ///
    /// Entering [`OperatingMode::Shutdown`] does not reset `SHUNT_CAL` or
    /// `CONFIG2.RANGE`, so the cached calibration stays valid and measurements
    /// resume correctly scaled. That is unlike a power cycle, an EN-pin
    /// toggle, a General Call reset, or [`Ina4230::reset`], all of which
    /// require calling [`Ina4230::calibrate`] again.
    ///
    /// # Errors
    ///
    /// Returns [`Ina4230Error::Bus`] if an I²C bus error occurs. A failed read
    /// aborts before anything is written; a failed write leaves the device's
    /// mode unknown, since I²C gives no way to learn whether the device acted
    /// on the transaction.
    pub async fn set_mode(&mut self, mode: OperatingMode) -> Result<(), Ina4230Error<I2c::Error>> {
        self.device.config_1().modify_async(|w| w.set_mode(mode.into())).await
    }

    // ── Conversion timing ─────────────────────────────────────────────────────

    /// Read `CONFIG1.AVG`, `VBUSCT`, and `VSHCT` as a [`ConversionTiming`].
    ///
    /// All three fields come from a single register read, so they are a
    /// coherent snapshot. This reads the device rather than a cache, so it
    /// reflects a power cycle, an EN-pin toggle, a General Call reset, or a
    /// write by another bus controller.
    ///
    /// To read one field on its own, use [`Ina4230::averaging`],
    /// [`Ina4230::bus_conversion_time`], or
    /// [`Ina4230::shunt_conversion_time`].
    ///
    /// # Errors
    ///
    /// Returns [`Ina4230Error::Bus`] if an I²C bus error occurs.
    pub async fn conversion_timing(&mut self) -> Result<ConversionTiming, Ina4230Error<I2c::Error>> {
        let r = self.device.config_1().read_async().await?;
        Ok(ConversionTiming {
            averaging: r.avg().into(),
            bus_conversion_time: r.vbusct().into(),
            shunt_conversion_time: r.vshct().into(),
        })
    }

    /// Set `CONFIG1.AVG`, `VBUSCT`, and `VSHCT` together.
    ///
    /// Performs a single read-modify-write of `CONFIG1`, so only bits 11:3
    /// move and `ACTIVE_CHANNEL` and `MODE` keep the values read back from the
    /// device. This is the cheapest and only coherent way to change more than
    /// one timing field: [`Ina4230::set_averaging`],
    /// [`Ina4230::set_bus_conversion_time`], and
    /// [`Ina4230::set_shunt_conversion_time`] each cost their own
    /// read-modify-write and their own `CVRF` clear, and using several of them
    /// in sequence exposes intermediate timing configurations to the device.
    ///
    /// # Writing clears conversion ready
    ///
    /// Every call writes `CONFIG1`, and that clears `CVRF` (datasheet 6.4.1
    /// and Table 7-20). A poll in progress through [`Ina4230::read_flags`]
    /// will therefore wait for the next conversion to complete; see
    /// [`Flags::conversion_ready`].
    ///
    /// # Writing retriggers a triggered mode
    ///
    /// Triggering happens on the write, not on a change of value. If the
    /// selected [`OperatingMode`] is a triggered one, every call starts
    /// another conversion sequence even when the requested timing is identical
    /// to what the device already holds; the write is never suppressed as
    /// redundant. See [`Ina4230::set_mode`].
    ///
    /// # Response time against noise
    ///
    /// Short conversion times improve alert response but admit more noise;
    /// long conversion times reverse that trade-off. Averaging delays output
    /// register updates and `CVRF` without slowing limit checks, because the
    /// device compares every conversion rather than the averaged result — see
    /// [`Flags::limit_alerts`].
    ///
    /// # Errors
    ///
    /// Returns [`Ina4230Error::Bus`] if an I²C bus error occurs. A failed read
    /// aborts before anything is written; a failed write leaves the device's
    /// timing and triggered-conversion state unknown, since I²C gives no way
    /// to learn whether the device acted on the transaction.
    pub async fn set_conversion_timing(&mut self, timing: ConversionTiming) -> Result<(), Ina4230Error<I2c::Error>> {
        self.device
            .config_1()
            .modify_async(|w| {
                w.set_avg(timing.averaging.into());
                w.set_vbusct(timing.bus_conversion_time.into());
                w.set_vshct(timing.shunt_conversion_time.into());
            })
            .await
    }

    /// Read `CONFIG1.AVG` as an [`Averaging`].
    ///
    /// The averaging count is device-global, not per-channel; it is the
    /// [`ConversionTiming::averaging`] field. Limit alerts do not see it: the
    /// device compares every conversion rather than the average, as
    /// [`Flags::limit_alerts`] describes.
    ///
    /// This performs one uncached device read, so it reflects a power cycle,
    /// an EN-pin toggle, a General Call reset, or a write by another bus
    /// controller. See [`Ina4230::set_averaging`] for the write side effects,
    /// and [`Ina4230::conversion_timing`] to read all three timing fields as
    /// one coherent snapshot.
    ///
    /// # Errors
    ///
    /// Returns [`Ina4230Error::Bus`] if an I²C bus error occurs.
    pub async fn averaging(&mut self) -> Result<Averaging, Ina4230Error<I2c::Error>> {
        Ok(self.device.config_1().read_async().await?.avg().into())
    }

    /// Set `CONFIG1.AVG`.
    ///
    /// The averaging count is device-global, not per-channel; it is the
    /// [`ConversionTiming::averaging`] field, and its encodings are
    /// [`Averaging`]. Read it back with [`Ina4230::averaging`]. Averaging
    /// quietens the measurement registers but not the limit alerts, which
    /// compare every conversion — see [`Flags::limit_alerts`].
    ///
    /// Performs a read-modify-write of `CONFIG1`, so only bits 11:9 move and
    /// every other field keeps the value read back from the device.
    ///
    /// # Prefer the combined setter for multi-field changes
    ///
    /// [`Ina4230::set_conversion_timing`] is the cheaper and coherent path
    /// whenever more than one timing field changes: one read-modify-write and
    /// one `CVRF` clear, instead of one of each per independent setter, and no
    /// intermediate timing configuration reaching the device.
    ///
    /// # Writing clears conversion ready
    ///
    /// Every call writes `CONFIG1`, and that clears `CVRF` (datasheet 6.4.1
    /// and Table 7-20). A poll in progress through [`Ina4230::read_flags`]
    /// will therefore wait for the next conversion to complete; see
    /// [`Flags::conversion_ready`].
    ///
    /// # Writing retriggers a triggered mode
    ///
    /// If the selected [`OperatingMode`] is a triggered one, every call starts
    /// another conversion sequence even when the requested count is unchanged;
    /// the write is never suppressed as redundant. See [`Ina4230::set_mode`].
    ///
    /// # Errors
    ///
    /// Returns [`Ina4230Error::Bus`] if an I²C bus error occurs. A failed read
    /// aborts before anything is written; a failed write leaves the device's
    /// timing and triggered-conversion state unknown, since I²C gives no way
    /// to learn whether the device acted on the transaction.
    pub async fn set_averaging(&mut self, averaging: Averaging) -> Result<(), Ina4230Error<I2c::Error>> {
        self.device
            .config_1()
            .modify_async(|w| w.set_avg(averaging.into()))
            .await
    }

    /// Read `CONFIG1.VBUSCT` as a [`BusConversionTime`].
    ///
    /// The bus-voltage conversion time is device-global, not per-channel; it
    /// is the [`ConversionTiming::bus_conversion_time`] field.
    ///
    /// This performs one uncached device read, so it reflects a power cycle,
    /// an EN-pin toggle, a General Call reset, or a write by another bus
    /// controller. See [`Ina4230::set_bus_conversion_time`] for the write side
    /// effects, and [`Ina4230::conversion_timing`] to read all three timing
    /// fields as one coherent snapshot.
    ///
    /// # Errors
    ///
    /// Returns [`Ina4230Error::Bus`] if an I²C bus error occurs.
    pub async fn bus_conversion_time(&mut self) -> Result<BusConversionTime, Ina4230Error<I2c::Error>> {
        Ok(self.device.config_1().read_async().await?.vbusct().into())
    }

    /// Set `CONFIG1.VBUSCT`.
    ///
    /// The bus-voltage conversion time is device-global, not per-channel; it
    /// is the [`ConversionTiming::bus_conversion_time`] field, and its
    /// encodings are [`BusConversionTime`]. Read it back with
    /// [`Ina4230::bus_conversion_time`].
    ///
    /// Performs a read-modify-write of `CONFIG1`, so only bits 8:6 move and
    /// every other field keeps the value read back from the device.
    ///
    /// # Prefer the combined setter for multi-field changes
    ///
    /// [`Ina4230::set_conversion_timing`] is the cheaper and coherent path
    /// whenever more than one timing field changes: one read-modify-write and
    /// one `CVRF` clear, instead of one of each per independent setter, and no
    /// intermediate timing configuration reaching the device.
    ///
    /// # Writing clears conversion ready
    ///
    /// Every call writes `CONFIG1`, and that clears `CVRF` (datasheet 6.4.1
    /// and Table 7-20). A poll in progress through [`Ina4230::read_flags`]
    /// will therefore wait for the next conversion to complete; see
    /// [`Flags::conversion_ready`].
    ///
    /// # Writing retriggers a triggered mode
    ///
    /// If the selected [`OperatingMode`] is a triggered one, every call starts
    /// another conversion sequence even when the requested time is unchanged;
    /// the write is never suppressed as redundant. See [`Ina4230::set_mode`].
    ///
    /// # Errors
    ///
    /// Returns [`Ina4230Error::Bus`] if an I²C bus error occurs. A failed read
    /// aborts before anything is written; a failed write leaves the device's
    /// timing and triggered-conversion state unknown, since I²C gives no way
    /// to learn whether the device acted on the transaction.
    pub async fn set_bus_conversion_time(
        &mut self,
        conversion_time: BusConversionTime,
    ) -> Result<(), Ina4230Error<I2c::Error>> {
        self.device
            .config_1()
            .modify_async(|w| w.set_vbusct(conversion_time.into()))
            .await
    }

    /// Read `CONFIG1.VSHCT` as a [`ShuntConversionTime`].
    ///
    /// The shunt-voltage conversion time is device-global, not per-channel; it
    /// is the [`ConversionTiming::shunt_conversion_time`] field.
    ///
    /// This performs one uncached device read, so it reflects a power cycle,
    /// an EN-pin toggle, a General Call reset, or a write by another bus
    /// controller. See [`Ina4230::set_shunt_conversion_time`] for the write
    /// side effects, and [`Ina4230::conversion_timing`] to read all three
    /// timing fields as one coherent snapshot.
    ///
    /// # Errors
    ///
    /// Returns [`Ina4230Error::Bus`] if an I²C bus error occurs.
    pub async fn shunt_conversion_time(&mut self) -> Result<ShuntConversionTime, Ina4230Error<I2c::Error>> {
        Ok(self.device.config_1().read_async().await?.vshct().into())
    }

    /// Set `CONFIG1.VSHCT`.
    ///
    /// The shunt-voltage conversion time is device-global, not per-channel; it
    /// is the [`ConversionTiming::shunt_conversion_time`] field, and its
    /// encodings are [`ShuntConversionTime`]. Read it back with
    /// [`Ina4230::shunt_conversion_time`].
    ///
    /// Performs a read-modify-write of `CONFIG1`, so only bits 5:3 move and
    /// every other field keeps the value read back from the device.
    ///
    /// # Prefer the combined setter for multi-field changes
    ///
    /// [`Ina4230::set_conversion_timing`] is the cheaper and coherent path
    /// whenever more than one timing field changes: one read-modify-write and
    /// one `CVRF` clear, instead of one of each per independent setter, and no
    /// intermediate timing configuration reaching the device.
    ///
    /// # Writing clears conversion ready
    ///
    /// Every call writes `CONFIG1`, and that clears `CVRF` (datasheet 6.4.1
    /// and Table 7-20). A poll in progress through [`Ina4230::read_flags`]
    /// will therefore wait for the next conversion to complete; see
    /// [`Flags::conversion_ready`].
    ///
    /// # Writing retriggers a triggered mode
    ///
    /// If the selected [`OperatingMode`] is a triggered one, every call starts
    /// another conversion sequence even when the requested time is unchanged;
    /// the write is never suppressed as redundant. See [`Ina4230::set_mode`].
    ///
    /// # Errors
    ///
    /// Returns [`Ina4230Error::Bus`] if an I²C bus error occurs. A failed read
    /// aborts before anything is written; a failed write leaves the device's
    /// timing and triggered-conversion state unknown, since I²C gives no way
    /// to learn whether the device acted on the transaction.
    pub async fn set_shunt_conversion_time(
        &mut self,
        conversion_time: ShuntConversionTime,
    ) -> Result<(), Ina4230Error<I2c::Error>> {
        self.device
            .config_1()
            .modify_async(|w| w.set_vshct(conversion_time.into()))
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
