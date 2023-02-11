mod event;
pub mod settings;
mod ui;

use crate::data::{CustomReminder, Definitions, GameMode};
use gw2_mumble::MumbleLink;
use log::{error, info};
use settings::ReminderSettings;
use std::{
    collections::BTreeMap,
    time::{Duration, Instant},
};

/// Reminder UI component.
#[derive(Debug)]
pub struct Reminder {
    /// Mumble link.
    mumble: Option<MumbleLink>,

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
    pub fn new() -> Self {
        Self {
            mumble: match MumbleLink::new() {
                Ok(link) => Some(link),
                Err(err) => {
                    error!("failed to grab mumblelink: {err}");
                    None
                }
            },
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
            self.trigger_custom(remind);
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
    pub fn trigger_custom(&mut self, remind: &CustomReminder) {
        // TODO: setting for each custom reminder
        if self.settings.custom {
            let applies = if let Some(mumble) = &self.mumble {
                let link = mumble.read();
                remind.mode.is_map(link.context.map_id)
            } else {
                // no mumble, only apply all game modes
                remind.mode == GameMode::All
            };

            if applies {
                info!("Custom reminder triggered");
                self.custom_triggers.insert(remind.id, Instant::now());
            }
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
