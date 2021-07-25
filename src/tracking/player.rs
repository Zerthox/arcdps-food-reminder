pub use super::buff::{Buff, Food, Utility};
pub use crate::arc_util::game::{Profession, Specialization};
use std::cmp;

// TODO: track buff duration & reset to unset when duration runs out
// TODO: timestamps for buff apply to avoid out of order

/// Struct representing a player.
#[derive(Debug, Clone)]
pub struct Player {
    /// Player id given by the game.
    pub id: usize,

    /// Player character name.
    pub character: String,

    /// Player account name.
    pub account: String,

    /// Whether the player is the local player.
    pub is_self: bool,

    /// Profession (class) of the player character.
    pub profession: Profession,

    /// Current elite specialization the player has equipped.
    pub elite: Specialization,

    /// Current squad subgroup the player is in.
    pub subgroup: usize,

    /// Whether the player is currently in combat.
    pub combat: bool,

    /// Current food buff applied to the player.
    pub food: Buff<Food>,

    /// Current utility buff applied to the player.
    pub util: Buff<Utility>,
}

impl Player {
    /// Creates a new player.
    pub fn new<C, A>(
        id: usize,
        character: C,
        account: A,
        is_self: bool,
        profession: Profession,
        elite: Specialization,
        subgroup: usize,
    ) -> Self
    where
        C: Into<String>,
        A: Into<String>,
    {
        Self {
            id,
            character: character.into(),
            account: account.into(),
            is_self,
            profession,
            elite,
            subgroup,
            combat: false,
            food: Buff::Unset,
            util: Buff::Unset,
        }
    }

    /// Sets all unset buffs to none.
    pub fn unset_to_none(&mut self) {
        if self.food == Buff::Unset {
            self.food = Buff::None;
        }
        if self.util == Buff::Unset {
            self.util = Buff::None;
        }
    }

    /// Enters the player into combat.
    pub fn enter_combat(&mut self, new_subgroup: Option<usize>) {
        self.combat = true;
        if let Some(sub) = new_subgroup {
            self.subgroup = sub;
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

    /// Removes the food buff from the player.
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

    /// Removes the utility buff from the player.
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
