use serde::{Deserialize, Serialize};
use std::cmp::{Ordering, PartialOrd};

/// Struct representing a buff.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Buff {
    /// Current state of the buff.
    pub state: BuffState,

    /// Timestamp of the last update.
    pub time: u64,

    /// Event id of the last update.
    pub event_id: u64,
}

impl Buff {
    /// Creates a new buff.
    pub const fn new(state: BuffState, time: u64, event_id: u64) -> Self {
        Self {
            state,
            time,
            event_id,
        }
    }

    /// Updates the buff state.
    ///
    /// Returns `false` if this update was ignored due to out of order.
    pub fn update(&mut self, state: BuffState, time: u64, event_id: u64) -> bool {
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

/// Possible buff states.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum BuffState {
    /// Buff state is not set (yet).
    ///
    /// This is the initial value.
    Unknown,

    /// No buff is applied.
    None,

    /// Some buff is applied.
    Some(u32),
}
