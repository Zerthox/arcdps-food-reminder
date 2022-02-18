use arc_util::settings::Settings;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{self, File},
    path::Path,
    sync::{Arc, Mutex},
};

/// Default definitions.
const DEFINITIONS: &str = include_str!("../data/definitions.json");

/// Shared buff definitions data.
#[derive(Debug, Clone)]
pub struct Definitions {
    data: Arc<Mutex<HashMap<u32, BuffDef>>>,
}

#[allow(dead_code)]
impl Definitions {
    /// Creates an empty set of definitions.
    pub fn new() -> Self {
        Self {
            data: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Adds new definitions data.
    pub fn add_data(
        &mut self,
        food: impl IntoIterator<Item = BuffData>,
        util: impl IntoIterator<Item = BuffData>,
    ) {
        let mut data = self.data.lock().unwrap();
        for entry in food {
            if let Some(proc) = entry.proc {
                data.insert(proc, BuffDef::Proc);
            }
            data.insert(entry.id, BuffDef::Food(entry));
        }
        for entry in util {
            if let Some(proc) = entry.proc {
                data.insert(proc, BuffDef::Proc);
            }
            data.insert(entry.id, BuffDef::Util(entry));
        }
    }

    /// Attempts to load definitions from a given file.
    pub fn load(&mut self, file: impl AsRef<Path>) {
        if let Some(path) = Settings::config_path(file) {
            // write out defaults in case the file does not exist
            if !path.exists() {
                let _ = fs::write(&path, DEFINITIONS);
            }

            // try to read definition data from file
            if let Some(DefData { food, utility }) = File::open(path)
                .ok()
                .and_then(|reader| serde_json::from_reader(reader).ok())
            {
                self.add_data(food, utility)
            }
        }
    }

    /// Returns the buff definition with the given id.
    pub fn get(&self, buff_id: u32) -> Option<BuffDef> {
        self.data.lock().unwrap().get(&buff_id).cloned()
    }

    /// Returns all food definitions.
    pub fn all_food(&self) -> Vec<BuffData> {
        self.data
            .lock()
            .unwrap()
            .values()
            .filter_map(|entry| match entry {
                BuffDef::Food(data) => Some(data.clone()),
                _ => None,
            })
            .collect()
    }

    /// Returns all utility definitions.
    pub fn all_util(&self) -> Vec<BuffData> {
        self.data
            .lock()
            .unwrap()
            .values()
            .filter_map(|entry| match entry {
                BuffDef::Util(data) => Some(data.clone()),
                _ => None,
            })
            .collect()
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

#[derive(Debug, Serialize, Deserialize)]
struct DefData {
    pub food: Vec<BuffData>,
    pub utility: Vec<BuffData>,
}
