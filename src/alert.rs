//! Alert slot configuration.
//!
//! The INA4230 has four alert slots. Each slot pairs an `ALERT_CONFIG`
//! register, which selects a condition and the channel to watch, with an
//! `ALERT_LIMIT` register holding the threshold for that condition.
//!
//! A slot is not a channel. Datasheet Table 7-20 describes each limit flag as
//! "independent of channel", and Table 7-8 gives `ALERT_CONFIG` a `CHANNEL`
//! field, so slot 2 may watch channel 4.

use crate::device;
use crate::units::{BusVoltage, Power, ShuntVoltage};

/// One of the four alert slots.
///
/// Distinct from [`crate::Channel`]: a slot selects which of the four
/// `ALERT_CONFIG`/`ALERT_LIMIT` register pairs is addressed, and each pair
/// names its own target channel.
///
/// The datasheet calls these `ALERT1`..`ALERT4` (Table 7-7) and
/// `LIMIT1`..`LIMIT4` (Table 7-9). The variants are bare numbers because the
/// type name already supplies the noun.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum AlertSlot {
    /// Slot 1: `ALERT_LIMIT` 0x06, `ALERT_CONFIG` 0x07.
    One = 0,
    /// Slot 2: `ALERT_LIMIT` 0x0E, `ALERT_CONFIG` 0x0F.
    Two = 1,
    /// Slot 3: `ALERT_LIMIT` 0x16, `ALERT_CONFIG` 0x17.
    Three = 2,
    /// Slot 4: `ALERT_LIMIT` 0x1E, `ALERT_CONFIG` 0x1F.
    Four = 3,
}

impl AlertSlot {
    /// Every slot, in index order.
    pub const ALL: [Self; 4] = [Self::One, Self::Two, Self::Three, Self::Four];

    /// Zero-based index, usable for array lookup and register striding.
    #[must_use]
    pub const fn index(self) -> usize {
        self as usize
    }
}

impl From<AlertSlot> for device::AlertSlot {
    fn from(slot: AlertSlot) -> Self {
        match slot {
            AlertSlot::One => Self::One,
            AlertSlot::Two => Self::Two,
            AlertSlot::Three => Self::Three,
            AlertSlot::Four => Self::Four,
        }
    }
}

/// A condition that asserts the ALERT pin, together with its threshold.
///
/// Each variant carries the threshold in the unit that variant implies, so a
/// bus threshold cannot be paired with a shunt function — there is no way to
/// spell it. Datasheet 7.1.5 makes `ALERT_LIMIT` reinterpret its format
/// according to the selected function, and this is how that reinterpretation
/// is made safe.
///
/// The five variants are `ALERT_MASK` encodings 1 through 5 (Table 7-8).
/// Encodings 0, 6 and 7 are all "reserved, no effect" and are aliases rather
/// than distinct states, so they are not variants: a slot is disarmed with
/// [`Ina4230::clear_alert`](crate::Ina4230::clear_alert), and a disarmed slot
/// reads back from [`Ina4230::alert`](crate::Ina4230::alert) as [`None`].
///
/// Deliberately not `#[non_exhaustive]`: the hardware defines exactly these
/// five functions and cannot grow one, so an exhaustive `match` should keep
/// telling you when you have missed a case.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Alert {
    /// Shunt voltage rose above the threshold. `ALERT_MASK` = 1 (SOL).
    ShuntOver(ShuntVoltage),
    /// Shunt voltage fell below the threshold. `ALERT_MASK` = 2 (SUL).
    ShuntUnder(ShuntVoltage),
    /// Bus voltage rose above the threshold. `ALERT_MASK` = 3 (BOL).
    BusOver(BusVoltage),
    /// Bus voltage fell below the threshold. `ALERT_MASK` = 4 (BUL).
    BusUnder(BusVoltage),
    /// Power rose above the threshold. `ALERT_MASK` = 5 (POL).
    PowerOver(Power),
}

impl Alert {
    /// This alert's `ALERT_MASK` encoding (datasheet Table 7-8).
    #[must_use]
    pub const fn mask_encoding(self) -> u8 {
        match self {
            Self::ShuntOver(_) => 1,
            Self::ShuntUnder(_) => 2,
            Self::BusOver(_) => 3,
            Self::BusUnder(_) => 4,
            Self::PowerOver(_) => 5,
        }
    }

    /// Whether encoding this alert's threshold requires the target channel's
    /// calibration.
    ///
    /// Shunt thresholds scale with [`crate::AdcRange`] and power thresholds
    /// with `CURRENT_LSB`, both of which live in the calibration. Bus
    /// thresholds have a fixed 1.6 mV LSB and need nothing.
    #[must_use]
    pub const fn needs_calibration(self) -> bool {
        matches!(self, Self::ShuntOver(_) | Self::ShuntUnder(_) | Self::PowerOver(_))
    }

    /// The generated `ALERT_MASK` enum value for this alert.
    pub(crate) const fn alert_function(self) -> device::AlertFunction {
        match self {
            Self::ShuntOver(_) => device::AlertFunction::ShuntOverLimit,
            Self::ShuntUnder(_) => device::AlertFunction::ShuntUnderLimit,
            Self::BusOver(_) => device::AlertFunction::BusOverLimit,
            Self::BusUnder(_) => device::AlertFunction::BusUnderLimit,
            Self::PowerOver(_) => device::AlertFunction::PowerOverLimit,
        }
    }
}

/// `ALERT_CONFIG.CHANNEL` uses its own generated enum, distinct from the one
/// indexing `channel-regs`, so it needs its own conversion.
impl From<crate::units::Channel> for device::AlertChannel {
    fn from(channel: crate::units::Channel) -> Self {
        match channel {
            crate::units::Channel::Ch1 => Self::Ch1,
            crate::units::Channel::Ch2 => Self::Ch2,
            crate::units::Channel::Ch3 => Self::Ch3,
            crate::units::Channel::Ch4 => Self::Ch4,
        }
    }
}
