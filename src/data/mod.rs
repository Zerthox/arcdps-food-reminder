mod constants;
mod structs;

use crate::util::parse_jsonc;
use std::{fs, io, path::Path};

pub use constants::*;
pub use structs::*;

/// Returns the default definitions.
fn default_definitions() -> DefData {
    include!(concat!(env!("OUT_DIR"), "/definitions.rs"))
}

/// Shared buff definitions data.
#[derive(Debug)]
pub struct Definitions {
    /// Buff definitions data.
    ///
    /// Sorted alphabetically for UI usage.
    data: Vec<DefinitionEntry>,
}

impl Definitions {
    /// Creates a new empty set of definitions.
    pub const fn empty() -> Self {
        Self { data: Vec::new() }
    }

    /// Creates a new set of definitions with the default definitions.
    pub fn with_defaults() -> Self {
        let mut defs = Self::empty();

        // add default defs data
        defs.add_data(default_definitions());

        defs
    }

    /// Updates an old buff entry or inserts iut as a new entry.
    fn update_or_insert(&mut self, new: DefinitionEntry) {
        if let Some(old) = self.data.iter_mut().find(|entry| entry.id == new.id) {
            *old = new;
        } else {
            self.data.push(new);
        }
    }

    /// Adds new buff definitions.
    pub fn add_buffs(
        &mut self,
        food: impl IntoIterator<Item = BuffData>,
        util: impl IntoIterator<Item = BuffData>,
        ignored: impl IntoIterator<Item = u32>,
    ) {
        // insert
        for entry in food.into_iter() {
            self.update_or_insert(DefinitionEntry::new_food(entry));
        }
        for entry in util.into_iter() {
            self.update_or_insert(DefinitionEntry::new_util(entry));
        }
        for id in ignored.into_iter() {
            self.update_or_insert(DefinitionEntry::new(id, DefinitionKind::Ignore));
        }

        // sort
        self.data.sort_by(|a, b| a.def.name().cmp(b.def.name()));
    }

    /// Add definitions from a [`DefData`] collection.
    pub fn add_data(&mut self, data: DefData) {
        self.add_buffs(data.food, data.utility, data.ignore);
    }

    /// Attempts to load custom definitions from a given file.
    pub fn try_load(&mut self, path: impl AsRef<Path>) -> Result<(), LoadError> {
        // read file
        let content = fs::read_to_string(path).map_err(|err| match err.kind() {
            io::ErrorKind::NotFound => LoadError::NotFound,
            _ => LoadError::FailedToRead,
        })?;

        // parse & add data
        let data = parse_jsonc(&content).ok_or(LoadError::InvalidJSON)?;
        self.add_data(data);

        Ok(())
    }

    /// Returns the kind for the given buff id & name.
    pub fn get_buff(&self, id: u32, name: Option<&str>) -> BuffKind {
        if id == REINFORCED {
            BuffKind::Reinforced
        } else if let Some(def) = self.get_definition(id) {
            match def {
                DefinitionKind::Food(data) => BuffKind::Food(Some(data)),
                DefinitionKind::Util(data) => BuffKind::Util(Some(data)),
                DefinitionKind::Ignore => BuffKind::Ignore,
            }
        } else {
            match name {
                Some("Nourishment") => BuffKind::Food(None),
                Some("Enhancement") => BuffKind::Util(None),
                _ => BuffKind::Unknown,
            }
        }
    }

    /// Returns the definition for the buff with the given id.
    pub fn get_definition(&self, buff_id: u32) -> Option<&DefinitionKind> {
        self.data.iter().find_map(|entry| {
            if entry.id == buff_id {
                Some(&entry.def)
            } else {
                None
            }
        })
    }

    /// Returns all food definitions.
    pub fn all_food(&self) -> impl Iterator<Item = &BuffData> + Clone {
        self.data.iter().filter_map(|entry| match &entry.def {
            DefinitionKind::Food(data) => Some(data),
            _ => None,
        })
    }

    /// Returns all utility definitions.
    pub fn all_util(&self) -> impl Iterator<Item = &BuffData> + Clone {
        self.data.iter().filter_map(|entry| match &entry.def {
            DefinitionKind::Util(data) => Some(data),
            _ => None,
        })
    }
}

/// Buff definitions entry.
#[derive(Debug, Clone)]
pub struct DefinitionEntry {
    pub id: u32,
    pub def: DefinitionKind,
}

impl DefinitionEntry {
    /// Creates a new definitions entry.
    pub const fn new(id: u32, def: DefinitionKind) -> Self {
        Self { id, def }
    }

    /// Creates a new definitions entry for a food buff.
    pub const fn new_food(data: BuffData) -> Self {
        Self::new(data.id, DefinitionKind::Food(data))
    }

    /// Creates a new definitions entry for an utility buff.
    pub const fn new_util(data: BuffData) -> Self {
        Self::new(data.id, DefinitionKind::Util(data))
    }
}

/// Buff definition kind.
#[derive(Debug, Clone)]
pub enum DefinitionKind {
    Food(BuffData),
    Util(BuffData),
    Ignore,
}

impl DefinitionKind {
    pub fn name(&self) -> &str {
        match self {
            DefinitionKind::Food(data) => data.name.as_str(),
            DefinitionKind::Util(data) => data.name.as_str(),
            DefinitionKind::Ignore => "",
        }
    }
}

/// Buff kind.
#[derive(Debug, Clone)]
pub enum BuffKind<'a> {
    Unknown,
    Reinforced,
    Food(Option<&'a BuffData>),
    Util(Option<&'a BuffData>),
    Ignore,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LoadError {
    NotFound,
    FailedToRead,
    InvalidJSON,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn definitions() {
        let DefData {
            food,
            utility,
            ignore,
        } = default_definitions();

        assert!(!food.is_empty());
        assert!(!utility.is_empty());
        assert!(!ignore.is_empty());

        assert!(food.iter().any(|entry| entry.id == MALNOURISHED));
        assert!(utility.iter().any(|entry| entry.id == DIMINISHED));
    }
}
