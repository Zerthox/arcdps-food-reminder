use super::{ExtrasState, Plugin};
use crate::data::{BuffKind, DIMINISHED, MALNOURISHED, REINFORCED};
use arc_util::{
    api::calc_delta,
    tracking::{Entry, Player},
};
use arcdps::{
    extras::{ExtrasAddonInfo, UserInfo, UserInfoIter, UserRole},
    Agent, BuffRemove, CombatEvent, StateChange,
};
use log::{debug, info, log_enabled, Level};

/// Minimum time (ms) since the last [`StateChange::BuffInitial`] event for the buff check to trigger.
const CHECK_TIME_DIFF: u64 = 100;

impl Plugin {
    /// Handles a combat event from area stats.
    pub fn area_event(
        &mut self,
        event: Option<CombatEvent>,
        src: Option<Agent>,
        dst: Option<Agent>,
        skill_name: Option<&str>,
        event_id: u64,
        _revision: u64,
    ) {
        // ignore events without source
        if let Some(src) = src {
            // check for combat event
            if let Some(event) = event {
                match event.is_statechange {
                    StateChange::LogStart => {
                        let target_id = event.src_agent;

                        if log_enabled!(Level::Debug) {
                            let delta = calc_delta(&event);
                            debug!("Log for id {} started with {:?} delta", target_id, delta);
                        }

                        // change buffs to none, initial buffs should be reported right after
                        for entry in self.tracker.players.iter_mut() {
                            entry.data.buffs_to_none(event.time);
                        }

                        // start encounter, set check as pending
                        self.tracker.encounter = Some(target_id);
                        if self.reminder.settings.encounter_start {
                            self.pending_check = Some(event.time);
                        }
                    }

                    StateChange::LogNPCUpdate => {
                        let target_id = event.src_agent;

                        debug!(
                            "Log changed from id {:?} to id {}",
                            self.tracker.encounter, target_id
                        );

                        // update encounter, set check as pending
                        self.tracker.encounter = Some(target_id);
                        if self.reminder.settings.encounter_start {
                            self.pending_check = Some(event.time);
                        }
                    }

                    StateChange::LogEnd => {
                        let target_id = event.src_agent;
                        debug!("Log for id {} ended", target_id);

                        // check self buffs
                        if self.reminder.settings.encounter_end {
                            self.check_self_all();
                        }

                        // end encounter
                        self.tracker.encounter = None;
                    }

                    statechange @ (StateChange::None
                    | StateChange::ApiDelayed
                    | StateChange::BuffInitial) => {
                        // FIXME: tracking "nourishment" & "enhancement" buff names need adjustment for other client languages

                        if let BuffRemove::None = event.is_buff_remove {
                            if event.buff != 0 && event.buff_dmg == 0 {
                                if let Some(dst) = dst {
                                    self.buff_apply(
                                        dst.id,
                                        event.skill_id,
                                        skill_name,
                                        statechange,
                                        event_id,
                                        event.time,
                                    );
                                }
                            }
                        } else {
                            self.buff_remove(
                                src.id,
                                event.skill_id,
                                skill_name,
                                statechange,
                                event_id,
                                event.time,
                            );
                        }
                    }
                    _ => {}
                }

                // handle pending check
                if let Some(last_time) = self.pending_check {
                    if event.is_statechange == StateChange::BuffInitial {
                        // initial buffs are still being reported, refresh time
                        self.pending_check = Some(event.time);
                    } else if event.time >= last_time + CHECK_TIME_DIFF {
                        // check self buffs
                        self.pending_check = None;
                        self.check_self_all();
                    }
                }
            } else {
                // check for player tracking change
                if src.elite == 0 {
                    if src.prof != 0 {
                        // add player
                        if let Some(player) =
                            dst.and_then(|dst| Player::from_tracking_change(src, dst))
                        {
                            self.tracker.add_player(player);
                        }
                    } else {
                        // remove player
                        self.tracker.remove_player(src.id);
                    }
                }
            }
        }
    }

    /// Handles a buff apply event.
    fn buff_apply(
        &mut self,
        player_id: usize,
        buff_id: u32,
        buff_name: Option<&str>,
        statechange: StateChange,
        event_id: u64,
        time: u64,
    ) {
        if let Some(Entry { player, data }) = self.tracker.players.player_mut(player_id) {
            match self.defs.get_buff(buff_id, buff_name) {
                BuffKind::Reinforced => {
                    debug!(
                        "Reinf apply id {} time {} statechange {}",
                        event_id, time, statechange
                    );
                    if data.apply_reinf(time) {
                        info!(
                            "Reinforced ({}) applied to {}",
                            REINFORCED, player.character
                        );
                    }
                }
                BuffKind::Food(food) => {
                    debug!(
                        "Food apply id {} time {} statechange {}",
                        event_id, time, statechange
                    );
                    if data.apply_food(buff_id, time) {
                        if let Some(food) = food {
                            info!(
                                "Food {} ({}) applied to {}",
                                food.name, food.id, player.character
                            );
                        } else {
                            info!("Unknown Food {} applied to {}", buff_id, player.character);
                        }

                        // trigger reminder on malnourished
                        if self.reminder.settings.always_mal_dim
                            && player.is_self
                            && buff_id == MALNOURISHED
                        {
                            self.reminder.trigger_food();
                        }
                    }
                }
                BuffKind::Util(util) => {
                    debug!(
                        "Util apply id {} time {} statechange {}",
                        event_id, time, statechange
                    );
                    if data.apply_util(buff_id, time) {
                        if let Some(util) = util {
                            info!(
                                "Utility {} ({}) applied to {}",
                                util.name, util.id, player.character
                            );
                        } else {
                            info!(
                                "Unknown Utility {} applied to {}",
                                buff_id, player.character
                            );
                        }

                        // trigger reminder on diminished
                        if self.reminder.settings.always_mal_dim
                            && player.is_self
                            && buff_id == DIMINISHED
                        {
                            self.reminder.trigger_util();
                        }
                    }
                }
                BuffKind::Ignore => {
                    info!("Ignored buff {} applied to {}", buff_id, player.character);
                }
                _ => {}
            }
        }
    }

    /// Handles a buff remove event.
    fn buff_remove(
        &mut self,
        player_id: usize,
        buff_id: u32,
        buff_name: Option<&str>,
        statechange: StateChange,
        event_id: u64,
        time: u64,
    ) {
        if let Some(Entry { player, data }) = self.tracker.players.player_mut(player_id) {
            match self.defs.get_buff(buff_id, buff_name) {
                BuffKind::Reinforced => {
                    debug!(
                        "Reinf remove id {} time {} statechange {}",
                        event_id, time, statechange
                    );
                    if data.remove_reinf(time) {
                        info!(
                            "Reinforced ({}) removed from {}",
                            REINFORCED, player.character
                        );

                        // check for reinforced running out
                        if self.reminder.settings.during_encounter && player.is_self {
                            self.check_self_reinforced();
                        }
                    }
                }
                BuffKind::Food(food) => {
                    debug!(
                        "Food remove id {} time {} statechange {}",
                        event_id, time, statechange
                    );
                    if data.remove_food(buff_id, time) {
                        if let Some(food) = food {
                            info!(
                                "Food {} ({}) removed from {}",
                                food.name, food.id, player.character
                            );
                        } else {
                            info!("Unknown Food {} removed from {}", buff_id, player.character);
                        }

                        // check for food running out
                        if self.reminder.settings.during_encounter && player.is_self {
                            self.check_self_food();
                        }
                    }
                }
                BuffKind::Util(util) => {
                    debug!(
                        "Utility remove id {} time {} statechange {}",
                        event_id, time, statechange
                    );
                    if data.remove_util(buff_id, time) {
                        if let Some(util) = util {
                            info!(
                                "Utility {} ({}) removed from {}",
                                util.name, util.id, player.character
                            );
                        } else {
                            info!(
                                "Unknown Utility {} removed from {}",
                                buff_id, player.character
                            );
                        }

                        // check for utility running out
                        if self.reminder.settings.during_encounter && player.is_self {
                            self.check_self_util();
                        }
                    }
                }
                BuffKind::Ignore => {
                    info!("Ignored buff {} removed from {}", buff_id, player.character)
                }
                _ => {}
            }
        }
    }

    /// Handles initialization from unofficial extras.
    pub fn extras_init(&mut self, extras_info: ExtrasAddonInfo, _account_name: Option<&str>) {
        self.extras = if extras_info.is_compatible() {
            ExtrasState::Found
        } else {
            ExtrasState::Incompatible
        };
    }

    /// Handles a squad update from unofficial extras.
    pub fn extras_squad_update(&mut self, users: UserInfoIter) {
        for user in users {
            if let UserInfo {
                account_name: Some(name),
                role: UserRole::SquadLeader | UserRole::Lieutenant | UserRole::Member,
                subgroup,
                ..
            } = user
            {
                if let Some(entry) = self
                    .tracker
                    .players
                    .iter_mut()
                    .find(|entry| entry.player.account == name)
                {
                    entry.player.subgroup = subgroup as usize + 1;

                    debug!(
                        "Updated subgroup {} for {}",
                        entry.player.subgroup, entry.player.character
                    );
                }
            }
        }
    }
}
