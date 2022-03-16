use crate::{
    data::{BuffData, DefData},
    util::parse_jsonc,
};
use std::{collections::HashMap, fs, path::Path};

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

/// Returns the default definitions.
fn default_definitions() -> DefData {
    include!(concat!(env!("OUT_DIR"), "/definitions.rs"))
}

/// Shared buff definitions data.
#[derive(Debug)]
pub struct Definitions {
    data: HashMap<u32, BuffDef>,
}

#[allow(dead_code)]
impl Definitions {
    /// Creates a new empty set of definitions.
    pub fn empty() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    /// Creates a new set of definitions with the default definitions.
    pub fn with_defaults() -> Self {
        let mut defs = Self::empty();
        let DefData {
            food,
            utility,
            ignore,
        } = default_definitions();
        defs.add_data(food, utility, ignore);
        defs
    }

    /// Creates a set of definitions from data.
    pub fn add_data(
        &mut self,
        food: impl IntoIterator<Item = BuffData>,
        util: impl IntoIterator<Item = BuffData>,
        ignore: impl IntoIterator<Item = u32>,
    ) {
        let food = food.into_iter();
        let util = util.into_iter();
        let ignore = ignore.into_iter();

        // reserve capacity
        let (food_size, _) = food.size_hint();
        let (util_size, _) = util.size_hint();
        let (ingnore_size, _) = ignore.size_hint();
        self.data.reserve(food_size + util_size + ingnore_size);

        // insert data
        for entry in food {
            self.data.insert(entry.id, BuffDef::Food(entry));
        }
        for entry in util {
            self.data.insert(entry.id, BuffDef::Util(entry));
        }
        for id in ignore {
            self.data.insert(id, BuffDef::Ignore(id));
        }
    }

    /// Attempts to load custom definitions from a given file.
    pub fn try_load(&mut self, path: impl AsRef<Path>) -> Result<(), ()> {
        // read file
        let content = fs::read_to_string(path).map_err(|_| ())?;

        // parse & add data
        let DefData {
            food,
            utility,
            ignore,
        } = parse_jsonc(&content).ok_or(())?;
        self.add_data(food, utility, ignore);

        Ok(())
    }

    /// Returns the buff definition with the given id.
    pub fn get(&self, buff_id: u32) -> Option<&BuffDef> {
        self.data.get(&buff_id)
    }

    /// Returns all food definitions.
    pub fn all_food(&self) -> impl Iterator<Item = &BuffData> {
        self.data.values().filter_map(|entry| match entry {
            BuffDef::Food(data) => Some(data),
            _ => None,
        })
    }

    /// Returns all utility definitions.
    pub fn all_util(&self) -> impl Iterator<Item = &BuffData> {
        self.data.values().filter_map(|entry| match entry {
            BuffDef::Util(data) => Some(data),
            _ => None,
        })
    }
}

#[derive(Debug, Clone)]
pub enum BuffDef {
    Food(BuffData),
    Util(BuffData),
    Ignore(u32),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn definitions() {
        let DefData {
            food,
            utility,
            ignore,
        } = default_definitions();

        assert!(!food.is_empty());
        assert!(!utility.is_empty());
        assert!(!ignore.is_empty());

        assert!(food.iter().any(|entry| entry.id == MALNOURISHED));
        assert!(utility.iter().any(|entry| entry.id == DIMINISHED));
    }
}
