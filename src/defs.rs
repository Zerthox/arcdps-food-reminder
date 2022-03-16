use crate::{
    data::{BuffData, DefData},
    util::parse_jsonc,
};
use std::{collections::HashMap, fs, path::Path};

/// Malnourished buff id.
pub const MALNOURISHED: u32 = 46587;

/// Diminished buff id.
pub const DIMINISHED: u32 = 46668;

/// Returns the default definitions.
fn default_definitions() -> DefData {
    include!(concat!(env!("OUT_DIR"), "/definitions.rs"))
}

/// Shared buff definitions data.
#[derive(Debug)]
pub struct Definitions {
    data: HashMap<u32, BuffDef>,
    bosses: Vec<u32>,
}

#[allow(dead_code)]
impl Definitions {
    /// Creates a new empty set of definitions.
    pub fn empty() -> Self {
        Self {
            data: HashMap::new(),
            bosses: Vec::new(),
        }
    }

    /// Creates a new set of definitions with the default definitions.
    pub fn with_defaults() -> Self {
        let mut defs = Self::empty();

        // add default defs data
        let data = default_definitions();
        defs.add_data(data);

        defs
    }

    /// Adds new buff definitions.
    pub fn add_buffs(
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

    /// Adds new boss ids.
    pub fn add_bosses(&mut self, bosses: impl IntoIterator<Item = u32>) {
        self.bosses.extend(bosses);
    }

    /// Add definitions from a [`DefData`] collection.
    pub fn add_data(&mut self, data: DefData) {
        let DefData {
            food,
            utility,
            ignore,
            bosses,
        } = data;
        self.add_buffs(food, utility, ignore);
        self.add_bosses(bosses);
    }

    /// Attempts to load custom definitions from a given file.
    pub fn try_load(&mut self, path: impl AsRef<Path>) -> Result<(), ()> {
        // read file
        let content = fs::read_to_string(path).map_err(|_| ())?;

        // parse & add data
        let data = parse_jsonc(&content).ok_or(())?;
        self.add_data(data);

        Ok(())
    }

    /// Returns the buff definition with the given id.
    pub fn get_buff(&self, buff_id: u32) -> Option<&BuffDef> {
        self.data.get(&buff_id)
    }

    /// Checks whether the is is in the list of bosses.
    pub fn is_boss(&self, id: u32) -> bool {
        self.bosses.contains(&id)
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
            bosses,
        } = default_definitions();

        assert!(!food.is_empty());
        assert!(!utility.is_empty());
        assert!(!ignore.is_empty());
        assert!(!bosses.is_empty());

        assert!(food.iter().any(|entry| entry.id == MALNOURISHED));
        assert!(utility.iter().any(|entry| entry.id == DIMINISHED));
    }
}
