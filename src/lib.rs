//! Platform-agnostic Rust driver for the Texas Instruments INA4230 quad-channel
//! power and energy sense monitor, based on the [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://docs.rs/embedded-hal
//!
//! For further details of the device architecture and operation, please refer
//! to the official [`Datasheet`].
//!
//! [`Datasheet`]: https://www.ti.com/lit/ds/symlink/ina4230.pdf

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
pub use crate::device::*;

/// Maximum register data size in bytes (energy registers are 32-bit = 4 bytes).
const LARGEST_REG_SIZE_BYTES: usize = 4;

// ── Error type ────────────────────────────────────────────────────────────────

/// INA4230 driver error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ina4230Error<I2cError> {
    /// An error occurred on the I²C bus.
    Bus(I2cError),
    /// A measurement was requested before calibration was configured.
    NotCalibrated,
    /// Math overflow — current and power data may be invalid.
    /// Occurs when the current exceeds the configured full-scale range.
    MathOverflow,
    /// Energy register overflow on the specified channel.
    /// Occurs when the accumulated energy exceeds the 40-bit register maximum.
    EnergyOverflow(Channel),
}

impl<E: embedded_hal_async::i2c::Error> sensor::Error for Ina4230Error<E> {
    fn kind(&self) -> sensor::ErrorKind {
        match self {
            Self::Bus(_) => sensor::ErrorKind::Peripheral,
            Self::NotCalibrated => sensor::ErrorKind::NotReady,
            Self::MathOverflow | Self::EnergyOverflow(_) => sensor::ErrorKind::Saturated,
        }
    }
}

// ── DeviceInterface ───────────────────────────────────────────────────────────

/// Async I²C interface adapter for the INA4230.
pub struct DeviceInterface<I2c: embedded_hal_async::i2c::I2c> {
    /// The underlying async I²C bus.
    pub i2c: I2c,
    /// 7-bit I²C address of this device instance.
    pub address: u8,
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

// ── Address pins ──────────────────────────────────────────────────────────────

/// Logic level of an I²C address pin (A0 or A1) for device address selection.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AddrPinState {
    /// Address pin tied to GND (default).
    #[default]
    Gnd,
    /// Address pin tied to VS.
    Vs,
    /// Address pin tied to SDA.
    Sda,
    /// Address pin tied to SCL.
    Scl,
}

/// Trait for converting an (A0, A1) pair of address pin states into
/// the corresponding I²C address.
pub trait ToAddress {
    /// Convert pin strapping to a 7-bit I²C address.
    fn to_address(self) -> u8;
}

impl ToAddress for (AddrPinState, AddrPinState) {
    fn to_address(self) -> u8 {
        match self {
            (AddrPinState::Gnd, AddrPinState::Gnd) => 0x40,
            (AddrPinState::Vs, AddrPinState::Gnd) => 0x41,
            (AddrPinState::Gnd, AddrPinState::Sda) => 0x42,
            (AddrPinState::Gnd, AddrPinState::Scl) => 0x43,
            (AddrPinState::Gnd, AddrPinState::Vs) => 0x44,
            (AddrPinState::Vs, AddrPinState::Vs) => 0x45,
            (AddrPinState::Vs, AddrPinState::Sda) => 0x46,
            (AddrPinState::Vs, AddrPinState::Scl) => 0x47,
            (AddrPinState::Sda, AddrPinState::Gnd) => 0x48,
            (AddrPinState::Sda, AddrPinState::Vs) => 0x49,
            (AddrPinState::Sda, AddrPinState::Sda) => 0x4A,
            (AddrPinState::Sda, AddrPinState::Scl) => 0x4B,
            (AddrPinState::Scl, AddrPinState::Gnd) => 0x4C,
            (AddrPinState::Scl, AddrPinState::Vs) => 0x4D,
            (AddrPinState::Scl, AddrPinState::Sda) => 0x4E,
            (AddrPinState::Scl, AddrPinState::Scl) => 0x4F,
        }
    }
}

// ── Channel ───────────────────────────────────────────────────────────────────

/// One of the four measurement channels on the INA4230.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(usize)]
pub enum Channel {
    /// Channel 1
    Ch1 = 0,
    /// Channel 2
    Ch2 = 1,
    /// Channel 3
    Ch3 = 2,
    /// Channel 4
    Ch4 = 3,
}

impl Channel {
    fn to_bit(self) -> u8 {
        match self {
            Channel::Ch1 => 0b0001,
            Channel::Ch2 => 0b0010,
            Channel::Ch3 => 0b0100,
            Channel::Ch4 => 0b1000,
        }
    }
}

// ── ADC Range ─────────────────────────────────────────────────────────────────

/// ADC full-scale input range for shunt voltage measurement.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AdcRange {
    /// ±81.92 mV full scale, LSB = 2.5 µV (default)
    #[default]
    Range0,
    /// ±20.48 mV full scale, LSB = 625 nV. `SHUNT_CAL` divided by 4.
    Range1,
}

// ── Physical-unit type aliases ────────────────────────────────────────────────

/// Voltage in millivolts.
pub type MilliVolts = f32;
/// Current in milliamperes.
pub type MilliAmps = f32;
/// Power in milliwatts.
pub type MilliWatts = f32;
/// Energy in millijoules.
pub type MilliJoules = f32;

// ── Sensor traits ─────────────────────────────────────────────────────────────

/// Async voltage sensor — reads bus or shunt voltage per channel.
pub trait VoltageSensor: sensor::ErrorType {
    /// Read the bus voltage for the given channel, in millivolts (LSB = 1.6 mV).
    async fn bus_voltage(&mut self, channel: Channel) -> Result<MilliVolts, Self::Error>;
    /// Read the shunt voltage for the given channel, in millivolts.
    async fn shunt_voltage(&mut self, channel: Channel) -> Result<MilliVolts, Self::Error>;
}

impl<T: VoltageSensor + ?Sized> VoltageSensor for &mut T {
    async fn bus_voltage(&mut self, channel: Channel) -> Result<MilliVolts, Self::Error> {
        T::bus_voltage(self, channel).await
    }
    async fn shunt_voltage(&mut self, channel: Channel) -> Result<MilliVolts, Self::Error> {
        T::shunt_voltage(self, channel).await
    }
}

/// Async current sensor — reads calculated current per channel.
pub trait CurrentSensor: sensor::ErrorType {
    /// Read the calculated current for the given channel, in milliamperes.
    /// Requires [`Ina4230::calibrate`] to have been called first.
    async fn current(&mut self, channel: Channel) -> Result<MilliAmps, Self::Error>;
}

impl<T: CurrentSensor + ?Sized> CurrentSensor for &mut T {
    async fn current(&mut self, channel: Channel) -> Result<MilliAmps, Self::Error> {
        T::current(self, channel).await
    }
}

/// Async power sensor — reads calculated power per channel.
pub trait PowerSensor: sensor::ErrorType {
    /// Read the calculated power for the given channel, in milliwatts.
    /// Requires [`Ina4230::calibrate`] to have been called first.
    async fn power(&mut self, channel: Channel) -> Result<MilliWatts, Self::Error>;
}

impl<T: PowerSensor + ?Sized> PowerSensor for &mut T {
    async fn power(&mut self, channel: Channel) -> Result<MilliWatts, Self::Error> {
        T::power(self, channel).await
    }
}

/// Async energy sensor — reads accumulated energy per channel.
pub trait EnergySensor: sensor::ErrorType {
    /// Read the accumulated energy for the given channel, in millijoules.
    /// Requires [`Ina4230::calibrate`] to have been called first.
    async fn energy(&mut self, channel: Channel) -> Result<MilliJoules, Self::Error>;
}

impl<T: EnergySensor + ?Sized> EnergySensor for &mut T {
    async fn energy(&mut self, channel: Channel) -> Result<MilliJoules, Self::Error> {
        T::energy(self, channel).await
    }
}

// ── Ina4230 driver struct ─────────────────────────────────────────────────────

/// High-level driver for the INA4230 quad-channel power and energy monitor.
pub struct Ina4230<I2c: embedded_hal_async::i2c::I2c> {
    /// The generated low-level register accessor.
    device: Device<DeviceInterface<I2c>>,
    /// `CURRENT_LSB` per channel in A/LSB. None means not yet calibrated.
    current_lsb_a: [Option<f32>; 4],
    /// ADC input range per channel.
    adc_range: [AdcRange; 4],
}

impl<I2c: embedded_hal_async::i2c::I2c> Ina4230<I2c> {
    /// Create a new driver instance.
    ///
    /// `a0` and `a1` select the I²C address via the pin strapping on the device.
    pub fn new(i2c: I2c, a0: AddrPinState, a1: AddrPinState) -> Self {
        Self {
            device: Device::new(DeviceInterface {
                i2c,
                address: (a0, a1).to_address(),
            }),
            current_lsb_a: [None; 4],
            adc_range: [AdcRange::Range0; 4],
        }
    }

    /// Release the underlying I²C bus.
    pub fn release(self) -> I2c {
        self.device.free().i2c
    }

    // ── Device management ─────────────────────────────────────────────────

    /// Issue a full device reset (`CONFIG2.RST = 1`). All registers return to
    /// power-on defaults. The bit self-clears.
    ///
    /// # Errors
    ///
    /// Returns [`Ina4230Error::Bus`] if an I²C bus error occurs.
    pub async fn reset(&mut self) -> Result<(), Ina4230Error<I2c::Error>> {
        self.device.config_2().write_async(|w| w.set_rst(true)).await
    }

    /// Read the manufacturer ID register. Returns `0x5449` ("TI") on a healthy device.
    ///
    /// # Errors
    ///
    /// Returns [`Ina4230Error::Bus`] if an I²C bus error occurs.
    pub async fn manufacturer_id(&mut self) -> Result<u16, Ina4230Error<I2c::Error>> {
        Ok(self.device.manufacturer_id().read_async().await?.id())
    }

    /// Poll the Conversion Ready Flag (`FLAGS.CVRF`). Returns `true` when all
    /// enabled channels have completed conversion and averaging.
    ///
    /// # Errors
    ///
    /// Returns [`Ina4230Error::Bus`] if an I²C bus error occurs.
    pub async fn conversion_ready(&mut self) -> Result<bool, Ina4230Error<I2c::Error>> {
        Ok(self.device.flags().read_async().await?.cvrf())
    }

    /// Read the full flags register in one call.
    ///
    /// # Errors
    ///
    /// Returns [`Ina4230Error::Bus`] if an I²C bus error occurs.
    pub async fn flags(&mut self) -> Result<Flags, Ina4230Error<I2c::Error>> {
        self.device.flags().read_async().await
    }

    /// Enable or disable a channel in `CONFIG1.ACTIVE_CHANNEL`.
    ///
    /// # Errors
    ///
    /// Returns [`Ina4230Error::Bus`] if an I²C bus error occurs.
    pub async fn set_channel_active(&mut self, channel: Channel, active: bool) -> Result<(), Ina4230Error<I2c::Error>> {
        let bit = channel.to_bit();
        self.device
            .config_1()
            .modify_async(|w| {
                let mut active_channels = w.active_channel();
                if active {
                    active_channels |= bit;
                } else {
                    active_channels &= !bit;
                }
                w.set_active_channel(active_channels);
            })
            .await
    }

    /// Check the FLAGS register for overflow conditions.
    ///
    /// Returns [`Ina4230Error::MathOverflow`] if current or power data may be
    /// invalid, or [`Ina4230Error::EnergyOverflow`] if the energy accumulator
    /// has overflowed on any channel. Reading FLAGS clears all flags.
    ///
    /// # Errors
    ///
    /// Returns [`Ina4230Error::Bus`] if an I²C bus error occurs.
    /// Returns [`Ina4230Error::MathOverflow`] if the math overflow flag is set.
    /// Returns [`Ina4230Error::EnergyOverflow`] if an energy overflow flag is set.
    pub async fn check_flags(&mut self) -> Result<(), Ina4230Error<I2c::Error>> {
        let flags = self.device.flags().read_async().await?;
        if flags.ovf() {
            Err(Ina4230Error::MathOverflow)
        } else if flags.energyof_ch1() {
            Err(Ina4230Error::EnergyOverflow(Channel::Ch1))
        } else if flags.energyof_ch2() {
            Err(Ina4230Error::EnergyOverflow(Channel::Ch2))
        } else if flags.energyof_ch3() {
            Err(Ina4230Error::EnergyOverflow(Channel::Ch3))
        } else if flags.energyof_ch4() {
            Err(Ina4230Error::EnergyOverflow(Channel::Ch4))
        } else {
            Ok(())
        }
    }

    // ── Calibration ───────────────────────────────────────────────────────

    /// Write the calibration register for a single channel.
    ///
    /// - `current_lsb_a`: desired current resolution in A/LSB (e.g. `100e-6` for 100 µA/LSB)
    /// - `shunt_ohms`: shunt resistor value in Ω (e.g. `0.010` for 10 mΩ)
    /// - `adc_range`: ADC full-scale input range (use [`AdcRange::Range0`] for default ±81.92 mV)
    ///
    /// Formula (ADCRANGE = 0): `SHUNT_CAL = 0.00512 / (CURRENT_LSB × R_SHUNT)`
    /// Formula (ADCRANGE = 1): `SHUNT_CAL = 0.00512 / (CURRENT_LSB × R_SHUNT) / 4`
    ///
    /// # Errors
    ///
    /// Returns [`Ina4230Error::Bus`] if an I²C bus error occurs.
    pub async fn calibrate(
        &mut self,
        channel: Channel,
        current_lsb_a: f32,
        shunt_ohms: f32,
        adc_range: AdcRange,
    ) -> Result<(), Ina4230Error<I2c::Error>> {
        let idx = channel as usize;
        self.current_lsb_a[idx] = Some(current_lsb_a);
        self.adc_range[idx] = adc_range;

        // Update CONFIG2.RANGE bit for this channel
        // Bit0=CH1, Bit1=CH2, Bit2=CH3, Bit3=CH4
        let bit = 1u8 << idx;
        self.device
            .config_2()
            .modify_async(|w| {
                let mut range = w.range();
                match adc_range {
                    AdcRange::Range0 => range &= !bit,
                    AdcRange::Range1 => range |= bit,
                }
                w.set_range(range);
            })
            .await?;

        // Write the calibration register
        let cal = Self::shunt_cal_value(current_lsb_a, shunt_ohms, adc_range);
        match channel {
            Channel::Ch1 => {
                self.device
                    .calibration_ch_1()
                    .write_async(|w| w.set_shunt_cal(cal))
                    .await
            }
            Channel::Ch2 => {
                self.device
                    .calibration_ch_2()
                    .write_async(|w| w.set_shunt_cal(cal))
                    .await
            }
            Channel::Ch3 => {
                self.device
                    .calibration_ch_3()
                    .write_async(|w| w.set_shunt_cal(cal))
                    .await
            }
            Channel::Ch4 => {
                self.device
                    .calibration_ch_4()
                    .write_async(|w| w.set_shunt_cal(cal))
                    .await
            }
        }
    }

    /// Write the calibration register for all four channels.
    ///
    /// Each channel can have independent parameters. `params` is ordered
    /// `[Ch1, Ch2, Ch3, Ch4]` as `(current_lsb_a, shunt_ohms, adc_range)`.
    ///
    /// # Errors
    ///
    /// Returns [`Ina4230Error::Bus`] if an I²C bus error occurs.
    pub async fn calibrate_all(&mut self, params: [(f32, f32, AdcRange); 4]) -> Result<(), Ina4230Error<I2c::Error>> {
        for (ch, (current_lsb_a, shunt_ohms, adc_range)) in [Channel::Ch1, Channel::Ch2, Channel::Ch3, Channel::Ch4]
            .iter()
            .zip(params.iter())
        {
            self.calibrate(*ch, *current_lsb_a, *shunt_ohms, *adc_range).await?;
        }
        Ok(())
    }

    // ── Internal unit-conversion helpers ──────────────────────────────────

    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn shunt_cal_value(current_lsb_a: f32, shunt_ohms: f32, adc_range: AdcRange) -> u16 {
        const SHUNT_CAL_MAX: u32 = 0x7FFF;
        // INA4230 datasheet §8.1.2: SHUNT_CAL = 0.00512 / (CURRENT_LSB × R_SHUNT)
        let val = 0.00512_f32 / (current_lsb_a * shunt_ohms);
        let val = match adc_range {
            AdcRange::Range0 => val,
            AdcRange::Range1 => val / 4.0,
        };
        (val as u32).min(SHUNT_CAL_MAX) as u16
    }

    fn bus_mv(raw: u16) -> MilliVolts {
        f32::from(raw) * 1.6
    }

    #[allow(clippy::cast_possible_wrap)]
    fn shunt_mv(&self, channel: Channel, raw: u16) -> MilliVolts {
        let signed = raw as i16;
        let lsb_mv = match self.adc_range[channel as usize] {
            AdcRange::Range0 => 0.0025,    // 2.5 µV
            AdcRange::Range1 => 0.000_625, // 625 nV
        };
        f32::from(signed) * lsb_mv
    }

    #[allow(clippy::cast_possible_wrap)]
    fn current_ma(&self, channel: Channel, raw: u16) -> Result<MilliAmps, Ina4230Error<I2c::Error>> {
        let lsb = self.current_lsb_a[channel as usize].ok_or(Ina4230Error::NotCalibrated)?;
        let signed = raw as i16;
        Ok(f32::from(signed) * lsb * 1000.0)
    }

    fn power_mw(&self, channel: Channel, raw: u16) -> Result<MilliWatts, Ina4230Error<I2c::Error>> {
        let lsb = self.current_lsb_a[channel as usize].ok_or(Ina4230Error::NotCalibrated)?;
        Ok(f32::from(raw) * 32.0 * lsb * 1000.0)
    }

    fn energy_mj(&self, channel: Channel, raw: u32) -> Result<MilliJoules, Ina4230Error<I2c::Error>> {
        let lsb = self.current_lsb_a[channel as usize].ok_or(Ina4230Error::NotCalibrated)?;
        #[allow(clippy::cast_precision_loss)]
        Ok(raw as f32 * 32.0 * lsb * 1000.0)
    }
}

// ── Trait implementations ─────────────────────────────────────────────────────

impl<I2c: embedded_hal_async::i2c::I2c> sensor::ErrorType for Ina4230<I2c> {
    type Error = Ina4230Error<I2c::Error>;
}

impl<I2c: embedded_hal_async::i2c::I2c> VoltageSensor for Ina4230<I2c> {
    async fn bus_voltage(&mut self, channel: Channel) -> Result<MilliVolts, Self::Error> {
        let raw = match channel {
            Channel::Ch1 => self.device.bus_voltage_ch_1().read_async().await?.vbus(),
            Channel::Ch2 => self.device.bus_voltage_ch_2().read_async().await?.vbus(),
            Channel::Ch3 => self.device.bus_voltage_ch_3().read_async().await?.vbus(),
            Channel::Ch4 => self.device.bus_voltage_ch_4().read_async().await?.vbus(),
        };
        Ok(Self::bus_mv(raw))
    }

    async fn shunt_voltage(&mut self, channel: Channel) -> Result<MilliVolts, Self::Error> {
        let raw = match channel {
            Channel::Ch1 => self.device.shunt_voltage_ch_1().read_async().await?.vshunt(),
            Channel::Ch2 => self.device.shunt_voltage_ch_2().read_async().await?.vshunt(),
            Channel::Ch3 => self.device.shunt_voltage_ch_3().read_async().await?.vshunt(),
            Channel::Ch4 => self.device.shunt_voltage_ch_4().read_async().await?.vshunt(),
        };
        Ok(self.shunt_mv(channel, raw))
    }
}

impl<I2c: embedded_hal_async::i2c::I2c> CurrentSensor for Ina4230<I2c> {
    async fn current(&mut self, channel: Channel) -> Result<MilliAmps, Self::Error> {
        let raw = match channel {
            Channel::Ch1 => self.device.current_ch_1().read_async().await?.current(),
            Channel::Ch2 => self.device.current_ch_2().read_async().await?.current(),
            Channel::Ch3 => self.device.current_ch_3().read_async().await?.current(),
            Channel::Ch4 => self.device.current_ch_4().read_async().await?.current(),
        };
        self.current_ma(channel, raw)
    }
}

impl<I2c: embedded_hal_async::i2c::I2c> PowerSensor for Ina4230<I2c> {
    async fn power(&mut self, channel: Channel) -> Result<MilliWatts, Self::Error> {
        let raw = match channel {
            Channel::Ch1 => self.device.power_ch_1().read_async().await?.power(),
            Channel::Ch2 => self.device.power_ch_2().read_async().await?.power(),
            Channel::Ch3 => self.device.power_ch_3().read_async().await?.power(),
            Channel::Ch4 => self.device.power_ch_4().read_async().await?.power(),
        };
        self.power_mw(channel, raw)
    }
}

impl<I2c: embedded_hal_async::i2c::I2c> EnergySensor for Ina4230<I2c> {
    async fn energy(&mut self, channel: Channel) -> Result<MilliJoules, Self::Error> {
        let raw = match channel {
            Channel::Ch1 => self.device.energy_ch_1().read_async().await?.energy(),
            Channel::Ch2 => self.device.energy_ch_2().read_async().await?.energy(),
            Channel::Ch3 => self.device.energy_ch_3().read_async().await?.energy(),
            Channel::Ch4 => self.device.energy_ch_4().read_async().await?.energy(),
        };
        self.energy_mj(channel, raw)
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use device_driver::Block;
    use embedded_hal_mock::eh1::i2c::{Mock, Transaction};

    use super::*;

    #[tokio::test]
    async fn read_manufacturer_id() {
        // ManufacturerId: address 0x7E, 2 bytes BE, resets to 0x5449 ("TI" in ASCII)
        let expectations = vec![Transaction::write_read(
            (AddrPinState::Gnd, AddrPinState::Gnd).to_address(),
            vec![0x7E],
            vec![0x54, 0x49],
        )];
        let i2c = Mock::new(&expectations);
        let mut dev = Device::new(DeviceInterface {
            i2c,
            address: (AddrPinState::Gnd, AddrPinState::Gnd).to_address(),
        });
        let id = dev.manufacturer_id().read_async().await.unwrap();
        assert_eq!(id.id(), 0x5449);
        dev.interface().i2c.done();
    }

    #[tokio::test]
    async fn write_calibration_ch1() {
        // calibration_ch_1: address 0x05, 2 bytes BE
        // shunt_cal for 100µA/LSB, 10mΩ: 0.00512 / (100e-6 * 0.010) = 5120
        let cal: u16 = 5120;
        let [hi, lo] = cal.to_be_bytes();
        let expectations = vec![Transaction::write(
            (AddrPinState::Gnd, AddrPinState::Gnd).to_address(),
            vec![0x05, hi, lo],
        )];
        let i2c = Mock::new(&expectations);
        let mut dev = Device::new(DeviceInterface {
            i2c,
            address: (AddrPinState::Gnd, AddrPinState::Gnd).to_address(),
        });
        dev.calibration_ch_1()
            .write_async(|w| w.set_shunt_cal(cal))
            .await
            .unwrap();
        dev.interface().i2c.done();
    }

    #[tokio::test]
    async fn bus_voltage_ch1_trait() {
        // bus_voltage_ch_1: address 0x01, 2 bytes BE
        // raw = 5000 → 5000 * 1.6 mV = 8000.0 mV
        let raw: u16 = 5000;
        let [hi, lo] = raw.to_be_bytes();
        let expectations = vec![Transaction::write_read(
            (AddrPinState::Gnd, AddrPinState::Gnd).to_address(),
            vec![0x01],
            vec![hi, lo],
        )];
        let i2c = Mock::new(&expectations);
        let mut sensor = Ina4230::new(i2c, AddrPinState::Gnd, AddrPinState::Gnd);
        let mv = sensor.bus_voltage(Channel::Ch1).await.unwrap();
        assert!((mv - 8000.0).abs() < 0.1, "expected 8000.0 mV, got {mv}");
        sensor.release().done();
    }

    #[tokio::test]
    async fn current_ch1_trait() {
        let cal: u16 = 5120;
        let [cal_hi, cal_lo] = cal.to_be_bytes();
        let raw: u16 = 1000;
        let [hi, lo] = raw.to_be_bytes();
        let addr = (AddrPinState::Gnd, AddrPinState::Gnd).to_address();
        let expectations = vec![
            // calibrate: read CONFIG2
            Transaction::write_read(addr, vec![0x21], vec![0x00, 0x00]),
            // calibrate: write CONFIG2 with range bit cleared
            Transaction::write(addr, vec![0x21, 0x00, 0x00]),
            // calibrate: write calibration_ch_1 (0x05)
            Transaction::write(addr, vec![0x05, cal_hi, cal_lo]),
            // current read (0x02)
            Transaction::write_read(addr, vec![0x02], vec![hi, lo]),
        ];
        let i2c = Mock::new(&expectations);
        let mut sensor = Ina4230::new(i2c, AddrPinState::Gnd, AddrPinState::Gnd);
        sensor
            .calibrate(Channel::Ch1, 100e-6, 0.010, AdcRange::Range0)
            .await
            .unwrap();
        let ma = sensor.current(Channel::Ch1).await.unwrap();
        assert!((ma - 100.0).abs() < 0.01, "expected 100.0 mA, got {ma}");
        sensor.release().done();
    }

    #[tokio::test]
    async fn current_returns_error_when_not_calibrated() {
        // Attempting to read current before calibrate() should return NotCalibrated
        let raw: u16 = 1000;
        let [hi, lo] = raw.to_be_bytes();
        let expectations = vec![Transaction::write_read(
            (AddrPinState::Gnd, AddrPinState::Gnd).to_address(),
            vec![0x02],
            vec![hi, lo],
        )];
        let i2c = Mock::new(&expectations);
        let mut sensor = Ina4230::new(i2c, AddrPinState::Gnd, AddrPinState::Gnd);
        let result = sensor.current(Channel::Ch1).await;
        assert!(matches!(result, Err(Ina4230Error::NotCalibrated)));
        sensor.release().done();
    }

    #[tokio::test]
    async fn calibrate_stores_per_channel() {
        let cal: u16 = 5120;
        let [cal_hi, cal_lo] = cal.to_be_bytes();
        let raw: u16 = 1000;
        let [hi, lo] = raw.to_be_bytes();
        let addr = (AddrPinState::Gnd, AddrPinState::Gnd).to_address();
        let expectations = vec![
            // calibrate CH1: read CONFIG2
            Transaction::write_read(addr, vec![0x21], vec![0x00, 0x00]),
            // calibrate CH1: write CONFIG2
            Transaction::write(addr, vec![0x21, 0x00, 0x00]),
            // calibrate CH1: write calibration_ch_1 (0x05)
            Transaction::write(addr, vec![0x05, cal_hi, cal_lo]),
            // current CH2 read (0x0A)
            Transaction::write_read(addr, vec![0x0A], vec![hi, lo]),
        ];
        let i2c = Mock::new(&expectations);
        let mut sensor = Ina4230::new(i2c, AddrPinState::Gnd, AddrPinState::Gnd);
        sensor
            .calibrate(Channel::Ch1, 100e-6, 0.010, AdcRange::Range0)
            .await
            .unwrap();
        let result = sensor.current(Channel::Ch2).await;
        assert!(matches!(result, Err(Ina4230Error::NotCalibrated)));
        sensor.release().done();
    }

    #[tokio::test]
    async fn bus_voltage_all_channels() {
        // Verify correct register addresses for all four bus voltage channels:
        // CH1=0x01, CH2=0x09, CH3=0x11, CH4=0x19
        let raw: u16 = 2000; // 2000 * 1.6 mV = 3200.0 mV
        let [hi, lo] = raw.to_be_bytes();
        let expectations = vec![
            Transaction::write_read(
                (AddrPinState::Gnd, AddrPinState::Gnd).to_address(),
                vec![0x01],
                vec![hi, lo],
            ),
            Transaction::write_read(
                (AddrPinState::Gnd, AddrPinState::Gnd).to_address(),
                vec![0x09],
                vec![hi, lo],
            ),
            Transaction::write_read(
                (AddrPinState::Gnd, AddrPinState::Gnd).to_address(),
                vec![0x11],
                vec![hi, lo],
            ),
            Transaction::write_read(
                (AddrPinState::Gnd, AddrPinState::Gnd).to_address(),
                vec![0x19],
                vec![hi, lo],
            ),
        ];
        let i2c = Mock::new(&expectations);
        let mut sensor = Ina4230::new(i2c, AddrPinState::Gnd, AddrPinState::Gnd);
        for ch in [Channel::Ch1, Channel::Ch2, Channel::Ch3, Channel::Ch4] {
            let mv = sensor.bus_voltage(ch).await.unwrap();
            assert!((mv - 3200.0).abs() < 0.1, "expected 3200.0 mV, got {mv}");
        }
        sensor.release().done();
    }

    #[tokio::test]
    async fn calibrate_all_independent_channels() {
        let ch1_cal: u16 = 5120;
        let ch2_cal: u16 = 1280;
        let ch3_cal: u16 = 20480;
        let ch4_cal: u16 = 5120;
        let [h1, l1] = ch1_cal.to_be_bytes();
        let [h2, l2] = ch2_cal.to_be_bytes();
        let [h3, l3] = ch3_cal.to_be_bytes();
        let [h4, l4] = ch4_cal.to_be_bytes();
        let addr = (AddrPinState::Gnd, AddrPinState::Gnd).to_address();
        let expectations = vec![
            // calibrate CH1: read/write CONFIG2, write cal reg
            Transaction::write_read(addr, vec![0x21], vec![0x00, 0x00]),
            Transaction::write(addr, vec![0x21, 0x00, 0x00]),
            Transaction::write(addr, vec![0x05, h1, l1]),
            // calibrate CH2: read/write CONFIG2, write cal reg
            Transaction::write_read(addr, vec![0x21], vec![0x00, 0x00]),
            Transaction::write(addr, vec![0x21, 0x00, 0x00]),
            Transaction::write(addr, vec![0x0D, h2, l2]),
            // calibrate CH3: read/write CONFIG2, write cal reg
            Transaction::write_read(addr, vec![0x21], vec![0x00, 0x00]),
            Transaction::write(addr, vec![0x21, 0x00, 0x00]),
            Transaction::write(addr, vec![0x15, h3, l3]),
            // calibrate CH4: read/write CONFIG2, write cal reg
            Transaction::write_read(addr, vec![0x21], vec![0x00, 0x00]),
            Transaction::write(addr, vec![0x21, 0x00, 0x00]),
            Transaction::write(addr, vec![0x1D, h4, l4]),
        ];
        let i2c = Mock::new(&expectations);
        let mut sensor = Ina4230::new(i2c, AddrPinState::Gnd, AddrPinState::Gnd);
        sensor
            .calibrate_all([
                (100e-6, 0.010, AdcRange::Range0),
                (200e-6, 0.020, AdcRange::Range0),
                (50e-6, 0.005, AdcRange::Range0),
                (100e-6, 0.010, AdcRange::Range0),
            ])
            .await
            .unwrap();
        sensor.release().done();
    }

    #[tokio::test]
    async fn i2c_address_all_combinations() {
        // Verify all 16 address pin combinations per datasheet Table 6-1
        assert_eq!((AddrPinState::Gnd, AddrPinState::Gnd).to_address(), 0x40);
        assert_eq!((AddrPinState::Vs, AddrPinState::Gnd).to_address(), 0x41);
        assert_eq!((AddrPinState::Gnd, AddrPinState::Sda).to_address(), 0x42);
        assert_eq!((AddrPinState::Gnd, AddrPinState::Scl).to_address(), 0x43);
        assert_eq!((AddrPinState::Gnd, AddrPinState::Vs).to_address(), 0x44);
        assert_eq!((AddrPinState::Vs, AddrPinState::Vs).to_address(), 0x45);
        assert_eq!((AddrPinState::Vs, AddrPinState::Sda).to_address(), 0x46);
        assert_eq!((AddrPinState::Vs, AddrPinState::Scl).to_address(), 0x47);
        assert_eq!((AddrPinState::Sda, AddrPinState::Gnd).to_address(), 0x48);
        assert_eq!((AddrPinState::Sda, AddrPinState::Vs).to_address(), 0x49);
        assert_eq!((AddrPinState::Sda, AddrPinState::Sda).to_address(), 0x4A);
        assert_eq!((AddrPinState::Sda, AddrPinState::Scl).to_address(), 0x4B);
        assert_eq!((AddrPinState::Scl, AddrPinState::Gnd).to_address(), 0x4C);
        assert_eq!((AddrPinState::Scl, AddrPinState::Vs).to_address(), 0x4D);
        assert_eq!((AddrPinState::Scl, AddrPinState::Sda).to_address(), 0x4E);
        assert_eq!((AddrPinState::Scl, AddrPinState::Scl).to_address(), 0x4F);
    }
}
