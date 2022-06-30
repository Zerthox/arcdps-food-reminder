use serde::{Deserialize, Serialize};
use std::cmp::{Ordering, PartialOrd};

/// Struct representing a buff.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrackedState<T> {
    /// Current state.
    pub state: T,

    /// Timestamp of the last update.
    pub time: u64,

    /// Event id of the last update.
    pub event_id: u64,
}

impl<T> TrackedState<T> {
    /// Creates a new tracked state.
    pub const fn new(state: T) -> Self {
        Self {
            state,
            time: 0,
            event_id: 0,
        }
    }

    /// Updates the state.
    ///
    /// Returns `false` if this update was ignored due to out of order.
    pub fn update(&mut self, state: T, time: u64, event_id: u64) -> bool {
        // check for later time or same time & later event id
        match (time.cmp(&self.time), event_id > self.event_id) {
            (Ordering::Greater, _) | (Ordering::Equal, true) => {
                self.state = state;
                self.time = time;
                self.event_id = event_id;
                true
            }
            _ => false,
        }
    }
}

impl<T> Default for TrackedState<T>
where
    T: Default,
{
    fn default() -> Self {
        Self::new(T::default())
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
