use serde::{Deserialize, Serialize};

/// Buff definitions data.
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct DefData {
    pub food: Vec<BuffData>,
    pub utility: Vec<BuffData>,
    pub ignore: Vec<u32>,
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
