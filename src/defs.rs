use crate::data::{DIMINISHED, MALNOURISHED};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::File, path::Path};

/// Default definitions.
const DEFINITIONS: &str = include_str!("../data/definitions.json");

/// Shared buff definitions data.
#[derive(Debug)]
pub struct Definitions {
    data: HashMap<u32, BuffDef>,
}

#[allow(dead_code)]
impl Definitions {
    /// Creates a new set of definitions.
    ///
    /// This only includes malnourished & diminished.
    pub fn empty() -> Self {
        let mut data = HashMap::new();

        // insert malnourished & diminished
        let malnourished = BuffData::simple(MALNOURISHED, "Malnourished", "MALN");
        let diminished = BuffData::simple(DIMINISHED, "Diminished", "DIM");
        data.insert(malnourished.id, BuffDef::Food(malnourished));
        data.insert(diminished.id, BuffDef::Util(diminished));

        Self { data }
    }

    /// Creates a new set of definitions with the default definitions.
    pub fn with_defaults() -> Self {
        let mut defs = Self::empty();
        let DefData { food, utility } = serde_json::from_str(DEFINITIONS).unwrap();
        defs.add_data(food, utility);
        defs
    }

    /// Creates a set of definitions from data.
    pub fn add_data(
        &mut self,
        food: impl IntoIterator<Item = BuffData>,
        util: impl IntoIterator<Item = BuffData>,
    ) {
        let food = food.into_iter();
        let util = util.into_iter();

        // reserve capacity
        let (food_size, _) = food.size_hint();
        let (util_size, _) = util.size_hint();
        self.data.reserve(food_size + util_size);

        // insert data
        for entry in food {
            if let Some(proc) = entry.proc {
                self.data.insert(proc, BuffDef::Proc);
            }
            self.data.insert(entry.id, BuffDef::Food(entry));
        }
        for entry in util {
            if let Some(proc) = entry.proc {
                self.data.insert(proc, BuffDef::Proc);
            }
            self.data.insert(entry.id, BuffDef::Util(entry));
        }
    }

    /// Attempts to load custom definitions from a given file.
    pub fn try_load(&mut self, path: impl AsRef<Path>) -> Result<(), ()> {
        // open file
        let reader = File::open(path).map_err(|_| ())?;

        // read & add data
        let DefData { food, utility } = serde_json::from_reader(reader).map_err(|_| ())?;
        self.add_data(food, utility);

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
    Proc,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuffData {
    pub id: u32,
    pub name: String,

    #[serde(default)]
    pub stats: Vec<String>,

    pub display: String,

    pub proc: Option<u32>,
}

#[allow(dead_code)]
impl BuffData {
    /// Creates a new buff data entry.
    pub fn new(
        id: u32,
        name: impl Into<String>,
        stats: impl IntoIterator<Item = String>,
        display: impl Into<String>,
        proc: Option<u32>,
    ) -> Self {
        Self {
            id,
            name: name.into(),
            stats: stats.into_iter().collect(),
            display: display.into(),
            proc,
        }
    }

    /// Creates a new buff data entry without stats & proc id.
    pub fn simple(id: u32, name: impl Into<String>, display: impl Into<String>) -> Self {
        Self::new(id, name, Vec::new(), display, None)
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
struct DefData {
    pub food: Vec<BuffData>,
    pub utility: Vec<BuffData>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn definitions() {
        let _: DefData = serde_json::from_str(DEFINITIONS).expect("invalid definitions file");
    }
}
