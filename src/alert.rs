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
