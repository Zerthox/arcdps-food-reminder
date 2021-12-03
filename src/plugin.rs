use crate::{
    data::{Boss, Food, Utility},
    reminder::Reminder,
    tracking::{
        player::{BuffState, Player},
        Encounter, Tracker,
    },
    win,
};
use arc_util::{
    api::{BuffRemove, StateChange},
    exports,
    settings::Settings,
    ui::{align::LeftAlign, Component, Hideable, Window, Windowed},
};
use arcdps::{
    imgui::{im_str, ImString, Ui},
    Agent, CombatEvent,
};
use std::{convert::TryFrom, time::Duration};

#[cfg(feature = "demo")]
use crate::demo::Demo;

#[cfg(feature = "log")]
use {arc_util::api, arc_util::ui::components::log::Log};

/// Plugin version.
const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Settings file name.
const SETTINGS_FILE: &str = "arcdps_food_reminder.json";

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
    debug: Window<Log>,
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
            debug: Log::new().windowed_with_name("Food Reminder Debug Log"),
        }
    }

    /// Loads the plugin.
    pub fn load(&mut self) {
        #[cfg(feature = "log")]
        self.debug.log("Plugin load");

        // load settings
        let mut settings = Settings::from_file(SETTINGS_FILE);

        #[cfg(feature = "log")]
        self.debug.log(format!("Loaded {:?}", settings));

        // load component settings
        settings.load_component(&mut self.tracker);
        settings.load_component(&mut self.reminder);

        #[cfg(feature = "demo")]
        settings.load_component(&mut self.demo);
    }

    /// Unloads the plugin.
    pub fn unload(&mut self) {
        let mut settings = Settings::from_file(SETTINGS_FILE);

        settings.store_data("version", VERSION);

        // update component settings
        settings.store_component(&self.tracker);
        settings.store_component(&self.reminder);

        #[cfg(feature = "demo")]
        settings.store_component(&self.demo);

        // save settings
        settings.save_file();
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
                        let boss = Boss::try_from(target_id).ok();
                        self.tracker.start_encounter(boss);

                        #[cfg(feature = "log")]
                        if let Some(boss) = boss {
                            self.debug
                                .log(format!("Log for {} started with {:?} delta", boss, delta));
                        } else {
                            self.debug.log(format!(
                                "Log for id {} started with {:?} delta",
                                target_id, delta
                            ))
                        }

                        // check self buffs
                        self.check_self_food();
                        self.check_self_util();
                    }
                    StateChange::LogEnd => {
                        // log end

                        #[cfg(feature = "log")]
                        {
                            // grab target id
                            let target_id = event.src_agent;

                            // check for known boss
                            if let Ok(boss) = Boss::try_from(target_id) {
                                self.debug.log(format!("Log for {} ended", boss));
                            } else {
                                self.debug.log(format!("Log for id {} ended", target_id));
                            }
                        }

                        // check self buffs
                        self.check_self_food();
                        self.check_self_util();

                        // end encounter
                        self.tracker.end_encounter();
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

                                        // check for food running out
                                        if player.is_self {
                                            self.check_self_food();
                                        }
                                    }
                                } else if let Ok(util) = Utility::try_from(buff_id) {
                                    if player.remove_util(Some(util), event_id) {
                                        #[cfg(feature = "log")]
                                        self.debug.log(format!(
                                            "Utility {:?} removed from {:?}",
                                            util, player
                                        ));

                                        // check for utility running out
                                        if player.is_self {
                                            self.check_self_util();
                                        }
                                    }
                                } else if let Some("Nourishment") = skill_name {
                                    if player.remove_food(None, event_id) {
                                        #[cfg(feature = "log")]
                                        self.debug.log(format!(
                                            "Unknown Food with id {} removed from {:?}",
                                            buff_id, player
                                        ));

                                        // check for food running out
                                        if player.is_self {
                                            self.check_self_food();
                                        }
                                    }
                                } else if let Some("Enhancement") = skill_name {
                                    if player.remove_util(None, event_id) {
                                        #[cfg(feature = "log")]
                                        self.debug.log(format!(
                                            "Unknown Utility with id {} removed from {:?}",
                                            buff_id, player
                                        ));

                                        // check for utility running out
                                        if player.is_self {
                                            self.check_self_util();
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

    /// Whether the local player can be reminded.
    fn can_remind(&self) -> bool {
        if self.reminder.settings.only_bosses {
            matches!(self.tracker.encounter(), Encounter::Boss(_))
        } else {
            self.tracker.in_encounter()
        }
    }

    /// Checks for missing food on the local player.
    fn check_self_food(&mut self) {
        if let Some(player) = self.tracker.get_self() {
            if self.can_remind()
                && matches!(
                    player.food.state,
                    BuffState::None | BuffState::Known(Food::Malnourished)
                )
            {
                self.reminder.trigger_food();

                #[cfg(feature = "log")]
                self.debug.log("Food reminder triggered");
            }
        }
    }

    /// Checks for missing utility on the local player.
    fn check_self_util(&mut self) {
        if let Some(player) = self.tracker.get_self() {
            if self.can_remind()
                && matches!(
                    player.util.state,
                    BuffState::None | BuffState::Known(Utility::Diminished)
                )
            {
                self.reminder.trigger_util();

                #[cfg(feature = "log")]
                self.debug.log("Utility reminder triggered");
            }
        }
    }

    /// Callback for standalone UI creation.
    pub fn render_windows(&mut self, ui: &Ui, not_loading: bool) {
        // reminder, log & demo render always
        self.reminder.render(ui);

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

    /// Callback for option UI creation.
    pub fn render_options(&mut self, ui: &Ui) {
        let mut align = LeftAlign::with_margin(10.0);
        ui.align_text_to_frame_padding();

        align.item(ui, || ui.text(im_str!("Tracker Hotkey")));

        align.item(ui, || {
            let mut key_buffer = ImString::with_capacity(3);
            key_buffer.push_str(
                &self
                    .tracker
                    .hotkey()
                    .map(|keycode| keycode.to_string())
                    .unwrap_or_default(),
            );

            ui.push_item_width(ui.calc_text_size(im_str!("0000"), false, 0.0)[0]);
            if ui
                .input_text(im_str!("##food-reminder-tracker-hotkey"), &mut key_buffer)
                .chars_decimal(true)
                .build()
            {
                let result = key_buffer.to_str();
                if result.is_empty() {
                    self.tracker.set_hotkey(None);
                } else if let Ok(keycode) = result.parse() {
                    self.tracker.set_hotkey(Some(keycode));
                }
            }
        });

        align.item(ui, || {
            let name = self
                .tracker
                .hotkey()
                .map(|keycode| win::keycode_to_name(keycode as u32))
                .flatten()
                .unwrap_or_default();
            ui.text(name);
        });

        ui.checkbox(
            im_str!("Remind only on bosses"),
            &mut self.reminder.settings.only_bosses,
        );
        ui.checkbox(
            im_str!("Remind on encounter start"),
            &mut self.reminder.settings.encounter_start,
        );
        ui.checkbox(
            im_str!("Remind on encounter end"),
            &mut self.reminder.settings.encounter_end,
        );
        ui.checkbox(
            im_str!("Remind during encounter"),
            &mut self.reminder.settings.during_encounter,
        );

        let mut duration_buffer = ImString::with_capacity(5);
        duration_buffer.push_str(&self.reminder.settings.duration.as_millis().to_string());

        ui.push_item_width(ui.calc_text_size(im_str!("000000"), false, 0.0)[0]);
        if ui
            .input_text(
                im_str!("Reminder duration (ms)##food-reminder-duration-input"),
                &mut duration_buffer,
            )
            .chars_decimal(true)
            .build()
        {
            if let Ok(num) = duration_buffer.to_str().parse() {
                self.reminder.settings.duration = Duration::from_millis(num);
            }
        }
    }

    /// Callback for ArcDPS option checkboxes.
    pub fn render_window_options(&mut self, ui: &Ui, option_name: Option<&str>) -> bool {
        if option_name.is_none() {
            ui.checkbox(im_str!("Food Tracker"), self.tracker.is_visible_mut());

            #[cfg(feature = "demo")]
            ui.checkbox(im_str!("Food Demo"), self.demo.is_visible_mut());

            #[cfg(feature = "log")]
            ui.checkbox(im_str!("Food Debug Log"), self.debug.is_visible_mut());
        }
        false
    }

    /// Handles a key event.
    pub fn key_event(&mut self, key: usize, down: bool, prev_down: bool) -> bool {
        // check for down
        if down && !prev_down {
            // check for hotkeys
            if Some(key) == self.tracker.hotkey() {
                self.tracker.toggle_visibility();
                return false;
            }
        }
        true
    }
}

impl Default for Plugin {
    fn default() -> Self {
        Self::new()
    }
}
