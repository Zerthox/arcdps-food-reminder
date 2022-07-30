use super::{ExtrasState, Plugin};
use crate::data::{DefKind, DIMINISHED, MALNOURISHED, REINFORCED};
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
        dest: Option<Agent>,
        skill_name: Option<&str>,
        event_id: u64,
        _revision: u64,
    ) {
        // ignore events without source
        if let Some(src) = src {
            // check for combat event
            if let Some(event) = event {
                match event.is_statechange {
                    StateChange::EnterCombat => {
                        // combat enter

                        if let Some(entry) = self.tracker.players.player_mut(src.id) {
                            let player = &mut entry.player;
                            player.elite = src.elite.into();
                            player.enter_combat(Some(event.dst_agent));

                            debug!("Combat enter for {}", entry.player.character);
                        }
                    }

                    StateChange::ExitCombat => {
                        // combat exit

                        if let Some(entry) = self.tracker.players.player_mut(src.id) {
                            entry.player.exit_combat();

                            debug!("Combat exit for {}", entry.player.character);
                        }
                    }

                    StateChange::LogStart => {
                        // log start

                        let target_id = event.src_agent;

                        if log_enabled!(Level::Debug) {
                            let delta = calc_delta(&event);
                            debug!("Log for id {} started with {:?} delta", target_id, delta);
                        }

                        // change buffs to none
                        // initial buffs should be reported right after
                        for entry in self.tracker.players.iter_mut() {
                            entry.data.buffs_to_none(event.time);
                        }

                        // start encounter
                        self.tracker.encounter = Some(target_id);

                        // set check as pending
                        self.pending_check = Some(event.time);
                    }

                    StateChange::LogEnd => {
                        // log end

                        if log_enabled!(Level::Debug) {
                            let target_id = event.src_agent;
                            debug!("Log for id {} ended", target_id);
                        }

                        // check self buffs
                        if self.reminder.settings.encounter_end {
                            self.check_self_food();
                            self.check_self_util();
                            self.check_self_reinforced();
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
                                // buff applied

                                // check for tracked player
                                if let Some(dest) = dest {
                                    if let Some(Entry { player, data }) =
                                        self.tracker.players.player_mut(dest.id)
                                    {
                                        let buff_id = event.skill_id;

                                        // check type of buff
                                        if buff_id == REINFORCED {
                                            debug!(
                                                "Reinf apply id {} time {} statechange {}",
                                                event_id, event.time, statechange
                                            );
                                            if data.apply_reinf(event.time) {
                                                info!(
                                                    "Reinforced ({}) applied to {}",
                                                    REINFORCED, player.character
                                                );
                                            }
                                        } else if let Some(buff_type) = self.defs.get_buff(buff_id)
                                        {
                                            match buff_type {
                                                DefKind::Food(food) => {
                                                    debug!(
                                                        "Food apply id {} time {} statechange {}",
                                                        event_id, event.time, statechange
                                                    );
                                                    if data.apply_food(food.id, event.time) {
                                                        info!(
                                                            "Food {} ({}) applied to {}",
                                                            food.name, food.id, player.character
                                                        );
                                                        // trigger reminder on malnourished
                                                        if self.reminder.settings.always_mal_dim
                                                            && player.is_self
                                                            && food.id == MALNOURISHED
                                                        {
                                                            self.reminder.trigger_food();
                                                        }
                                                    }
                                                }
                                                DefKind::Util(util) => {
                                                    debug!(
                                                        "Util apply id {} time {} statechange {}",
                                                        event_id, event.time, statechange
                                                    );
                                                    if data.apply_util(util.id, event.time) {
                                                        info!(
                                                            "Utility {} ({}) applied to {}",
                                                            util.name, util.id, player.character
                                                        );

                                                        // trigger reminder on diminished
                                                        if self.reminder.settings.always_mal_dim
                                                            && player.is_self
                                                            && util.id == DIMINISHED
                                                        {
                                                            self.reminder.trigger_util();
                                                        }
                                                    }
                                                }
                                                DefKind::Ignore => {
                                                    info!(
                                                        "Ignored buff {} applied to {}",
                                                        buff_id, player.character
                                                    );
                                                }
                                            }
                                        } else if let Some("Nourishment") = skill_name {
                                            if data.apply_food(buff_id, event.time) {
                                                info!(
                                                    "Unknown Food {} applied to {}",
                                                    buff_id, player.character
                                                );
                                            }
                                        } else if let Some("Enhancement") = skill_name {
                                            if data.apply_util(buff_id, event.time) {
                                                info!(
                                                    "Unknown Utility {} applied to {}",
                                                    buff_id, player.character
                                                );
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            // buff removed

                            // check for tracked player
                            if let Some(Entry { player, data }) =
                                self.tracker.players.player_mut(src.id)
                            {
                                let buff_id = event.skill_id;

                                // check type of buff
                                if buff_id == REINFORCED {
                                    debug!(
                                        "Reinf remove id {} time {} statechange {}",
                                        event_id, event.time, statechange
                                    );
                                    if data.remove_reinf(event.time) {
                                        info!(
                                            "Reinforced ({}) removed from {}",
                                            REINFORCED, player.character
                                        );

                                        // check for reinforced running out
                                        if self.reminder.settings.during_encounter && player.is_self
                                        {
                                            self.check_self_reinforced();
                                        }
                                    }
                                } else if let Some(buff_type) = self.defs.get_buff(buff_id) {
                                    match buff_type {
                                        DefKind::Food(food) => {
                                            debug!(
                                                "Food remove id {} time {} statechange {}",
                                                event_id, event.time, statechange
                                            );
                                            if data.remove_food(food.id, event.time) {
                                                info!(
                                                    "Food {} ({}) removed from {}",
                                                    food.name, food.id, player.character
                                                );

                                                // check for food running out
                                                if self.reminder.settings.during_encounter
                                                    && player.is_self
                                                {
                                                    self.check_self_food();
                                                }
                                            }
                                        }
                                        DefKind::Util(util) => {
                                            debug!(
                                                "Util remove id {} time {} statechange {}",
                                                event_id, event.time, statechange
                                            );
                                            if data.remove_util(util.id, event.time) {
                                                info!(
                                                    "Utility {} ({}) removed from {}",
                                                    util.name, util.id, player.character
                                                );

                                                // check for utility running out
                                                if self.reminder.settings.during_encounter
                                                    && player.is_self
                                                {
                                                    self.check_self_util();
                                                }
                                            }
                                        }
                                        DefKind::Ignore => {
                                            info!(
                                                "Ignored buff {} removed from {}",
                                                buff_id, player.character
                                            );
                                        }
                                    }
                                } else if let Some("Nourishment") = skill_name {
                                    if data.remove_food(buff_id, event.time) {
                                        info!(
                                            "Unknown Food {} removed from {}",
                                            buff_id, player.character
                                        );

                                        // check for food running out
                                        if self.reminder.settings.during_encounter && player.is_self
                                        {
                                            self.check_self_food();
                                        }
                                    }
                                } else if let Some("Enhancement") = skill_name {
                                    if data.remove_util(buff_id, event.time) {
                                        info!(
                                            "Unknown Utility {} removed from {}",
                                            buff_id, player.character
                                        );

                                        // check for utility running out
                                        if self.reminder.settings.during_encounter && player.is_self
                                        {
                                            self.check_self_util();
                                        }
                                    }
                                }
                            }
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
                        self.pending_check = None;

                        // check self buffs
                        if self.reminder.settings.encounter_start {
                            self.check_self_food();
                            self.check_self_util();
                            self.check_self_reinforced();
                        }
                    }
                }
            } else {
                // check for player tracking change
                if src.elite == 0 {
                    if src.prof != 0 {
                        // player added

                        if let (
                            Some(char_name),
                            Some(Agent {
                                name: Some(dest_name),
                                prof,
                                elite,
                                team: sub,
                                is_self,
                                ..
                            }),
                        ) = (src.name, dest)
                        {
                            let acc_name = dest_name.strip_prefix(':').unwrap_or(dest_name);
                            let player = Player::new(
                                src.id,
                                char_name,
                                acc_name,
                                is_self != 0,
                                prof.into(),
                                elite.into(),
                                sub as usize,
                            );

                            self.tracker.add_player(player);
                        }
                    } else {
                        // player removed

                        self.tracker.remove_player(src.id);
                    }
                }
            }
        }
    }

    /// Handles initialization from unofficial extras.
    pub fn extras_init(&mut self, extras_info: ExtrasAddonInfo, _account_name: Option<&str>) {
        self.extras = if extras_info.check_compat() {
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
                let acc_name = name.strip_prefix(':').unwrap_or(name);
                if let Some(entry) = self
                    .tracker
                    .players
                    .iter_mut()
                    .find(|entry| entry.player.account == acc_name)
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
