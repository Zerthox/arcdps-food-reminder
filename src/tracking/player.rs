pub use super::buff::{Buff, BuffState, Food, Utility};
pub use crate::arc_util::game::{Profession, Specialization};
use serde::{Deserialize, Serialize};
use std::cmp;

// TODO: track buff duration & reset to unset when duration runs out?

/// Struct representing a player.
#[derive(Debug, Clone, Serialize, Deserialize)]
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
            food: Buff::new(BuffState::Unset, 0),
            util: Buff::new(BuffState::Unset, 0),
        }
    }

    /// Sets all unset buffs to none.
    pub fn unset_to_none(&mut self, time: u64) {
        if self.food.state == BuffState::Unset {
            self.food.update(BuffState::None, time);
        }
        if self.util.state == BuffState::Unset {
            self.util.update(BuffState::None, time);
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
    ///
    /// Returns `true` if this update changed the buff state.
    pub fn apply_food(&mut self, food: Food, event_id: u64) -> bool {
        self.food.update(BuffState::Known(food), event_id)
    }

    /// Applies a unknown food buff to the player.
    ///
    /// Returns `false` if this update was ignored.
    pub fn apply_unknown_food(&mut self, id: u32, event_id: u64) -> bool {
        self.food.update(BuffState::Unknown(id), event_id)
    }

    /// Removes the current food buff from the player.
    ///
    /// Has no effect if the current buff is different from the passed buff.
    /// Passing [`None`] indicates a [`BuffState::Unknown`].
    /// [`BuffState::Unset`] is always removed.
    ///
    /// Returns `false` if this update was ignored.
    pub fn remove_food(&mut self, food: Option<Food>, event_id: u64) -> bool {
        let changed = match (food, self.food.state) {
            (_, BuffState::Unset) | (None, BuffState::Unknown(_)) => true,
            (Some(removed), BuffState::Known(applied)) => removed == applied,
            _ => false,
        };
        if changed {
            self.food.update(BuffState::None, event_id)
        } else {
            false
        }
    }

    /// Applies an utility buff to the player.
    ///
    /// Returns `false` if this update was ignored.
    pub fn apply_util(&mut self, util: Utility, event_id: u64) -> bool {
        self.util.update(BuffState::Known(util), event_id)
    }

    /// Applies an unknown utility buff to the player.
    ///
    /// Returns `false` if this update was ignored.
    pub fn apply_unknown_util(&mut self, id: u32, event_id: u64) -> bool {
        self.util.update(BuffState::Unknown(id), event_id)
    }

    /// Removes the current utility buff from the player.
    ///
    /// Has no effect if the current buff is different from the passed buff.
    /// Passing [`None`] indicates a [`BuffState::Unknown`].
    /// [`BuffState::Unset`] is always removed.
    ///
    /// Returns `false` if this update was ignored.
    pub fn remove_util(&mut self, util: Option<Utility>, event_id: u64) -> bool {
        let changed = match (util, self.util.state) {
            (_, BuffState::Unset) | (None, BuffState::Unknown(_)) => true,
            (Some(removed), BuffState::Known(applied)) => removed == applied,
            _ => false,
        };
        if changed {
            self.util.update(BuffState::None, event_id)
        } else {
            false
        }
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
