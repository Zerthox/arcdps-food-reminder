use super::buff::{Buff, BuffState};
use serde::{Deserialize, Serialize};
use std::cmp;

pub use arc_util::game::{Player, Profession, Specialization};

// TODO: track buff duration & reset to unset when duration runs out?

/// Struct representing a tracker entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entry {
    /// Player this entry corresponds to.
    pub player: Player,

    /// Current food buff applied to the player.
    pub food: Buff,

    /// Current utility buff applied to the player.
    pub util: Buff,
}

impl Entry {
    /// Creates a new entry.
    pub const fn new(player: Player) -> Self {
        Self::with_states(player, BuffState::Unknown, BuffState::Unknown)
    }

    /// Creates a new entry with initial buff states.
    pub const fn with_states(player: Player, food: BuffState, util: BuffState) -> Self {
        Self {
            player,
            food: Buff::new(food, 0, 0),
            util: Buff::new(util, 0, 0),
        }
    }

    /// Sets all unset buffs to none.
    pub fn unset_to_none(&mut self, time: u64, event_id: u64) {
        if self.food.state == BuffState::Unknown {
            self.food.update(BuffState::None, time, event_id);
        }
        if self.util.state == BuffState::Unknown {
            self.util.update(BuffState::None, time, event_id);
        }
    }

    /// Applies a food buff to the player.
    ///
    /// Returns `true` if this update changed the buff state.
    pub fn apply_food(&mut self, food: u32, time: u64, event_id: u64) -> bool {
        self.food.update(BuffState::Some(food), time, event_id)
    }

    /// Removes the current food buff from the player.
    ///
    /// Has no effect if the current buff is different from the passed buff.
    /// Passing [`None`] indicates a [`BuffState::Unknown`].
    /// [`BuffState::Unset`] is always removed.
    ///
    /// Returns `false` if this update was ignored.
    pub fn remove_food(&mut self, food: u32, time: u64, event_id: u64) -> bool {
        let changed = match self.food.state {
            BuffState::Some(applied) => food == applied,
            _ => true,
        };
        if changed {
            self.food.update(BuffState::None, time, event_id)
        } else {
            false
        }
    }

    /// Applies an utility buff to the player.
    ///
    /// Returns `false` if this update was ignored.
    pub fn apply_util(&mut self, util: u32, time: u64, event_id: u64) -> bool {
        self.util.update(BuffState::Some(util), time, event_id)
    }

    /// Removes the current utility buff from the player.
    ///
    /// Has no effect if the current buff is different from the passed buff.
    /// Passing [`None`] indicates a [`BuffState::Unknown`].
    /// [`BuffState::Unset`] is always removed.
    ///
    /// Returns `false` if this update was ignored.
    pub fn remove_util(&mut self, util: u32, time: u64, event_id: u64) -> bool {
        let changed = match self.util.state {
            BuffState::Some(applied) => util == applied,
            _ => true,
        };
        if changed {
            self.util.update(BuffState::None, time, event_id)
        } else {
            false
        }
    }
}

impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        self.player == other.player
    }
}

impl Eq for Entry {}

impl cmp::PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.player.partial_cmp(&other.player)
    }
}

impl cmp::Ord for Entry {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.player.cmp(&other.player)
    }
}

impl From<Player> for Entry {
    fn from(player: Player) -> Self {
        Self::new(player)
    }
}
