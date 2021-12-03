pub use crate::data::{Food, Utility};
use serde::{Deserialize, Serialize};

/// Struct representing a buff.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Buff<T> {
    /// Current state of the buff.
    pub state: BuffState<T>,

    /// ID of the last event update.
    pub event_id: u64,
}

impl<T> Buff<T> {
    /// Creates a new buff.
    pub fn new(state: BuffState<T>, event_id: u64) -> Self {
        Self { state, event_id }
    }

    /// Updates the buff state.
    ///
    /// Returns `false` if this update was ignored due to out of order.
    pub fn update(&mut self, state: BuffState<T>, event_id: u64) -> bool {
        if event_id > self.event_id {
            self.state = state;
            self.event_id = event_id;
            true
        } else {
            false
        }
    }
}

/// Possible buff states.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BuffState<T> {
    /// Buff state is not set (yet).
    ///
    /// This is the initial value.
    Unset,

    /// No buff is applied.
    None,

    /// Some buff is applied but not recognized.
    Unknown(u32),

    /// Some buff is applied and recognized.
    Known(T),
}
