use super::Demo;
use crate::tracking::entry::Entry;
use arc_util::{settings::HasSettings, ui::Hideable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct DemoSettings {
    players: Vec<Entry>,
    tracker: bool,
}

#[allow(clippy::derivable_impls)]
impl Default for DemoSettings {
    fn default() -> Self {
        Self {
            players: Vec::new(),
            tracker: false,
        }
    }
}

impl HasSettings for Demo {
    type Settings = DemoSettings;

    const SETTINGS_ID: &'static str = "demo";

    fn current_settings(&self) -> Self::Settings {
        Self::Settings {
            players: self.tracker.all_players().cloned().collect(),
            tracker: self.tracker.is_visible(),
        }
    }

    fn load_settings(&mut self, loaded: Self::Settings) {
        for loaded in loaded.players {
            let id = loaded.player.id;
            self.tracker.add_player(loaded.player);
            let entry = self.tracker.player_mut(id).unwrap();
            entry.food = loaded.food;
            entry.util = loaded.util;
        }
        self.tracker.set_visibility(loaded.tracker);
    }
}
