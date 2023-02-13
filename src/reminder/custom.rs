use crate::data::{FRACTAL_MAPS, RAID_MAPS};
use serde::{Deserialize, Serialize};

/// Custom buff to remind for.
// TODO: stacks?
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomReminder {
    /// Id of the buff.
    pub id: u32,

    /// Name of the reminder (usually buff name).
    pub name: String,

    /// [`GameMode`] this reminder is restricted to.
    #[serde(default)]
    pub mode: GameMode,
}

impl CustomReminder {
    /// Creates a new custom reminder.
    pub fn new(id: u32, name: impl Into<String>, mode: GameMode) -> Self {
        Self {
            id,
            name: name.into(),
            mode,
        }
    }

    /// Returns the default custom reminders.
    pub fn defaults() -> Vec<Self> {
        vec![
            Self::new(9283, "Reinforced", GameMode::All),
            // fractal potions
            Self::new(32473, "Offensive", GameMode::Fractal),
            Self::new(32134, "Defensive", GameMode::Fractal),
            Self::new(33024, "Mobility", GameMode::Fractal),
        ]
    }
}

/// Game mode.
#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum GameMode {
    #[default]
    All,
    Raid,
    Fractal,
}

impl GameMode {
    /// Checks whether the [`GameMode`] includes the map id.
    pub fn is_map(&self, map_id: u32) -> bool {
        match self {
            GameMode::All => true,
            GameMode::Raid => RAID_MAPS.contains(&map_id),
            GameMode::Fractal => FRACTAL_MAPS.contains(&map_id),
        }
    }
}
