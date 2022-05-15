use crate::builds::Builds;

use super::{
    buff::BuffState,
    entry::{Entry, Player},
    Tracker,
};
use arc_util::settings::HasSettings;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};

/// Settings for the tracker.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct TrackerSettings {
    /// Whether to save the buffs on own characters.
    pub save_chars: bool,

    /// Hotkey for the tracker window.
    pub hotkey: Option<u32>,

    /// Whether to show the subgroup column.
    pub show_sub: bool,

    /// Color for subgroup numbers.
    pub color_sub: Color,

    /// Color for player names.
    pub color_name: Color,
}

impl TrackerSettings {
    pub const fn new() -> Self {
        Self {
            save_chars: true,
            hotkey: Some(Tracker::DEFAULT_HOTKEY),
            show_sub: true,
            color_sub: Color::Sub,
            color_name: Color::Prof,
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
    pub builds: Builds,
}

impl TrackerState {
    pub const fn new() -> Self {
        Self {
            settings: TrackerSettings::new(),
            own_chars: Vec::new(),
            builds: Builds::new(),
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
            builds: self.builds.clone(),
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

        self.builds.load_settings(loaded.builds);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Color {
    None,
    Sub,
    Prof,
}

impl Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::Sub => write!(f, "Subgroup"),
            Self::Prof => write!(f, "Profession"),
        }
    }
}
