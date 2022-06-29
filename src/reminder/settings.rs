use super::Reminder;
use arc_util::settings::HasSettings;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ReminderSettings {
    pub duration: Duration,
    pub position: f32,
    pub only_bosses: bool,
    pub encounter_start: bool,
    pub encounter_end: bool,
    pub during_encounter: bool,
    pub always_mal_dim: bool,
    pub reinforced: bool,
}

impl ReminderSettings {
    /// Creates new reminder settings with the defaults.
    pub const fn new() -> Self {
        Self {
            duration: Reminder::DEFAULT_DURATION,
            position: 0.2,
            only_bosses: true,
            encounter_start: true,
            encounter_end: true,
            during_encounter: true,
            always_mal_dim: true,
            reinforced: true,
        }
    }
}

impl Default for ReminderSettings {
    fn default() -> Self {
        Self::new()
    }
}

impl HasSettings for Reminder {
    type Settings = ReminderSettings;

    const SETTINGS_ID: &'static str = "reminder";

    fn current_settings(&self) -> Self::Settings {
        self.settings.clone()
    }

    fn load_settings(&mut self, loaded: Self::Settings) {
        self.settings = loaded;
    }
}
