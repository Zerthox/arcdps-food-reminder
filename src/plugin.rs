use crate::{
    arc_util::{
        api::{BuffRemove, StateChange},
        exports,
        game::{Boss, Food, Utility},
    },
    reminder::Reminder,
    settings::Settings,
    tracking::{
        player::{BuffState, Player},
        Tracker,
    },
    ui::{
        window::{Window, Windowed},
        Component, Hideable,
    },
    win::VirtualKey,
};
use arcdps::{
    imgui::{im_str, Ui},
    Agent, CombatEvent,
};
use std::convert::TryFrom;

#[cfg(feature = "demo")]
use crate::demo::Demo;

#[cfg(feature = "log")]
use crate::{arc_util::api, log::DebugLog};

/// Hotkey for the tracker.
const TRACKER_KEY: usize = VirtualKey::F.0 as usize;

/// Hotkey for the demo.
#[cfg(feature = "demo")]
const DEMO_KEY: usize = VirtualKey::D.0 as usize;

/// Hotkey for the log.
#[cfg(feature = "log")]
const LOG_KEY: usize = VirtualKey::L.0 as usize;

/// Main plugin.
#[derive(Debug)]
pub struct Plugin {
    /// Food reminder.
    reminder: Reminder,

    /// Food tracker window.
    tracker: Window<Tracker>,

    /// Demo window.
    #[cfg(feature = "demo")]
    demo: Window<Demo>,

    /// Debug log window.
    #[cfg(feature = "log")]
    debug: Window<DebugLog>,
}

impl Plugin {
    /// Creates a new plugin.
    pub fn new() -> Self {
        Self {
            reminder: Reminder::new(),
            tracker: Tracker::new().windowed(),

            #[cfg(feature = "demo")]
            demo: Demo::new().windowed(),

            #[cfg(feature = "log")]
            debug: DebugLog::new().windowed(),
        }
    }

    /// Loads the plugin.
    pub fn load(&mut self) {
        #[cfg(feature = "log")]
        self.debug.log("Plugin load");

        // load settings
        let mut settings = Settings::load_or_initial();

        #[cfg(feature = "log")]
        self.debug.log(format!("Loaded {:?}", settings));

        // load tracker settings
        settings.load_component(&mut self.tracker);

        // load demo settings
        #[cfg(feature = "demo")]
        settings.load_component(&mut self.demo);

        // load log settings
        #[cfg(feature = "log")]
        settings.load_component(&mut self.debug);
    }

    /// Unloads the plugin.
    pub fn unload(&mut self) {
        let mut settings = Settings::load_or_initial();

        // update tracker settings
        settings.store_component(&self.tracker);

        // update demo settings
        #[cfg(feature = "demo")]
        settings.store_component(&self.demo);

        // update log settings
        #[cfg(feature = "log")]
        settings.store_component(&self.debug);

        // save settings
        settings.save();
    }

    /// Handles a combat event.
    pub fn combat_event(
        &mut self,
        event: Option<&CombatEvent>,
        src: Option<Agent>,
        dest: Option<Agent>,
        skill_name: Option<&str>,
        event_id: u64,
        _revision: u64,
    ) {
        // source should always be set, but we dont want to cause a crash
        if let Some(src) = src {
            // check for combat event
            if let Some(event) = event {
                match event.is_statechange.into() {
                    StateChange::EnterCombat => {
                        // combat enter

                        if let Some(player) = self.tracker.player_mut(src.id) {
                            player.enter_combat(Some(event.dst_agent));

                            #[cfg(feature = "log")]
                            self.debug.log(format!("Combat enter for {:?}", player));
                        }
                    }
                    StateChange::ExitCombat => {
                        // combat exit

                        if let Some(player) = self.tracker.player_mut(src.id) {
                            player.exit_combat();

                            #[cfg(feature = "log")]
                            self.debug.log(format!("Combat exit for {:?}", player));
                        }
                    }
                    StateChange::LogStart => {
                        // log start

                        #[cfg(feature = "log")]
                        let delta = api::calc_delta(event);

                        // change unset buffs to none
                        // initial buffs should be reported right after
                        for player in self.tracker.all_players_mut() {
                            player.unset_to_none(event_id);
                        }

                        // grab target id
                        let target_id = event.src_agent;

                        // check for known boss
                        if let Ok(boss) = Boss::try_from(target_id) {
                            self.tracker.start_encounter(boss);

                            #[cfg(feature = "log")]
                            self.debug
                                .log(format!("Log for {} started with {:?} delta", boss, delta));

                            // check self buffs
                            if self.no_self_food() {
                                self.reminder.trigger_food();

                                #[cfg(feature = "log")]
                                self.debug.log("Food reminder triggered on encounter start");
                            }
                            if self.no_self_util() {
                                self.reminder.trigger_util();

                                #[cfg(feature = "log")]
                                self.debug
                                    .log("Utility reminder triggered on encounter start");
                            }
                        } else {
                            #[cfg(feature = "log")]
                            self.debug.log(format!(
                                "Log for id {} started with {:?} delta",
                                target_id, delta
                            ))
                        }
                    }
                    StateChange::LogEnd => {
                        // log end

                        // grab target id
                        let target_id = event.src_agent;

                        // check for known boss
                        #[cfg_attr(not(feature = "log"), allow(unused))]
                        if let Ok(boss) = Boss::try_from(target_id) {
                            self.tracker.end_encounter();

                            #[cfg(feature = "log")]
                            self.debug.log(format!("Log for {} ended", boss));

                            // check self buffs
                            if self.no_self_food() {
                                self.reminder.trigger_food();

                                #[cfg(feature = "log")]
                                self.debug.log("Food reminder triggered on encounter end");
                            }
                            if self.no_self_util() {
                                self.reminder.trigger_util();

                                #[cfg(feature = "log")]
                                self.debug
                                    .log("Utility reminder triggered on encounter end");
                            }
                        } else {
                            #[cfg(feature = "log")]
                            self.debug.log(format!("Log for id {} ended", target_id));
                        }
                    }
                    _ => {
                        // TODO: should we restrict this to specific state change kinds?
                        // FIXME: tracking "nourishment" & "enhancement" buff names need adjustment for other client languages

                        let buff_remove = BuffRemove::from(event.is_buff_remove);
                        if buff_remove == BuffRemove::None {
                            if event.buff != 0 && event.buff_dmg == 0 {
                                // buff applied

                                // check for tracked player
                                if let Some(dest) = dest {
                                    if let Some(player) = self.tracker.player_mut(dest.id) {
                                        let buff_id = event.skill_id;

                                        // check for food & util
                                        if let Ok(food) = Food::try_from(buff_id) {
                                            if player.apply_food(food, event_id) {
                                                #[cfg(feature = "log")]
                                                self.debug.log(format!(
                                                    "Food {} applied to {:?}",
                                                    food, player
                                                ));
                                            }
                                        } else if let Ok(util) = Utility::try_from(buff_id) {
                                            if player.apply_util(util, event_id) {
                                                #[cfg(feature = "log")]
                                                self.debug.log(format!(
                                                    "Utility {} applied to {:?}",
                                                    util, player
                                                ));
                                            }
                                        } else if let Some("Nourishment") = skill_name {
                                            if player.apply_unknown_food(buff_id, event_id) {
                                                #[cfg(feature = "log")]
                                                self.debug.log(format!(
                                                    "Unknown Food with id {} applied to {:?}",
                                                    buff_id, player
                                                ));
                                            }
                                        } else if let Some("Enhancement") = skill_name {
                                            if player.apply_unknown_util(buff_id, event_id) {
                                                #[cfg(feature = "log")]
                                                self.debug.log(format!(
                                                    "Unknown Utility with id {} applied to {:?}",
                                                    buff_id, player
                                                ));
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            // buff removed

                            // check for tracked player
                            if let Some(player) = self.tracker.player_mut(src.id) {
                                let buff_id = event.skill_id;

                                // check for food & util
                                if let Ok(food) = Food::try_from(buff_id) {
                                    if player.remove_food(Some(food), event_id) {
                                        #[cfg(feature = "log")]
                                        self.debug.log(format!(
                                            "Food {:?} removed from {:?}",
                                            food, player
                                        ));

                                        // check for food running out during encounter
                                        if player.is_self
                                            && self.tracker.in_encounter()
                                            && self.no_self_food()
                                        {
                                            self.reminder.trigger_food();

                                            #[cfg(feature = "log")]
                                            self.debug
                                                .log("Food reminder triggered during encounter");
                                        }
                                    }
                                } else if let Ok(util) = Utility::try_from(buff_id) {
                                    if player.remove_util(Some(util), event_id) {
                                        #[cfg(feature = "log")]
                                        self.debug.log(format!(
                                            "Utility {:?} removed from {:?}",
                                            util, player
                                        ));

                                        // check for utility running out during encounter
                                        if player.is_self
                                            && self.tracker.in_encounter()
                                            && self.no_self_util()
                                        {
                                            self.reminder.trigger_util();

                                            #[cfg(feature = "log")]
                                            self.debug
                                                .log("Utility reminder triggered during encounter");
                                        }
                                    }
                                } else if let Some("Nourishment") = skill_name {
                                    if player.remove_food(None, event_id) {
                                        #[cfg(feature = "log")]
                                        self.debug.log(format!(
                                            "Unknown Food with id {} removed from {:?}",
                                            buff_id, player
                                        ));

                                        // check for food running out during encounter
                                        if player.is_self
                                            && self.tracker.in_encounter()
                                            && self.no_self_food()
                                        {
                                            self.reminder.trigger_food();

                                            #[cfg(feature = "log")]
                                            self.debug.log(
                                                "Unknown Food reminder triggered during encounter",
                                            );
                                        }
                                    }
                                } else if let Some("Enhancement") = skill_name {
                                    if player.remove_util(None, event_id) {
                                        #[cfg(feature = "log")]
                                        self.debug.log(format!(
                                            "Unknown Utility with id {} removed from {:?}",
                                            buff_id, player
                                        ));

                                        // check for utility running out during encounter
                                        if player.is_self
                                            && self.tracker.in_encounter()
                                            && self.no_self_util()
                                        {
                                            self.reminder.trigger_util();

                                            #[cfg(feature = "log")]
                                            self.debug
                                                .log("Unknown Utility reminder triggered during encounter");
                                        }
                                    }
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
                                self_: is_self,
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
                        if let Some(player) = removed {
                            self.debug.log(format!("Removed {:?}", player));
                        }
                    }
                }
            }
        }
    }

    /// Returns `true` if the local player (self) has no Food buff applied.
    fn no_self_food(&self) -> bool {
        match self.tracker.get_self() {
            Some(player) => matches!(
                player.food.state,
                BuffState::None | BuffState::Known(Food::Malnourished)
            ),
            None => false,
        }
    }

    /// Returns `true` if the local player (self) has no Utility buff applied.
    fn no_self_util(&self) -> bool {
        match self.tracker.get_self() {
            Some(player) => matches!(
                player.util.state,
                BuffState::None | BuffState::Known(Utility::Diminished)
            ),
            None => false,
        }
    }

    /// Handles a key event.
    pub fn key_event(&mut self, key: usize, down: bool, prev_down: bool) -> bool {
        // check for down
        if down && !prev_down {
            // check for hotkeys
            #[cfg_attr(
                not(any(feature = "demo", feature = "log")),
                allow(clippy::single_match)
            )]
            match key {
                TRACKER_KEY => {
                    self.tracker.toggle_visibility();
                    return false;
                }
                #[cfg(feature = "demo")]
                DEMO_KEY => {
                    self.demo.toggle_visibility();
                    return false;
                }
                #[cfg(feature = "log")]
                LOG_KEY => {
                    self.debug.toggle_visibility();
                    return false;
                }
                _ => {}
            }
        }
        true
    }

    /// Callback for standalone UI creation.
    pub fn render_windows(&mut self, ui: &Ui, not_loading: bool) {
        // log & demo render always
        #[cfg(feature = "demo")]
        self.demo.render(ui);

        #[cfg(feature = "log")]
        self.debug.render(ui);

        // other ui renders conditionally
        let ui_settings = exports::ui_settings();
        if !ui_settings.hidden && (not_loading || ui_settings.draw_always) {
            self.tracker.render(ui);
        }
    }

    /// Callback for ArcDPS option checkboxes.
    pub fn render_options(&mut self, ui: &Ui, option_name: Option<&str>) -> bool {
        if option_name.is_none() {
            ui.checkbox(im_str!("Food Tracker"), self.tracker.is_visible_mut());

            #[cfg(feature = "demo")]
            ui.checkbox(im_str!("Food Demo"), self.demo.is_visible_mut());

            #[cfg(feature = "log")]
            ui.checkbox(im_str!("Food Debug Log"), self.debug.is_visible_mut());
        }
        false
    }
}

impl Default for Plugin {
    fn default() -> Self {
        Self::new()
    }
}
