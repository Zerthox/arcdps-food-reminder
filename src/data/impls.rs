use super::*;
use crate::colors::{self, Color};

impl DefData {
    /// Returns the default definitions data.
    pub fn with_defaults() -> Self {
        include!(concat!(env!("OUT_DIR"), "/definitions.rs"))
    }

    /// Returns the total number of definition entries.
    pub fn len(&self) -> usize {
        self.food.len() + self.utility.len() + self.ignore.len()
    }

    /// Converts this into an iterator over all entries.
    pub fn into_entries(self) -> impl Iterator<Item = DefinitionEntry> {
        self.food
            .into_iter()
            .map(DefinitionEntry::new_food)
            .chain(self.utility.into_iter().map(DefinitionEntry::new_util))
            .chain(self.ignore.into_iter().map(DefinitionEntry::new_ignore))
    }
}

impl Rarity {
    /// Returns the color associated with the [`Rarity`].
    pub fn color(&self) -> Option<Color> {
        match self {
            Self::Basic => None,
            Self::Fine => Some(colors::FINE),
            Self::Masterwork => Some(colors::MASTERWORK),
            Self::Rare => Some(colors::RARE),
            Self::Exotic => Some(colors::EXOTIC),
            Self::Ascended => Some(colors::ASCENDED),
            Self::Legendary => Some(colors::LEGENDARY),
        }
    }
}
