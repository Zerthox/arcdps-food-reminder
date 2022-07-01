use super::{
    entry::{Entry, Player},
    state::BuffState,
    Tracker,
};
use crate::builds::Builds;
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

    #[serde(default)]
    pub food: BuffState<u32>,

    #[serde(default)]
    pub util: BuffState<u32>,

    #[serde(default)]
    pub reinforced: BuffState<()>,
}

impl SettingsEntry {
    pub const fn new(
        player: Player,
        food: BuffState<u32>,
        util: BuffState<u32>,
        reinforced: BuffState<()>,
    ) -> Self {
        Self {
            player,
            food,
            util,
            reinforced,
        }
    }
}

impl From<Entry> for SettingsEntry {
    fn from(entry: Entry) -> Self {
        Self::new(
            entry.player,
            entry.food.state,
            entry.util.state,
            entry.reinf.state,
        )
    }
}

impl From<SettingsEntry> for Entry {
    fn from(entry: SettingsEntry) -> Self {
        Self::with_buffs(entry.player, entry.food, entry.util, entry.reinforced)
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
                    .cloned()
                    .map(Into::into)
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
            self.chars_cache = loaded.own_chars.into_iter().map(Into::into).collect();
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
