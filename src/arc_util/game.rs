//! General GW2 related enums.
//!
//! These are also used in the ArcDPS API, but may be useful outside.

use num_enum::{FromPrimitive, TryFromPrimitive};
use strum_macros::{AsRefStr, Display};

/// GW2 client language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive, Display, AsRefStr)]
#[repr(u8)]
pub enum Language {
    English = 0,
    French = 2,
    German = 3,
    Spanish = 4,
    Chinese = 5,
}

/// Player profession.
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, Display, AsRefStr)]
#[repr(u32)]
pub enum Profession {
    /// Unknown or invalid.
    #[num_enum(default)]
    Unknown = 0,

    Guardian = 1,
    Warrior = 2,
    Engineer = 3,
    Ranger = 4,
    Thief = 5,
    Elementalist = 6,
    Mesmer = 7,
    Necromancer = 8,
    Revenant = 9,
}

// TODO: document unclear attributes
/// Buff formula attributes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive)]
#[repr(u16)]
pub enum Attribute {
    None,

    Power,
    Precision,
    Toughness,
    Vitality,
    Ferocity,
    Healing,
    Condition,
    Concentration,
    Expertise,

    Armor,
    Agony,
    StatInc,
    FlatInc,
    PhysInc,
    CondInc,
    PhysRec,
    CondRec,
    Attackspeed,
    SiphonInc,
    SiphonRec,

    /// Unknown or invalid.
    #[num_enum(default)]
    Unknown = 65535,
}

// TODO: add non-boon buffs
/// Buff.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive, Display, AsRefStr)]
#[repr(u32)]
pub enum Buff {
    Might = 740,
    Fury = 725,
    Quickness = 1187,
    Alacrity = 30328,
    Resolution = 873,
    Protection = 717,
    Regeneration = 718,
    Swiftness = 719,
    Vigor = 726,
    Stability = 1122,
    Resistance = 26980,
    Aegis = 743,
}
