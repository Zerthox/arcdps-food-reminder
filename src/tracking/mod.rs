pub mod buff;
pub mod settings;
pub mod ui;

use crate::builds::Builds;
use arc_util::tracking::{CachedTracker, Player};
use buff::{BuffState, Buffs};
use log::debug;
use settings::TrackerSettings;
use std::cmp::Reverse;
use windows::System::VirtualKey;

/// Player tracker.
#[derive(Debug)]
pub struct Tracker {
    /// Current tracker settings.
    pub settings: TrackerSettings,

    /// Currently tracked players.
    pub players: CachedTracker<Buffs>,

    /// Current ongoing encounter.
    pub encounter: Option<usize>,

    /// Current sorting.
    sorting: Sorting,

    /// Current sorting direction.
    reverse: bool,

    /// Reset confirm state for own characters.
    chars_reset: bool,

    /// User-defined builds.
    builds: Builds,
}

#[allow(dead_code)]
impl Tracker {
    /// Default hotkey for tracker.
    pub const DEFAULT_HOTKEY: u32 = VirtualKey::F.0 as u32;

    /// Creates a new tracker.
    pub const fn new() -> Self {
        Self {
            settings: TrackerSettings::new(),
            players: CachedTracker::for_self(),
            encounter: None,
            sorting: Sorting::Sub,
            reverse: false,
            chars_reset: false,
            builds: Builds::new(),
        }
    }

    /// Adds a new tracked player.
    pub fn add_player(&mut self, player: Player) {
        debug!("Added {} ({})", player.character, player.id);
        self.players.add_player_default(player);

        // refresh sorting
        self.refresh_sort();
    }

    /// Removes a tracked player, returning `true` if they were tracked.
    pub fn remove_player(&mut self, id: usize) -> bool {
        self.players.remove_player(id)
    }

    /// Sorts the players in the tracker table.
    fn refresh_sort(&mut self) {
        match (self.sorting, self.reverse) {
            (Sorting::Sub, false) => self.players.sort_by_key(|entry| entry.player.subgroup),
            (Sorting::Sub, true) => self
                .players
                .sort_by_key(|entry| Reverse(entry.player.subgroup)),

            (Sorting::Name, false) => self
                .players
                .sort_by(|a, b| a.player.character.cmp(&b.player.character)),
            (Sorting::Name, true) => self
                .players
                .sort_by(|a, b| Reverse(&a.player.character).cmp(&Reverse(&b.player.character))),

            (Sorting::Food, false) => self.players.sort_by_key(|entry| entry.data.food.state),
            (Sorting::Food, true) => self
                .players
                .sort_by_key(|entry| Reverse(entry.data.food.state)),

            (Sorting::Util, false) => self.players.sort_by_key(|entry| entry.data.util.state),
            (Sorting::Util, true) => self
                .players
                .sort_by_key(|entry| Reverse(entry.data.util.state)),

            (Sorting::Reinf, false) => self.players.sort_by_key(|entry| entry.data.reinf.state),
            (Sorting::Reinf, true) => self
                .players
                .sort_by_key(|entry| Reverse(entry.data.reinf.state)),
        }
    }
}

/// Current column sorted by.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Sorting {
    Sub,
    Name,
    Food,
    Util,
    Reinf,
}
