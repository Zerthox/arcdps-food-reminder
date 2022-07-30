use super::Demo;
use crate::tracking::{buff::TrackedBuff, settings::SettingsEntry};
use arc_util::{settings::HasSettings, tracking::Entry, ui::Hideable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct DemoSettings {
    players: Vec<SettingsEntry>,
    tracker: bool,
}

impl DemoSettings {
    pub const fn new() -> Self {
        Self {
            players: Vec::new(),
            tracker: false,
        }
    }
}

impl Default for DemoSettings {
    fn default() -> Self {
        Self::new()
    }
}

impl HasSettings for Demo {
    type Settings = DemoSettings;

    const SETTINGS_ID: &'static str = "demo";

    fn current_settings(&self) -> Self::Settings {
        Self::Settings {
            players: self
                .tracker
                .players
                .iter()
                .cloned()
                .map(Into::into)
                .collect(),
            tracker: self.tracker.is_visible(),
        }
    }

    fn load_settings(&mut self, loaded: Self::Settings) {
        for loaded in loaded.players {
            let id = loaded.player.id;
            self.tracker.add_player(loaded.player);
            let Entry { data, .. } = self.tracker.players.player_mut(id).unwrap();
            data.food = TrackedBuff::new(loaded.food);
            data.util = TrackedBuff::new(loaded.util);
            data.reinf = TrackedBuff::new(loaded.reinforced);
        }
        self.tracker.set_visibility(loaded.tracker);
    }
}
