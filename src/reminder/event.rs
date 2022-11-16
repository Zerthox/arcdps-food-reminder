use super::{Encounter, Reminder};
use crate::{
    data::{DIMINISHED, MALNOURISHED},
    tracking::buff::{BuffState, Buffs},
};
use arc_util::tracking::CachedTracker;
use log::debug;

/// Minimum time (ms) since the encounter start for the buff check to trigger.
const CHECK_TIME_DIFF: u64 = 250;

impl Reminder {
    /// Returns the current encounter id.
    pub fn current_encounter(&self) -> Option<usize> {
        self.encounter.as_ref().map(|encounter| encounter.target_id)
    }

    /// Handles encounter start.
    pub fn start_encounter(&mut self, target_id: usize, start_time: u64) {
        self.encounter = Some(Encounter {
            target_id,
            start_time,
            pending_check: self.settings.encounter_start,
        });
    }

    /// Handles encounter target change.
    pub fn change_encounter(&mut self, target_id: usize, time: u64) {
        // only change id, otherwise start as new encounter
        // pending check will be handled later
        if let Some(encounter) = &mut self.encounter {
            encounter.target_id = target_id;
        } else {
            self.start_encounter(target_id, time);
        }
    }

    /// Handles encounter end.
    pub fn end_encounter(&mut self, players: &CachedTracker<Buffs>) {
        if self.settings.encounter_end {
            self.check_self_all(players);
        }
        self.encounter = None;
    }

    /// Updates pending buff check.
    pub fn update_pending_check(&mut self, players: &CachedTracker<Buffs>, time: u64) {
        // handle pending check
        if let Some(encounter) = &mut self.encounter {
            if encounter.pending_check && time >= encounter.start_time + CHECK_TIME_DIFF {
                encounter.pending_check = false;
                self.check_self_all(players);
            }
        }
    }

    /// Handles a buff apply to self.
    pub fn self_buff_apply(&mut self, buff_id: u32) {
        if self.settings.always_mal_dim {
            match buff_id {
                MALNOURISHED => self.trigger_food(),
                DIMINISHED => self.trigger_util(),
                _ => {}
            }
        }
    }

    /// Handles a food remove from self.
    pub fn self_food_remove(&mut self, buffs: &Buffs) {
        if self.settings.during_encounter {
            self.check_food(buffs);
        }
    }

    /// Handles an utility remove from self.
    pub fn self_util_remove(&mut self, buffs: &Buffs) {
        if self.settings.during_encounter {
            self.check_util(buffs);
        }
    }

    /// Handles a reinforced remove from self.
    pub fn self_reinf_remove(&mut self, buffs: &Buffs) {
        if self.settings.during_encounter {
            self.check_reinforced(buffs);
        }
    }

    /// Whether reminders can be triggered.
    fn can_remind(&self) -> bool {
        match &self.encounter {
            Some(encounter) if self.settings.only_bosses => encounter.target_id > 1,
            Some(_) => true,
            None => false,
        }
    }

    /// Performs a check for all reminders.
    fn check_self_all(&mut self, players: &CachedTracker<Buffs>) {
        if let Some(player) = players.get_self() {
            self.check_food(&player.data);
            self.check_util(&player.data);
            self.check_reinforced(&player.data);
        }
    }

    /// Checks for missing food buff.
    fn check_food(&mut self, buffs: &Buffs) {
        if self.can_remind() {
            let Buffs { food, .. } = buffs;
            debug!("Checking food on self: {:?}", food.state);
            if let BuffState::None | BuffState::Some(MALNOURISHED) = food.state {
                self.trigger_food();
            }
        }
    }

    /// Checks for missing utility buff.
    fn check_util(&mut self, buffs: &Buffs) {
        if self.can_remind() {
            let Buffs { util, .. } = buffs;
            debug!("Checking utility on self: {:?}", util.state);
            if let BuffState::None | BuffState::Some(DIMINISHED) = util.state {
                self.trigger_util();
            }
        }
    }

    /// Checks for missing reinforced buff.
    fn check_reinforced(&mut self, buffs: &Buffs) {
        if self.can_remind() {
            let Buffs { reinf, .. } = buffs;
            debug!("Checking reinforced on self: {:?}", reinf.state);
            if let BuffState::None = reinf.state {
                self.trigger_reinforced();
            }
        }
    }
}
