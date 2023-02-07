use serde::{Deserialize, Serialize};

/// Buff definitions data.
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct DefData {
    pub reminders: Vec<CustomReminder>,
    pub food: Vec<BuffData>,
    pub utility: Vec<BuffData>,
    pub ignore: Vec<u32>,
}

/// Custom buff to remind for.
// TODO: stacks?
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomReminder {
    /// Id of the buff.
    pub id: u32,

    /// Name of the reminder (usually buff name).
    pub name: String,

    /// [`GameMode`] this reminder is restricted to.
    pub mode: Option<GameMode>,
}

/// Game mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum GameMode {
    Raid,
    Fractal,
}

/// Single buff data entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuffData {
    pub id: u32,
    pub name: String,

    #[serde(default)]
    pub stats: Vec<String>,

    pub display: String,
}
