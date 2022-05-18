use crate::{
    data::{BuffData, DefData},
    util::parse_jsonc,
};
use std::{fs, path::Path};

/// Malnourished buff id.
pub const MALNOURISHED: u32 = 46587;

/// Diminished buff id.
pub const DIMINISHED: u32 = 46668;

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
    data: Vec<DefEntry>,

    /// Boss ids.
    ///
    /// Sorted by id.
    bosses: Vec<u32>,
}

#[allow(dead_code)]
impl Definitions {
    /// Creates a new empty set of definitions.
    pub const fn empty() -> Self {
        Self {
            data: Vec::new(),
            bosses: Vec::new(),
        }
    }

    /// Creates a new set of definitions with the default definitions.
    pub fn with_defaults() -> Self {
        let mut defs = Self::empty();

        // add default defs data
        defs.add_data(default_definitions());

        defs
    }

    /// Updates an old buff entry or inserts iut as a new entry.
    fn update_or_insert(&mut self, new: DefEntry) {
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
            self.update_or_insert(DefEntry::new_food(entry));
        }
        for entry in util.into_iter() {
            self.update_or_insert(DefEntry::new_util(entry));
        }
        for id in ignored.into_iter() {
            self.update_or_insert(DefEntry::new(id, DefKind::Ignore));
        }

        // sort
        self.data.sort_by(|a, b| a.def.name().cmp(b.def.name()));
    }

    /// Adds new boss ids.
    pub fn add_bosses(&mut self, bosses: impl IntoIterator<Item = u32>) {
        // insert
        self.bosses.extend(bosses);

        // sort
        self.bosses.sort_unstable();
    }

    /// Add definitions from a [`DefData`] collection.
    pub fn add_data(&mut self, data: DefData) {
        let DefData {
            food,
            utility,
            ignore,
            bosses,
        } = data;
        self.add_buffs(food, utility, ignore);
        self.add_bosses(bosses);
    }

    /// Attempts to load custom definitions from a given file.
    pub fn try_load(&mut self, path: impl AsRef<Path>) -> Result<(), ()> {
        // read file
        let content = fs::read_to_string(path).map_err(|_| ())?;

        // parse & add data
        let data = parse_jsonc(&content).ok_or(())?;
        self.add_data(data);

        Ok(())
    }

    /// Returns the buff definition with the given id.
    pub fn get_buff(&self, buff_id: u32) -> Option<&DefKind> {
        self.data.iter().find_map(|entry| {
            if entry.id == buff_id {
                Some(&entry.def)
            } else {
                None
            }
        })
    }

    /// Checks whether the is is in the list of bosses.
    pub fn is_boss(&self, id: u32) -> bool {
        self.bosses.binary_search(&id).is_ok()
    }

    /// Returns all food definitions.
    pub fn all_food(&self) -> impl Iterator<Item = &BuffData> {
        self.data.iter().filter_map(|entry| match &entry.def {
            DefKind::Food(data) => Some(data),
            _ => None,
        })
    }

    /// Returns all utility definitions.
    pub fn all_util(&self) -> impl Iterator<Item = &BuffData> {
        self.data.iter().filter_map(|entry| match &entry.def {
            DefKind::Util(data) => Some(data),
            _ => None,
        })
    }
}

/// Buff definitions entry.
#[derive(Debug, Clone)]
pub struct DefEntry {
    pub id: u32,
    pub def: DefKind,
}

impl DefEntry {
    /// Creates a new definitions entry.
    pub const fn new(id: u32, def: DefKind) -> Self {
        Self { id, def }
    }

    /// Creates a new definitions entry for a food buff.
    pub const fn new_food(data: BuffData) -> Self {
        Self::new(data.id, DefKind::Food(data))
    }

    /// Creates a new definitions entry for an utility buff.
    pub const fn new_util(data: BuffData) -> Self {
        Self::new(data.id, DefKind::Util(data))
    }
}

/// Buff definitions kind.
#[derive(Debug, Clone)]
pub enum DefKind {
    /// Food buff.
    Food(BuffData),

    /// Utility buff.
    Util(BuffData),

    /// Ignored buff.
    Ignore,
}

impl DefKind {
    pub fn name(&self) -> &str {
        match self {
            DefKind::Food(data) => data.name.as_str(),
            DefKind::Util(data) => data.name.as_str(),
            DefKind::Ignore => "",
        }
    }
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
            bosses,
        } = default_definitions();

        assert!(!food.is_empty());
        assert!(!utility.is_empty());
        assert!(!ignore.is_empty());
        assert!(!bosses.is_empty());

        assert!(food.iter().any(|entry| entry.id == MALNOURISHED));
        assert!(utility.iter().any(|entry| entry.id == DIMINISHED));
    }
}
