pub mod custom;
pub mod event;
pub mod settings;
pub mod ui;

use self::custom::{CustomReminder, GameMode};
use self::settings::ReminderSettings;
use gw2_mumble::MumbleLink;
use log::{error, info};
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

    /// Returns the custom reminder for the buff with the given id.
    pub fn custom(&self, buff_id: u32) -> Option<&CustomReminder> {
        self.settings
            .custom
            .iter()
            .find(|entry| entry.id == buff_id)
    }

    /// Returns all custom reminders.
    pub fn all_custom(&self) -> &[CustomReminder] {
        &self.settings.custom
    }

    /// Triggers all reminders.
    pub fn trigger_all(&mut self) {
        self.trigger_food();
        self.trigger_util();
        let ids = self
            .all_custom()
            .iter()
            .map(|remind| remind.id)
            .collect::<Vec<_>>();
        for id in ids {
            self.trigger_custom(id);
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
        // TODO: setting for each custom reminder
        if let Some(remind) = self.custom(id) {
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
