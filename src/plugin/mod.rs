pub mod event;
pub mod ui;

use crate::{
    data::{Definitions, LoadError, DIMINISHED, MALNOURISHED},
    reminder::Reminder,
    tracking::{buff::BuffState, Tracker},
};
use arc_util::{
    settings::Settings,
    ui::{Window, WindowOptions},
};
use semver::Version;
use std::fs;

#[cfg(feature = "demo")]
use crate::demo::Demo;

#[cfg(feature = "log")]
use arc_util::ui::log::Log;

/// Plugin version.
const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Settings file name.
const SETTINGS_FILE: &str = "arcdps_food_reminder.json";

/// Definitions file name.
const DEFINITIONS_FILE: &str = "arcdps_food_reminder_definitions.json";

/// Main plugin.
#[derive(Debug)]
pub struct Plugin {
    /// State of unofficial extras.
    extras: ExtrasState,

    /// Definitions.
    defs: Definitions,

    /// State of loading custom definitions.
    defs_state: Result<(), LoadError>,

    /// Food reminder.
    reminder: Reminder,

    /// Whether there is a pending food check for the current encounter.
    ///
    /// Stores the timestamp of last relevant event.
    pending_check: Option<u64>,

    /// Food tracker window.
    tracker: Window<Tracker>,

    /// Demo window.
    #[cfg(feature = "demo")]
    demo: Window<Demo>,

    /// Debug log window.
    #[cfg(feature = "log")]
    debug: Window<Log>,

    /// Confirmation for reset.
    reset_confirm: bool,
}

impl Plugin {
    /// Creates a new plugin.
    pub fn new() -> Self {
        Self {
            extras: ExtrasState::Missing,
            defs: Definitions::with_defaults(),
            defs_state: Err(LoadError::NotFound),
            reminder: Reminder::new(),
            pending_check: None,

            tracker: Window::new(
                WindowOptions {
                    auto_resize: true,
                    ..WindowOptions::new("Food Tracker")
                },
                Tracker::new(),
            ),

            #[cfg(feature = "demo")]
            demo: Window::new(
                WindowOptions {
                    auto_resize: true,
                    ..WindowOptions::new("Food Demo")
                },
                Demo::new(),
            ),

            #[cfg(feature = "log")]
            debug: Window::new(
                WindowOptions {
                    width: 600.0,
                    height: 300.0,
                    ..WindowOptions::new("Food Debug Log")
                },
                Log::new(),
            ),

            reset_confirm: false,
        }
    }

    /// Loads the plugin.
    pub fn load(&mut self) {
        #[cfg(feature = "log")]
        self.debug.log(format!("Food Reminder v{} load", VERSION));

        // load settings
        let mut settings = Settings::from_file(SETTINGS_FILE);
        let settings_version: Option<Version> = settings.load_data("version");

        #[cfg(feature = "log")]
        self.debug.log(format!(
            "Loaded settings from version {}",
            match &settings_version {
                Some(version) => version.to_string(),
                None => "unknown".into(),
            }
        ));

        // load component settings
        settings.load_component(&mut self.tracker);
        settings.load_component(&mut self.reminder);

        #[cfg(feature = "demo")]
        {
            settings.load_component(&mut self.demo);
            self.refresh_demo_settings();
        }

        // load custom defs
        if let Some(defs_path) = Settings::config_path(DEFINITIONS_FILE) {
            const DEFAULTS_CHANGE: Version = Version::new(0, 4, 0);

            // check for minimum version
            if matches!(settings_version, Some(version) if version >= DEFAULTS_CHANGE) {
                if defs_path.exists() {
                    // try loading custom defs
                    self.defs_state = self.defs.try_load(&defs_path);

                    if self.defs_state.is_ok() {
                        #[cfg(feature = "log")]
                        self.debug.log(format!(
                            "Loaded custom definitions from \"{}\"",
                            defs_path.display()
                        ));
                    } else {
                        #[cfg(feature = "log")]
                        self.debug.log(format!(
                            "Failed to load custom definitions from \"{}\"",
                            defs_path.display()
                        ));
                    }
                }
            } else {
                // settings are from old version, remove old defs file
                let _ = fs::remove_file(defs_path);

                #[cfg(feature = "log")]
                self.debug.log("Removed definitions from old version");
            }
        }
    }

    /// Unloads the plugin.
    pub fn unload(&mut self) {
        let mut settings = Settings::from_file(SETTINGS_FILE);

        settings.store_data("version", VERSION);

        // update component settings
        settings.store_component(&self.tracker);
        settings.store_component(&self.reminder);

        #[cfg(feature = "demo")]
        settings.store_component(&self.demo);

        // save settings
        settings.save_file();
    }

    /// Whether the local player can be reminded.
    fn can_remind(&self) -> bool {
        match self.tracker.encounter() {
            Some(boss_id) if self.reminder.settings.only_bosses => boss_id > 1,
            Some(_) => true,
            None => false,
        }
    }

    /// Checks for missing food on the local player.
    fn check_self_food(&mut self) {
        if let Some(entry) = self.tracker.get_self() {
            if self.can_remind() {
                let food = entry.food.state;

                #[cfg(feature = "log")]
                self.debug.log(format!("Checking food on self: {:?}", food));

                if let BuffState::None | BuffState::Some(MALNOURISHED) = food {
                    self.reminder.trigger_food();

                    #[cfg(feature = "log")]
                    self.debug.log("Food reminder triggered");
                }
            }
        }
    }

    /// Checks for missing utility on the local player.
    fn check_self_util(&mut self) {
        if let Some(entry) = self.tracker.get_self() {
            if self.can_remind() {
                let util = entry.util.state;

                #[cfg(feature = "log")]
                self.debug
                    .log(format!("Checking utility on self: {:?}", util));

                if let BuffState::None | BuffState::Some(DIMINISHED) = util {
                    self.reminder.trigger_util();

                    #[cfg(feature = "log")]
                    self.debug.log("Utility reminder triggered");
                }
            }
        }
    }

    fn check_self_reinforced(&mut self) {
        if let Some(entry) = self.tracker.get_self() {
            if self.can_remind() {
                let reinf = entry.reinforced.state;

                #[cfg(feature = "log")]
                self.debug
                    .log(format!("Checking reinforced on self: {:?}", reinf));

                if !reinf {
                    self.reminder.trigger_reinforced();

                    #[cfg(feature = "log")]
                    self.debug.log("Reinforced reminder triggered");
                }
            }
        }
    }

    /// Propagates settings from reminder & tracker to demo versions.
    #[cfg(feature = "demo")]
    fn refresh_demo_settings(&mut self) {
        let demo = &mut *self.demo;
        demo.reminder.settings = self.reminder.settings.clone();
        demo.tracker.settings = self.tracker.settings.clone();
    }
}

impl Default for Plugin {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ExtrasState {
    Missing,
    Incompatible,
    Found,
}
