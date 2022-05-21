pub mod build;
pub mod ui;

use arc_util::settings::HasSettings;
use serde::{Deserialize, Serialize};

pub use build::Build;

/// Component for user-defined builds.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Builds {
    /// User-defined builds
    pub entries: Vec<Build>,

    /// Whether to display tags as table column.
    pub display_tags: bool,

    /// Whether to filter builds by profession.
    pub filter_prof: bool,

    /// Current search contents.
    pub search: String,

    /// Edit mode.
    #[serde(skip)]
    edit: bool,
}

impl Builds {
    pub const fn new() -> Self {
        Self {
            entries: Vec::new(),
            display_tags: true,
            filter_prof: false,
            search: String::new(),
            edit: false,
        }
    }
}

impl Default for Builds {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
enum Action {
    None,
    Remove(usize),
    Up(usize),
    Down(usize),
}

impl HasSettings for Builds {
    type Settings = Builds;

    const SETTINGS_ID: &'static str = "builds";

    fn current_settings(&self) -> Self::Settings {
        self.clone()
    }

    fn load_settings(&mut self, loaded: Self::Settings) {
        self.entries = loaded.entries;
        self.filter_prof = loaded.filter_prof;
    }
}
