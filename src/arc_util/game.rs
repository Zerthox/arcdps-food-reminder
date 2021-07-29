//! General GW2 related enums.
//!
//! These are also used in the ArcDPS API, but may be useful outside.

use num_enum::{FromPrimitive, IntoPrimitive, TryFromPrimitive};
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, IntoStaticStr};

/// GW2 client language.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    IntoPrimitive,
    TryFromPrimitive,
    Display,
    IntoStaticStr,
    Serialize,
    Deserialize,
)]
#[repr(u8)]
pub enum Language {
    English = 0,
    French = 2,
    German = 3,
    Spanish = 4,
    Chinese = 5,
}

/// Player profession.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    IntoPrimitive,
    FromPrimitive,
    Display,
    IntoStaticStr,
    Serialize,
    Deserialize,
)]
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

/// Player specializations.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    IntoPrimitive,
    FromPrimitive,
    Display,
    IntoStaticStr,
    Serialize,
    Deserialize,
)]
#[repr(u32)]
pub enum Specialization {
    /// Unknown or invalid.
    #[num_enum(default)]
    Unknown = 0,

    // mesmer
    Dueling = 1,
    Domination = 10,
    Inspiration = 23,
    Illusions = 24,
    Chronomancer = 40,
    Chaos = 45,
    Mirage = 59,

    // necromancer
    DeathMagic = 2,
    BloodMagic = 19,
    Reaper = 34,
    Curses = 39,
    SoulReaping = 50,
    Spite = 53,
    Scourge = 60,

    // revenant
    Invocation = 3,
    Retribution = 9,
    Corruption = 14,
    Devastation = 15,
    Salvation = 12,
    Herald = 52,
    Renegade = 63,

    // warrior
    Strength = 4,
    Tactics = 11,
    Berserker = 18,
    Defense = 22,
    Arms = 36,
    Discipline = 51,
    Spellbreaker = 61,

    // ranger
    Druid = 5,
    Marksmanship = 8,
    NatureMagic = 25,
    Skirmishing = 30,
    Beastmastery = 32,
    WildernessSurvival = 33,
    Soulbeast = 55,

    // engineer
    Explosives = 6,
    Tools = 21,
    Alchemy = 29,
    Firearms = 38,
    Scrapper = 43,
    Inventions = 47,
    Holosmith = 57,

    // thief
    Daredevil = 7,
    ShadowArts = 20,
    DeadlyArts = 28,
    CriticalStrikes = 35,
    Trickery = 44,
    Acrobatics = 54,
    Deadeye = 58,

    // guardian
    Valor = 13,
    Radiance = 16,
    Dragonhunter = 27,
    Zeal = 42,
    Virtues = 46,
    Honor = 49,
    Firebrand = 62,

    // elementalist
    Water = 17,
    Earth = 26,
    Fire = 31,
    Arcane = 37,
    Air = 41,
    Tempest = 48,
    Weaver = 56,
}

// TODO: document unclear attributes
/// Buff formula attributes.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    IntoPrimitive,
    FromPrimitive,
    Display,
    IntoStaticStr,
    Serialize,
    Deserialize,
)]
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

// entity enums generated from data
include!(concat!(env!("OUT_DIR"), "/boss.rs"));
include!(concat!(env!("OUT_DIR"), "/raidboss.rs"));
include!(concat!(env!("OUT_DIR"), "/fractalboss.rs"));

// buff enums generated from data
include!(concat!(env!("OUT_DIR"), "/buff.rs"));
include!(concat!(env!("OUT_DIR"), "/boon.rs"));
include!(concat!(env!("OUT_DIR"), "/food.rs"));
include!(concat!(env!("OUT_DIR"), "/util.rs"));
