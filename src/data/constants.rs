use arc_util::game::Profession;

/// Malnourished buff id.
pub const MALNOURISHED: u32 = 46587;

/// Diminished buff id.
pub const DIMINISHED: u32 = 46668;

/// Reinforced Armor buff id.
pub const REINFORCED: u32 = 9283;

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
