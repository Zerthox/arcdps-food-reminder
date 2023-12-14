pub mod event;
pub mod ui;

use crate::{
    data::{Definitions, LoadError},
    reminder::Reminder,
    tracking::Tracker,
};
use arc_util::{
    settings::Settings,
    ui::{Window, WindowOptions},
};
use log::{info, warn};
use once_cell::sync::Lazy;
use semver::Version;
use std::sync::Mutex;
use std::{fs, sync::MutexGuard};

#[cfg(feature = "demo")]
use crate::demo::Demo;

/// Plugin version.
const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Settings file name.
const SETTINGS_FILE: &str = "arcdps_food_reminder.json";

/// Definitions file name.
const DEFINITIONS_FILE: &str = "arcdps_food_reminder_definitions.json";

/// Main plugin instance.
// FIXME: a single mutex for the whole thing is potentially inefficient
static PLUGIN: Lazy<Mutex<Plugin>> = Lazy::new(|| Mutex::new(Plugin::new()));

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

    /// Food tracker window.
    tracker: Window<Tracker>,

    /// Demo window.
    #[cfg(feature = "demo")]
    demo: Window<Demo>,

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

            tracker: Window::new(
                "Food Tracker",
                Tracker::new(),
                WindowOptions {
                    auto_resize: true,
                    ..Default::default()
                },
            ),

            #[cfg(feature = "demo")]
            demo: Window::new(
                "Food Demo",
                Demo::new(),
                WindowOptions {
                    auto_resize: true,
                    ..Default::default()
                },
            ),

            reset_confirm: false,
        }
    }

    /// Acquires access to the plugin instance.
    pub fn lock() -> MutexGuard<'static, Self> {
        PLUGIN.lock().unwrap()
    }

    /// Helper to convert [`MutexGuard`] to a mutable [`Plugin`] reference.
    pub fn as_mut(&mut self) -> &mut Self {
        self
    }

    /// Loads the plugin.
    pub fn load(&mut self) {
        // TODO: update notification
        info!("v{} load", VERSION);

        let d3d_version = arcdps::d3d_version();
        if d3d_version != 11 {
            warn!("directx version {d3d_version}");
        }

        // load settings
        let mut settings = Settings::from_file(SETTINGS_FILE);
        let settings_version: Option<Version> = settings.load_data("version");

        info!(
            "Loaded settings from version {}",
            match &settings_version {
                Some(version) => version.to_string(),
                None => "unknown".into(),
            }
        );

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
                        info!("Loaded custom definitions from \"{}\"", defs_path.display());
                    } else {
                        warn!(
                            "Failed to load custom definitions from \"{}\"",
                            defs_path.display()
                        );
                    }
                }
            } else {
                // settings are from old version, remove old defs file
                let _ = fs::remove_file(defs_path);

                info!("Removed definitions from old version");
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
