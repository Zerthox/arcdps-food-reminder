use super::{
    buff::BuffState,
    entry::{Entry, Player},
    Tracker,
};
use arc_util::settings::HasSettings;
use serde::{Deserialize, Serialize};

/// Settings for the tracker.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct TrackerSettings {
    /// Whether to save the buffs on own characters.
    pub save_chars: bool,

    /// Hotkey for the tracker window.
    pub hotkey: Option<usize>,
}

impl TrackerSettings {
    pub const fn new() -> Self {
        Self {
            save_chars: true,
            hotkey: Some(Tracker::DEFAULT_HOTKEY),
        }
    }
}

impl Default for TrackerSettings {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct TrackerState {
    pub settings: TrackerSettings,
    pub own_chars: Vec<SettingsEntry>,
}

impl TrackerState {
    pub const fn new() -> Self {
        Self {
            settings: TrackerSettings::new(),
            own_chars: Vec::new(),
        }
    }
}

impl Default for TrackerState {
    fn default() -> Self {
        Self::new()
    }
}

/// Saved Player entry.
#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsEntry {
    pub player: Player,
    pub food: BuffState,
    pub util: BuffState,
}

impl SettingsEntry {
    pub const fn new(player: Player, food: BuffState, util: BuffState) -> Self {
        Self { player, food, util }
    }
}

// required to save window settings
impl HasSettings for Tracker {
    type Settings = TrackerState;

    const SETTINGS_ID: &'static str = "tracker";

    fn current_settings(&self) -> Self::Settings {
        Self::Settings {
            settings: self.settings.clone(),
            own_chars: if self.settings.save_chars {
                self.get_self()
                    .into_iter()
                    .chain(&self.chars_cache)
                    .map(|entry| {
                        SettingsEntry::new(entry.player.clone(), entry.food.state, entry.util.state)
                    })
                    .collect()
            } else {
                Vec::new()
            },
        }
    }

    fn load_settings(&mut self, loaded: Self::Settings) {
        self.settings = loaded.settings;
        if self.settings.save_chars {
            self.chars_cache = loaded
                .own_chars
                .into_iter()
                .map(|entry| Entry::with_states(entry.player, entry.food, entry.util))
                .collect();
        }
    }
}
