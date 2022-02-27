use crate::data::{DIMINISHED, MALNOURISHED};
use arc_util::settings::Settings;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{self, File},
    path::Path,
};

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
    pub fn new() -> Self {
        let mut data = HashMap::new();

        // insert malnourished & diminished
        let malnourished = BuffData::simple(MALNOURISHED, "Malnourished", "MALN");
        let diminished = BuffData::simple(DIMINISHED, "Diminished", "DIM");
        data.insert(malnourished.id, BuffDef::Food(malnourished));
        data.insert(diminished.id, BuffDef::Util(diminished));

        Self { data }
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
    ///
    /// Saves & uses the default definitions if there is an error.
    pub fn try_load(&mut self, file: impl AsRef<Path>) {
        if let Some(path) = Settings::config_path(file) {
            if path.exists() {
                // try to read definition data from file
                if let Some(DefData { food, utility }) = File::open(path)
                    .ok()
                    .and_then(|reader| serde_json::from_reader(reader).ok())
                {
                    self.add_data(food, utility);
                }
            } else {
                // save & use defaults
                let _ = fs::write(&path, DEFINITIONS);
                let DefData { food, utility } = serde_json::from_str(DEFINITIONS).unwrap();
                self.add_data(food, utility);
            }
        }
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

impl Default for Definitions {
    fn default() -> Self {
        Self::new()
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

#[derive(Debug, Serialize, Deserialize)]
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
