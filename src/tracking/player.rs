pub use crate::arc_util::game::Profession;

/// Struct representing a player.
#[derive(Debug, Clone)]
pub struct Player {
    pub id: usize,
    pub character_name: String,
    pub account_name: String,
    pub profession: Profession,
    pub subgroup: u16,
    pub combat: bool,
}

impl Player {
    /// Creates a new player.
    pub fn new<N, A>(
        id: usize,
        name: N,
        account_name: A,
        profession: Profession,
        subgroup: u16,
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
            subgroup,
            combat: false,
        }
    }

    /// Enters the player into combat.
    pub fn enter_combat(&mut self) {
        self.combat = true;
    }

    /// Exits the player from combat.
    pub fn exit_combat(&mut self) {
        self.combat = false;
    }
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Player {}
