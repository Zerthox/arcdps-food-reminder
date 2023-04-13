use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

// TODO: track buff duration & reset to unset when duration runs out?

/// Buff information.
#[derive(Debug, Clone)]
pub struct Buffs {
    /// Current food buff applied to the player.
    pub food: TrackedBuff<u32>,

    /// Current utility buff applied to the player.
    pub util: TrackedBuff<u32>,

    /// Custom tracked buffs.
    pub custom: BTreeMap<u32, TrackedBuff<()>>,
}

impl Buffs {
    /// Creates new buff information with initial states.
    pub fn new() -> Self {
        Self::with_states(BuffState::Unknown, BuffState::Unknown, BTreeMap::new())
    }

    /// Creates new buff information with given buff states.
    pub fn with_states(
        food: BuffState<u32>,
        util: BuffState<u32>,
        custom: BTreeMap<u32, BuffState<()>>,
    ) -> Self {
        Self::with_buffs(
            TrackedBuff::new(food),
            TrackedBuff::new(util),
            custom
                .into_iter()
                .map(|(id, state)| (id, TrackedBuff::new(state)))
                .collect(),
        )
    }

    /// Creates new buff information with given tracked buffs.
    pub const fn with_buffs(
        food: TrackedBuff<u32>,
        util: TrackedBuff<u32>,
        custom: BTreeMap<u32, TrackedBuff<()>>,
    ) -> Self {
        Self { food, util, custom }
    }

    /// Resets all buffs.
    pub fn reset_buffs(&mut self) {
        self.food = Default::default();
        self.util = Default::default();
        self.custom = Default::default();
    }

    /// Sets all unset buff states to none.
    pub fn unset_to_none(&mut self, time: u64, custom_ids: impl Iterator<Item = u32>) {
        self.food.update_if_unknown(BuffState::None, time);
        self.util.update_if_unknown(BuffState::None, time);
        for id in custom_ids {
            self.custom
                .entry(id)
                .or_default()
                .update_if_unknown(BuffState::None, time);
        }
    }

    /// Applies a food buff to the player.
    ///
    /// Returns `true` if this update changed the buff state.
    pub fn apply_food(&mut self, food: u32, time: u64) -> bool {
        self.food.update(BuffState::Some(food), time, true)
    }

    /// Removes the current food buff from the player.
    ///
    /// Has no effect if the current buff is different from the passed buff.
    /// Passing [`None`] indicates a [`BuffState::Unknown`].
    /// [`BuffState::Unset`] is always removed.
    ///
    /// Returns `false` if this update was ignored.
    pub fn remove_food(&mut self, food: u32, time: u64) -> bool {
        let changed = match self.food.state {
            BuffState::Some(applied) => food == applied,
            _ => true,
        };
        if changed {
            self.food.update(BuffState::None, time, false)
        } else {
            false
        }
    }

    /// Applies an utility buff to the player.
    ///
    /// Returns `false` if this update was ignored.
    pub fn apply_util(&mut self, util: u32, time: u64) -> bool {
        self.util.update(BuffState::Some(util), time, true)
    }

    /// Removes the current utility buff from the player.
    ///
    /// Has no effect if the current buff is different from the passed buff.
    /// Passing [`None`] indicates a [`BuffState::Unknown`].
    /// [`BuffState::Unset`] is always removed.
    ///
    /// Returns `false` if this update was ignored.
    pub fn remove_util(&mut self, util: u32, time: u64) -> bool {
        let changed = match self.util.state {
            BuffState::Some(applied) => util == applied,
            _ => true,
        };
        if changed {
            self.util.update(BuffState::None, time, false)
        } else {
            false
        }
    }

    /// Applies a custom tracked buff to the player.
    ///
    /// Returns `false` if this update was ignored.
    pub fn apply_custom(&mut self, id: u32, time: u64) -> bool {
        self.custom
            .entry(id)
            .or_default()
            .update(BuffState::Some(()), time, true)
    }

    /// Removes a custom tracked buff from the player.
    ///
    /// Returns `false` if this update was ignored.
    pub fn remove_custom(&mut self, id: u32, time: u64) -> bool {
        if let Some(buff) = self.custom.get_mut(&id) {
            buff.update(BuffState::None, time, false)
        } else {
            false
        }
    }

    /// Returns the [`BuffState`] of the given custom buff id.
    pub fn custom_state(&self, id: u32) -> BuffState<()> {
        self.custom
            .get(&id)
            .map(|buff| buff.state)
            .unwrap_or_default()
    }
}

impl Default for Buffs {
    fn default() -> Self {
        Self::new()
    }
}

/// Struct representing a tracked buff.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrackedBuff<T> {
    /// Current buff state.
    pub state: BuffState<T>,

    /// Timestamp of the last update.
    pub time: u64,
}

impl<T> TrackedBuff<T> {
    /// Creates a new tracked buff.
    pub const fn new(state: BuffState<T>) -> Self {
        Self { state, time: 0 }
    }

    /// Updates the tracked buff.
    ///
    /// `time` is the timestamp of the event.
    /// `overwrite` determines whether the same time replaces the current state.
    ///
    /// Returns `false` if this update was ignored due to out of order.
    pub fn update(&mut self, state: BuffState<T>, time: u64, overwrite: bool) -> bool {
        // check for later time or same time & overwrite
        if time > self.time || (overwrite && time == self.time) {
            self.state = state;
            self.time = time;
            true
        } else {
            false
        }
    }

    /// Updates the tracked buff state if it is currently [`BuffState::Unknown`].
    ///
    /// Returns `false` if this update was ignored.
    pub fn update_if_unknown(&mut self, state: BuffState<T>, time: u64) -> bool {
        if let BuffState::Unknown = self.state {
            self.state = state;
            self.time = time;
            true
        } else {
            false
        }
    }
}

impl<T> Default for TrackedBuff<T>
where
    T: Default,
{
    fn default() -> Self {
        Self::new(BuffState::default())
    }
}

/// Possible buff states.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum BuffState<T> {
    /// Buff state is not set (yet).
    ///
    /// This is the initial value.
    #[default]
    Unknown,

    /// No buff is applied.
    None,

    /// Some buff is applied.
    Some(T),
}
