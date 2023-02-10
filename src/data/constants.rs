// TODO: use raid & fractal maps for reminder filtering
#![allow(unused)]

use arcdps::Profession;

/// Malnourished buff id.
pub const MALNOURISHED: u32 = 46587;

/// Diminished buff id.
pub const DIMINISHED: u32 = 46668;

/// Professions sorted alphabetically.
pub const PROFESSIONS: &[Profession] = &[
    Profession::Unknown,
    Profession::Elementalist,
    Profession::Engineer,
    Profession::Guardian,
    Profession::Mesmer,
    Profession::Necromancer,
    Profession::Ranger,
    Profession::Revenant,
    Profession::Thief,
    Profession::Warrior,
];

/// Ids of all raid maps.
pub const RAID_MAPS: &[u32] = &[
    1155, // aerodrome
    1062, // spirit vale
    1149, // salvation pass
    1156, // stronghold of the faithful
    1188, // bastion of the penitent
    1264, // hall of chains
    1303, // mythwright gambit
    1323, // key of ahdashim
];

/// Ids of all fractal maps.
pub const FRACTAL_MAPS: &[u32] = &[
    872,  // mistlock observatory
    954,  // volcanic
    947,  // uncategorized
    948,  // snowblind
    950,  // urban battleground
    949,  // swampland
    952,  // cliffside
    951,  // aquatic ruins
    953,  // underground facility
    958,  // solid ocean
    955,  // molten furnance
    959,  // molten boss
    956,  // aetherblade
    957,  // thaumanova reactor
    960,  // captain mai trin boss
    1164, // chaos
    1177, // nightmare
    1205, // shattered observatory
    1267, // twilight oasis
    1290, // deepstone
    1309, // siren's reef
    1384, // sunqua peak
];
