use arc_util::game::Profession;
use serde::{Deserialize, Serialize};

/// Build data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Build {
    pub prof: Profession,
    pub name: String,
    pub food: u32,
    pub util: u32,
}

impl Build {
    /// Creates a new build.
    pub fn new(prof: Profession, name: impl Into<String>, food: u32, util: u32) -> Self {
        Self {
            prof,
            name: name.into(),
            food,
            util,
        }
    }
}
