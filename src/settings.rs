use crate::arc_util::exports;
use serde::{Deserialize, Serialize};
use std::{fs::File, path::PathBuf};

/// Name of the config file.
const CONFIG_NAME: &str = "arcdps_food_reminder.json";

/// Settings state.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub tracker_open: bool,
}

impl Settings {
    /// Returns the initial settings.
    pub fn initial() -> Self {
        Self {
            tracker_open: false,
        }
    }

    /// Reads the settings from the settings file.
    pub fn load() -> Option<Self> {
        let path = Self::config_path()?;
        let file = File::open(path).ok()?;
        let settings = serde_json::from_reader(file).ok()?;
        Some(settings)
    }

    /// Saves settings.
    ///
    /// This may silently fail.
    pub fn save(&self) {
        if let Some(path) = Self::config_path() {
            if let Ok(file) = File::create(path) {
                #[allow(unused_must_use)]
                {
                    serde_json::to_writer_pretty(file, self);
                }
            }
        }
    }

    /// Returns the path to the config file.
    pub fn config_path() -> Option<PathBuf> {
        exports::get_config_path().map(|mut path| {
            if !path.is_dir() {
                path.pop();
            }
            path.push(CONFIG_NAME);
            path
        })
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self::initial()
    }
}
