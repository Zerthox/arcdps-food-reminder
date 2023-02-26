use super::{
    buff::{BuffState, Buffs},
    Tracker,
};
use crate::{builds::Builds, data::REINFORCED};
use arc_util::{
    settings::HasSettings,
    tracking::{Entry, Player},
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use strum::EnumIter;

/// Settings for the tracker.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct TrackerSettings {
    /// Whether to save the buffs on own characters.
    pub save_chars: bool,

    /// Hotkey for the tracker window.
    pub hotkey: Option<u32>,

    /// Whether to show table header icons.
    pub show_icons: bool,

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
            show_icons: true,
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
    pub buffs: BTreeMap<u32, BuffState<()>>,

    /// Reinforced state for backwards compatibility.
    #[serde(skip_serializing)]
    pub reinforced: Option<BuffState<()>>,
}

impl SettingsEntry {
    pub const fn new(
        player: Player,
        food: BuffState<u32>,
        util: BuffState<u32>,
        buffs: BTreeMap<u32, BuffState<()>>,
    ) -> Self {
        Self {
            player,
            food,
            util,
            reinforced: None,
            buffs,
        }
    }
}

impl From<Entry<Buffs>> for SettingsEntry {
    fn from(entry: Entry<Buffs>) -> Self {
        Self::new(
            entry.player,
            entry.data.food.state,
            entry.data.util.state,
            entry
                .data
                .custom
                .into_iter()
                .map(|(id, buff)| (id, buff.state))
                .collect(),
        )
    }
}

impl From<SettingsEntry> for Entry<Buffs> {
    fn from(mut entry: SettingsEntry) -> Self {
        // load old reinforced
        if let Some(reinf) = entry.reinforced {
            entry.buffs.insert(REINFORCED, reinf);
        }

        Self::new(
            entry.player,
            Buffs::with_states(entry.food, entry.util, entry.buffs),
        )
    }
}

impl HasSettings for Tracker {
    type Settings = TrackerState;

    const SETTINGS_ID: &'static str = "tracker";

    fn current_settings(&self) -> Self::Settings {
        Self::Settings {
            settings: self.settings.clone(),
            own_chars: if self.settings.save_chars {
                self.players
                    .get_self()
                    .into_iter()
                    .chain(self.players.cache_iter())
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
            self.players
                .cache_multiple(loaded.own_chars.into_iter().map(Into::into));
        }

        self.builds.load_settings(loaded.builds);
    }
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, EnumIter, Serialize, Deserialize,
)]
pub enum Color {
    None,
    Sub,
    Prof,
}

impl AsRef<str> for Color {
    fn as_ref(&self) -> &str {
        match self {
            Self::None => "None",
            Self::Sub => "Subgroup",
            Self::Prof => "Profession",
        }
    }
}
