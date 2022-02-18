use super::{
    buff::BuffState,
    entry::{Entry, Player},
    Tracker,
};
use arc_util::settings::HasSettings;
use serde::{Deserialize, Serialize};

/// Saved Player entry.
#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsEntry {
    player: Player,
    food: BuffState,
    util: BuffState,
}

impl SettingsEntry {
    pub const fn new(player: Player, food: BuffState, util: BuffState) -> Self {
        Self { player, food, util }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct TrackerSettings {
    save_chars: bool,
    own_chars: Vec<SettingsEntry>,
}

impl Default for TrackerSettings {
    fn default() -> Self {
        Self {
            save_chars: true,
            own_chars: Vec::new(),
        }
    }
}

// required to save window settings
impl HasSettings for Tracker {
    type Settings = TrackerSettings;

    const SETTINGS_ID: &'static str = "tracker";

    fn current_settings(&self) -> Self::Settings {
        Self::Settings {
            save_chars: self.save_chars,
            own_chars: if self.save_chars {
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
        self.save_chars = loaded.save_chars;
        if self.save_chars {
            self.chars_cache = loaded
                .own_chars
                .into_iter()
                .map(|entry| Entry::with_states(entry.player, entry.food, entry.util))
                .collect();
        }
    }
}
