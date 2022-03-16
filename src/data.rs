#![allow(dead_code)]

use serde::{de::DeserializeOwned, Deserialize, Serialize};

/// Malnourished buff id.
pub const MALNOURISHED: u32 = 46587;

/// Diminished buff id.
pub const DIMINISHED: u32 = 46668;

/// Boss ids.
pub const BOSSES: &[usize] = &[
    15438, // Vale Guardian
    15429, // Gorseval
    15375, // Sabetha
    16123, // Slothasor
    16088, // Berg
    16137, // Zane
    16125, // Narella
    16115, // Matthias
    16253, // MacLeod
    16235, // Keep Construct
    16246, // Xera
    17194, // Cairn
    17172, // Mursaat Overseer
    17188, // Samarog
    17154, // Deimos
    19767, // Soulless Horror
    19828, // Desmina
    19691, // Broken King
    19536, // Soul Eater
    19651, // Eye of Judgement
    19844, // Eye of Fate
    19450, // Dhuum
    43974, // Conjured Amalgamate
    10142, // Conjured Amalgamate Right Arm
    37464, // Conjured Amalgamate Left Arm
    21105, // Nikare
    21089, // Kenut
    20934, // Qadim
    22006, // Adina
    21964, // Sabir
    22000, // Qadim the Peerless
    17021, // MAMA
    17028, // Siax
    16948, // Ensolyss
    17632, // Skorvald
    17949, // Artsariiv
    17759, // Arkk
    23254, // Ai
];

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

/// Parses JSONC from an input string.
pub fn parse_jsonc<T>(input: &str) -> Option<T>
where
    T: DeserializeOwned,
{
    jsonc_parser::parse_to_serde_value(input)
        .ok()
        .and_then(|value| value)
        .and_then(|value| serde_json::from_value(value).ok())
}
