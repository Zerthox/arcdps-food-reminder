pub use crate::arc_util::game::{Food, Utility};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
            Self::CilantroSousVideSteak | Self::SweetSpicyButternut | Self::TruffleSteak => "PWR",
            Self::CurryButternut => "PREC",
            Self::VeggiePizza | Self::RedLentil => "EXP",
            Self::RiceBall => "HEAL",
        }
    }
}

impl Categorize for Utility {
    fn categorize(&self) -> &'static str {
        match self {
            Self::Diminished => "NONE",
            Self::SharpeningStone => "PWR",
            Self::ToxicCrystal | Self::TuningIcicle => "CND",
            Self::BountifulOil => "HEAL",
        }
    }
}
