use crate::{
    arc_util::{
        api::{BuffRemove, StateChange},
        exports,
        game::{Food, Utility},
    },
    tracking::{player::Player, Tracker},
    ui::{
        window::{Window, WindowProps},
        Component,
    },
};
use arcdps::{
    imgui::{im_str, Ui},
    Agent, CombatEvent,
};
use std::convert::TryFrom;

#[cfg(feature = "log")]
use crate::{arc_util::api, log::DebugLog};

/// Main plugin instance.
#[derive(Debug)]
pub struct Plugin {
    /// Food tracker window.
    tracker: Window<Tracker>,

    #[cfg(feature = "log")]
    debug: Window<DebugLog>,
}

impl Plugin {
    /// Creates a new plugin.
    pub fn new() -> Self {
        Self {
            // tracker window
            tracker: Window::create((
                WindowProps::new("Food Tracker")
                    .visible(false)
                    .auto_resize(true),
                (),
            )),

            #[cfg(feature = "log")]
            debug: Window::create((
                WindowProps::new("Food Reminder Debug Log").visible(true),
                (),
            )),
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
                        #[cfg(feature = "log")]
                        let delta = api::calc_delta(event);

                        if let Some(player) = self.tracker.get_player_mut(src.id) {
                            // FIXME: is this the right subgroup?
                            player.enter_combat(Some(event.dst_agent));

                            #[cfg(feature = "log")]
                            {
                                self.debug.log(format!("Combat enter for {:?}", player));
                                self.debug.log(format!("Delta to combat {:?}", delta));
                            }
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
                    _ => {
                        let buff_remove = BuffRemove::from(event.is_buff_remove);
                        if buff_remove == BuffRemove::None {
                            if event.buff != 0 && event.buff_dmg == 0 {
                                // buff applied

                                // check for tracker player
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

                                // check for food &
                                if Food::try_from(buff_id).is_ok() {
                                    player.remove_food();

                                    #[cfg(feature = "log")]
                                    self.debug.log(format!("Food removed from {:?}", player));
                                } else if Utility::try_from(buff_id).is_ok() {
                                    player.remove_util();

                                    #[cfg(feature = "log")]
                                    self.debug.log(format!("Utility removed from {:?}", player));
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
                                ..
                            }),
                        ) = (src.name, dest)
                        {
                            let acc_name = dest_name.strip_prefix(":").unwrap_or(dest_name);
                            let player = Player::new(
                                src.id,
                                char_name,
                                acc_name,
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

                        #[cfg_attr(not(feature = "log"), allow(unused))]
                        let removed = self.tracker.remove_player(src.id);

                        #[cfg(feature = "log")]
                        if let Some(player) = removed {
                            self.debug.log(format!("Removed {:?}", player));
                        }
                    }
                }
            }
        }
    }

    /// Handles a key event.
    pub fn key_event(&mut self, _key: usize, _down: bool, _prev_down: bool) -> bool {
        // TODO: open/close tracker window
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
