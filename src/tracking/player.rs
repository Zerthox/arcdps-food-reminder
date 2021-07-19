pub use super::buff::{Buff, Food, Utility};
pub use crate::arc_util::game::{Profession, Specialization};
use std::cmp;

/// Struct representing a player.
#[derive(Debug, Clone)]
pub struct Player {
    pub id: usize,
    pub character_name: String,
    pub account_name: String,
    pub profession: Profession,
    pub elite: Specialization,
    pub subgroup: usize,
    pub combat: bool,
    pub food: Buff<Food>,
    pub util: Buff<Utility>,
}

impl Player {
    /// Creates a new player.
    pub fn new<N, A>(
        id: usize,
        name: N,
        account_name: A,
        profession: Profession,
        elite: Specialization,
        subgroup: usize,
    ) -> Self
    where
        N: Into<String>,
        A: Into<String>,
    {
        Self {
            id,
            character_name: name.into(),
            account_name: account_name.into(),
            profession,
            elite,
            subgroup,
            combat: false,
            food: Buff::Unset, // TODO: track duration & reset to unset when duration runs out
            util: Buff::Unset,
        }
    }

    /// Enters the player into combat.
    pub fn enter_combat(&mut self, new_subgroup: Option<usize>) {
        self.combat = true;
        if let Some(sub) = new_subgroup {
            self.subgroup = sub;
        }

        // change unset buffs to none
        // if there is initial buffs, they will be set after
        if self.food == Buff::Unset {
            self.food = Buff::None;
        }
        if self.util == Buff::Unset {
            self.food = Buff::None;
        }
    }

    /// Exits the player from combat.
    pub fn exit_combat(&mut self) {
        self.combat = false;
    }

    /// Applies a food buff to the player.
    pub fn apply_food(&mut self, food: Food) {
        self.food = Buff::Known(food);
    }

    /// Applies a unknown food buff to the player.
    pub fn apply_unknown_food(&mut self) {
        self.food = Buff::Unknown;
    }

    /// Removes the current food buff to the player.
    pub fn remove_food(&mut self) {
        self.food = Buff::None;
    }

    /// Applies an utility buff to the player.
    pub fn apply_util(&mut self, util: Utility) {
        self.util = Buff::Known(util);
    }

    /// Applies an unknown utility buff to the player.
    pub fn apply_unknown_util(&mut self) {
        self.util = Buff::Unknown;
    }

    /// Removes the current utility buff to the player.
    pub fn remove_util(&mut self) {
        self.util = Buff::None;
    }
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Player {}

impl cmp::PartialOrd for Player {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl cmp::Ord for Player {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.id.cmp(&other.id)
    }
}
