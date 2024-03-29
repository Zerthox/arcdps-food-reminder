use crate::data::{DIMINISHED, MALNOURISHED};
use arcdps::Profession;
use serde::{Deserialize, Serialize};

/// Build data.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Build {
    pub prof: Profession,
    pub name: String,
    pub notes: String,
    pub food: u32,
    pub util: u32,

    #[serde(skip)]
    pub visible: bool,
}

impl Build {
    /// Creates a new build.
    pub fn new(
        prof: Profession,
        name: impl Into<String>,
        notes: impl Into<String>,
        food: u32,
        util: u32,
    ) -> Self {
        Self {
            prof,
            name: name.into(),
            notes: notes.into(),
            food,
            util,
            visible: true,
        }
    }

    /// Creates a new empty build.
    pub fn empty() -> Self {
        Self::new(Profession::Unknown, "", "", MALNOURISHED, DIMINISHED)
    }
}

impl Default for Build {
    fn default() -> Self {
        Self::empty()
    }
}
