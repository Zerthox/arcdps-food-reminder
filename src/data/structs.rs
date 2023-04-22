use serde::{Deserialize, Serialize};

/// Buff definitions data.
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct DefData {
    /// Food buff definitions.
    pub food: Vec<BuffData>,

    /// Utility buff definitions.
    pub utility: Vec<BuffData>,

    /// Ignored buffs.
    pub ignore: Vec<u32>,
}

/// Single buff data entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuffData {
    /// Ingame buff id.
    pub id: u32,

    /// Full name of the buff (or item applying it).
    pub name: String,

    /// Buff stats.
    #[serde(default)]
    pub stats: Vec<String>,

    /// Short display name in buff tracker table.
    pub display: String,

    /// Rarity of the item applying the effect.
    #[serde(default)]
    pub rarity: Rarity,
}

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum Rarity {
    #[default]
    Basic,
    Fine,
    Masterwork,
    Rare,
    Exotic,
    Ascended,
    Legendary,
}
