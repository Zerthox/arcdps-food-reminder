use super::{custom::CustomReminder, Reminder};
use arc_util::settings::HasSettings;
use serde::{Deserialize, Serialize};
use std::time::Duration;

// TODO: game mode setting for inbuilt food & util reminders

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ReminderSettings {
    /// Whether to remind for food.
    pub food: bool,

    /// Whether to remind for utility.
    pub util: bool,

    /// User-defined custom reminders for buffs.
    pub custom: Vec<CustomReminder>,

    /// Duration of the reminder display.
    pub duration: Duration,

    /// Position of the reminder display.
    pub position: f32,

    /// Whether to remind only for boss encounters.
    pub only_bosses: bool,

    /// Whether to remind on encounter start.
    pub encounter_start: bool,

    /// Whether to remind on encounter end.
    pub encounter_end: bool,

    /// Whether to remind during an encounter.
    pub during_encounter: bool,

    /// Whether to always remind when becoming malnourished/diminished.
    pub always_mal_dim: bool,
}

impl ReminderSettings {
    /// Creates new reminder settings with the defaults.
    pub fn new() -> Self {
        Self {
            food: true,
            util: true,
            custom: CustomReminder::defaults(),
            duration: Reminder::DEFAULT_DURATION,
            position: 0.2,
            only_bosses: true,
            encounter_start: true,
            encounter_end: true,
            during_encounter: true,
            always_mal_dim: true,
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
