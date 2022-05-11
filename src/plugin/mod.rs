pub mod event;
pub mod ui;

use crate::{
    defs::{Definitions, DIMINISHED, MALNOURISHED},
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
use arc_util::ui::components::log::Log;

/// Plugin version.
const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Settings file name.
const SETTINGS_FILE: &str = "arcdps_food_reminder.json";

/// Definitions file name.
const DEFINITIONS_FILE: &str = "arcdps_food_reminder_definitions.json";

/// Main plugin.
#[derive(Debug)]
pub struct Plugin {
    extras: bool,

    /// Definitions.
    defs: Definitions,

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
}

impl Plugin {
    /// Creates a new plugin.
    pub fn new() -> Self {
        Self {
            extras: false,
            defs: Definitions::with_defaults(),
            reminder: Reminder::new(),
            pending_check: None,

            tracker: Window::with_options(
                "Food Tracker",
                Tracker::new(),
                WindowOptions {
                    auto_resize: true,
                    context_menu: true,
                    ..WindowOptions::new()
                },
            ),

            #[cfg(feature = "demo")]
            demo: Window::with_options(
                "Food Demo",
                Demo::new(),
                WindowOptions {
                    auto_resize: true,
                    ..WindowOptions::new()
                },
            ),

            #[cfg(feature = "log")]
            debug: Window::with_options(
                "Food Debug Log",
                Log::new(),
                WindowOptions {
                    width: 600.0,
                    height: 300.0,
                    ..WindowOptions::new()
                },
            ),
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
        settings.load_component(&mut self.demo);

        // load custom defs
        if let Some(defs_path) = Settings::config_path(DEFINITIONS_FILE) {
            const DEFAULTS_CHANGE: Version = Version::new(0, 4, 0);

            // check for minimum version
            if matches!(settings_version, Some(version) if version >= DEFAULTS_CHANGE) {
                if defs_path.exists() {
                    // try loading custom defs
                    if self.defs.try_load(&defs_path).is_ok() {
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
            Some(boss_id) => {
                !self.reminder.settings.only_bosses || self.defs.is_boss(boss_id as u32)
            }
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
}

impl Default for Plugin {
    fn default() -> Self {
        Self::new()
    }
}
