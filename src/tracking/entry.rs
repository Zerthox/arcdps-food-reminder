use super::state::{BuffState, TrackedState};
use std::cmp;

pub use arc_util::game::{Player, Profession, Specialization};

// TODO: track buff duration & reset to unset when duration runs out?

/// Struct representing a tracker entry.
#[derive(Debug, Clone)]
pub struct Entry {
    /// Player this entry corresponds to.
    pub player: Player,

    /// Current food buff applied to the player.
    pub food: TrackedState<BuffState<u32>>,

    /// Current utility buff applied to the player.
    pub util: TrackedState<BuffState<u32>>,

    /// Whether the Reinforced Armor buff is applied to the player.
    pub reinf: TrackedState<BuffState<()>>,
}

impl Entry {
    /// Creates a new entry.
    pub const fn new(player: Player) -> Self {
        Self::with_states(
            player,
            BuffState::Unknown,
            BuffState::Unknown,
            BuffState::Unknown,
        )
    }

    /// Creates a new entry with initial buff states.
    pub const fn with_states(
        player: Player,
        food: BuffState<u32>,
        util: BuffState<u32>,
        reinforced: BuffState<()>,
    ) -> Self {
        Self {
            player,
            food: TrackedState::new(food),
            util: TrackedState::new(util),
            reinf: TrackedState::new(reinforced),
        }
    }

    /// Resets all buffs.
    pub fn reset_buffs(&mut self) {
        self.food = TrackedState::default();
        self.util = TrackedState::default();
        self.reinf = TrackedState::default();
    }

    /// Sets all buffs to none.
    pub fn buffs_to_none(&mut self, time: u64, event_id: u64) {
        self.food.update(BuffState::None, time, event_id);
        self.util.update(BuffState::None, time, event_id);
        self.reinf.update(BuffState::None, time, event_id);
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

    /// Applies the Reinforced Armor buff to the player.
    ///
    /// Returns `false` if this update was ignored.
    pub fn apply_reinf(&mut self, time: u64, event_id: u64) -> bool {
        self.reinf.update(BuffState::Some(()), time, event_id)
    }

    /// Removes the Reinforced Armor buff from the player.
    ///
    /// Returns `false` if this update was ignored.
    pub fn remove_reinf(&mut self, time: u64, event_id: u64) -> bool {
        self.reinf.update(BuffState::None, time, event_id)
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
