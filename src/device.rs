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
    /// Per-channel register bank.
    ///
    /// Datasheet Table 7-1: the eight per-channel register definitions are
    /// repeated four times at register-pointer offsets 0x00, 0x08, 0x10 and
    /// 0x18, covering pointer values 0x00 through 0x1F. The stride counts
    /// register-pointer values, not bytes; the ENERGY register transfers four
    /// bytes while every other register in the bank transfers two.
    ///
    /// Block operation:
    /// - Address: `0`
    #[doc(alias = "channel-regs")]
    pub fn channel_regs(&mut self, index: Channel) -> ChannelRegs<'_, I> {
        let address = self.base_address + 0 + u8::from(index) as u8 * 8;
        ChannelRegs::<'_, I>::new(::device_driver::Block::interface(self), address)
    }
    /// Alert slot register bank.
    ///
    /// Datasheet Table 7-1: ALERT_LIMIT at 0x06, 0x0E, 0x16, 0x1E and
    /// ALERT_CONFIG at 0x07, 0x0F, 0x17, 0x1F. Same stride as `channel-regs`,
    /// but the index is an alert slot, not a channel.
    ///
    /// Block operation:
    /// - Address: `0`
    #[doc(alias = "alert-regs")]
    pub fn alert_regs(&mut self, index: AlertSlot) -> AlertRegs<'_, I> {
        let address = self.base_address + 0 + u8::from(index) as u8 * 8;
        AlertRegs::<'_, I>::new(::device_driver::Block::interface(self), address)
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
    /// Flags register
    ///
    /// Datasheet Table 7-20: reading this register clears `CVRF` and the
    /// latched alert flags.
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
/// Per-channel register bank.
///
/// Datasheet Table 7-1: the eight per-channel register definitions are
/// repeated four times at register-pointer offsets 0x00, 0x08, 0x10 and
/// 0x18, covering pointer values 0x00 through 0x1F. The stride counts
/// register-pointer values, not bytes; the ENERGY register transfers four
/// bytes while every other register in the bank transfers two.
#[doc(alias = "channel-regs")]
#[derive(Debug)]
pub struct ChannelRegs<'i, I> {
    #[doc(hidden)]
    interface: &'i mut I,
    #[doc(hidden)]
    #[allow(unused)]
    base_address: u8,
}
impl<'i, I> ChannelRegs<'i, I> {
    /// Create a new instance of the block based on device interface
    #[doc(hidden)]
    fn new(interface: &'i mut I, base_address: u8) -> Self {
        Self {
            interface,
            base_address: base_address,
        }
    }
    /// Shunt voltage. 2's complement.
    ///
    /// LSB = 2.5 uV when `CONFIG2.RANGE` is 0, 625 nV when it is 1.
    ///
    /// Register operation:
    /// - Address: `0`
    /// - Reset value: `0`
    #[doc(alias = "shunt-voltage")]
    pub fn shunt_voltage(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, ShuntVoltage, u8, ::device_driver::RO, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 0;
        ::device_driver::RegisterOperation::new(self, address as u8, ShuntVoltage::default)
    }
    /// Bus voltage. Always positive. LSB = 1.6 mV.
    ///
    /// Register operation:
    /// - Address: `1`
    /// - Reset value: `0`
    #[doc(alias = "bus-voltage")]
    pub fn bus_voltage(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, BusVoltage, u8, ::device_driver::RO, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 1;
        ::device_driver::RegisterOperation::new(self, address as u8, BusVoltage::default)
    }
    /// Current. Value [A] = CURRENT_LSB x register value.
    ///
    /// Register operation:
    /// - Address: `2`
    /// - Reset value: `0`
    pub fn current(&mut self) -> ::device_driver::RegisterOperation<'_, Self, Current, u8, ::device_driver::RO, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 2;
        ::device_driver::RegisterOperation::new(self, address as u8, Current::default)
    }
    /// Power. Value [W] = 32 x CURRENT_LSB x register value. Unsigned.
    ///
    /// Register operation:
    /// - Address: `3`
    /// - Reset value: `0`
    pub fn power(&mut self) -> ::device_driver::RegisterOperation<'_, Self, Power, u8, ::device_driver::RO, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 3;
        ::device_driver::RegisterOperation::new(self, address as u8, Power::default)
    }
    /// Energy. Value [J] = 32 x CURRENT_LSB x register value. Unsigned.
    ///
    /// Datasheet Table 7-18: 32 bits wide.
    ///
    /// Register operation:
    /// - Address: `4`
    /// - Reset value: `0`
    pub fn energy(&mut self) -> ::device_driver::RegisterOperation<'_, Self, Energy, u8, ::device_driver::RO, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 4;
        ::device_driver::RegisterOperation::new(self, address as u8, Energy::default)
    }
    /// Calibration register.
    ///
    /// Register operation:
    /// - Address: `5`
    /// - Reset value: `0x00`
    pub fn calibration(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, Calibration, u8, ::device_driver::RW, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 5;
        ::device_driver::RegisterOperation::new(self, address as u8, || Calibration::from([0, 0]))
    }
}
impl<'i, I> ::device_driver::Block for ChannelRegs<'i, I> {
    type Interface = I;
    type RegisterAddressType = u8;
    type CommandAddressType = u8;
    type BufferAddressType = u8;
    type RegisterAddressMode = ();
    fn interface(&mut self) -> &mut Self::Interface {
        self.interface
    }
}
/// Alert slot register bank.
///
/// Datasheet Table 7-1: ALERT_LIMIT at 0x06, 0x0E, 0x16, 0x1E and
/// ALERT_CONFIG at 0x07, 0x0F, 0x17, 0x1F. Same stride as `channel-regs`,
/// but the index is an alert slot, not a channel.
#[doc(alias = "alert-regs")]
#[derive(Debug)]
pub struct AlertRegs<'i, I> {
    #[doc(hidden)]
    interface: &'i mut I,
    #[doc(hidden)]
    #[allow(unused)]
    base_address: u8,
}
impl<'i, I> AlertRegs<'i, I> {
    /// Create a new instance of the block based on device interface
    #[doc(hidden)]
    fn new(interface: &'i mut I, base_address: u8) -> Self {
        Self {
            interface,
            base_address: base_address,
        }
    }
    /// Alert limit register.
    ///
    /// Datasheet 7.1.5: the format follows the result register the
    /// selected alert function refers to. Shunt voltage limits are signed
    /// 16-bit, bus voltage limits are unsigned 15-bit (bit 15 reserved),
    /// and power limits are unsigned 16-bit.
    ///
    /// Register operation:
    /// - Address: `6`
    /// - Reset value: `0x00`
    #[doc(alias = "alert-limit")]
    pub fn alert_limit(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, AlertLimit, u8, ::device_driver::RW, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 6;
        ::device_driver::RegisterOperation::new(self, address as u8, || AlertLimit::from([0, 0]))
    }
    /// Alert configuration register.
    ///
    /// Register operation:
    /// - Address: `7`
    /// - Reset value: `0x00`
    #[doc(alias = "alert-config")]
    pub fn alert_config(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, AlertConfig, u8, ::device_driver::RW, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 7;
        ::device_driver::RegisterOperation::new(self, address as u8, || AlertConfig::from([0, 0]))
    }
}
impl<'i, I> ::device_driver::Block for AlertRegs<'i, I> {
    type Interface = I;
    type RegisterAddressType = u8;
    type CommandAddressType = u8;
    type BufferAddressType = u8;
    type RegisterAddressMode = ();
    fn interface(&mut self) -> &mut Self::Interface {
        self.interface
    }
}
#[doc(alias = "calibration")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct Calibration {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for Calibration {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::BE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl Calibration {
    /// `14:0` - Read the `shunt_cal` field.
    ///
    /// Shunt calibration value for current conversion.
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
    /// Shunt calibration value for current conversion.
    #[doc(alias = "shunt-cal")]
    pub fn set_shunt_cal(&mut self, value: u16) {
        let start = 0;
        let end = 14;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::BE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for Calibration {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for Calibration {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<Calibration> for [u8; 2] {
    fn from(val: Calibration) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for Calibration {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("Calibration");
        d.field("shunt_cal", &self.shunt_cal());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Calibration {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Calibration {{ ");
        defmt::write!(f, "shunt_cal: {=u16}, ", &self.shunt_cal());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for Calibration {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for Calibration {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for Calibration {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for Calibration {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for Calibration {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for Calibration {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for Calibration {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "energy")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct Energy {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 4],
}
unsafe impl ::device_driver::Fieldset for Energy {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::BE);
    const ZERO: Self = Self { bits: [0; 4] };
}
impl Energy {
    /// `31:0` - Read the `energy` field.
    ///
    /// Accumulated energy, unsigned.
    #[must_use]
    pub fn energy(&self) -> u32 {
        let start = 0;
        let end = 31;
        let raw = unsafe { ::device_driver::ops::load::<u32, ::device_driver::ops::BE>(&self.bits, start, end) };
        raw
    }
    /// `31:0` - Set the `energy` field.
    ///
    /// Accumulated energy, unsigned.
    pub fn set_energy(&mut self, value: u32) {
        let start = 0;
        let end = 31;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u32, ::device_driver::ops::BE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for Energy {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 4]> for Energy {
    fn from(bits: [u8; 4]) -> Self {
        Self { bits }
    }
}
impl From<Energy> for [u8; 4] {
    fn from(val: Energy) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for Energy {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("Energy");
        d.field("energy", &self.energy());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Energy {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Energy {{ ");
        defmt::write!(f, "energy: {=u32}, ", &self.energy());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for Energy {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for Energy {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for Energy {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for Energy {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for Energy {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for Energy {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for Energy {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "power")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct Power {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for Power {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::BE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl Power {
    /// `15:0` - Read the `power` field.
    ///
    /// Calculated power, unsigned.
    #[must_use]
    pub fn power(&self) -> u16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::BE>(&self.bits, start, end) };
        raw
    }
    /// `15:0` - Set the `power` field.
    ///
    /// Calculated power, unsigned.
    pub fn set_power(&mut self, value: u16) {
        let start = 0;
        let end = 15;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::BE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for Power {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for Power {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<Power> for [u8; 2] {
    fn from(val: Power) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for Power {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("Power");
        d.field("power", &self.power());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Power {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Power {{ ");
        defmt::write!(f, "power: {=u16}, ", &self.power());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for Power {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for Power {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for Power {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for Power {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for Power {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for Power {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for Power {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "current")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct Current {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for Current {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::BE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl Current {
    /// `15:0` - Read the `current` field.
    ///
    /// Calculated current, 2's complement.
    #[must_use]
    pub fn current(&self) -> i16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<i16, ::device_driver::ops::BE>(&self.bits, start, end) };
        raw
    }
    /// `15:0` - Set the `current` field.
    ///
    /// Calculated current, 2's complement.
    pub fn set_current(&mut self, value: i16) {
        let start = 0;
        let end = 15;
        let raw = value;
        unsafe { ::device_driver::ops::store::<i16, ::device_driver::ops::BE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for Current {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for Current {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<Current> for [u8; 2] {
    fn from(val: Current) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for Current {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("Current");
        d.field("current", &self.current());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Current {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Current {{ ");
        defmt::write!(f, "current: {=i16}, ", &self.current());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for Current {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for Current {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for Current {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for Current {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for Current {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for Current {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for Current {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "bus-voltage")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct BusVoltage {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for BusVoltage {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::BE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl BusVoltage {
    /// `15:0` - Read the `vbus` field.
    ///
    /// Bus voltage. Datasheet Table 7-13: 2's complement format,
    /// however always positive. Usable range 0 V to 52.4 V.
    #[must_use]
    pub fn vbus(&self) -> u16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::BE>(&self.bits, start, end) };
        raw
    }
    /// `15:0` - Set the `vbus` field.
    ///
    /// Bus voltage. Datasheet Table 7-13: 2's complement format,
    /// however always positive. Usable range 0 V to 52.4 V.
    pub fn set_vbus(&mut self, value: u16) {
        let start = 0;
        let end = 15;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::BE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for BusVoltage {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for BusVoltage {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<BusVoltage> for [u8; 2] {
    fn from(val: BusVoltage) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for BusVoltage {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("BusVoltage");
        d.field("vbus", &self.vbus());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BusVoltage {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "BusVoltage {{ ");
        defmt::write!(f, "vbus: {=u16}, ", &self.vbus());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for BusVoltage {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for BusVoltage {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for BusVoltage {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for BusVoltage {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for BusVoltage {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for BusVoltage {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for BusVoltage {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "shunt-voltage")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct ShuntVoltage {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for ShuntVoltage {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::BE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl ShuntVoltage {
    /// `15:0` - Read the `vshunt` field.
    ///
    /// Differential shunt voltage, 2's complement.
    #[must_use]
    pub fn vshunt(&self) -> i16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<i16, ::device_driver::ops::BE>(&self.bits, start, end) };
        raw
    }
    /// `15:0` - Set the `vshunt` field.
    ///
    /// Differential shunt voltage, 2's complement.
    pub fn set_vshunt(&mut self, value: i16) {
        let start = 0;
        let end = 15;
        let raw = value;
        unsafe { ::device_driver::ops::store::<i16, ::device_driver::ops::BE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for ShuntVoltage {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for ShuntVoltage {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<ShuntVoltage> for [u8; 2] {
    fn from(val: ShuntVoltage) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for ShuntVoltage {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("ShuntVoltage");
        d.field("vshunt", &self.vshunt());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ShuntVoltage {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ShuntVoltage {{ ");
        defmt::write!(f, "vshunt: {=i16}, ", &self.vshunt());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for ShuntVoltage {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for ShuntVoltage {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for ShuntVoltage {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for ShuntVoltage {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for ShuntVoltage {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for ShuntVoltage {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for ShuntVoltage {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "alert-config")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct AlertConfig {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for AlertConfig {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::BE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl AlertConfig {
    /// `4:3` - Read the `channel` field.
    ///
    /// Channel assignment for this alert.
    #[must_use]
    pub fn channel(&self) -> AlertChannel {
        let start = 3;
        let end = 4;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::BE>(&self.bits, start, end) };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `2:0` - Read the `alert_mask` field.
    ///
    /// Active alert function selection.
    #[doc(alias = "alert-mask")]
    #[must_use]
    pub fn alert_mask(&self) -> AlertFunction {
        let start = 0;
        let end = 2;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::BE>(&self.bits, start, end) };
        raw.into()
    }
    /// `4:3` - Set the `channel` field.
    ///
    /// Channel assignment for this alert.
    pub fn set_channel(&mut self, value: AlertChannel) {
        let start = 3;
        let end = 4;
        let raw = value.into();
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::BE>(raw, start, end, &mut self.bits) };
    }
    /// `2:0` - Set the `alert_mask` field.
    ///
    /// Active alert function selection.
    #[doc(alias = "alert-mask")]
    pub fn set_alert_mask(&mut self, value: AlertFunction) {
        let start = 0;
        let end = 2;
        let raw = value.into();
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::BE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for AlertConfig {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for AlertConfig {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<AlertConfig> for [u8; 2] {
    fn from(val: AlertConfig) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for AlertConfig {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("AlertConfig");
        d.field("channel", &self.channel());
        d.field("alert_mask", &self.alert_mask());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AlertConfig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "AlertConfig {{ ");
        defmt::write!(f, "channel: {}, ", &self.channel());
        defmt::write!(f, "alert_mask: {}, ", &self.alert_mask());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for AlertConfig {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for AlertConfig {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for AlertConfig {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for AlertConfig {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for AlertConfig {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for AlertConfig {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for AlertConfig {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "alert-limit")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct AlertLimit {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for AlertLimit {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::BE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl AlertLimit {
    /// `15:0` - Read the `limit` field.
    ///
    /// Alert threshold, in the format of the corresponding result
    /// register.
    #[must_use]
    pub fn limit(&self) -> u16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::BE>(&self.bits, start, end) };
        raw
    }
    /// `15:0` - Set the `limit` field.
    ///
    /// Alert threshold, in the format of the corresponding result
    /// register.
    pub fn set_limit(&mut self, value: u16) {
        let start = 0;
        let end = 15;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::BE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for AlertLimit {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for AlertLimit {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<AlertLimit> for [u8; 2] {
    fn from(val: AlertLimit) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for AlertLimit {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("AlertLimit");
        d.field("limit", &self.limit());
        d.finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AlertLimit {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "AlertLimit {{ ");
        defmt::write!(f, "limit: {=u16}, ", &self.limit());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for AlertLimit {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for AlertLimit {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for AlertLimit {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for AlertLimit {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for AlertLimit {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for AlertLimit {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for AlertLimit {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
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
    /// Shunt full scale input range per channel. Bit3=CH4, Bit2=CH3, Bit1=CH2, Bit0=CH1. 0=+/-81.92mV, 1=+/-20.48mV
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
    /// Shunt full scale input range per channel. Bit3=CH4, Bit2=CH3, Bit1=CH2, Bit0=CH1. 0=+/-81.92mV, 1=+/-20.48mV
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
/// Measurement channel selector.
///
/// The four per-channel register banks occupy register-pointer values
/// separated by 8, so the channel index doubles as the repeat index for
/// the `channel-regs` block.
#[doc(alias = "channel")]
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Channel {
    #[doc(alias = "ch-1")]
    Ch1 = 0,
    #[doc(alias = "ch-2")]
    Ch2 = 1,
    #[doc(alias = "ch-3")]
    Ch3 = 2,
    #[doc(alias = "ch-4")]
    Ch4 = 3,
}
impl core::convert::TryFrom<u8> for Channel {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::Ch1),
            1 => Ok(Self::Ch2),
            2 => Ok(Self::Ch3),
            3 => Ok(Self::Ch4),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "Channel",
            }),
        }
    }
}
impl From<Channel> for u8 {
    fn from(val: Channel) -> Self {
        match val {
            Channel::Ch1 => 0,
            Channel::Ch2 => 1,
            Channel::Ch3 => 2,
            Channel::Ch4 => 3,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for Channel {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
/// Alert slot selector.
///
/// The four ALERT_CONFIG/ALERT_LIMIT register pairs are *not* per-channel.
/// Datasheet Table 7-20 describes each LIMITn_ALERT flag as "independent of
/// channel", and Table 7-8 gives ALERT_CONFIG a CHANNEL field selecting
/// which channel the slot watches. Slot 2 may watch channel 4.
///
/// The datasheet calls these ALERT1..ALERT4 (Table 7-7) and LIMIT1..LIMIT4
/// (Table 7-9). The variants are bare numbers because the enum name
/// already supplies the noun.
#[doc(alias = "alert-slot")]
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AlertSlot {
    #[doc(alias = "one")]
    One = 0,
    #[doc(alias = "two")]
    Two = 1,
    #[doc(alias = "three")]
    Three = 2,
    #[doc(alias = "four")]
    Four = 3,
}
impl core::convert::TryFrom<u8> for AlertSlot {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::One),
            1 => Ok(Self::Two),
            2 => Ok(Self::Three),
            3 => Ok(Self::Four),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "AlertSlot",
            }),
        }
    }
}
impl From<AlertSlot> for u8 {
    fn from(val: AlertSlot) -> Self {
        match val {
            AlertSlot::One => 0,
            AlertSlot::Two => 1,
            AlertSlot::Three => 2,
            AlertSlot::Four => 3,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for AlertSlot {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
/// Which condition asserts the ALERT pin.
///
/// Datasheet Table 7-8 documents encodings 0, 6 and 7 all
/// as "reserved, no effect". They are aliases of one
/// another rather than illegal states, and 0 is the
/// power-on value, so all three decode to `no-effect`
/// and the conversion is infallible.
#[doc(alias = "alert-function")]
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AlertFunction {
    /// Reserved, no effect. Encodings 6 and 7 behave
    /// identically and collapse into this variant.
    #[doc(alias = "no-effect")]
    NoEffect = 0,
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
}
impl Default for AlertFunction {
    fn default() -> Self {
        Self::NoEffect
    }
}
impl From<u8> for AlertFunction {
    fn from(val: u8) -> Self {
        match val {
            1 => Self::ShuntOverLimit,
            2 => Self::ShuntUnderLimit,
            3 => Self::BusOverLimit,
            4 => Self::BusUnderLimit,
            5 => Self::PowerOverLimit,
            _ => Self::default(),
        }
    }
}
impl From<AlertFunction> for u8 {
    fn from(val: AlertFunction) -> Self {
        match val {
            AlertFunction::NoEffect => 0,
            AlertFunction::ShuntOverLimit => 1,
            AlertFunction::ShuntUnderLimit => 2,
            AlertFunction::BusOverLimit => 3,
            AlertFunction::BusUnderLimit => 4,
            AlertFunction::PowerOverLimit => 5,
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
