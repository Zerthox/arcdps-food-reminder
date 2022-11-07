pub mod build;
pub mod ui;

use arc_util::settings::HasSettings;
use serde::{Deserialize, Serialize};

pub use build::Build;

/// Component for user-defined builds.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Builds {
    /// User-defined builds.
    pub entries: Vec<Build>,

    /// Whether to display notes as table column.
    pub display_notes: bool,

    /// Whether to filter builds by profession.
    pub filter_prof: bool,

    /// Current search contents.
    #[serde(skip)]
    search: String,

    /// Edit mode.
    #[serde(skip)]
    edit: bool,
}

impl Builds {
    pub const fn new() -> Self {
        Self {
            entries: Vec::new(),
            display_notes: true,
            filter_prof: false,
            search: String::new(),
            edit: false,
        }
    }

    /// Refreshes build visibility according to search.
    fn refresh_search(&mut self) {
        for build in &mut self.entries {
            build.visible = self.search.is_empty()
                || build.name.to_lowercase().contains(&self.search)
                || build.notes.to_lowercase().contains(&self.search);
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
        self.display_notes = loaded.display_notes;
        self.filter_prof = loaded.filter_prof;
    }
}
