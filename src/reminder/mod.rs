mod event;
pub mod settings;
mod ui;

use crate::data::Definitions;
use log::info;
use settings::ReminderSettings;
use std::{
    collections::BTreeMap,
    time::{Duration, Instant},
};

/// Reminder UI component.
#[derive(Debug)]
pub struct Reminder {
    /// Current reminder settings.
    pub settings: ReminderSettings,

    /// Timestamp of food reminder trigger.
    food_trigger: Option<Instant>,

    /// Timestamp of utility reminder trigger.
    util_trigger: Option<Instant>,

    /// Timestamps of custom buff reminder triggers.
    custom_triggers: BTreeMap<u32, Instant>,

    /// Current ongoing encounter.
    encounter: Option<Encounter>,
}

impl Reminder {
    /// Default duration used by the reminder.
    pub const DEFAULT_DURATION: Duration = Duration::from_secs(5);

    /// Creates a new reminder.
    pub const fn new() -> Self {
        Self {
            settings: ReminderSettings::new(),
            food_trigger: None,
            util_trigger: None,
            custom_triggers: BTreeMap::new(),
            encounter: None,
        }
    }

    /// Triggers all reminders.
    pub fn trigger_all(&mut self, defs: &Definitions) {
        self.trigger_food();
        self.trigger_util();
        for remind in defs.all_custom_reminder() {
            self.trigger_custom(remind.id);
        }
    }

    /// Triggers the food reminder.
    pub fn trigger_food(&mut self) {
        if self.settings.food {
            info!("Food reminder triggered");
            self.food_trigger = Some(Instant::now());
        }
    }

    /// Triggers the utility reminder.
    pub fn trigger_util(&mut self) {
        if self.settings.util {
            info!("Utility reminder triggered");
            self.util_trigger = Some(Instant::now());
        }
    }

    /// Triggers the custom buff reminder.
    pub fn trigger_custom(&mut self, id: u32) {
        if self.settings.custom {
            info!("Custom reminder triggered");
            self.custom_triggers.insert(id, Instant::now());
        }
    }
}

impl Default for Reminder {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
struct Encounter {
    /// Id of the encounter target.
    pub target_id: usize,

    /// Time the encounter started (or target changed).
    pub start_time: u64,

    /// Whether there is a pending check for the encounter.
    pub pending_check: bool,
}
