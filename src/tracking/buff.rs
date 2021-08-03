pub use crate::arc_util::game::{Food, Utility};
use serde::{Deserialize, Serialize};

/// Struct representing a buff.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Buff<T> {
    /// Current state of the buff.
    pub state: BuffState<T>,

    /// ID of the last event update.
    pub event_id: u64,
}

impl<T> Buff<T> {
    /// Creates a new buff.
    pub fn new(state: BuffState<T>, event_id: u64) -> Self {
        Self { state, event_id }
    }

    /// Updates the buff state.
    ///
    /// Returns `false` if this update was ignored due to out of order.
    pub fn update(&mut self, state: BuffState<T>, event_id: u64) -> bool {
        if event_id > self.event_id {
            self.state = state;
            self.event_id = event_id;
            true
        } else {
            false
        }
    }
}

/// Possible buff states.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BuffState<T> {
    /// Buff state is not set (yet).
    ///
    /// This is the initial value.
    Unset,

    /// No buff is applied.
    None,

    /// Some buff is applied but not recognized.
    Unknown,

    /// Some buff is applied and recognized.
    Known(T),
}

/// Helper trait to categorize buffs.
pub trait Categorize {
    /// Returns the buff category as a string.
    fn categorize(&self) -> &'static str;
}

impl Categorize for Food {
    fn categorize(&self) -> &'static str {
        match self {
            Self::Malnourished => "NONE",

            Self::CilantroSousVideSteak
            | Self::PeppercornSousVideSteak
            | Self::CloveSousVideSteak
            | Self::SesameSousVideSteak
            | Self::MintSousVideSteak
            | Self::SweetSpicyButternut
            | Self::SpicyMoaWings => "PWR",

            Self::PeppercornCoqAuVin
            | Self::SesameCoqAuVin
            | Self::MintCoqAuVin
            | Self::CurryButternut
            | Self::TruffleSteak => "PREC",

            Self::CilantroMeatFlatbread
            | Self::SesameMeatFlatbread
            | Self::PepperedMeatFlatbread
            | Self::BeefRendang => "CND",

            Self::SalsaVeggieFlatbread
            | Self::PeppercornVeggieFlatbread
            | Self::SesameVeggieFlatbread
            | Self::MintVeggieFlatbread
            | Self::VeggiePizza
            | Self::RedLentil
            | Self::FireMeatChili => "EXP",

            Self::PeppercornBeefCarpaccio
            | Self::SesameBeefCarpaccio
            | Self::SalsaEggsBenedict
            | Self::SesameEggsBenedict
            | Self::SoulPastry
            | Self::EggsBenedict => "CONC",

            Self::SpicedFruitSalad | Self::MintFruitSalad | Self::RiceBall => "HEAL",

            Self::PeppercornOysterSoup
            | Self::CloveOysterSoup
            | Self::SesameOysterSoup
            | Self::MintOysterSoup
            | Self::Starcake => "ALL",
        }
    }
}

impl Categorize for Utility {
    fn categorize(&self) -> &'static str {
        match self {
            Self::Diminished => "NONE",
            Self::SuperiorStone | Self::Fruitcake | Self::FuriousStone => "PWR",
            Self::ToxicCrystal | Self::MasterCrystal | Self::TuningIcicle => "CND",
            Self::PotentOil | Self::EnhancedOil | Self::ToxicOil | Self::PeppermintOil => "CONC",
            Self::BountifulOil => "HEAL",
            Self::StrengthWrit | Self::AccuracyWrit | Self::MaliceWrit => "WRIT",
            Self::PumpkinOil | Self::CrystallizedNougat | Self::SharpeningSkull => "RES",
            Self::ScarletSlaying => "SLAY",
        }
    }
}
