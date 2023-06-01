use super::{ExtrasState, Plugin};
use crate::{data::BuffKind, tracking::Sorting};
use arc_util::{
    api::calc_delta,
    tracking::{Entry, Player},
};
use arcdps::{
    extras::{ExtrasAddonInfo, UserInfo, UserInfoIter, UserRole},
    Activation, Agent, BuffRemove, CombatEvent, StateChange,
};
use log::{debug, info, log_enabled, Level};

impl Plugin {
    /// Handles a combat event from area stats.
    pub fn area_event(
        event: Option<CombatEvent>,
        src: Option<Agent>,
        dst: Option<Agent>,
        skill_name: Option<&str>,
        event_id: u64,
        _revision: u64,
    ) {
        // TODO: put relevant event information in priority queue, update state in worker thread?
        // or split into important (buff initial), unimportant (encounter start) and last seen time (atomic)

        // ignore events without source
        if let Some(src) = src {
            // check for combat event
            if let Some(event) = event {
                match event.is_statechange {
                    StateChange::LogStart => {
                        let mut guard = Self::lock();
                        let plugin = guard.as_mut();
                        let target_id = event.src_agent;
                        if log_enabled!(Level::Debug) {
                            let delta = calc_delta(&event);
                            debug!("Log for id {} started with {:?} delta", target_id, delta);
                        }

                        // change buffs to none, initial buffs should be reported right after
                        for entry in plugin.tracker.players.iter_mut() {
                            entry.data.unset_to_none(
                                event.time,
                                plugin.reminder.all_custom().iter().map(|remind| remind.id),
                            );
                        }

                        // refresh if food or util sorting
                        plugin.tracker.refresh_sort_if(Sorting::Food);
                        plugin.tracker.refresh_sort_if(Sorting::Util);

                        plugin.reminder.start_encounter(target_id, event.time);
                    }

                    StateChange::LogNPCUpdate => {
                        let mut plugin = Self::lock();
                        let target_id = event.src_agent;
                        debug!(
                            "Log changed from {:?} to id {}",
                            plugin.reminder.current_encounter(),
                            target_id
                        );
                        plugin.reminder.change_encounter(target_id, event.time);
                    }

                    StateChange::LogEnd => {
                        let mut guard = Self::lock();
                        let plugin = guard.as_mut();
                        let target_id = event.src_agent;
                        debug!("Log for id {} ended", target_id);
                        plugin.reminder.end_encounter(&plugin.tracker.players);
                    }

                    StateChange::None | StateChange::ApiDelayed | StateChange::BuffInitial => {
                        if event.is_activation == Activation::None {
                            match event.is_buff_remove {
                                BuffRemove::None => {
                                    if event.buff != 0 && event.buff_dmg == 0 {
                                        if let Some(dst) = dst {
                                            Self::lock().buff_apply(
                                                dst.id,
                                                event.skill_id,
                                                skill_name,
                                                &event,
                                                event_id,
                                            );
                                        }
                                    }
                                }

                                // remove on all or single manual
                                BuffRemove::All | BuffRemove::Manual => Self::lock().buff_remove(
                                    src.id,
                                    event.skill_id,
                                    skill_name,
                                    &event,
                                    event_id,
                                ),

                                BuffRemove::Single | BuffRemove::Unknown(_) => {}
                            }
                        }
                    }
                    _ => {}
                }

                // buff initial events will happen at the start
                if event.is_statechange != StateChange::BuffInitial {
                    let mut guard = Self::lock();
                    let plugin = guard.as_mut();
                    plugin
                        .reminder
                        .update_pending_check(&plugin.tracker.players, event.time);
                }
            } else {
                // check for player tracking change
                if src.elite == 0 {
                    let mut plugin = Self::lock();
                    if src.prof != 0 {
                        // add player
                        if let Some(player) =
                            dst.and_then(|dst| Player::from_tracking_change(src, dst))
                        {
                            plugin.tracker.add_player(player);
                        }
                    } else {
                        // remove player
                        plugin.tracker.remove_player(src.id);
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
        event: &CombatEvent,
        event_id: u64,
    ) {
        if let Some(Entry { player, data }) = self.tracker.players.player_mut(player_id) {
            if let Some(remind) = self.reminder.custom(buff_id) {
                debug!(
                    "Custom {} apply id {} time {} statechange {}",
                    remind.display_name(),
                    event_id,
                    event.time,
                    event.is_statechange
                );
                if data.apply_custom(buff_id, event.time) {
                    info!(
                        "{} ({}) applied to {}",
                        remind.display_name(),
                        buff_id,
                        player.character
                    );
                }
            } else {
                match self.defs.buff_kind(buff_id, buff_name) {
                    BuffKind::Food(food) => {
                        debug!(
                            "Food apply id {} time {} statechange {}",
                            event_id, event.time, event.is_statechange
                        );
                        if data.apply_food(buff_id, event.time) {
                            if let Some(food) = food {
                                info!(
                                    "Food {} ({}) applied to {}",
                                    food.name, food.id, player.character
                                );
                            } else {
                                info!("Unknown Food {} applied to {}", buff_id, player.character);
                            }

                            if player.is_self {
                                self.reminder.self_buff_apply(buff_id);
                            }

                            self.tracker.refresh_sort_if(Sorting::Food);
                        }
                    }
                    BuffKind::Util(util) => {
                        debug!(
                            "Util apply id {} time {} statechange {}",
                            event_id, event.time, event.is_statechange
                        );
                        if data.apply_util(buff_id, event.time) {
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

                            if player.is_self {
                                self.reminder.self_buff_apply(buff_id);
                            }

                            self.tracker.refresh_sort_if(Sorting::Util);
                        }
                    }
                    BuffKind::Ignore => {
                        info!("Ignored buff {} applied to {}", buff_id, player.character);
                    }
                    BuffKind::Unknown => {}
                }
            }
        }
    }

    /// Handles a buff remove event.
    fn buff_remove(
        &mut self,
        player_id: usize,
        buff_id: u32,
        buff_name: Option<&str>,
        event: &CombatEvent,
        event_id: u64,
    ) {
        if let Some(Entry { player, data }) = self.tracker.players.player_mut(player_id) {
            if let Some(remind) = self.reminder.custom(buff_id) {
                debug!(
                    "Custom {} remove id {} time {} statechange {} kind {}",
                    remind.display_name(),
                    event_id,
                    event.time,
                    event.is_statechange,
                    event.is_buff_remove
                );
                if data.remove_custom(buff_id, event.time) {
                    info!(
                        "{} ({}) removed from {}",
                        remind.display_name(),
                        buff_id,
                        player.character
                    );

                    // check for custom buff running out
                    if player.is_self {
                        self.reminder.self_custom_remove(data);
                    }
                }
            } else {
                match self.defs.buff_kind(buff_id, buff_name) {
                    BuffKind::Food(food) => {
                        debug!(
                            "Food remove id {} time {} statechange {} kind {}",
                            event_id, event.time, event.is_statechange, event.is_buff_remove
                        );
                        if data.remove_food(buff_id, event.time) {
                            if let Some(food) = food {
                                info!(
                                    "Food {} ({}) removed from {}",
                                    food.name, food.id, player.character
                                );
                            } else {
                                info!("Unknown Food {} removed from {}", buff_id, player.character);
                            }

                            // check for food running out
                            if player.is_self {
                                self.reminder.self_food_remove(data);
                            }

                            self.tracker.refresh_sort_if(Sorting::Food);
                        }
                    }
                    BuffKind::Util(util) => {
                        debug!(
                            "Utility remove id {} time {} statechange {} kind {}",
                            event_id, event.time, event.is_statechange, event.is_buff_remove
                        );
                        if data.remove_util(buff_id, event.time) {
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
                            if player.is_self {
                                self.reminder.self_util_remove(data);
                            }

                            self.tracker.refresh_sort_if(Sorting::Util);
                        }
                    }
                    BuffKind::Ignore => {
                        info!("Ignored buff {} removed from {}", buff_id, player.character)
                    }
                    BuffKind::Unknown => {}
                }
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
                if let Some(Entry { player, .. }) = self
                    .tracker
                    .players
                    .iter_mut()
                    .find(|entry| entry.player.account == name)
                {
                    player.subgroup = subgroup as usize + 1;

                    debug!(
                        "Updated subgroup {} for {}",
                        player.subgroup, player.character
                    );
                }
            }
        }

        self.tracker.refresh_sort_if(Sorting::Sub);
    }
}
