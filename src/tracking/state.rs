use serde::{Deserialize, Serialize};

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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum BuffState<T> {
    /// Buff state is not set (yet).
    ///
    /// This is the initial value.
    Unknown,

    /// No buff is applied.
    None,

    /// Some buff is applied.
    Some(T),
}

impl<T> Default for BuffState<T> {
    fn default() -> Self {
        Self::Unknown
    }
}
