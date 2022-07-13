use super::{ExtrasState, Plugin};
use crate::data::{DefKind, DIMINISHED, MALNOURISHED, REINFORCED};
use arc_util::player::Player;
use arcdps::{
    extras::{ExtrasAddonInfo, UserInfo, UserInfoIter, UserRole},
    Agent, BuffRemove, CombatEvent, StateChange,
};

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

                        if let Some(entry) = self.tracker.player_mut(src.id) {
                            let player = &mut entry.player;
                            player.elite = src.elite.into();
                            player.enter_combat(Some(event.dst_agent));

                            #[cfg(feature = "log")]
                            self.debug.log(format!("Combat enter for {:?}", entry));
                        }
                    }

                    StateChange::ExitCombat => {
                        // combat exit

                        if let Some(entry) = self.tracker.player_mut(src.id) {
                            entry.player.exit_combat();

                            #[cfg(feature = "log")]
                            self.debug.log(format!("Combat exit for {:?}", entry));
                        }
                    }

                    StateChange::LogStart => {
                        // log start

                        #[cfg(feature = "log")]
                        // let delta = api::calc_delta(event);
                        let delta = 0; // TODO

                        // change buffs to none
                        // initial buffs should be reported right after
                        for entry in self.tracker.all_players_mut() {
                            entry.buffs_to_none(event.time, event_id);
                        }

                        // start encounter
                        let target_id = event.src_agent;
                        self.tracker.start_encounter(target_id);

                        // set check as pending
                        self.pending_check = Some(event.time);

                        #[cfg(feature = "log")]
                        self.debug.log(format!(
                            "Log for id {} started with {:?} delta",
                            target_id, delta
                        ));
                    }

                    StateChange::LogEnd => {
                        // log end

                        #[cfg(feature = "log")]
                        {
                            let target_id = event.src_agent;
                            self.debug.log(format!("Log for id {} ended", target_id));
                        }

                        // check self buffs
                        if self.reminder.settings.encounter_end {
                            self.check_self_food();
                            self.check_self_util();
                            self.check_self_reinforced();
                        }

                        // end encounter
                        self.tracker.end_encounter();
                    }

                    #[cfg_attr(not(feature = "log"), allow(unused))]
                    statechange => {
                        // TODO: should we restrict this to specific state change kinds?
                        // FIXME: tracking "nourishment" & "enhancement" buff names need adjustment for other client languages

                        if let BuffRemove::None = event.is_buff_remove.into() {
                            if event.buff != 0 && event.buff_dmg == 0 {
                                // buff applied

                                // check for tracked player
                                if let Some(dest) = dest {
                                    if let Some(entry) = self.tracker.player_mut(dest.id) {
                                        let buff_id = event.skill_id;

                                        // check type of buff
                                        if buff_id == REINFORCED {
                                            if entry.apply_reinf(event.time, event_id) {
                                                #[cfg(feature = "log")]
                                                self.debug.log(format!(
                                                    "Reinforced applied on {:?} to {:?}",
                                                    statechange, entry
                                                ));
                                            }
                                        } else if let Some(buff_type) = self.defs.get_buff(buff_id)
                                        {
                                            match buff_type {
                                                DefKind::Food(food) => {
                                                    if entry
                                                        .apply_food(food.id, event.time, event_id)
                                                    {
                                                        #[cfg(feature = "log")]
                                                        self.debug.log(format!(
                                                            "Food {:?} applied on {:?} to {:?}",
                                                            food, statechange, entry
                                                        ));

                                                        // trigger reminder on malnourished
                                                        if self.reminder.settings.always_mal_dim
                                                            && entry.player.is_self
                                                            && food.id == MALNOURISHED
                                                        {
                                                            self.reminder.trigger_food();

                                                            #[cfg(feature = "log")]
                                                            self.debug.log(format!(
                                                                "Food Malnourished reminder triggered on {:?}",
                                                                statechange
                                                            ));
                                                        }
                                                    }
                                                }
                                                DefKind::Util(util) => {
                                                    if entry
                                                        .apply_util(util.id, event.time, event_id)
                                                    {
                                                        #[cfg(feature = "log")]
                                                        self.debug.log(format!(
                                                            "Utility {:?} applied on {:?} to {:?}",
                                                            util, statechange, entry
                                                        ));

                                                        // trigger reminder on diminished
                                                        if self.reminder.settings.always_mal_dim
                                                            && entry.player.is_self
                                                            && util.id == DIMINISHED
                                                        {
                                                            self.reminder.trigger_util();

                                                            #[cfg(feature = "log")]
                                                            self.debug.log(format!(
                                                                "Utility Diminished reminder triggered on {:?}",
                                                                statechange,
                                                            ));
                                                        }
                                                    }
                                                }
                                                DefKind::Ignore => {
                                                    #[cfg(feature = "log")]
                                                    self.debug.log(format!(
                                                        "Ignored application of {} to {:?}",
                                                        buff_id, entry
                                                    ));
                                                }
                                            }
                                        } else if let Some("Nourishment") = skill_name {
                                            if entry.apply_food(buff_id, event.time, event_id) {
                                                #[cfg(feature = "log")]
                                                self.debug.log(format!(
                                                    "Unknown Food with id {} applied on {:?} to {:?}",
                                                    buff_id, statechange, entry
                                                ));
                                            }
                                        } else if let Some("Enhancement") = skill_name {
                                            if entry.apply_util(buff_id, event.time, event_id) {
                                                #[cfg(feature = "log")]
                                                self.debug.log(format!(
                                                    "Unknown Utility with id {} applied on {:?} to {:?}",
                                                    buff_id, statechange, entry
                                                ));
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            // buff removed

                            // check for tracked player
                            if let Some(entry) = self.tracker.player_mut(src.id) {
                                let buff_id = event.skill_id;

                                // check type of buff
                                if buff_id == REINFORCED {
                                    if entry.remove_reinf(event.time, event_id) {
                                        #[cfg(feature = "log")]
                                        self.debug.log(format!(
                                            "Reinforced removed on {:?} from {:?}",
                                            statechange, entry
                                        ));

                                        // check for reinforced running out
                                        if self.reminder.settings.during_encounter
                                            && entry.player.is_self
                                        {
                                            self.check_self_reinforced();
                                        }
                                    }
                                } else if let Some(buff_type) = self.defs.get_buff(buff_id) {
                                    match buff_type {
                                        DefKind::Food(food) => {
                                            if entry.remove_food(food.id, event.time, event_id) {
                                                #[cfg(feature = "log")]
                                                self.debug.log(format!(
                                                    "Food {:?} removed on {:?} from {:?}",
                                                    food, statechange, entry
                                                ));

                                                // check for food running out
                                                if self.reminder.settings.during_encounter
                                                    && entry.player.is_self
                                                {
                                                    self.check_self_food();
                                                }
                                            }
                                        }
                                        DefKind::Util(util) => {
                                            if entry.remove_util(util.id, event.time, event_id) {
                                                #[cfg(feature = "log")]
                                                self.debug.log(format!(
                                                    "Utility {:?} removed on {:?} from {:?}",
                                                    util, statechange, entry
                                                ));

                                                // check for utility running out
                                                if self.reminder.settings.during_encounter
                                                    && entry.player.is_self
                                                {
                                                    self.check_self_util();
                                                }
                                            }
                                        }
                                        DefKind::Ignore => {
                                            #[cfg(feature = "log")]
                                            self.debug.log(format!(
                                                "Ignored removal of {} from {:?}",
                                                buff_id, entry
                                            ));
                                        }
                                    }
                                } else if let Some("Nourishment") = skill_name {
                                    if entry.remove_food(buff_id, event.time, event_id) {
                                        #[cfg(feature = "log")]
                                        self.debug.log(format!(
                                            "Unknown Food with id {} removed on {:?} from {:?}",
                                            buff_id, statechange, entry
                                        ));

                                        // check for food running out
                                        if self.reminder.settings.during_encounter
                                            && entry.player.is_self
                                        {
                                            self.check_self_food();
                                        }
                                    }
                                } else if let Some("Enhancement") = skill_name {
                                    if entry.remove_util(buff_id, event.time, event_id) {
                                        #[cfg(feature = "log")]
                                        self.debug.log(format!(
                                            "Unknown Utility with id {} removed on {:?} from {:?}",
                                            buff_id, statechange, entry
                                        ));

                                        // check for utility running out
                                        if self.reminder.settings.during_encounter
                                            && entry.player.is_self
                                        {
                                            self.check_self_util();
                                        }
                                    }
                                }
                            }
                        }

                        // handle pending check
                        if let Some(last_time) = self.pending_check {
                            if statechange == StateChange::BuffInitial {
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

                            #[cfg(feature = "log")]
                            self.debug.log(format!("Added {:?}", player));

                            self.tracker.add_player(player);
                        }
                    } else {
                        // player removed

                        let id = src.id;

                        #[cfg_attr(not(feature = "log"), allow(unused))]
                        let removed = self.tracker.remove_player(id);

                        #[cfg(feature = "log")]
                        if let Some(entry) = removed {
                            self.debug.log(format!("Removed {:?}", entry));
                        }
                    }
                }
            }
        }
    }

    /// Handles initialization from unofficial extras.
    // TODO: update for new API
    pub fn extras_init(&mut self, extras_info: ExtrasAddonInfo, _account_name: Option<&str>) {
        self.extras = if extras_info.check_compat() {
            ExtrasState::Found
        } else {
            ExtrasState::Incompatible
        }
    }

    /// Handles a squad update from unofficial extras.
    pub fn extras_squad_update(&mut self, users: UserInfoIter) {
        for user in users {
            #[cfg(feature = "log")]
            self.debug.log(format!("Squad update: {:?}", user));

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
                    .all_players_mut()
                    .find(|entry| entry.player.account == acc_name)
                {
                    entry.player.subgroup = subgroup as usize + 1;

                    #[cfg(feature = "log")]
                    self.debug.log(format!(
                        "Updated subgroup {} for {}",
                        entry.player.subgroup, entry.player.character
                    ));
                }
            }
        }
    }
}
