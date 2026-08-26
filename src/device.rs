// This code was generated using device-driver `2.1.0` (),
// a tool distributed under MIT OR Apache-2.0 by Dion Dokter <dev@diondokter.nl>
//
// For more information about device-driver, visit the website: https://device-driver.com

/// Root block of the Device driver
#[derive(Debug)]
pub struct Device<I> {
    interface: I,
    #[doc(hidden)]
    #[allow(unused)]
    base_address: u8,
}
impl<I> Device<I> {
    /// Create a new instance of the device
    pub const fn new(interface: I) -> Self {
        Self {
            interface,
            base_address: 0,
        }
    }
    /// Drop the driver instance and reclaim the interface
    pub fn free(self) -> I {
        self.interface
    }
    /// Configuration register 1
    ///
    /// Register operation:
    /// - Address: `32`
    /// - Reset value: `0xF127`
    #[doc(alias = "config-1")]
    pub fn config_1(&mut self) -> ::device_driver::RegisterOperation<'_, Self, Config1, u8, ::device_driver::RW, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 32;
        ::device_driver::RegisterOperation::new(self, address as u8, || Config1::from([241, 39]))
    }
    /// Configuration register 2
    ///
    /// Register operation:
    /// - Address: `33`
    /// - Reset value: `0x00`
    #[doc(alias = "config-2")]
    pub fn config_2(&mut self) -> ::device_driver::RegisterOperation<'_, Self, Config2, u8, ::device_driver::RW, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 33;
        ::device_driver::RegisterOperation::new(self, address as u8, || Config2::from([0, 0]))
    }
    /// Calibration register channel 1
    ///
    /// Register operation:
    /// - Address: `5`
    /// - Reset value: `0x00`
    #[doc(alias = "calibration-ch-1")]
    pub fn calibration_ch_1(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, CalibrationCh1, u8, ::device_driver::RW, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 5;
        ::device_driver::RegisterOperation::new(self, address as u8, || CalibrationCh1::from([0, 0]))
    }
    /// Calibration register channel 2
    ///
    /// Register operation:
    /// - Address: `13`
    /// - Reset value: `0x00`
    #[doc(alias = "calibration-ch-2")]
    pub fn calibration_ch_2(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, CalibrationCh2, u8, ::device_driver::RW, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 13;
        ::device_driver::RegisterOperation::new(self, address as u8, || CalibrationCh2::from([0, 0]))
    }
    /// Calibration register channel 3
    ///
    /// Register operation:
    /// - Address: `21`
    /// - Reset value: `0x00`
    #[doc(alias = "calibration-ch-3")]
    pub fn calibration_ch_3(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, CalibrationCh3, u8, ::device_driver::RW, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 21;
        ::device_driver::RegisterOperation::new(self, address as u8, || CalibrationCh3::from([0, 0]))
    }
    /// Calibration register channel 4
    ///
    /// Register operation:
    /// - Address: `29`
    /// - Reset value: `0x00`
    #[doc(alias = "calibration-ch-4")]
    pub fn calibration_ch_4(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, CalibrationCh4, u8, ::device_driver::RW, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 29;
        ::device_driver::RegisterOperation::new(self, address as u8, || CalibrationCh4::from([0, 0]))
    }
    /// Alert configuration register 1
    ///
    /// Register operation:
    /// - Address: `7`
    /// - Reset value: `0x00`
    #[doc(alias = "alert-config-1")]
    pub fn alert_config_1(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, AlertConfig1, u8, ::device_driver::RW, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 7;
        ::device_driver::RegisterOperation::new(self, address as u8, || AlertConfig1::from([0, 0]))
    }
    /// Alert configuration register 2
    ///
    /// Register operation:
    /// - Address: `15`
    /// - Reset value: `0x00`
    #[doc(alias = "alert-config-2")]
    pub fn alert_config_2(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, AlertConfig2, u8, ::device_driver::RW, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 15;
        ::device_driver::RegisterOperation::new(self, address as u8, || AlertConfig2::from([0, 0]))
    }
    /// Alert configuration register 3
    ///
    /// Register operation:
    /// - Address: `23`
    /// - Reset value: `0x00`
    #[doc(alias = "alert-config-3")]
    pub fn alert_config_3(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, AlertConfig3, u8, ::device_driver::RW, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 23;
        ::device_driver::RegisterOperation::new(self, address as u8, || AlertConfig3::from([0, 0]))
    }
    /// Alert configuration register 4
    ///
    /// Register operation:
    /// - Address: `31`
    /// - Reset value: `0x00`
    #[doc(alias = "alert-config-4")]
    pub fn alert_config_4(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, AlertConfig4, u8, ::device_driver::RW, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 31;
        ::device_driver::RegisterOperation::new(self, address as u8, || AlertConfig4::from([0, 0]))
    }
    /// Alert limit register 1. Format matches corresponding result register.
    ///
    /// Register operation:
    /// - Address: `6`
    /// - Reset value: `0x00`
    #[doc(alias = "alert-limit-1")]
    pub fn alert_limit_1(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, AlertLimit1, u8, ::device_driver::RW, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 6;
        ::device_driver::RegisterOperation::new(self, address as u8, || AlertLimit1::from([0, 0]))
    }
    /// Alert limit register 2
    ///
    /// Register operation:
    /// - Address: `14`
    /// - Reset value: `0x00`
    #[doc(alias = "alert-limit-2")]
    pub fn alert_limit_2(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, AlertLimit2, u8, ::device_driver::RW, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 14;
        ::device_driver::RegisterOperation::new(self, address as u8, || AlertLimit2::from([0, 0]))
    }
    /// Alert limit register 3
    ///
    /// Register operation:
    /// - Address: `22`
    /// - Reset value: `0x00`
    #[doc(alias = "alert-limit-3")]
    pub fn alert_limit_3(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, AlertLimit3, u8, ::device_driver::RW, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 22;
        ::device_driver::RegisterOperation::new(self, address as u8, || AlertLimit3::from([0, 0]))
    }
    /// Alert limit register 4
    ///
    /// Register operation:
    /// - Address: `30`
    /// - Reset value: `0x00`
    #[doc(alias = "alert-limit-4")]
    pub fn alert_limit_4(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, AlertLimit4, u8, ::device_driver::RW, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 30;
        ::device_driver::RegisterOperation::new(self, address as u8, || AlertLimit4::from([0, 0]))
    }
    /// Shunt voltage channel 1. 2's complement. LSB = 2.5uV (range=0) or 625nV (range=1)
    ///
    /// Register operation:
    /// - Address: `0`
    /// - Reset value: `0`
    #[doc(alias = "shunt-voltage-ch-1")]
    pub fn shunt_voltage_ch_1(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, ShuntVoltageCh1, u8, ::device_driver::RO, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 0;
        ::device_driver::RegisterOperation::new(self, address as u8, ShuntVoltageCh1::default)
    }
    /// Shunt voltage channel 2
    ///
    /// Register operation:
    /// - Address: `8`
    /// - Reset value: `0`
    #[doc(alias = "shunt-voltage-ch-2")]
    pub fn shunt_voltage_ch_2(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, ShuntVoltageCh2, u8, ::device_driver::RO, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 8;
        ::device_driver::RegisterOperation::new(self, address as u8, ShuntVoltageCh2::default)
    }
    /// Shunt voltage channel 3
    ///
    /// Register operation:
    /// - Address: `16`
    /// - Reset value: `0`
    #[doc(alias = "shunt-voltage-ch-3")]
    pub fn shunt_voltage_ch_3(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, ShuntVoltageCh3, u8, ::device_driver::RO, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 16;
        ::device_driver::RegisterOperation::new(self, address as u8, ShuntVoltageCh3::default)
    }
    /// Shunt voltage channel 4
    ///
    /// Register operation:
    /// - Address: `24`
    /// - Reset value: `0`
    #[doc(alias = "shunt-voltage-ch-4")]
    pub fn shunt_voltage_ch_4(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, ShuntVoltageCh4, u8, ::device_driver::RO, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 24;
        ::device_driver::RegisterOperation::new(self, address as u8, ShuntVoltageCh4::default)
    }
    /// Bus voltage channel 1. Always positive. LSB = 1.6mV
    ///
    /// Register operation:
    /// - Address: `1`
    /// - Reset value: `0`
    #[doc(alias = "bus-voltage-ch-1")]
    pub fn bus_voltage_ch_1(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, BusVoltageCh1, u8, ::device_driver::RO, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 1;
        ::device_driver::RegisterOperation::new(self, address as u8, BusVoltageCh1::default)
    }
    /// Bus voltage channel 2
    ///
    /// Register operation:
    /// - Address: `9`
    /// - Reset value: `0`
    #[doc(alias = "bus-voltage-ch-2")]
    pub fn bus_voltage_ch_2(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, BusVoltageCh2, u8, ::device_driver::RO, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 9;
        ::device_driver::RegisterOperation::new(self, address as u8, BusVoltageCh2::default)
    }
    /// Bus voltage channel 3
    ///
    /// Register operation:
    /// - Address: `17`
    /// - Reset value: `0`
    #[doc(alias = "bus-voltage-ch-3")]
    pub fn bus_voltage_ch_3(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, BusVoltageCh3, u8, ::device_driver::RO, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 17;
        ::device_driver::RegisterOperation::new(self, address as u8, BusVoltageCh3::default)
    }
    /// Bus voltage channel 4
    ///
    /// Register operation:
    /// - Address: `25`
    /// - Reset value: `0`
    #[doc(alias = "bus-voltage-ch-4")]
    pub fn bus_voltage_ch_4(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, BusVoltageCh4, u8, ::device_driver::RO, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 25;
        ::device_driver::RegisterOperation::new(self, address as u8, BusVoltageCh4::default)
    }
    /// Current channel 1. Value [A] = CURRENT_LSB x register_value
    ///
    /// Register operation:
    /// - Address: `2`
    /// - Reset value: `0`
    #[doc(alias = "current-ch-1")]
    pub fn current_ch_1(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, CurrentCh1, u8, ::device_driver::RO, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 2;
        ::device_driver::RegisterOperation::new(self, address as u8, CurrentCh1::default)
    }
    /// Current channel 2
    ///
    /// Register operation:
    /// - Address: `10`
    /// - Reset value: `0`
    #[doc(alias = "current-ch-2")]
    pub fn current_ch_2(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, CurrentCh2, u8, ::device_driver::RO, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 10;
        ::device_driver::RegisterOperation::new(self, address as u8, CurrentCh2::default)
    }
    /// Current channel 3
    ///
    /// Register operation:
    /// - Address: `18`
    /// - Reset value: `0`
    #[doc(alias = "current-ch-3")]
    pub fn current_ch_3(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, CurrentCh3, u8, ::device_driver::RO, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 18;
        ::device_driver::RegisterOperation::new(self, address as u8, CurrentCh3::default)
    }
    /// Current channel 4
    ///
    /// Register operation:
    /// - Address: `26`
    /// - Reset value: `0`
    #[doc(alias = "current-ch-4")]
    pub fn current_ch_4(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, CurrentCh4, u8, ::device_driver::RO, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 26;
        ::device_driver::RegisterOperation::new(self, address as u8, CurrentCh4::default)
    }
    /// Power channel 1. Value [W] = 32 x CURRENT_LSB x register_value. Unsigned.
    ///
    /// Register operation:
    /// - Address: `3`
    /// - Reset value: `0`
    #[doc(alias = "power-ch-1")]
    pub fn power_ch_1(&mut self) -> ::device_driver::RegisterOperation<'_, Self, PowerCh1, u8, ::device_driver::RO, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 3;
        ::device_driver::RegisterOperation::new(self, address as u8, PowerCh1::default)
    }
    /// Power channel 2
    ///
    /// Register operation:
    /// - Address: `11`
    /// - Reset value: `0`
    #[doc(alias = "power-ch-2")]
    pub fn power_ch_2(&mut self) -> ::device_driver::RegisterOperation<'_, Self, PowerCh2, u8, ::device_driver::RO, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 11;
        ::device_driver::RegisterOperation::new(self, address as u8, PowerCh2::default)
    }
    /// Power channel 3
    ///
    /// Register operation:
    /// - Address: `19`
    /// - Reset value: `0`
    #[doc(alias = "power-ch-3")]
    pub fn power_ch_3(&mut self) -> ::device_driver::RegisterOperation<'_, Self, PowerCh3, u8, ::device_driver::RO, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 19;
        ::device_driver::RegisterOperation::new(self, address as u8, PowerCh3::default)
    }
    /// Power channel 4
    ///
    /// Register operation:
    /// - Address: `27`
    /// - Reset value: `0`
    #[doc(alias = "power-ch-4")]
    pub fn power_ch_4(&mut self) -> ::device_driver::RegisterOperation<'_, Self, PowerCh4, u8, ::device_driver::RO, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 27;
        ::device_driver::RegisterOperation::new(self, address as u8, PowerCh4::default)
    }
    /// Energy channel 1. Value [J] = 32 x CURRENT_LSB x register_value. Unsigned.
    ///
    /// Register operation:
    /// - Address: `4`
    /// - Reset value: `0`
    #[doc(alias = "energy-ch-1")]
    pub fn energy_ch_1(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, EnergyCh1, u8, ::device_driver::RO, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 4;
        ::device_driver::RegisterOperation::new(self, address as u8, EnergyCh1::default)
    }
    /// Energy channel 2
    ///
    /// Register operation:
    /// - Address: `12`
    /// - Reset value: `0`
    #[doc(alias = "energy-ch-2")]
    pub fn energy_ch_2(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, EnergyCh2, u8, ::device_driver::RO, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 12;
        ::device_driver::RegisterOperation::new(self, address as u8, EnergyCh2::default)
    }
    /// Energy channel 3
    ///
    /// Register operation:
    /// - Address: `20`
    /// - Reset value: `0`
    #[doc(alias = "energy-ch-3")]
    pub fn energy_ch_3(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, EnergyCh3, u8, ::device_driver::RO, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 20;
        ::device_driver::RegisterOperation::new(self, address as u8, EnergyCh3::default)
    }
    /// Energy channel 4
    ///
    /// Register operation:
    /// - Address: `28`
    /// - Reset value: `0`
    #[doc(alias = "energy-ch-4")]
    pub fn energy_ch_4(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, EnergyCh4, u8, ::device_driver::RO, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 28;
        ::device_driver::RegisterOperation::new(self, address as u8, EnergyCh4::default)
    }
    /// Flags register
    ///
    /// Register operation:
    /// - Address: `34`
    /// - Reset value: `0x00`
    pub fn flags(&mut self) -> ::device_driver::RegisterOperation<'_, Self, Flags, u8, ::device_driver::RO, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 34;
        ::device_driver::RegisterOperation::new(self, address as u8, || Flags::from([0, 0]))
    }
    /// Manufacturer ID. Reads back 0x5449 ('TI' in ASCII)
    ///
    /// Register operation:
    /// - Address: `126`
    /// - Reset value: `0x5449`
    #[doc(alias = "manufacturer-id")]
    pub fn manufacturer_id(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, ManufacturerId, u8, ::device_driver::RO, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 126;
        ::device_driver::RegisterOperation::new(self, address as u8, || ManufacturerId::from([84, 73]))
    }
}
impl<I> ::device_driver::Block for Device<I> {
    type Interface = I;
    type RegisterAddressType = u8;
    type CommandAddressType = u8;
    type BufferAddressType = u8;
    type RegisterAddressMode = ();
    fn interface(&mut self) -> &mut Self::Interface {
        &mut self.interface
    }
}
#[doc(alias = "manufacturer-id")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct ManufacturerId {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for ManufacturerId {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::BE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl ManufacturerId {
    /// `15:0` - Read the `id` field.
    ///
    /// Manufacturer ID
    #[must_use]
    pub fn id(&self) -> u16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::BE>(&self.bits, start, end) };
        raw
    }
    /// `15:0` - Set the `id` field.
    ///
    /// Manufacturer ID
    pub fn set_id(&mut self, value: u16) {
        let start = 0;
        let end = 15;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::BE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for ManufacturerId {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for ManufacturerId {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<ManufacturerId> for [u8; 2] {
    fn from(val: ManufacturerId) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for ManufacturerId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("ManufacturerId");
        d.field("id", &self.id());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ManufacturerId {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ManufacturerId {{ ");
        defmt::write!(f, "id: {=u16}, ", &self.id());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for ManufacturerId {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for ManufacturerId {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for ManufacturerId {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for ManufacturerId {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for ManufacturerId {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for ManufacturerId {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for ManufacturerId {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "flags")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct Flags {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for Flags {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::BE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl Flags {
    /// `bit 15` - Read the `limit4_alert` field.
    ///
    /// Alert limit 4 exceeded
    #[doc(alias = "limit4-alert")]
    #[must_use]
    pub fn limit4_alert(&self) -> bool {
        let start = 15;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::BE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 14` - Read the `limit3_alert` field.
    ///
    /// Alert limit 3 exceeded
    #[doc(alias = "limit3-alert")]
    #[must_use]
    pub fn limit3_alert(&self) -> bool {
        let start = 14;
        let end = 14;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::BE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 13` - Read the `limit2_alert` field.
    ///
    /// Alert limit 2 exceeded
    #[doc(alias = "limit2-alert")]
    #[must_use]
    pub fn limit2_alert(&self) -> bool {
        let start = 13;
        let end = 13;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::BE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 12` - Read the `limit1_alert` field.
    ///
    /// Alert limit 1 exceeded
    #[doc(alias = "limit1-alert")]
    #[must_use]
    pub fn limit1_alert(&self) -> bool {
        let start = 12;
        let end = 12;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::BE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 11` - Read the `energyof_ch4` field.
    ///
    /// Energy register overflow channel 4
    #[doc(alias = "energyof-ch4")]
    #[must_use]
    pub fn energyof_ch4(&self) -> bool {
        let start = 11;
        let end = 11;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::BE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 10` - Read the `energyof_ch3` field.
    ///
    /// Energy register overflow channel 3
    #[doc(alias = "energyof-ch3")]
    #[must_use]
    pub fn energyof_ch3(&self) -> bool {
        let start = 10;
        let end = 10;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::BE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 9` - Read the `energyof_ch2` field.
    ///
    /// Energy register overflow channel 2
    #[doc(alias = "energyof-ch2")]
    #[must_use]
    pub fn energyof_ch2(&self) -> bool {
        let start = 9;
        let end = 9;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::BE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 8` - Read the `energyof_ch1` field.
    ///
    /// Energy register overflow channel 1
    #[doc(alias = "energyof-ch1")]
    #[must_use]
    pub fn energyof_ch1(&self) -> bool {
        let start = 8;
        let end = 8;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::BE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 7` - Read the `cvrf` field.
    ///
    /// Conversion ready flag. Set when all conversions and averaging complete. Cleared by writing CONFIG1 or reading FLAGS.
    #[must_use]
    pub fn cvrf(&self) -> bool {
        let start = 7;
        let end = 7;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::BE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 6` - Read the `ovf` field.
    ///
    /// Math overflow flag. Indicates current and power data may be invalid.
    #[must_use]
    pub fn ovf(&self) -> bool {
        let start = 6;
        let end = 6;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::BE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 15` - Set the `limit4_alert` field.
    ///
    /// Alert limit 4 exceeded
    #[doc(alias = "limit4-alert")]
    pub fn set_limit4_alert(&mut self, value: bool) {
        let start = 15;
        let end = 15;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::BE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 14` - Set the `limit3_alert` field.
    ///
    /// Alert limit 3 exceeded
    #[doc(alias = "limit3-alert")]
    pub fn set_limit3_alert(&mut self, value: bool) {
        let start = 14;
        let end = 14;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::BE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 13` - Set the `limit2_alert` field.
    ///
    /// Alert limit 2 exceeded
    #[doc(alias = "limit2-alert")]
    pub fn set_limit2_alert(&mut self, value: bool) {
        let start = 13;
        let end = 13;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::BE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 12` - Set the `limit1_alert` field.
    ///
    /// Alert limit 1 exceeded
    #[doc(alias = "limit1-alert")]
    pub fn set_limit1_alert(&mut self, value: bool) {
        let start = 12;
        let end = 12;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::BE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 11` - Set the `energyof_ch4` field.
    ///
    /// Energy register overflow channel 4
    #[doc(alias = "energyof-ch4")]
    pub fn set_energyof_ch4(&mut self, value: bool) {
        let start = 11;
        let end = 11;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::BE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 10` - Set the `energyof_ch3` field.
    ///
    /// Energy register overflow channel 3
    #[doc(alias = "energyof-ch3")]
    pub fn set_energyof_ch3(&mut self, value: bool) {
        let start = 10;
        let end = 10;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::BE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 9` - Set the `energyof_ch2` field.
    ///
    /// Energy register overflow channel 2
    #[doc(alias = "energyof-ch2")]
    pub fn set_energyof_ch2(&mut self, value: bool) {
        let start = 9;
        let end = 9;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::BE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 8` - Set the `energyof_ch1` field.
    ///
    /// Energy register overflow channel 1
    #[doc(alias = "energyof-ch1")]
    pub fn set_energyof_ch1(&mut self, value: bool) {
        let start = 8;
        let end = 8;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::BE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 7` - Set the `cvrf` field.
    ///
    /// Conversion ready flag. Set when all conversions and averaging complete. Cleared by writing CONFIG1 or reading FLAGS.
    pub fn set_cvrf(&mut self, value: bool) {
        let start = 7;
        let end = 7;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::BE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 6` - Set the `ovf` field.
    ///
    /// Math overflow flag. Indicates current and power data may be invalid.
    pub fn set_ovf(&mut self, value: bool) {
        let start = 6;
        let end = 6;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::BE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for Flags {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for Flags {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<Flags> for [u8; 2] {
    fn from(val: Flags) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for Flags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("Flags");
        d.field("limit4_alert", &self.limit4_alert());
        d.field("limit3_alert", &self.limit3_alert());
        d.field("limit2_alert", &self.limit2_alert());
        d.field("limit1_alert", &self.limit1_alert());
        d.field("energyof_ch4", &self.energyof_ch4());
        d.field("energyof_ch3", &self.energyof_ch3());
        d.field("energyof_ch2", &self.energyof_ch2());
        d.field("energyof_ch1", &self.energyof_ch1());
        d.field("cvrf", &self.cvrf());
        d.field("ovf", &self.ovf());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flags {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Flags {{ ");
        defmt::write!(f, "limit4_alert: {=bool}, ", &self.limit4_alert());
        defmt::write!(f, "limit3_alert: {=bool}, ", &self.limit3_alert());
        defmt::write!(f, "limit2_alert: {=bool}, ", &self.limit2_alert());
        defmt::write!(f, "limit1_alert: {=bool}, ", &self.limit1_alert());
        defmt::write!(f, "energyof_ch4: {=bool}, ", &self.energyof_ch4());
        defmt::write!(f, "energyof_ch3: {=bool}, ", &self.energyof_ch3());
        defmt::write!(f, "energyof_ch2: {=bool}, ", &self.energyof_ch2());
        defmt::write!(f, "energyof_ch1: {=bool}, ", &self.energyof_ch1());
        defmt::write!(f, "cvrf: {=bool}, ", &self.cvrf());
        defmt::write!(f, "ovf: {=bool}, ", &self.ovf());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for Flags {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for Flags {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for Flags {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for Flags {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for Flags {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for Flags {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for Flags {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "energy-ch-4")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct EnergyCh4 {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 4],
}
unsafe impl ::device_driver::Fieldset for EnergyCh4 {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::BE);
    const ZERO: Self = Self { bits: [0; 4] };
}
impl EnergyCh4 {
    /// `31:0` - Read the `energy` field.
    ///
    /// Accumulated energy in joules, unsigned
    #[must_use]
    pub fn energy(&self) -> u32 {
        let start = 0;
        let end = 31;
        let raw = unsafe { ::device_driver::ops::load::<u32, ::device_driver::ops::BE>(&self.bits, start, end) };
        raw
    }
    /// `31:0` - Set the `energy` field.
    ///
    /// Accumulated energy in joules, unsigned
    pub fn set_energy(&mut self, value: u32) {
        let start = 0;
        let end = 31;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u32, ::device_driver::ops::BE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for EnergyCh4 {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 4]> for EnergyCh4 {
    fn from(bits: [u8; 4]) -> Self {
        Self { bits }
    }
}
impl From<EnergyCh4> for [u8; 4] {
    fn from(val: EnergyCh4) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for EnergyCh4 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("EnergyCh4");
        d.field("energy", &self.energy());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EnergyCh4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "EnergyCh4 {{ ");
        defmt::write!(f, "energy: {=u32}, ", &self.energy());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for EnergyCh4 {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for EnergyCh4 {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for EnergyCh4 {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for EnergyCh4 {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for EnergyCh4 {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for EnergyCh4 {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for EnergyCh4 {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "energy-ch-3")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct EnergyCh3 {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 4],
}
unsafe impl ::device_driver::Fieldset for EnergyCh3 {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::BE);
    const ZERO: Self = Self { bits: [0; 4] };
}
impl EnergyCh3 {
    /// `31:0` - Read the `energy` field.
    ///
    /// Accumulated energy in joules, unsigned
    #[must_use]
    pub fn energy(&self) -> u32 {
        let start = 0;
        let end = 31;
        let raw = unsafe { ::device_driver::ops::load::<u32, ::device_driver::ops::BE>(&self.bits, start, end) };
        raw
    }
    /// `31:0` - Set the `energy` field.
    ///
    /// Accumulated energy in joules, unsigned
    pub fn set_energy(&mut self, value: u32) {
        let start = 0;
        let end = 31;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u32, ::device_driver::ops::BE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for EnergyCh3 {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 4]> for EnergyCh3 {
    fn from(bits: [u8; 4]) -> Self {
        Self { bits }
    }
}
impl From<EnergyCh3> for [u8; 4] {
    fn from(val: EnergyCh3) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for EnergyCh3 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("EnergyCh3");
        d.field("energy", &self.energy());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EnergyCh3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "EnergyCh3 {{ ");
        defmt::write!(f, "energy: {=u32}, ", &self.energy());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for EnergyCh3 {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for EnergyCh3 {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for EnergyCh3 {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for EnergyCh3 {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for EnergyCh3 {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for EnergyCh3 {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for EnergyCh3 {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "energy-ch-2")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct EnergyCh2 {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 4],
}
unsafe impl ::device_driver::Fieldset for EnergyCh2 {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::BE);
    const ZERO: Self = Self { bits: [0; 4] };
}
impl EnergyCh2 {
    /// `31:0` - Read the `energy` field.
    ///
    /// Accumulated energy in joules, unsigned
    #[must_use]
    pub fn energy(&self) -> u32 {
        let start = 0;
        let end = 31;
        let raw = unsafe { ::device_driver::ops::load::<u32, ::device_driver::ops::BE>(&self.bits, start, end) };
        raw
    }
    /// `31:0` - Set the `energy` field.
    ///
    /// Accumulated energy in joules, unsigned
    pub fn set_energy(&mut self, value: u32) {
        let start = 0;
        let end = 31;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u32, ::device_driver::ops::BE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for EnergyCh2 {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 4]> for EnergyCh2 {
    fn from(bits: [u8; 4]) -> Self {
        Self { bits }
    }
}
impl From<EnergyCh2> for [u8; 4] {
    fn from(val: EnergyCh2) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for EnergyCh2 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("EnergyCh2");
        d.field("energy", &self.energy());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EnergyCh2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "EnergyCh2 {{ ");
        defmt::write!(f, "energy: {=u32}, ", &self.energy());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for EnergyCh2 {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for EnergyCh2 {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for EnergyCh2 {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for EnergyCh2 {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for EnergyCh2 {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for EnergyCh2 {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for EnergyCh2 {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "energy-ch-1")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct EnergyCh1 {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 4],
}
unsafe impl ::device_driver::Fieldset for EnergyCh1 {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::BE);
    const ZERO: Self = Self { bits: [0; 4] };
}
impl EnergyCh1 {
    /// `31:0` - Read the `energy` field.
    ///
    /// Accumulated energy in joules, unsigned
    #[must_use]
    pub fn energy(&self) -> u32 {
        let start = 0;
        let end = 31;
        let raw = unsafe { ::device_driver::ops::load::<u32, ::device_driver::ops::BE>(&self.bits, start, end) };
        raw
    }
    /// `31:0` - Set the `energy` field.
    ///
    /// Accumulated energy in joules, unsigned
    pub fn set_energy(&mut self, value: u32) {
        let start = 0;
        let end = 31;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u32, ::device_driver::ops::BE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for EnergyCh1 {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 4]> for EnergyCh1 {
    fn from(bits: [u8; 4]) -> Self {
        Self { bits }
    }
}
impl From<EnergyCh1> for [u8; 4] {
    fn from(val: EnergyCh1) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for EnergyCh1 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("EnergyCh1");
        d.field("energy", &self.energy());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EnergyCh1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "EnergyCh1 {{ ");
        defmt::write!(f, "energy: {=u32}, ", &self.energy());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for EnergyCh1 {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for EnergyCh1 {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for EnergyCh1 {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for EnergyCh1 {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for EnergyCh1 {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for EnergyCh1 {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for EnergyCh1 {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "power-ch-4")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct PowerCh4 {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for PowerCh4 {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::BE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl PowerCh4 {
    /// `15:0` - Read the `power` field.
    ///
    /// Calculated power in watts, unsigned
    #[must_use]
    pub fn power(&self) -> u16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::BE>(&self.bits, start, end) };
        raw
    }
    /// `15:0` - Set the `power` field.
    ///
    /// Calculated power in watts, unsigned
    pub fn set_power(&mut self, value: u16) {
        let start = 0;
        let end = 15;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::BE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for PowerCh4 {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for PowerCh4 {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<PowerCh4> for [u8; 2] {
    fn from(val: PowerCh4) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for PowerCh4 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("PowerCh4");
        d.field("power", &self.power());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PowerCh4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PowerCh4 {{ ");
        defmt::write!(f, "power: {=u16}, ", &self.power());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for PowerCh4 {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for PowerCh4 {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for PowerCh4 {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for PowerCh4 {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for PowerCh4 {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for PowerCh4 {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for PowerCh4 {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "power-ch-3")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct PowerCh3 {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for PowerCh3 {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::BE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl PowerCh3 {
    /// `15:0` - Read the `power` field.
    ///
    /// Calculated power in watts, unsigned
    #[must_use]
    pub fn power(&self) -> u16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::BE>(&self.bits, start, end) };
        raw
    }
    /// `15:0` - Set the `power` field.
    ///
    /// Calculated power in watts, unsigned
    pub fn set_power(&mut self, value: u16) {
        let start = 0;
        let end = 15;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::BE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for PowerCh3 {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for PowerCh3 {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<PowerCh3> for [u8; 2] {
    fn from(val: PowerCh3) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for PowerCh3 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("PowerCh3");
        d.field("power", &self.power());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PowerCh3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PowerCh3 {{ ");
        defmt::write!(f, "power: {=u16}, ", &self.power());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for PowerCh3 {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for PowerCh3 {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for PowerCh3 {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for PowerCh3 {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for PowerCh3 {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for PowerCh3 {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for PowerCh3 {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "power-ch-2")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct PowerCh2 {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for PowerCh2 {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::BE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl PowerCh2 {
    /// `15:0` - Read the `power` field.
    ///
    /// Calculated power in watts, unsigned
    #[must_use]
    pub fn power(&self) -> u16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::BE>(&self.bits, start, end) };
        raw
    }
    /// `15:0` - Set the `power` field.
    ///
    /// Calculated power in watts, unsigned
    pub fn set_power(&mut self, value: u16) {
        let start = 0;
        let end = 15;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::BE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for PowerCh2 {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for PowerCh2 {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<PowerCh2> for [u8; 2] {
    fn from(val: PowerCh2) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for PowerCh2 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("PowerCh2");
        d.field("power", &self.power());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PowerCh2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PowerCh2 {{ ");
        defmt::write!(f, "power: {=u16}, ", &self.power());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for PowerCh2 {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for PowerCh2 {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for PowerCh2 {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for PowerCh2 {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for PowerCh2 {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for PowerCh2 {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for PowerCh2 {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "power-ch-1")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct PowerCh1 {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for PowerCh1 {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::BE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl PowerCh1 {
    /// `15:0` - Read the `power` field.
    ///
    /// Calculated power in watts, unsigned
    #[must_use]
    pub fn power(&self) -> u16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::BE>(&self.bits, start, end) };
        raw
    }
    /// `15:0` - Set the `power` field.
    ///
    /// Calculated power in watts, unsigned
    pub fn set_power(&mut self, value: u16) {
        let start = 0;
        let end = 15;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::BE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for PowerCh1 {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for PowerCh1 {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<PowerCh1> for [u8; 2] {
    fn from(val: PowerCh1) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for PowerCh1 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("PowerCh1");
        d.field("power", &self.power());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PowerCh1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PowerCh1 {{ ");
        defmt::write!(f, "power: {=u16}, ", &self.power());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for PowerCh1 {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for PowerCh1 {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for PowerCh1 {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for PowerCh1 {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for PowerCh1 {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for PowerCh1 {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for PowerCh1 {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "current-ch-4")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct CurrentCh4 {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for CurrentCh4 {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::BE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl CurrentCh4 {
    /// `15:0` - Read the `current` field.
    ///
    /// Calculated current in amperes, 2's complement
    #[must_use]
    pub fn current(&self) -> u16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::BE>(&self.bits, start, end) };
        raw
    }
    /// `15:0` - Set the `current` field.
    ///
    /// Calculated current in amperes, 2's complement
    pub fn set_current(&mut self, value: u16) {
        let start = 0;
        let end = 15;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::BE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for CurrentCh4 {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for CurrentCh4 {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<CurrentCh4> for [u8; 2] {
    fn from(val: CurrentCh4) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for CurrentCh4 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("CurrentCh4");
        d.field("current", &self.current());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CurrentCh4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CurrentCh4 {{ ");
        defmt::write!(f, "current: {=u16}, ", &self.current());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for CurrentCh4 {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for CurrentCh4 {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for CurrentCh4 {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for CurrentCh4 {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for CurrentCh4 {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for CurrentCh4 {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for CurrentCh4 {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "current-ch-3")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct CurrentCh3 {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for CurrentCh3 {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::BE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl CurrentCh3 {
    /// `15:0` - Read the `current` field.
    ///
    /// Calculated current in amperes, 2's complement
    #[must_use]
    pub fn current(&self) -> u16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::BE>(&self.bits, start, end) };
        raw
    }
    /// `15:0` - Set the `current` field.
    ///
    /// Calculated current in amperes, 2's complement
    pub fn set_current(&mut self, value: u16) {
        let start = 0;
        let end = 15;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::BE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for CurrentCh3 {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for CurrentCh3 {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<CurrentCh3> for [u8; 2] {
    fn from(val: CurrentCh3) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for CurrentCh3 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("CurrentCh3");
        d.field("current", &self.current());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CurrentCh3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CurrentCh3 {{ ");
        defmt::write!(f, "current: {=u16}, ", &self.current());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for CurrentCh3 {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for CurrentCh3 {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for CurrentCh3 {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for CurrentCh3 {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for CurrentCh3 {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for CurrentCh3 {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for CurrentCh3 {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "current-ch-2")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct CurrentCh2 {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for CurrentCh2 {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::BE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl CurrentCh2 {
    /// `15:0` - Read the `current` field.
    ///
    /// Calculated current in amperes, 2's complement
    #[must_use]
    pub fn current(&self) -> u16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::BE>(&self.bits, start, end) };
        raw
    }
    /// `15:0` - Set the `current` field.
    ///
    /// Calculated current in amperes, 2's complement
    pub fn set_current(&mut self, value: u16) {
        let start = 0;
        let end = 15;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::BE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for CurrentCh2 {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for CurrentCh2 {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<CurrentCh2> for [u8; 2] {
    fn from(val: CurrentCh2) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for CurrentCh2 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("CurrentCh2");
        d.field("current", &self.current());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CurrentCh2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CurrentCh2 {{ ");
        defmt::write!(f, "current: {=u16}, ", &self.current());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for CurrentCh2 {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for CurrentCh2 {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for CurrentCh2 {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for CurrentCh2 {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for CurrentCh2 {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for CurrentCh2 {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for CurrentCh2 {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "current-ch-1")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct CurrentCh1 {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for CurrentCh1 {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::BE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl CurrentCh1 {
    /// `15:0` - Read the `current` field.
    ///
    /// Calculated current in amperes, 2's complement
    #[must_use]
    pub fn current(&self) -> u16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::BE>(&self.bits, start, end) };
        raw
    }
    /// `15:0` - Set the `current` field.
    ///
    /// Calculated current in amperes, 2's complement
    pub fn set_current(&mut self, value: u16) {
        let start = 0;
        let end = 15;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::BE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for CurrentCh1 {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for CurrentCh1 {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<CurrentCh1> for [u8; 2] {
    fn from(val: CurrentCh1) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for CurrentCh1 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("CurrentCh1");
        d.field("current", &self.current());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CurrentCh1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CurrentCh1 {{ ");
        defmt::write!(f, "current: {=u16}, ", &self.current());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for CurrentCh1 {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for CurrentCh1 {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for CurrentCh1 {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for CurrentCh1 {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for CurrentCh1 {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for CurrentCh1 {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for CurrentCh1 {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "bus-voltage-ch-4")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct BusVoltageCh4 {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for BusVoltageCh4 {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::BE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl BusVoltageCh4 {
    /// `15:0` - Read the `vbus` field.
    ///
    /// Bus voltage
    #[must_use]
    pub fn vbus(&self) -> u16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::BE>(&self.bits, start, end) };
        raw
    }
    /// `15:0` - Set the `vbus` field.
    ///
    /// Bus voltage
    pub fn set_vbus(&mut self, value: u16) {
        let start = 0;
        let end = 15;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::BE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for BusVoltageCh4 {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for BusVoltageCh4 {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<BusVoltageCh4> for [u8; 2] {
    fn from(val: BusVoltageCh4) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for BusVoltageCh4 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("BusVoltageCh4");
        d.field("vbus", &self.vbus());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BusVoltageCh4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "BusVoltageCh4 {{ ");
        defmt::write!(f, "vbus: {=u16}, ", &self.vbus());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for BusVoltageCh4 {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for BusVoltageCh4 {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for BusVoltageCh4 {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for BusVoltageCh4 {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for BusVoltageCh4 {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for BusVoltageCh4 {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for BusVoltageCh4 {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "bus-voltage-ch-3")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct BusVoltageCh3 {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for BusVoltageCh3 {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::BE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl BusVoltageCh3 {
    /// `15:0` - Read the `vbus` field.
    ///
    /// Bus voltage
    #[must_use]
    pub fn vbus(&self) -> u16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::BE>(&self.bits, start, end) };
        raw
    }
    /// `15:0` - Set the `vbus` field.
    ///
    /// Bus voltage
    pub fn set_vbus(&mut self, value: u16) {
        let start = 0;
        let end = 15;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::BE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for BusVoltageCh3 {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for BusVoltageCh3 {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<BusVoltageCh3> for [u8; 2] {
    fn from(val: BusVoltageCh3) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for BusVoltageCh3 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("BusVoltageCh3");
        d.field("vbus", &self.vbus());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BusVoltageCh3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "BusVoltageCh3 {{ ");
        defmt::write!(f, "vbus: {=u16}, ", &self.vbus());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for BusVoltageCh3 {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for BusVoltageCh3 {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for BusVoltageCh3 {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for BusVoltageCh3 {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for BusVoltageCh3 {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for BusVoltageCh3 {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for BusVoltageCh3 {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "bus-voltage-ch-2")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct BusVoltageCh2 {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for BusVoltageCh2 {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::BE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl BusVoltageCh2 {
    /// `15:0` - Read the `vbus` field.
    ///
    /// Bus voltage
    #[must_use]
    pub fn vbus(&self) -> u16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::BE>(&self.bits, start, end) };
        raw
    }
    /// `15:0` - Set the `vbus` field.
    ///
    /// Bus voltage
    pub fn set_vbus(&mut self, value: u16) {
        let start = 0;
        let end = 15;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::BE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for BusVoltageCh2 {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for BusVoltageCh2 {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<BusVoltageCh2> for [u8; 2] {
    fn from(val: BusVoltageCh2) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for BusVoltageCh2 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("BusVoltageCh2");
        d.field("vbus", &self.vbus());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BusVoltageCh2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "BusVoltageCh2 {{ ");
        defmt::write!(f, "vbus: {=u16}, ", &self.vbus());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for BusVoltageCh2 {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for BusVoltageCh2 {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for BusVoltageCh2 {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for BusVoltageCh2 {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for BusVoltageCh2 {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for BusVoltageCh2 {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for BusVoltageCh2 {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "bus-voltage-ch-1")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct BusVoltageCh1 {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for BusVoltageCh1 {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::BE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl BusVoltageCh1 {
    /// `15:0` - Read the `vbus` field.
    ///
    /// Bus voltage, always positive, 2's complement format
    #[must_use]
    pub fn vbus(&self) -> u16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::BE>(&self.bits, start, end) };
        raw
    }
    /// `15:0` - Set the `vbus` field.
    ///
    /// Bus voltage, always positive, 2's complement format
    pub fn set_vbus(&mut self, value: u16) {
        let start = 0;
        let end = 15;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::BE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for BusVoltageCh1 {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for BusVoltageCh1 {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<BusVoltageCh1> for [u8; 2] {
    fn from(val: BusVoltageCh1) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for BusVoltageCh1 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("BusVoltageCh1");
        d.field("vbus", &self.vbus());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BusVoltageCh1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "BusVoltageCh1 {{ ");
        defmt::write!(f, "vbus: {=u16}, ", &self.vbus());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for BusVoltageCh1 {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for BusVoltageCh1 {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for BusVoltageCh1 {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for BusVoltageCh1 {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for BusVoltageCh1 {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for BusVoltageCh1 {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for BusVoltageCh1 {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "shunt-voltage-ch-4")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct ShuntVoltageCh4 {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for ShuntVoltageCh4 {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::BE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl ShuntVoltageCh4 {
    /// `15:0` - Read the `vshunt` field.
    ///
    /// Differential shunt voltage, 2's complement
    #[must_use]
    pub fn vshunt(&self) -> u16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::BE>(&self.bits, start, end) };
        raw
    }
    /// `15:0` - Set the `vshunt` field.
    ///
    /// Differential shunt voltage, 2's complement
    pub fn set_vshunt(&mut self, value: u16) {
        let start = 0;
        let end = 15;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::BE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for ShuntVoltageCh4 {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for ShuntVoltageCh4 {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<ShuntVoltageCh4> for [u8; 2] {
    fn from(val: ShuntVoltageCh4) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for ShuntVoltageCh4 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("ShuntVoltageCh4");
        d.field("vshunt", &self.vshunt());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ShuntVoltageCh4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ShuntVoltageCh4 {{ ");
        defmt::write!(f, "vshunt: {=u16}, ", &self.vshunt());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for ShuntVoltageCh4 {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for ShuntVoltageCh4 {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for ShuntVoltageCh4 {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for ShuntVoltageCh4 {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for ShuntVoltageCh4 {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for ShuntVoltageCh4 {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for ShuntVoltageCh4 {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "shunt-voltage-ch-3")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct ShuntVoltageCh3 {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for ShuntVoltageCh3 {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::BE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl ShuntVoltageCh3 {
    /// `15:0` - Read the `vshunt` field.
    ///
    /// Differential shunt voltage, 2's complement
    #[must_use]
    pub fn vshunt(&self) -> u16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::BE>(&self.bits, start, end) };
        raw
    }
    /// `15:0` - Set the `vshunt` field.
    ///
    /// Differential shunt voltage, 2's complement
    pub fn set_vshunt(&mut self, value: u16) {
        let start = 0;
        let end = 15;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::BE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for ShuntVoltageCh3 {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for ShuntVoltageCh3 {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<ShuntVoltageCh3> for [u8; 2] {
    fn from(val: ShuntVoltageCh3) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for ShuntVoltageCh3 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("ShuntVoltageCh3");
        d.field("vshunt", &self.vshunt());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ShuntVoltageCh3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ShuntVoltageCh3 {{ ");
        defmt::write!(f, "vshunt: {=u16}, ", &self.vshunt());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for ShuntVoltageCh3 {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for ShuntVoltageCh3 {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for ShuntVoltageCh3 {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for ShuntVoltageCh3 {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for ShuntVoltageCh3 {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for ShuntVoltageCh3 {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for ShuntVoltageCh3 {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "shunt-voltage-ch-2")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct ShuntVoltageCh2 {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for ShuntVoltageCh2 {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::BE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl ShuntVoltageCh2 {
    /// `15:0` - Read the `vshunt` field.
    ///
    /// Differential shunt voltage, 2's complement
    #[must_use]
    pub fn vshunt(&self) -> u16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::BE>(&self.bits, start, end) };
        raw
    }
    /// `15:0` - Set the `vshunt` field.
    ///
    /// Differential shunt voltage, 2's complement
    pub fn set_vshunt(&mut self, value: u16) {
        let start = 0;
        let end = 15;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::BE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for ShuntVoltageCh2 {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for ShuntVoltageCh2 {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<ShuntVoltageCh2> for [u8; 2] {
    fn from(val: ShuntVoltageCh2) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for ShuntVoltageCh2 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("ShuntVoltageCh2");
        d.field("vshunt", &self.vshunt());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ShuntVoltageCh2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ShuntVoltageCh2 {{ ");
        defmt::write!(f, "vshunt: {=u16}, ", &self.vshunt());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for ShuntVoltageCh2 {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for ShuntVoltageCh2 {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for ShuntVoltageCh2 {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for ShuntVoltageCh2 {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for ShuntVoltageCh2 {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for ShuntVoltageCh2 {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for ShuntVoltageCh2 {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "shunt-voltage-ch-1")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct ShuntVoltageCh1 {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for ShuntVoltageCh1 {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::BE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl ShuntVoltageCh1 {
    /// `15:0` - Read the `vshunt` field.
    ///
    /// Differential shunt voltage, 2's complement
    #[must_use]
    pub fn vshunt(&self) -> u16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::BE>(&self.bits, start, end) };
        raw
    }
    /// `15:0` - Set the `vshunt` field.
    ///
    /// Differential shunt voltage, 2's complement
    pub fn set_vshunt(&mut self, value: u16) {
        let start = 0;
        let end = 15;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::BE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for ShuntVoltageCh1 {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for ShuntVoltageCh1 {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<ShuntVoltageCh1> for [u8; 2] {
    fn from(val: ShuntVoltageCh1) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for ShuntVoltageCh1 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("ShuntVoltageCh1");
        d.field("vshunt", &self.vshunt());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ShuntVoltageCh1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ShuntVoltageCh1 {{ ");
        defmt::write!(f, "vshunt: {=u16}, ", &self.vshunt());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for ShuntVoltageCh1 {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for ShuntVoltageCh1 {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for ShuntVoltageCh1 {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for ShuntVoltageCh1 {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for ShuntVoltageCh1 {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for ShuntVoltageCh1 {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for ShuntVoltageCh1 {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "alert-limit-4")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct AlertLimit4 {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for AlertLimit4 {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::BE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl AlertLimit4 {
    /// `15:0` - Read the `limit` field.
    ///
    /// Alert threshold
    #[must_use]
    pub fn limit(&self) -> u16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::BE>(&self.bits, start, end) };
        raw
    }
    /// `15:0` - Set the `limit` field.
    ///
    /// Alert threshold
    pub fn set_limit(&mut self, value: u16) {
        let start = 0;
        let end = 15;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::BE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for AlertLimit4 {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for AlertLimit4 {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<AlertLimit4> for [u8; 2] {
    fn from(val: AlertLimit4) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for AlertLimit4 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("AlertLimit4");
        d.field("limit", &self.limit());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AlertLimit4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "AlertLimit4 {{ ");
        defmt::write!(f, "limit: {=u16}, ", &self.limit());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for AlertLimit4 {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for AlertLimit4 {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for AlertLimit4 {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for AlertLimit4 {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for AlertLimit4 {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for AlertLimit4 {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for AlertLimit4 {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "alert-limit-3")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct AlertLimit3 {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for AlertLimit3 {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::BE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl AlertLimit3 {
    /// `15:0` - Read the `limit` field.
    ///
    /// Alert threshold
    #[must_use]
    pub fn limit(&self) -> u16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::BE>(&self.bits, start, end) };
        raw
    }
    /// `15:0` - Set the `limit` field.
    ///
    /// Alert threshold
    pub fn set_limit(&mut self, value: u16) {
        let start = 0;
        let end = 15;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::BE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for AlertLimit3 {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for AlertLimit3 {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<AlertLimit3> for [u8; 2] {
    fn from(val: AlertLimit3) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for AlertLimit3 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("AlertLimit3");
        d.field("limit", &self.limit());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AlertLimit3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "AlertLimit3 {{ ");
        defmt::write!(f, "limit: {=u16}, ", &self.limit());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for AlertLimit3 {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for AlertLimit3 {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for AlertLimit3 {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for AlertLimit3 {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for AlertLimit3 {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for AlertLimit3 {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for AlertLimit3 {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "alert-limit-2")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct AlertLimit2 {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for AlertLimit2 {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::BE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl AlertLimit2 {
    /// `15:0` - Read the `limit` field.
    ///
    /// Alert threshold
    #[must_use]
    pub fn limit(&self) -> u16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::BE>(&self.bits, start, end) };
        raw
    }
    /// `15:0` - Set the `limit` field.
    ///
    /// Alert threshold
    pub fn set_limit(&mut self, value: u16) {
        let start = 0;
        let end = 15;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::BE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for AlertLimit2 {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for AlertLimit2 {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<AlertLimit2> for [u8; 2] {
    fn from(val: AlertLimit2) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for AlertLimit2 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("AlertLimit2");
        d.field("limit", &self.limit());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AlertLimit2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "AlertLimit2 {{ ");
        defmt::write!(f, "limit: {=u16}, ", &self.limit());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for AlertLimit2 {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for AlertLimit2 {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for AlertLimit2 {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for AlertLimit2 {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for AlertLimit2 {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for AlertLimit2 {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for AlertLimit2 {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "alert-limit-1")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct AlertLimit1 {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for AlertLimit1 {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::BE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl AlertLimit1 {
    /// `15:0` - Read the `limit` field.
    ///
    /// Alert threshold. Shunt=signed 16-bit, Bus=unsigned 15-bit, Power=unsigned 16-bit
    #[must_use]
    pub fn limit(&self) -> u16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::BE>(&self.bits, start, end) };
        raw
    }
    /// `15:0` - Set the `limit` field.
    ///
    /// Alert threshold. Shunt=signed 16-bit, Bus=unsigned 15-bit, Power=unsigned 16-bit
    pub fn set_limit(&mut self, value: u16) {
        let start = 0;
        let end = 15;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::BE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for AlertLimit1 {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for AlertLimit1 {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<AlertLimit1> for [u8; 2] {
    fn from(val: AlertLimit1) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for AlertLimit1 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("AlertLimit1");
        d.field("limit", &self.limit());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AlertLimit1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "AlertLimit1 {{ ");
        defmt::write!(f, "limit: {=u16}, ", &self.limit());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for AlertLimit1 {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for AlertLimit1 {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for AlertLimit1 {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for AlertLimit1 {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for AlertLimit1 {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for AlertLimit1 {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for AlertLimit1 {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "alert-config-4")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct AlertConfig4 {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for AlertConfig4 {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::BE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl AlertConfig4 {
    /// `4:3` - Read the `channel` field.
    ///
    /// Channel assignment for this alert
    #[must_use]
    pub fn channel(&self) -> u8 {
        let start = 3;
        let end = 4;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::BE>(&self.bits, start, end) };
        raw
    }
    /// `2:0` - Read the `alert_mask` field.
    ///
    /// Active alert function selection
    #[doc(alias = "alert-mask")]
    #[must_use]
    pub fn alert_mask(&self) -> u8 {
        let start = 0;
        let end = 2;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::BE>(&self.bits, start, end) };
        raw
    }
    /// `4:3` - Set the `channel` field.
    ///
    /// Channel assignment for this alert
    pub fn set_channel(&mut self, value: u8) {
        let start = 3;
        let end = 4;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::BE>(raw, start, end, &mut self.bits) };
    }
    /// `2:0` - Set the `alert_mask` field.
    ///
    /// Active alert function selection
    #[doc(alias = "alert-mask")]
    pub fn set_alert_mask(&mut self, value: u8) {
        let start = 0;
        let end = 2;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::BE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for AlertConfig4 {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for AlertConfig4 {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<AlertConfig4> for [u8; 2] {
    fn from(val: AlertConfig4) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for AlertConfig4 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("AlertConfig4");
        d.field("channel", &self.channel());
        d.field("alert_mask", &self.alert_mask());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AlertConfig4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "AlertConfig4 {{ ");
        defmt::write!(f, "channel: {=u8}, ", &self.channel());
        defmt::write!(f, "alert_mask: {=u8}, ", &self.alert_mask());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for AlertConfig4 {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for AlertConfig4 {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for AlertConfig4 {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for AlertConfig4 {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for AlertConfig4 {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for AlertConfig4 {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for AlertConfig4 {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "alert-config-3")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct AlertConfig3 {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for AlertConfig3 {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::BE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl AlertConfig3 {
    /// `4:3` - Read the `channel` field.
    ///
    /// Channel assignment for this alert
    #[must_use]
    pub fn channel(&self) -> u8 {
        let start = 3;
        let end = 4;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::BE>(&self.bits, start, end) };
        raw
    }
    /// `2:0` - Read the `alert_mask` field.
    ///
    /// Active alert function selection
    #[doc(alias = "alert-mask")]
    #[must_use]
    pub fn alert_mask(&self) -> u8 {
        let start = 0;
        let end = 2;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::BE>(&self.bits, start, end) };
        raw
    }
    /// `4:3` - Set the `channel` field.
    ///
    /// Channel assignment for this alert
    pub fn set_channel(&mut self, value: u8) {
        let start = 3;
        let end = 4;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::BE>(raw, start, end, &mut self.bits) };
    }
    /// `2:0` - Set the `alert_mask` field.
    ///
    /// Active alert function selection
    #[doc(alias = "alert-mask")]
    pub fn set_alert_mask(&mut self, value: u8) {
        let start = 0;
        let end = 2;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::BE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for AlertConfig3 {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for AlertConfig3 {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<AlertConfig3> for [u8; 2] {
    fn from(val: AlertConfig3) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for AlertConfig3 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("AlertConfig3");
        d.field("channel", &self.channel());
        d.field("alert_mask", &self.alert_mask());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AlertConfig3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "AlertConfig3 {{ ");
        defmt::write!(f, "channel: {=u8}, ", &self.channel());
        defmt::write!(f, "alert_mask: {=u8}, ", &self.alert_mask());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for AlertConfig3 {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for AlertConfig3 {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for AlertConfig3 {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for AlertConfig3 {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for AlertConfig3 {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for AlertConfig3 {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for AlertConfig3 {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "alert-config-2")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct AlertConfig2 {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for AlertConfig2 {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::BE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl AlertConfig2 {
    /// `4:3` - Read the `channel` field.
    ///
    /// Channel assignment for this alert
    #[must_use]
    pub fn channel(&self) -> u8 {
        let start = 3;
        let end = 4;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::BE>(&self.bits, start, end) };
        raw
    }
    /// `2:0` - Read the `alert_mask` field.
    ///
    /// Active alert function selection
    #[doc(alias = "alert-mask")]
    #[must_use]
    pub fn alert_mask(&self) -> u8 {
        let start = 0;
        let end = 2;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::BE>(&self.bits, start, end) };
        raw
    }
    /// `4:3` - Set the `channel` field.
    ///
    /// Channel assignment for this alert
    pub fn set_channel(&mut self, value: u8) {
        let start = 3;
        let end = 4;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::BE>(raw, start, end, &mut self.bits) };
    }
    /// `2:0` - Set the `alert_mask` field.
    ///
    /// Active alert function selection
    #[doc(alias = "alert-mask")]
    pub fn set_alert_mask(&mut self, value: u8) {
        let start = 0;
        let end = 2;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::BE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for AlertConfig2 {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for AlertConfig2 {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<AlertConfig2> for [u8; 2] {
    fn from(val: AlertConfig2) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for AlertConfig2 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("AlertConfig2");
        d.field("channel", &self.channel());
        d.field("alert_mask", &self.alert_mask());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AlertConfig2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "AlertConfig2 {{ ");
        defmt::write!(f, "channel: {=u8}, ", &self.channel());
        defmt::write!(f, "alert_mask: {=u8}, ", &self.alert_mask());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for AlertConfig2 {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for AlertConfig2 {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for AlertConfig2 {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for AlertConfig2 {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for AlertConfig2 {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for AlertConfig2 {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for AlertConfig2 {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "alert-config-1")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct AlertConfig1 {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for AlertConfig1 {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::BE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl AlertConfig1 {
    /// `4:3` - Read the `channel` field.
    ///
    /// Channel assignment for this alert
    #[must_use]
    pub fn channel(&self) -> AlertChannel {
        let start = 3;
        let end = 4;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::BE>(&self.bits, start, end) };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `2:0` - Read the `alert_mask` field.
    ///
    /// Active alert function selection
    #[doc(alias = "alert-mask")]
    #[must_use]
    pub fn alert_mask(&self) -> AlertFunction {
        let start = 0;
        let end = 2;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::BE>(&self.bits, start, end) };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `4:3` - Set the `channel` field.
    ///
    /// Channel assignment for this alert
    pub fn set_channel(&mut self, value: AlertChannel) {
        let start = 3;
        let end = 4;
        let raw = value.into();
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::BE>(raw, start, end, &mut self.bits) };
    }
    /// `2:0` - Set the `alert_mask` field.
    ///
    /// Active alert function selection
    #[doc(alias = "alert-mask")]
    pub fn set_alert_mask(&mut self, value: AlertFunction) {
        let start = 0;
        let end = 2;
        let raw = value.into();
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::BE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for AlertConfig1 {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for AlertConfig1 {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<AlertConfig1> for [u8; 2] {
    fn from(val: AlertConfig1) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for AlertConfig1 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("AlertConfig1");
        d.field("channel", &self.channel());
        d.field("alert_mask", &self.alert_mask());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AlertConfig1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "AlertConfig1 {{ ");
        defmt::write!(f, "channel: {}, ", &self.channel());
        defmt::write!(f, "alert_mask: {}, ", &self.alert_mask());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for AlertConfig1 {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for AlertConfig1 {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for AlertConfig1 {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for AlertConfig1 {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for AlertConfig1 {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for AlertConfig1 {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for AlertConfig1 {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "calibration-ch-4")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct CalibrationCh4 {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for CalibrationCh4 {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::BE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl CalibrationCh4 {
    /// `14:0` - Read the `shunt_cal` field.
    ///
    /// Shunt calibration value for current conversion
    #[doc(alias = "shunt-cal")]
    #[must_use]
    pub fn shunt_cal(&self) -> u16 {
        let start = 0;
        let end = 14;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::BE>(&self.bits, start, end) };
        raw
    }
    /// `14:0` - Set the `shunt_cal` field.
    ///
    /// Shunt calibration value for current conversion
    #[doc(alias = "shunt-cal")]
    pub fn set_shunt_cal(&mut self, value: u16) {
        let start = 0;
        let end = 14;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::BE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for CalibrationCh4 {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for CalibrationCh4 {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<CalibrationCh4> for [u8; 2] {
    fn from(val: CalibrationCh4) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for CalibrationCh4 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("CalibrationCh4");
        d.field("shunt_cal", &self.shunt_cal());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CalibrationCh4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CalibrationCh4 {{ ");
        defmt::write!(f, "shunt_cal: {=u16}, ", &self.shunt_cal());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for CalibrationCh4 {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for CalibrationCh4 {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for CalibrationCh4 {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for CalibrationCh4 {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for CalibrationCh4 {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for CalibrationCh4 {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for CalibrationCh4 {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "calibration-ch-3")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct CalibrationCh3 {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for CalibrationCh3 {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::BE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl CalibrationCh3 {
    /// `14:0` - Read the `shunt_cal` field.
    ///
    /// Shunt calibration value for current conversion
    #[doc(alias = "shunt-cal")]
    #[must_use]
    pub fn shunt_cal(&self) -> u16 {
        let start = 0;
        let end = 14;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::BE>(&self.bits, start, end) };
        raw
    }
    /// `14:0` - Set the `shunt_cal` field.
    ///
    /// Shunt calibration value for current conversion
    #[doc(alias = "shunt-cal")]
    pub fn set_shunt_cal(&mut self, value: u16) {
        let start = 0;
        let end = 14;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::BE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for CalibrationCh3 {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for CalibrationCh3 {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<CalibrationCh3> for [u8; 2] {
    fn from(val: CalibrationCh3) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for CalibrationCh3 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("CalibrationCh3");
        d.field("shunt_cal", &self.shunt_cal());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CalibrationCh3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CalibrationCh3 {{ ");
        defmt::write!(f, "shunt_cal: {=u16}, ", &self.shunt_cal());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for CalibrationCh3 {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for CalibrationCh3 {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for CalibrationCh3 {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for CalibrationCh3 {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for CalibrationCh3 {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for CalibrationCh3 {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for CalibrationCh3 {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "calibration-ch-2")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct CalibrationCh2 {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for CalibrationCh2 {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::BE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl CalibrationCh2 {
    /// `14:0` - Read the `shunt_cal` field.
    ///
    /// Shunt calibration value for current conversion
    #[doc(alias = "shunt-cal")]
    #[must_use]
    pub fn shunt_cal(&self) -> u16 {
        let start = 0;
        let end = 14;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::BE>(&self.bits, start, end) };
        raw
    }
    /// `14:0` - Set the `shunt_cal` field.
    ///
    /// Shunt calibration value for current conversion
    #[doc(alias = "shunt-cal")]
    pub fn set_shunt_cal(&mut self, value: u16) {
        let start = 0;
        let end = 14;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::BE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for CalibrationCh2 {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for CalibrationCh2 {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<CalibrationCh2> for [u8; 2] {
    fn from(val: CalibrationCh2) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for CalibrationCh2 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("CalibrationCh2");
        d.field("shunt_cal", &self.shunt_cal());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CalibrationCh2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CalibrationCh2 {{ ");
        defmt::write!(f, "shunt_cal: {=u16}, ", &self.shunt_cal());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for CalibrationCh2 {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for CalibrationCh2 {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for CalibrationCh2 {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for CalibrationCh2 {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for CalibrationCh2 {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for CalibrationCh2 {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for CalibrationCh2 {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "calibration-ch-1")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct CalibrationCh1 {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for CalibrationCh1 {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::BE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl CalibrationCh1 {
    /// `14:0` - Read the `shunt_cal` field.
    ///
    /// Shunt calibration value for current conversion
    #[doc(alias = "shunt-cal")]
    #[must_use]
    pub fn shunt_cal(&self) -> u16 {
        let start = 0;
        let end = 14;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::BE>(&self.bits, start, end) };
        raw
    }
    /// `14:0` - Set the `shunt_cal` field.
    ///
    /// Shunt calibration value for current conversion
    #[doc(alias = "shunt-cal")]
    pub fn set_shunt_cal(&mut self, value: u16) {
        let start = 0;
        let end = 14;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::BE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for CalibrationCh1 {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for CalibrationCh1 {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<CalibrationCh1> for [u8; 2] {
    fn from(val: CalibrationCh1) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for CalibrationCh1 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("CalibrationCh1");
        d.field("shunt_cal", &self.shunt_cal());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CalibrationCh1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CalibrationCh1 {{ ");
        defmt::write!(f, "shunt_cal: {=u16}, ", &self.shunt_cal());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for CalibrationCh1 {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for CalibrationCh1 {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for CalibrationCh1 {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for CalibrationCh1 {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for CalibrationCh1 {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for CalibrationCh1 {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for CalibrationCh1 {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "config-2")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct Config2 {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for Config2 {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::BE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl Config2 {
    /// `bit 15` - Read the `rst` field.
    ///
    /// System reset. Set to 1 to reset all registers to default. Self-clears.
    #[must_use]
    pub fn rst(&self) -> bool {
        let start = 15;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::BE>(&self.bits, start, end) };
        raw > 0
    }
    /// `11:8` - Read the `acc_rst` field.
    ///
    /// Energy accumulator reset per channel. Bit11=CH4, Bit10=CH3, Bit9=CH2, Bit8=CH1. Bits self-clear after write.
    #[doc(alias = "acc-rst")]
    #[must_use]
    pub fn acc_rst(&self) -> u8 {
        let start = 8;
        let end = 11;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::BE>(&self.bits, start, end) };
        raw
    }
    /// `bit 7` - Read the `cnvr_mask` field.
    ///
    /// Conversion ready flag on ALERT pin enable
    #[doc(alias = "cnvr-mask")]
    #[must_use]
    pub fn cnvr_mask(&self) -> bool {
        let start = 7;
        let end = 7;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::BE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 6` - Read the `enof_mask` field.
    ///
    /// Energy overflow alert enable
    #[doc(alias = "enof-mask")]
    #[must_use]
    pub fn enof_mask(&self) -> bool {
        let start = 6;
        let end = 6;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::BE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 5` - Read the `alert_latch` field.
    ///
    /// Alert pin latch enable. When set, alert latches until flags register is read.
    #[doc(alias = "alert-latch")]
    #[must_use]
    pub fn alert_latch(&self) -> bool {
        let start = 5;
        let end = 5;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::BE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 4` - Read the `alert_pol` field.
    ///
    /// Alert pin polarity
    #[doc(alias = "alert-pol")]
    #[must_use]
    pub fn alert_pol(&self) -> AlertPolarity {
        let start = 4;
        let end = 4;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::BE>(&self.bits, start, end) };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `3:0` - Read the `range` field.
    ///
    /// Shunt full scale input range per channel. Bit3=CH4, Bit2=CH3, Bit1=CH2, Bit0=CH1. 0=±81.92mV, 1=±20.48mV
    #[must_use]
    pub fn range(&self) -> u8 {
        let start = 0;
        let end = 3;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::BE>(&self.bits, start, end) };
        raw
    }
    /// `bit 15` - Set the `rst` field.
    ///
    /// System reset. Set to 1 to reset all registers to default. Self-clears.
    pub fn set_rst(&mut self, value: bool) {
        let start = 15;
        let end = 15;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::BE>(raw, start, end, &mut self.bits) };
    }
    /// `11:8` - Set the `acc_rst` field.
    ///
    /// Energy accumulator reset per channel. Bit11=CH4, Bit10=CH3, Bit9=CH2, Bit8=CH1. Bits self-clear after write.
    #[doc(alias = "acc-rst")]
    pub fn set_acc_rst(&mut self, value: u8) {
        let start = 8;
        let end = 11;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::BE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 7` - Set the `cnvr_mask` field.
    ///
    /// Conversion ready flag on ALERT pin enable
    #[doc(alias = "cnvr-mask")]
    pub fn set_cnvr_mask(&mut self, value: bool) {
        let start = 7;
        let end = 7;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::BE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 6` - Set the `enof_mask` field.
    ///
    /// Energy overflow alert enable
    #[doc(alias = "enof-mask")]
    pub fn set_enof_mask(&mut self, value: bool) {
        let start = 6;
        let end = 6;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::BE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 5` - Set the `alert_latch` field.
    ///
    /// Alert pin latch enable. When set, alert latches until flags register is read.
    #[doc(alias = "alert-latch")]
    pub fn set_alert_latch(&mut self, value: bool) {
        let start = 5;
        let end = 5;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::BE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 4` - Set the `alert_pol` field.
    ///
    /// Alert pin polarity
    #[doc(alias = "alert-pol")]
    pub fn set_alert_pol(&mut self, value: AlertPolarity) {
        let start = 4;
        let end = 4;
        let raw = value.into();
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::BE>(raw, start, end, &mut self.bits) };
    }
    /// `3:0` - Set the `range` field.
    ///
    /// Shunt full scale input range per channel. Bit3=CH4, Bit2=CH3, Bit1=CH2, Bit0=CH1. 0=±81.92mV, 1=±20.48mV
    pub fn set_range(&mut self, value: u8) {
        let start = 0;
        let end = 3;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::BE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for Config2 {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for Config2 {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<Config2> for [u8; 2] {
    fn from(val: Config2) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for Config2 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("Config2");
        d.field("rst", &self.rst());
        d.field("acc_rst", &self.acc_rst());
        d.field("cnvr_mask", &self.cnvr_mask());
        d.field("enof_mask", &self.enof_mask());
        d.field("alert_latch", &self.alert_latch());
        d.field("alert_pol", &self.alert_pol());
        d.field("range", &self.range());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Config2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Config2 {{ ");
        defmt::write!(f, "rst: {=bool}, ", &self.rst());
        defmt::write!(f, "acc_rst: {=u8}, ", &self.acc_rst());
        defmt::write!(f, "cnvr_mask: {=bool}, ", &self.cnvr_mask());
        defmt::write!(f, "enof_mask: {=bool}, ", &self.enof_mask());
        defmt::write!(f, "alert_latch: {=bool}, ", &self.alert_latch());
        defmt::write!(f, "alert_pol: {}, ", &self.alert_pol());
        defmt::write!(f, "range: {=u8}, ", &self.range());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for Config2 {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for Config2 {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for Config2 {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for Config2 {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for Config2 {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for Config2 {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for Config2 {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "config-1")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct Config1 {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for Config1 {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::BE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl Config1 {
    /// `15:12` - Read the `active_channel` field.
    ///
    /// Active channel enable bits. Bit15=CH4, Bit14=CH3, Bit13=CH2, Bit12=CH1
    #[doc(alias = "active-channel")]
    #[must_use]
    pub fn active_channel(&self) -> u8 {
        let start = 12;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::BE>(&self.bits, start, end) };
        raw
    }
    /// `11:9` - Read the `avg` field.
    ///
    /// Number of ADC conversion results to average
    #[must_use]
    pub fn avg(&self) -> Averaging {
        let start = 9;
        let end = 11;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::BE>(&self.bits, start, end) };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `8:6` - Read the `vbusct` field.
    ///
    /// Bus voltage conversion time
    #[must_use]
    pub fn vbusct(&self) -> BusConversionTime {
        let start = 6;
        let end = 8;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::BE>(&self.bits, start, end) };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `5:3` - Read the `vshct` field.
    ///
    /// Shunt voltage conversion time
    #[must_use]
    pub fn vshct(&self) -> ShuntConversionTime {
        let start = 3;
        let end = 5;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::BE>(&self.bits, start, end) };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `2:0` - Read the `mode` field.
    ///
    /// Operating mode
    #[must_use]
    pub fn mode(&self) -> Mode {
        let start = 0;
        let end = 2;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::BE>(&self.bits, start, end) };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `15:12` - Set the `active_channel` field.
    ///
    /// Active channel enable bits. Bit15=CH4, Bit14=CH3, Bit13=CH2, Bit12=CH1
    #[doc(alias = "active-channel")]
    pub fn set_active_channel(&mut self, value: u8) {
        let start = 12;
        let end = 15;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::BE>(raw, start, end, &mut self.bits) };
    }
    /// `11:9` - Set the `avg` field.
    ///
    /// Number of ADC conversion results to average
    pub fn set_avg(&mut self, value: Averaging) {
        let start = 9;
        let end = 11;
        let raw = value.into();
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::BE>(raw, start, end, &mut self.bits) };
    }
    /// `8:6` - Set the `vbusct` field.
    ///
    /// Bus voltage conversion time
    pub fn set_vbusct(&mut self, value: BusConversionTime) {
        let start = 6;
        let end = 8;
        let raw = value.into();
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::BE>(raw, start, end, &mut self.bits) };
    }
    /// `5:3` - Set the `vshct` field.
    ///
    /// Shunt voltage conversion time
    pub fn set_vshct(&mut self, value: ShuntConversionTime) {
        let start = 3;
        let end = 5;
        let raw = value.into();
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::BE>(raw, start, end, &mut self.bits) };
    }
    /// `2:0` - Set the `mode` field.
    ///
    /// Operating mode
    pub fn set_mode(&mut self, value: Mode) {
        let start = 0;
        let end = 2;
        let raw = value.into();
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::BE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for Config1 {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for Config1 {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<Config1> for [u8; 2] {
    fn from(val: Config1) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for Config1 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("Config1");
        d.field("active_channel", &self.active_channel());
        d.field("avg", &self.avg());
        d.field("vbusct", &self.vbusct());
        d.field("vshct", &self.vshct());
        d.field("mode", &self.mode());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Config1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Config1 {{ ");
        defmt::write!(f, "active_channel: {=u8}, ", &self.active_channel());
        defmt::write!(f, "avg: {}, ", &self.avg());
        defmt::write!(f, "vbusct: {}, ", &self.vbusct());
        defmt::write!(f, "vshct: {}, ", &self.vshct());
        defmt::write!(f, "mode: {}, ", &self.mode());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for Config1 {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for Config1 {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for Config1 {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for Config1 {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for Config1 {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for Config1 {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for Config1 {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "alert-function")]
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AlertFunction {
    #[doc(alias = "none")]
    None = 0,
    #[doc(alias = "shunt-over-limit")]
    ShuntOverLimit = 1,
    #[doc(alias = "shunt-under-limit")]
    ShuntUnderLimit = 2,
    #[doc(alias = "bus-over-limit")]
    BusOverLimit = 3,
    #[doc(alias = "bus-under-limit")]
    BusUnderLimit = 4,
    #[doc(alias = "power-over-limit")]
    PowerOverLimit = 5,
    #[doc(alias = "reserved-6")]
    Reserved6 = 6,
    #[doc(alias = "reserved-7")]
    Reserved7 = 7,
}
impl core::convert::TryFrom<u8> for AlertFunction {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::None),
            1 => Ok(Self::ShuntOverLimit),
            2 => Ok(Self::ShuntUnderLimit),
            3 => Ok(Self::BusOverLimit),
            4 => Ok(Self::BusUnderLimit),
            5 => Ok(Self::PowerOverLimit),
            6 => Ok(Self::Reserved6),
            7 => Ok(Self::Reserved7),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "AlertFunction",
            }),
        }
    }
}
impl From<AlertFunction> for u8 {
    fn from(val: AlertFunction) -> Self {
        match val {
            AlertFunction::None => 0,
            AlertFunction::ShuntOverLimit => 1,
            AlertFunction::ShuntUnderLimit => 2,
            AlertFunction::BusOverLimit => 3,
            AlertFunction::BusUnderLimit => 4,
            AlertFunction::PowerOverLimit => 5,
            AlertFunction::Reserved6 => 6,
            AlertFunction::Reserved7 => 7,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for AlertFunction {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[doc(alias = "alert-channel")]
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AlertChannel {
    #[doc(alias = "ch1")]
    Ch1 = 0,
    #[doc(alias = "ch2")]
    Ch2 = 1,
    #[doc(alias = "ch3")]
    Ch3 = 2,
    #[doc(alias = "ch4")]
    Ch4 = 3,
}
impl core::convert::TryFrom<u8> for AlertChannel {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::Ch1),
            1 => Ok(Self::Ch2),
            2 => Ok(Self::Ch3),
            3 => Ok(Self::Ch4),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "AlertChannel",
            }),
        }
    }
}
impl From<AlertChannel> for u8 {
    fn from(val: AlertChannel) -> Self {
        match val {
            AlertChannel::Ch1 => 0,
            AlertChannel::Ch2 => 1,
            AlertChannel::Ch3 => 2,
            AlertChannel::Ch4 => 3,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for AlertChannel {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[doc(alias = "alert-polarity")]
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AlertPolarity {
    #[doc(alias = "active-low")]
    ActiveLow = 0,
    #[doc(alias = "active-high")]
    ActiveHigh = 1,
}
impl core::convert::TryFrom<u8> for AlertPolarity {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::ActiveLow),
            1 => Ok(Self::ActiveHigh),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "AlertPolarity",
            }),
        }
    }
}
impl From<AlertPolarity> for u8 {
    fn from(val: AlertPolarity) -> Self {
        match val {
            AlertPolarity::ActiveLow => 0,
            AlertPolarity::ActiveHigh => 1,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for AlertPolarity {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
/// Operating mode
#[doc(alias = "mode")]
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mode {
    #[doc(alias = "shutdown")]
    Shutdown = 0,
    #[doc(alias = "shunt-triggered")]
    ShuntTriggered = 1,
    #[doc(alias = "bus-triggered")]
    BusTriggered = 2,
    #[doc(alias = "shunt-and-bus-triggered")]
    ShuntAndBusTriggered = 3,
    #[doc(alias = "shutdown-2")]
    Shutdown2 = 4,
    #[doc(alias = "continuous-shunt")]
    ContinuousShunt = 5,
    #[doc(alias = "continuous-bus")]
    ContinuousBus = 6,
    #[doc(alias = "continuous-shunt-and-bus")]
    ContinuousShuntAndBus = 7,
}
impl core::convert::TryFrom<u8> for Mode {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::Shutdown),
            1 => Ok(Self::ShuntTriggered),
            2 => Ok(Self::BusTriggered),
            3 => Ok(Self::ShuntAndBusTriggered),
            4 => Ok(Self::Shutdown2),
            5 => Ok(Self::ContinuousShunt),
            6 => Ok(Self::ContinuousBus),
            7 => Ok(Self::ContinuousShuntAndBus),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "Mode",
            }),
        }
    }
}
impl From<Mode> for u8 {
    fn from(val: Mode) -> Self {
        match val {
            Mode::Shutdown => 0,
            Mode::ShuntTriggered => 1,
            Mode::BusTriggered => 2,
            Mode::ShuntAndBusTriggered => 3,
            Mode::Shutdown2 => 4,
            Mode::ContinuousShunt => 5,
            Mode::ContinuousBus => 6,
            Mode::ContinuousShuntAndBus => 7,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for Mode {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
/// Shunt voltage conversion time
#[doc(alias = "shunt-conversion-time")]
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ShuntConversionTime {
    #[doc(alias = "us-140")]
    Us140 = 0,
    #[doc(alias = "us-204")]
    Us204 = 1,
    #[doc(alias = "us-332")]
    Us332 = 2,
    #[doc(alias = "us-588")]
    Us588 = 3,
    #[doc(alias = "us-1100")]
    Us1100 = 4,
    #[doc(alias = "us-2116")]
    Us2116 = 5,
    #[doc(alias = "us-4156")]
    Us4156 = 6,
    #[doc(alias = "us-8244")]
    Us8244 = 7,
}
impl core::convert::TryFrom<u8> for ShuntConversionTime {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::Us140),
            1 => Ok(Self::Us204),
            2 => Ok(Self::Us332),
            3 => Ok(Self::Us588),
            4 => Ok(Self::Us1100),
            5 => Ok(Self::Us2116),
            6 => Ok(Self::Us4156),
            7 => Ok(Self::Us8244),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "ShuntConversionTime",
            }),
        }
    }
}
impl From<ShuntConversionTime> for u8 {
    fn from(val: ShuntConversionTime) -> Self {
        match val {
            ShuntConversionTime::Us140 => 0,
            ShuntConversionTime::Us204 => 1,
            ShuntConversionTime::Us332 => 2,
            ShuntConversionTime::Us588 => 3,
            ShuntConversionTime::Us1100 => 4,
            ShuntConversionTime::Us2116 => 5,
            ShuntConversionTime::Us4156 => 6,
            ShuntConversionTime::Us8244 => 7,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for ShuntConversionTime {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
/// Bus voltage conversion time
#[doc(alias = "bus-conversion-time")]
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BusConversionTime {
    #[doc(alias = "us-140")]
    Us140 = 0,
    #[doc(alias = "us-204")]
    Us204 = 1,
    #[doc(alias = "us-332")]
    Us332 = 2,
    #[doc(alias = "us-588")]
    Us588 = 3,
    #[doc(alias = "us-1100")]
    Us1100 = 4,
    #[doc(alias = "us-2116")]
    Us2116 = 5,
    #[doc(alias = "us-4156")]
    Us4156 = 6,
    #[doc(alias = "us-8244")]
    Us8244 = 7,
}
impl core::convert::TryFrom<u8> for BusConversionTime {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::Us140),
            1 => Ok(Self::Us204),
            2 => Ok(Self::Us332),
            3 => Ok(Self::Us588),
            4 => Ok(Self::Us1100),
            5 => Ok(Self::Us2116),
            6 => Ok(Self::Us4156),
            7 => Ok(Self::Us8244),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "BusConversionTime",
            }),
        }
    }
}
impl From<BusConversionTime> for u8 {
    fn from(val: BusConversionTime) -> Self {
        match val {
            BusConversionTime::Us140 => 0,
            BusConversionTime::Us204 => 1,
            BusConversionTime::Us332 => 2,
            BusConversionTime::Us588 => 3,
            BusConversionTime::Us1100 => 4,
            BusConversionTime::Us2116 => 5,
            BusConversionTime::Us4156 => 6,
            BusConversionTime::Us8244 => 7,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for BusConversionTime {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
/// Averaging count
#[doc(alias = "averaging")]
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Averaging {
    #[doc(alias = "num-1")]
    Num1 = 0,
    #[doc(alias = "num-4")]
    Num4 = 1,
    #[doc(alias = "num-16")]
    Num16 = 2,
    #[doc(alias = "num-64")]
    Num64 = 3,
    #[doc(alias = "num-128")]
    Num128 = 4,
    #[doc(alias = "num-256")]
    Num256 = 5,
    #[doc(alias = "num-512")]
    Num512 = 6,
    #[doc(alias = "num-1024")]
    Num1024 = 7,
}
impl core::convert::TryFrom<u8> for Averaging {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::Num1),
            1 => Ok(Self::Num4),
            2 => Ok(Self::Num16),
            3 => Ok(Self::Num64),
            4 => Ok(Self::Num128),
            5 => Ok(Self::Num256),
            6 => Ok(Self::Num512),
            7 => Ok(Self::Num1024),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "Averaging",
            }),
        }
    }
}
impl From<Averaging> for u8 {
    fn from(val: Averaging) -> Self {
        match val {
            Averaging::Num1 => 0,
            Averaging::Num4 => 1,
            Averaging::Num16 => 2,
            Averaging::Num64 => 3,
            Averaging::Num128 => 4,
            Averaging::Num256 => 5,
            Averaging::Num512 => 6,
            Averaging::Num1024 => 7,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for Averaging {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
