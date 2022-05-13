//! Dummy windows for demo.

pub mod settings;
pub mod ui;

use crate::{
    defs::{BuffDef, Definitions},
    reminder::Reminder,
    tracking::{buff::BuffState, Tracker},
};
use arc_util::ui::{Window, WindowOptions};
use std::borrow::Cow;

/// Features demo.
#[derive(Debug)]
pub struct Demo {
    reminder: Reminder,
    all_foods: Vec<BuffState>,
    all_utils: Vec<BuffState>,
    tracker: Window<Tracker>,
}

impl Demo {
    /// Creates a new demo.
    pub fn new() -> Self {
        Self {
            reminder: Reminder::new(),
            all_foods: Vec::new(),
            all_utils: Vec::new(),
            tracker: Window::new(
                WindowOptions {
                    auto_resize: true,
                    ..WindowOptions::new("Demo Food Tracker")
                },
                Tracker::new(),
            ),
        }
    }

    /// Returns the display name for a given food buff.
    fn food_name(defs: &Definitions, buff: BuffState) -> Cow<'static, str> {
        match buff {
            BuffState::Unknown => "Unset".into(),
            BuffState::None => "None".into(),
            BuffState::Some(buff) => {
                if let Some(BuffDef::Food(food)) = defs.get_buff(buff) {
                    food.name.clone().into()
                } else {
                    "Unknown".into()
                }
            }
        }
    }

    /// Returns the display name for a given utility buff.
    fn util_name(defs: &Definitions, buff: BuffState) -> Cow<'static, str> {
        match buff {
            BuffState::Unknown => "Unset".into(),
            BuffState::None => "None".into(),
            BuffState::Some(buff) => {
                if let Some(BuffDef::Util(util)) = defs.get_buff(buff) {
                    util.name.clone().into()
                } else {
                    "Unknown".into()
                }
            }
        }
    }
}
