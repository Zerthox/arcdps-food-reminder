//! Dummy windows for demo.

pub mod settings;
pub mod ui;

use crate::{
    data::{DefinitionKind, Definitions},
    reminder::Reminder,
    tracking::{buff::BuffState, Tracker},
};
use arc_util::ui::{Window, WindowOptions};
use std::borrow::Cow;

/// Features demo.
#[derive(Debug)]
pub struct Demo {
    all_foods: Vec<BuffState<u32>>,
    all_utils: Vec<BuffState<u32>>,
    pub reminder: Reminder,
    pub tracker: Window<Tracker>,
}

impl Demo {
    /// Creates a new demo.
    pub fn new() -> Self {
        Self {
            all_foods: Vec::new(),
            all_utils: Vec::new(),
            reminder: Reminder::new(),
            tracker: Window::new(
                WindowOptions {
                    auto_resize: true,
                    ..WindowOptions::new("Food Tracker##demo")
                },
                Tracker::new(),
            ),
        }
    }

    /// Returns the display name for a given food buff.
    fn food_name(defs: &Definitions, buff: BuffState<u32>) -> Cow<'static, str> {
        match buff {
            BuffState::Unknown => "Unset".into(),
            BuffState::None => "None".into(),
            BuffState::Some(buff) => {
                if let Some(DefinitionKind::Food(food)) = defs.get_definition(buff) {
                    food.name.clone().into()
                } else {
                    "Unknown".into()
                }
            }
        }
    }

    /// Returns the display name for a given utility buff.
    fn util_name(defs: &Definitions, buff: BuffState<u32>) -> Cow<'static, str> {
        match buff {
            BuffState::Unknown => "Unset".into(),
            BuffState::None => "None".into(),
            BuffState::Some(buff) => {
                if let Some(DefinitionKind::Util(util)) = defs.get_definition(buff) {
                    util.name.clone().into()
                } else {
                    "Unknown".into()
                }
            }
        }
    }
}
