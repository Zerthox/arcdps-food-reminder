use crate::{
    arc_util::{
        api::{BuffRemove, StateChange},
        game::{Food, Utility},
    },
    log::DebugLog,
    tracking::{player::Player, Tracker},
    ui::{Component, Window, WindowProps},
    win,
};
use arcdps::{
    imgui::{im_str, Ui},
    Agent, CombatEvent,
};
use std::{convert::TryFrom, time::Duration};

/// Main plugin struct.
#[derive(Debug)]
pub struct Plugin {
    debug: Window<DebugLog>,
    tracker: Window<Tracker>,
}

impl Plugin {
    /// Creates a new plugin struct.
    pub fn new() -> Self {
        Self {
            debug: Window::create((
                WindowProps::new("Food Reminder Debug Log").visible(true),
                (),
            )),
            tracker: Window::create((
                WindowProps::new("Food Reminder")
                    .visible(false)
                    .auto_resize(true),
                (),
            )),
        }
    }

    /// Loads the plugin.
    pub fn load(&mut self) {
        self.debug.log("Load");
    }

    /// Unloads the plugin.
    pub fn unload(&self) {}

    /// Handles a combat event.
    pub fn combat_event(
        &mut self,
        event: Option<&CombatEvent>,
        src: Option<Agent>,
        dest: Option<Agent>,
        _skill_name: Option<&str>,
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

                        let now = Duration::from_millis(unsafe { win::timeGetTime() } as u64);
                        let event_time = Duration::from_millis(event.time);
                        let delta = now.saturating_sub(event_time);

                        if let Some(player) = self.tracker.get_player_mut(src.id) {
                            // TODO: update subgroup
                            player.enter_combat();
                            self.debug.log(format!("Combat enter for {:?}", player));
                            self.debug.log(format!(
                                "Delta {:?}, received time {:?} at {:?}",
                                delta, event_time, now,
                            ));
                        }
                    }
                    StateChange::ExitCombat => {
                        // combat exit

                        if let Some(player) = self.tracker.get_player_mut(src.id) {
                            player.exit_combat();
                            self.debug.log(format!("Combat exit for {:?}", player));
                        }
                    }
                    _ => {
                        let buff_remove = BuffRemove::from(event.is_buff_remove);
                        if buff_remove == BuffRemove::None {
                            if event.buff != 0 && event.buff_dmg == 0 {
                                // buff apply

                                // check for tracker player
                                if let Some(dest) = dest {
                                    if let Some(player) = self.tracker.get_player(dest.id) {
                                        let buff_id = event.skill_id;

                                        // check for food & util
                                        if let Ok(food) = Food::try_from(buff_id) {
                                            if food == Food::Malnourished {
                                                self.debug.log(format!(
                                                    "Malnourished applied to {:?}",
                                                    player
                                                ));
                                            } else {
                                                self.debug.log(format!(
                                                    "Food {} applied to {:?}",
                                                    food, player
                                                ));
                                            }
                                        } else if let Ok(util) = Utility::try_from(buff_id) {
                                            if util == Utility::Diminished {
                                                self.debug.log(format!(
                                                    "Diminished applied to {:?}",
                                                    player
                                                ));
                                            } else {
                                                self.debug.log(format!(
                                                    "Utility {} applied to {:?}",
                                                    util, player
                                                ));
                                            }
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
                                team: sub,
                                prof,
                                ..
                            }),
                        ) = (src.name, dest)
                        {
                            let acc_name = dest_name.strip_prefix(":").unwrap_or(dest_name);
                            let player = Player::new(src.id, char_name, acc_name, prof.into(), sub);
                            self.debug.log(format!("Added player {:?}", player));
                            self.tracker.add_player(player);
                        }
                    } else {
                        // player removed

                        if let Some(player) = self.tracker.remove_player(src.id) {
                            self.debug.log(format!("Removed player {:?}", player));
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

    /// Callback for standalone UI creation.
    pub fn render_windows(&mut self, ui: &Ui, not_loading: bool) {
        // log renders always
        self.debug.render(ui);

        // other ui renders conditionally
        // TODO: respect arc settings
        if not_loading {
            self.tracker.render(ui);
        }
    }

    /// Callback for ArcDPS option checkboxes.
    pub fn render_options(&mut self, ui: &Ui, option_name: Option<&str>) -> bool {
        if option_name.is_none() {
            ui.checkbox(im_str!("Food Reminder"), &mut self.tracker.shown);
            ui.checkbox(im_str!("Food Reminder Debug Log"), &mut self.debug.shown);
        }
        false
    }
}
