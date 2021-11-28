use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::{Deserialize, Serialize};
use std::convert::TryInto;
use strum_macros::{Display, EnumIter, IntoStaticStr};

// entity enums generated from data
include!(concat!(env!("OUT_DIR"), "/boss.rs"));
include!(concat!(env!("OUT_DIR"), "/raidboss.rs"));
include!(concat!(env!("OUT_DIR"), "/fractalboss.rs"));

impl From<RaidBoss> for Boss {
    fn from(raid_boss: RaidBoss) -> Self {
        // raid boss is always a valid boss
        usize::from(raid_boss).try_into().unwrap()
    }
}

impl From<FractalBoss> for Boss {
    fn from(fractal_boss: FractalBoss) -> Self {
        // fractal boss is always a valid boss
        usize::from(fractal_boss).try_into().unwrap()
    }
}

// buff enums generated from data
include!(concat!(env!("OUT_DIR"), "/buff.rs"));
include!(concat!(env!("OUT_DIR"), "/boon.rs"));
include!(concat!(env!("OUT_DIR"), "/food.rs"));
include!(concat!(env!("OUT_DIR"), "/util.rs"));

impl From<Boon> for Buff {
    fn from(boon: Boon) -> Self {
        // boon is always a valid buff
        u32::from(boon).try_into().unwrap()
    }
}

impl From<Food> for Buff {
    fn from(food: Food) -> Self {
        // food is always a valid buff
        u32::from(food).try_into().unwrap()
    }
}

impl From<Utility> for Buff {
    fn from(util: Utility) -> Self {
        // utility is always a valid buff
        u32::from(util).try_into().unwrap()
    }
}
