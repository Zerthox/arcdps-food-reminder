use crate::arc_util::exports;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::{Map, Value};
use std::{fs::File, path::PathBuf};

/// Helper trait for components with settings.
pub trait HasSettings {
    type Settings: Serialize + DeserializeOwned;

    /// Returns the component's settings name.
    fn settings_name() -> &'static str;

    /// Returns the component's current settings state.
    fn get_settings(&self) -> Self::Settings;

    /// Loads the component's settings from a loaded version.
    fn load_settings(&mut self, loaded: Self::Settings);
}

/// Name of the config file.
const CONFIG_NAME: &str = "arcdps_food_reminder.json";

/// Package version.
const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Settings state.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    version: String,
    components: Map<String, Value>,
}

impl Settings {
    /// Returns empty settings.
    pub fn new() -> Self {
        Self {
            version: VERSION.into(),
            components: Map::new(),
        }
    }

    /// Reads settings from the settings file.
    pub fn load() -> Option<Self> {
        let path = Self::config_path()?;
        let file = File::open(path).ok()?;
        let settings = serde_json::from_reader(file).ok()?;
        Some(settings)
    }

    /// Loads settings or defaults to the initial ones.
    pub fn load_or_initial() -> Self {
        Self::load().unwrap_or_default()
    }

    /// Saves settings to the settings file.
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
        exports::config_path().map(|mut path| {
            if !path.is_dir() {
                path.pop();
            }
            path.push(CONFIG_NAME);
            path
        })
    }

    /// Loads a component's settings from the settings map.
    pub fn load_component<T>(&mut self, component: &mut T)
    where
        T: HasSettings,
    {
        if let Some(value) = self.components.remove(T::settings_name()) {
            if let Ok(loaded) = serde_json::from_value(value) {
                component.load_settings(loaded)
            }
        }
    }

    /// Stores a component's settings in the settings map.
    ///
    /// Silently fails if the component's settings fail serialization.
    pub fn store_component<T>(&mut self, component: &T)
    where
        T: HasSettings,
    {
        if let Ok(value) = serde_json::to_value(component.get_settings()) {
            self.components.insert(T::settings_name().into(), value);
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self::new()
    }
}
