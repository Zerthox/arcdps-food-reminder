pub mod entry;
pub mod settings;
pub mod state;
pub mod ui;

use crate::builds::Builds;
use entry::{Entry, Player};
use settings::TrackerSettings;
use state::BuffState;
use std::cmp::Reverse;
use windows::System::VirtualKey;

/// Player tracker.
// TODO: split generic utility to track players and add to arc_util
#[derive(Debug)]
pub struct Tracker {
    /// Current tracker settings.
    pub settings: TrackerSettings,

    /// Currently tracked players.
    players: Vec<Entry>,

    /// Current ongoing encounter.
    encounter: Option<usize>,

    /// Current sorting.
    sorting: Sorting,

    /// Current sorting direction.
    reverse: bool,

    /// Cache for buffs on own characters of local player (self).
    chars_cache: Vec<Entry>,

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
            players: Vec::new(),
            encounter: None,
            sorting: Sorting::Sub,
            reverse: false,
            chars_cache: Vec::new(),
            chars_reset: false,
            builds: Builds::new(),
        }
    }

    /// Adds a new tracked player.
    pub fn add_player(&mut self, player: Player) {
        let mut added = Entry::new(player);

        // check for self
        if added.player.is_self {
            // check cache
            if let Some(index) = self
                .chars_cache
                .iter()
                .position(|entry| entry.player.character == added.player.character)
            {
                // use cached buffs
                let Entry {
                    food, util, reinf, ..
                } = self.chars_cache.remove(index);
                added = Entry::with_states(added.player, food, util, reinf);
            }
        }

        // insert entry
        self.players.push(added);

        // refresh sorting
        self.refresh_sort();
    }

    /// Removes a tracked player, returning the removed entry if they were tracked.
    pub fn remove_player(&mut self, id: usize) -> Option<Entry> {
        self.players
            .iter()
            .position(|entry| entry.player.id == id)
            .map(|index| {
                // remove entry, sorting will be preserved
                let removed = self.players.remove(index);

                // check for self
                if removed.player.is_self {
                    // cache own character buffs in case we play it again later
                    self.chars_cache.push(removed.clone());
                }

                // return removed entry
                removed
            })
    }

    /// Returns a reference to the local player (self).
    pub fn get_self(&self) -> Option<&Entry> {
        self.players.iter().find(|entry| entry.player.is_self)
    }

    /// Returns a mutable reference to the local player (self).
    pub fn get_self_mut(&mut self) -> Option<&mut Entry> {
        self.players.iter_mut().find(|entry| entry.player.is_self)
    }

    /// Returns a reference to a tracked player entry.
    pub fn player(&self, id: usize) -> Option<&Entry> {
        self.players.iter().find(|entry| entry.player.id == id)
    }

    /// Returns a mutable reference to a tracked player entry.
    pub fn player_mut(&mut self, id: usize) -> Option<&mut Entry> {
        self.players.iter_mut().find(|entry| entry.player.id == id)
    }

    /// Returns an iterator over all tracked player entries.
    pub fn all_players(&self) -> impl Iterator<Item = &Entry> + Clone {
        self.players.iter()
    }

    /// Returns a mutable iterator over all tracked player entries.
    pub fn all_players_mut(&mut self) -> impl Iterator<Item = &mut Entry> {
        self.players.iter_mut()
    }

    /// Returns the number of tracked players.
    pub fn len(&self) -> usize {
        self.players.len()
    }

    /// Returns `true` if there is no tracked players.
    pub fn is_empty(&self) -> bool {
        self.players.is_empty()
    }

    /// Starts an encounter.
    pub fn start_encounter(&mut self, target_id: usize) {
        self.encounter = Some(target_id);
    }

    /// Ends the current encounter.
    pub fn end_encounter(&mut self) {
        self.encounter = None;
    }

    /// Returns the encounter state.
    pub fn encounter(&self) -> Option<usize> {
        self.encounter
    }

    /// Returns `true` if there is an ongoing boss encounter.
    pub fn in_encounter(&self) -> bool {
        self.encounter.is_some()
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

            (Sorting::Food, false) => self.players.sort_by_key(|entry| entry.food.state),
            (Sorting::Food, true) => self.players.sort_by_key(|entry| Reverse(entry.food.state)),

            (Sorting::Util, false) => self.players.sort_by_key(|entry| entry.util.state),
            (Sorting::Util, true) => self.players.sort_by_key(|entry| Reverse(entry.util.state)),

            (Sorting::Reinf, false) => self.players.sort_by_key(|entry| entry.reinf.state),
            (Sorting::Reinf, true) => self.players.sort_by_key(|entry| Reverse(entry.reinf.state)),
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
