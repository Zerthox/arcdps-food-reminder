pub use crate::arc_util::game::{Food, Utility};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Buff<T> {
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
            | Self::SweetSpicyButternut
            | Self::TruffleSteak => "PWR",
            Self::CurryButternut => "PREC",
            Self::CilantroMeatFlatbread | Self::PepperedMeatFlatbread | Self::BeefRendang => "CND",
            Self::SalsaVeggieFlatbread
            | Self::PeppercornVeggieFlatbread
            | Self::VeggiePizza
            | Self::RedLentil
            | Self::FireMeatChili => "EXP",
            Self::SalsaEggsBenedict
            | Self::SesameEggsBenedict
            | Self::SoulPastry
            | Self::EggsBenedict => "CONC",
            Self::MintFruitSalad | Self::RiceBall => "HEAL",
            Self::Starcake => "ALL",
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
            Self::SharpeningSkull => "RES",
            Self::ScarletSlaying => "SLAY",
        }
    }
}
