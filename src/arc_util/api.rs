//! ArcDPS combat API utilities.

// we wont use all enum kinds here but keeping them makes sense
#![allow(unused)]

use arcdps::CombatEvent;
use num_enum::{FromPrimitive, TryFromPrimitive};
use std::mem;

/// Reads a buff **instance** id from an event.
///
/// This should only be used once the event is clearly a buff event.
pub fn read_buff_instance_id(event: &CombatEvent) -> u32 {
    unsafe { mem::transmute::<[u8; 4], u32>([event.pad61, event.pad62, event.pad63, event.pad64]) }
}

/// Reads a buff duration from an event.
///
/// This should only be used once the event is clearly a buff apply event.
pub fn read_buff_duration(event: &CombatEvent) -> u32 {
    unsafe { mem::transmute::<i32, u32>(event.value) }
}

/// Whether the entity is an ally or enemy.
///
/// *Arc calls this "iff" for if friend/foe.*
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive)]
#[repr(u8)]
pub enum Team {
    /// Allied entity.
    Friend,

    /// Enemy entity.
    Foe,

    /// Uncertain whether ally or enemy.
    #[num_enum(default)]
    Unknown,
}

/// Strike types.
///
/// *Arc calls this "combat result".*
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive)]
#[repr(u8)]
pub enum Strike {
    /// Normal damage strike.
    ///
    /// No crit, no glance.
    Normal,

    /// Strike was critical.
    Crit,

    /// Strike was glancing.
    Glance,

    /// Strike was blocked.
    ///
    /// Due to Aegis, Chrono Shield 4 etc.
    Block,

    /// Strike was evaded.
    ///
    /// Due to dodge, Mesmer Sword 2 etc.
    Evade,

    /// Strike interrupted something.
    Interrupt,

    /// Strike was absorbed.
    ///
    /// Usually due to a "true" invulnerability like Guardian Renewed Focus.
    Absorb,

    /// Strike missed.
    ///
    /// Due to blind etc.
    Blind,

    /// Skill killed the target.
    ///
    /// Not a damage strike.
    KillingBlow,

    /// Skill downed the target.
    ///
    /// Not a damage strike.
    Downed,

    /// Skill dealt breakbar damage.
    ///
    /// Not a damage strike.
    Breakbar,

    /// On-activation event.
    ///
    /// Not a damage strike.
    ///
    /// *Arc: Source hit target if damaging buff.*
    Activation,

    /// Unknown or invalid.
    #[num_enum(default)]
    Unknown,
}

/// Combat state change.
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive)]
#[repr(u8)]
pub enum Activation {
    /// Not used, different kind of event.
    None,

    /// Started skill/animation activation.
    Start,

    /// Unused as of 5th November 2019.
    QuicknessUnused,

    /// Stopped skill activation with reaching tooltip time.
    CancelFire,

    /// Stopped skill activation without reaching tooltip time.
    CancelCancel,

    /// Animation completed fully.
    Reset,

    /// Unknown or invalid.
    #[num_enum(default)]
    Unknown,
}

/// Combat state change.
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive)]
#[repr(u8)]
pub enum StateChange {
    /// Not used, different kind of event.
    None,

    /// Source entity entered combat.
    ///
    /// Destination contains the subgroup.
    EnterCombat,

    /// Source entity left combat.
    ExitCombat,

    /// Source entity is now alive.
    ChangeUp,

    /// Source entity is now dead.
    ChangeDead,

    /// Source entity is now downed.
    ChangeDown,

    /// Source entity is now in game tracking range.
    ///
    /// *Not used in realtime API.*
    Spawn,

    /// Source entity is no longer being tracked or out of game tracking range.
    ///
    /// *Not used in realtime API.*
    Despawn,

    /// Source entity health change.
    ///
    /// Destination contains percentage as `percent * 10000`.
    /// For example 99.5% will be `9950`.
    ///
    /// *Not used in realtime API.*
    HealthUpdate,

    /// Logging has started.
    ///
    /// Value contains the server Unix timestamp as `u32`.
    /// Buff damage contains the local Unix timestamp.
    ///
    /// Source id is `0x637261` (ArcDPS id) if log EVTC and species id if realtime API.
    LogStart,

    /// Logging has ended.
    ///
    /// Value contains the server Unix timestamp as `u32`.
    /// Buff damage contains the local Unix timestamp.
    ///
    /// Source id is `0x637261` (ArcDPS id) if log EVTC and species id if realtime API.
    LogEnd,

    /// Source entity swapped weapon set.
    ///
    /// Destination contains the current set id.
    /// 0`/`1` for underwater weapons and `4`/`5` for land weapons.
    WeaponSwap,

    /// Source entity maximum health change.
    ///
    /// Destination contains the new maximum health.
    ///
    /// *Not used in realtime API.*
    MaxHealthUpdate,

    /// Source entity is "recording" player.
    ///
    /// *Not used in realtime API.*
    PointOfView,

    /// Source entity contains the game text language.
    ///
    /// *Not used in realtime API.*
    Language,

    /// Source entity contains the game build.
    ///
    /// *Not used in realtime API.*
    GWBuild,

    /// Source entity contains the sever shard id.
    ///
    /// *Not used in realtime API.*
    ShardId,

    /// Source entity got a reward chest.
    ///
    /// Source is always self.
    /// Destination contains the reward id.
    /// Value contains the reward type.
    Reward,

    /// Appears once per buff per entity on logging start.
    ///
    /// *(`statechange == 18` and `buff == 18`, normal combat event otherwise)*
    BuffInitial,

    /// Source entity position change.
    ///
    /// Destination contains x/y/z as array of 3 floats.
    ///
    /// *Not used in realtime API.*
    Position,

    /// Source entity velocity change.
    ///
    /// Destination contains x/y/z as array of 3 floats.
    ///
    /// *Not used in realtime API.*
    Velocity,

    /// Source entity facing change.
    ///
    /// Destination contains x/y as array of 2 floats.
    ///
    /// *Not used in realtime API.*
    Facing,

    /// Source entity team change.
    ///
    /// Destination contains the new team id.
    TeamChange,

    /// Source entity is now an attack target.
    ///
    /// Destination is the parent entity (gadget type).
    /// Value contains the current targetable state.
    ///
    /// *Not used in realtime API.*
    AttackTarget,

    /// Source entity targetability change.
    ///
    /// Destination contains the new targetable state.
    /// `0` for no, `1` for yes. Default is yes.
    ///
    /// *Not used in realtime API.*
    Targetable,

    /// Source entity contains the map id.
    ///
    /// *Not used in realtime API.*
    MapId,

    /// Used internally by ArcDPS.
    /// Should not appear anywhere.
    ReplInfo,

    /// Source entity with active buff.
    ///
    /// Destination contains the stack id marked active.
    StackActive,

    /// Source entity with reset buff.
    ///
    /// Value is the duration to reset to (also marks inactive).
    /// Padding 61 contains the stack id.
    StackReset,

    /// Source entity is in guild.
    ///
    /// Destination until buff damage is 16 byte (`u8`) guid.
    ///
    /// Given in client form, needs minor rearrange for API form.
    Guild,

    /// Buff information.
    ///
    /// If `is_flanking` probably invulnerable.
    /// If `is_shields` probably invert.
    ///
    /// Offcycle contains the category.
    /// Padding 61 contains the stacking type.
    /// Padding 62 contains the probably resistance.
    /// Source master instance id contains the max stacks.
    /// Overstack value contains the duration cap.
    ///
    /// *Not used in realtime API.*
    BuffInfo,

    /// Buff formula.
    ///
    /// Time contains `type`, `attr1`, `attr2`, `param1`, `param2`, `param3`, `trait_src` and `trait_self` as array of 8 floats.
    /// Source instance id contains `buff_src` and `buff_self` as array of 2 floats.
    ///
    /// If `is_flanking` not NPC.
    /// If `is_shields` not player.
    /// If `is_offcycle` break.
    ///
    /// `overstack` is value of type determined by padding 61.
    ///
    /// Once per formula.
    ///
    /// *Not used in realtime API.*
    BuffFormula,

    /// Skill information.
    ///
    /// Time contains `recharge`, `range0`, `range1` and `tooltiptime` as array of 4 floats.
    ///
    /// *Not used in realtime API.*
    SkillInfo,

    /// Skill action.
    ///
    /// Source contains the action.
    /// Destination contains at which millisecond.
    ///
    /// One per timing.
    ///
    /// *Not used in realtime API.*
    SkillTiming,

    /// Source entity breakbar state change.
    ///
    /// Value is `u16` game enum (active, recover, immune, none).
    ///
    /// *Not used in realtime API.*
    BreakbarState,

    /// Breakbar percentage.
    ///
    /// Value contains percentage as float.
    ///
    /// *Not used in realtime API.*
    BreakbarPercent,

    /// Error.
    ///
    /// Time contains the error message as an array of up to 32 characters.
    ///
    /// *Not used in realtime API.*
    Error,

    /// Source entity has tag.
    ///
    /// Value is the id of the tag.
    /// Tag id is volatile, depends on game build.
    Tag,

    /// Source entity barrier change.
    ///
    /// Destination contains percentage as `percent * 10000`.
    /// For example 99.5% will be `9950`.
    ///
    /// *Not used in realtime API.*
    BarrierUpdate,

    /// Arc UI stats reset.
    ///
    /// Source entity contains the npc id of the active log.
    ///
    /// *Not used in log EVTC.*
    StatReset,

    /// Combat event with state change byte set to this.
    Extension,

    /// Combat event with state change byte set to this.
    ApiDelayed,

    /// Unknown or invalid.
    #[num_enum(default)]
    Unknown,
}

/// Combat buff remove.
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive)]
#[repr(u8)]
pub enum BuffRemove {
    /// Not used, different kind of event.
    None,

    /// Last or all stacks removed.
    ///
    /// Sent by server.
    All,

    /// Single stack removed.
    ///
    /// Happens for each stack on cleanse.
    ///
    /// Sent by server.
    Single,

    /// Single stack removed.
    ///
    /// Automatically by Arc on out of combat or all stack.
    /// Ignore for strip/cleanse calculation.
    /// Use for in/out volume.
    Manual,

    /// Unknown or invalid.
    #[num_enum(default)]
    Unknown,
}

/// Combat buff cycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive)]
#[repr(u8)]
pub enum BuffCycle {
    /// Damage happened on tick timer.
    Cycle,

    /// Damage happened outside tick timer (resistable).
    NotCycle,

    /// Retired since May 2021.
    NotCycleOrResist,

    /// Damage happened to target on hitting target.
    NotCycleDmgToTargetOnHit,

    /// Damage happened to source on hitting target.
    NotCycleDmgToSourceOnHit,

    /// Damage happened to target on source losing a stack.
    NotCycleDmgToTargetOnStackRemove,

    #[num_enum(default)]
    Unknown,
}

/// ArcDPS custom skill ids.
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u16)]
pub enum CustomSkill {
    /// Resurrect skill.
    ///
    /// Not custom but important and unnamed.
    Resurrect = 1066,

    /// Bandage downstate skill.
    ///
    /// Personal healing only.
    Bandage = 1175,

    /// Dodge skill.
    ///
    /// Will occur in `is_activation == normal` event.
    Dodge = 65001,
}

/// Buff info category.
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum BuffCategory {
    Boon = 0,
    Any = 1,
    Condition = 2,
    Food = 4,
    Upgrade = 6,
    Boost = 8,
    Trait = 11,
    Enhancement = 13,
    Stance = 16,
}
