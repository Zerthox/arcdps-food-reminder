use crate::{
    arc_util::{
        api::{BuffRemove, StateChange},
        exports,
        game::{Boss, Food, Utility},
    },
    tracking::{
        player::{Buff, Player},
        Tracker,
    },
    ui::{window::Window, Component},
    win::VirtualKey,
};
use arcdps::{
    imgui::{im_str, Ui},
    Agent, CombatEvent,
};
use std::convert::TryFrom;

#[cfg(feature = "log")]
use crate::{arc_util::api, log::DebugLog};

/// Hotkey for the tracker.
const TRACKER_KEY: usize = VirtualKey::F.0 as usize;

/// Hotkey for the log.
#[cfg(feature = "log")]
const LOG_KEY: usize = VirtualKey::L.0 as usize;

/// Main plugin instance.
#[derive(Debug)]
pub struct Plugin {
    /// Food tracker window.
    tracker: Window<Tracker>,

    /// Pressed keys.
    presses: KeyPresses,

    /// Debug log window.
    #[cfg(feature = "log")]
    debug: Window<DebugLog>,
}

impl Plugin {
    /// Creates a new plugin.
    pub fn new() -> Self {
        Self {
            // tracker window
            tracker: Window::<Tracker>::with_default("Food Tracker")
                .visible(false)
                .auto_resize(true),
            presses: KeyPresses::default(),

            #[cfg(feature = "log")]
            debug: Window::<DebugLog>::with_default("Food Debug Log")
                .visible(true)
                .width(600.0)
                .height(300.0),
        }
    }

    /// Loads the plugin.
    pub fn load(&mut self) {
        #[cfg(feature = "log")]
        self.debug.log("Plugin load");
    }

    /// Unloads the plugin.
    pub fn unload(&mut self) {}

    /// Handles a combat event.
    pub fn combat_event(
        &mut self,
        event: Option<&CombatEvent>,
        src: Option<Agent>,
        dest: Option<Agent>,
        skill_name: Option<&str>,
        _id: u64,
        _revision: u64,
    ) {
        // source should always be set, but we dont want to cause a crash
        if let Some(src) = src {
            // check for combat event
            if let Some(event) = event {
                match event.is_statechange.into() {
                    StateChange::EnterCombat => {
                        // combat enter

                        if let Some(player) = self.tracker.get_player_mut(src.id) {
                            player.enter_combat(Some(event.dst_agent));

                            #[cfg(feature = "log")]
                            self.debug.log(format!("Combat enter for {:?}", player));
                        }
                    }
                    StateChange::ExitCombat => {
                        // combat exit

                        if let Some(player) = self.tracker.get_player_mut(src.id) {
                            player.exit_combat();

                            #[cfg(feature = "log")]
                            self.debug.log(format!("Combat exit for {:?}", player));
                        }
                    }
                    StateChange::LogStart => {
                        // log start

                        #[cfg(feature = "log")]
                        let delta = api::calc_delta(event);

                        for player in self.tracker.get_players_mut() {
                            // initial buffs should be reported right after
                            player.unset_to_none();
                        }

                        // check for known boss
                        let target_id = event.src_agent;

                        #[cfg_attr(not(feature = "log"), allow(unused))]
                        if let Ok(boss) = Boss::try_from(target_id) {
                            #[cfg(feature = "log")]
                            self.debug
                                .log(format!("Log for {} started with {:?} delta", boss, delta));

                            // check self buffs
                            self.check_self_buffs();
                        } else {
                            #[cfg(feature = "log")]
                            self.debug.log(format!(
                                "Log for {} started with {:?} delta",
                                target_id, delta
                            ))
                        }
                    }
                    StateChange::LogEnd => {
                        // log end

                        // check for known boss
                        let target_id = event.src_agent;

                        #[cfg_attr(not(feature = "log"), allow(unused))]
                        if let Ok(boss) = Boss::try_from(target_id) {
                            #[cfg(feature = "log")]
                            self.debug.log(format!("Log for {} ended", boss));

                            // check self buffs
                            self.check_self_buffs();
                        } else {
                            #[cfg(feature = "log")]
                            self.debug.log(format!("Log for {} ended", target_id));
                        }
                    }
                    _ => {
                        // TODO: should we restrict this to specific state change kinds?
                        // TODO: can we reliably set unset to none on strike damage?
                        // FIXME: tracking "nourishment" & "enhancement" buff names may need adjustment for other client languages

                        let buff_remove = BuffRemove::from(event.is_buff_remove);
                        if buff_remove == BuffRemove::None {
                            if event.buff != 0 && event.buff_dmg == 0 {
                                // buff applied

                                // check for tracked player
                                if let Some(dest) = dest {
                                    if let Some(player) = self.tracker.get_player_mut(dest.id) {
                                        let buff_id = event.skill_id;

                                        // check for food & util
                                        if let Ok(food) = Food::try_from(buff_id) {
                                            player.apply_food(food);

                                            #[cfg(feature = "log")]
                                            self.debug.log(format!(
                                                "Food {} applied to {:?}",
                                                food, player
                                            ));
                                        } else if let Ok(util) = Utility::try_from(buff_id) {
                                            player.apply_util(util);

                                            #[cfg(feature = "log")]
                                            self.debug.log(format!(
                                                "Utility {} applied to {:?}",
                                                util, player
                                            ));
                                        } else if let Some("Nourishment") = skill_name {
                                            player.apply_unknown_food();

                                            #[cfg(feature = "log")]
                                            self.debug.log(format!(
                                                "Unknown Food with id {} applied to {:?}",
                                                buff_id, player
                                            ));
                                        } else if let Some("Enhancement") = skill_name {
                                            player.apply_unknown_util();

                                            #[cfg(feature = "log")]
                                            self.debug.log(format!(
                                                "Unknown Utility with id {} applied to {:?}",
                                                buff_id, player
                                            ));
                                        }
                                    }
                                }
                            }
                        } else {
                            // buff removed

                            // check for tracked player
                            if let Some(player) = self.tracker.get_player_mut(src.id) {
                                let buff_id = event.skill_id;

                                // check for food & util
                                if let Ok(food) = Food::try_from(buff_id) {
                                    // check for same as applied
                                    if player.food == Buff::Known(food) {
                                        player.remove_food();

                                        #[cfg(feature = "log")]
                                        self.debug.log(format!(
                                            "Food {:?} removed from {:?}",
                                            food, player
                                        ));
                                    }
                                } else if let Ok(util) = Utility::try_from(buff_id) {
                                    // check for same as applied
                                    if player.util == Buff::Known(util) {
                                        player.remove_util();

                                        #[cfg(feature = "log")]
                                        self.debug.log(format!(
                                            "Utility {:?} removed from {:?}",
                                            util, player
                                        ));
                                    }
                                } else if let Some("Nourishment") = skill_name {
                                    // check for same as applied
                                    if player.food == Buff::Unknown {
                                        player.remove_food();

                                        #[cfg(feature = "log")]
                                        self.debug.log(format!(
                                            "Unknown Food with id {} removed from {:?}",
                                            buff_id, player
                                        ));
                                    }
                                } else if let Some("Enhancement") = skill_name {
                                    // check for same as applied
                                    if player.util == Buff::Unknown {
                                        player.remove_util();

                                        #[cfg(feature = "log")]
                                        self.debug.log(format!(
                                            "Unknown Utility with id {} removed from {:?}",
                                            buff_id, player
                                        ));
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
                            let acc_name = dest_name.strip_prefix(":").unwrap_or(dest_name);
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

    /// Checks for Malnourished/Diminished on the local player (self).
    fn check_self_buffs(&mut self) {
        if let Some(player) = self.tracker.get_self() {
            if player.food == Buff::Known(Food::Malnourished) {
                #[cfg(feature = "log")]
                self.debug.log("Food reminder for self!");
            }
            if player.util == Buff::Known(Utility::Diminished) {
                #[cfg(feature = "log")]
                self.debug.log("Utility reminder for self!");
            }
        }
    }

    /// Handles a key event.
    pub fn key_event(&mut self, key: usize, down: bool, prev_down: bool) -> bool {
        // check for change
        if down != prev_down {
            let modifiers = exports::get_modifiers();

            // check for modifiers
            if key == modifiers.modifier1 as usize {
                self.presses.modifier1 = down;
            } else if key == modifiers.modifier2 as usize {
                self.presses.modifier2 = down;
            } else if down && self.modifiers_pressed() {
                // check for hotkeys
                match key {
                    TRACKER_KEY => {
                        self.tracker.toggle_visibility();
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
        }

        true
    }

    /// Checks whether both modifiers are currently pressed.
    fn modifiers_pressed(&self) -> bool {
        self.presses.modifier1 && self.presses.modifier2
    }

    /// Callback for standalone UI creation.
    pub fn render_windows(&mut self, ui: &Ui, not_loading: bool) {
        // log renders always
        #[cfg(feature = "log")]
        self.debug.render(ui);

        // other ui renders conditionally
        let ui_settings = exports::get_ui_settings();
        if !ui_settings.hidden && (not_loading || ui_settings.draw_always) {
            self.tracker.render(ui);
        }
    }

    /// Callback for ArcDPS option checkboxes.
    pub fn render_options(&mut self, ui: &Ui, option_name: Option<&str>) -> bool {
        if option_name.is_none() {
            ui.checkbox(im_str!("Food Tracker"), &mut self.tracker.shown);

            #[cfg(feature = "log")]
            ui.checkbox(im_str!("Food Reminder Debug Log"), &mut self.debug.shown);
        }
        false
    }
}

impl Default for Plugin {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct KeyPresses {
    modifier1: bool,
    modifier2: bool,
}
