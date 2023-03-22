use super::Demo;
use crate::tracking::buff::{BuffState, Buffs, TrackedBuff};
use arc_util::{
    settings::HasSettings,
    tracking::{Entry, Player},
    ui::Hideable,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct DemoSettings {
    players: Vec<DemoEntry>,
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
        for (i, loaded) in loaded.players.into_iter().enumerate() {
            self.tracker.add_player(loaded.player);
            let Entry { data, .. } = self.tracker.players.player_mut(i).unwrap();
            data.food = TrackedBuff::new(loaded.food);
            data.util = TrackedBuff::new(loaded.util);
            data.custom = loaded
                .buffs
                .into_iter()
                .map(|(id, state)| (id, TrackedBuff::new(state)))
                .collect();
        }
        self.tracker.set_visibility(loaded.tracker);
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct DemoEntry {
    pub player: Player,

    #[serde(default)]
    pub food: BuffState<u32>,

    #[serde(default)]
    pub util: BuffState<u32>,

    #[serde(default)]
    pub buffs: BTreeMap<u32, BuffState<()>>,
}

impl From<Entry<Buffs>> for DemoEntry {
    fn from(entry: Entry<Buffs>) -> Self {
        Self {
            player: entry.player,
            food: entry.data.food.state,
            util: entry.data.util.state,
            buffs: entry
                .data
                .custom
                .into_iter()
                .map(|(id, buff)| (id, buff.state))
                .collect(),
        }
    }
}
