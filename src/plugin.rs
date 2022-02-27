use crate::{
    data::{BOSSES, DIMINISHED, MALNOURISHED},
    defs::{BuffDef, Definitions},
    remind::Reminder,
    tracking::{buff::BuffState, Tracker},
    util,
};
use arc_util::{
    api::{BuffRemove, StateChange},
    exports,
    game::Player,
    settings::{HasSettings, Settings},
    ui::{align::LeftAlign, Component, Hideable, Window},
};
use arcdps::{imgui::Ui, Agent, CombatEvent};
use std::time::Duration;

#[cfg(feature = "demo")]
use crate::demo::Demo;

#[cfg(feature = "log")]
use arc_util::{api, ui::components::log::Log};

/// Plugin version.
const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Settings file name.
const SETTINGS_FILE: &str = "arcdps_food_reminder.json";

/// Definitions file name.
const DEFINITIONS_FILE: &str = "arcdps_food_reminder_definitions.json";

/// Main plugin.
#[derive(Debug)]
pub struct Plugin {
    /// Definitions.
    defs: Definitions,

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
            defs: Definitions::new(),
            reminder: Reminder::new(),
            tracker: Window::new("Food Tracker", Tracker::new()),

            #[cfg(feature = "demo")]
            demo: Window::new("Food Demo", Demo::new()),

            #[cfg(feature = "log")]
            debug: Window::new("Food Debug Log", Log::new()),
        }
    }

    /// Loads the plugin.
    pub fn load(&mut self) {
        #[cfg(feature = "log")]
        self.debug.log(format!("Food Reminder v{} load", VERSION));

        // load settings
        let mut settings = Settings::from_file(SETTINGS_FILE);

        #[cfg(feature = "log")]
        self.debug.log(format!("Loaded {:?}", settings));

        // load component settings
        settings.load_component(&mut self.tracker);
        settings.load_component(&mut self.reminder);

        #[cfg(feature = "demo")]
        settings.load_component(&mut self.demo);

        // load definitions
        self.defs.try_load(DEFINITIONS_FILE);
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
        // ignore events without source
        if let Some(src) = src {
            // check for combat event
            if let Some(event) = event {
                match event.is_statechange.into() {
                    StateChange::EnterCombat => {
                        // combat enter

                        if let Some(entry) = self.tracker.player_mut(src.id) {
                            entry.player.enter_combat(Some(event.dst_agent));

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
                        let delta = api::calc_delta(event);

                        // change unset buffs to none
                        // initial buffs should be reported right after
                        for entry in self.tracker.all_players_mut() {
                            entry.unset_to_none(event.time, event_id);
                        }

                        // start encounter
                        let target_id = event.src_agent;
                        self.tracker.start_encounter(target_id);

                        #[cfg(feature = "log")]
                        self.debug.log(format!(
                            "Log for id {} started with {:?} delta",
                            target_id, delta
                        ));

                        // check self buffs
                        // FIXME: need to wait for reports on initial buffs
                        if self.reminder.settings.encounter_start {
                            self.check_self_food();
                            self.check_self_util();
                        }
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
                                        if let Some(buff_type) = self.defs.get(buff_id) {
                                            match buff_type {
                                                BuffDef::Food(food) => {
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
                                                BuffDef::Util(util) => {
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
                                                BuffDef::Proc => {
                                                    #[cfg(feature = "log")]
                                                    self.debug.log(format!(
                                                        "Food proc {} applied to {:?}",
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
                                if let Some(buff_type) = self.defs.get(buff_id) {
                                    match buff_type {
                                        BuffDef::Food(food) => {
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
                                        BuffDef::Util(util) => {
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
                                        BuffDef::Proc => {
                                            #[cfg(feature = "log")]
                                            self.debug.log(format!(
                                                "Food proc {} removed from {:?}",
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
                        if let Some(entry) = removed {
                            self.debug.log(format!("Removed {:?}", entry));
                        }
                    }
                }
            }
        }
    }

    /// Whether the local player can be reminded.
    fn can_remind(&self) -> bool {
        match self.tracker.encounter() {
            Some(boss_id) => !self.reminder.settings.only_bosses || BOSSES.contains(&boss_id),
            None => false,
        }
    }

    /// Checks for missing food on the local player.
    fn check_self_food(&mut self) {
        if let Some(entry) = self.tracker.get_self() {
            if self.can_remind()
                && matches!(
                    entry.food.state,
                    BuffState::None | BuffState::Some(MALNOURISHED)
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
        if let Some(entry) = self.tracker.get_self() {
            if self.can_remind()
                && matches!(
                    entry.util.state,
                    BuffState::None | BuffState::Some(DIMINISHED)
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
        self.reminder.render(ui, &());

        #[cfg(feature = "demo")]
        self.demo.render(ui, &self.defs);

        #[cfg(feature = "log")]
        self.debug.render(ui, &());

        // other ui renders conditionally
        let ui_settings = exports::ui_settings();
        if !ui_settings.hidden && (not_loading || ui_settings.draw_always) {
            self.tracker.render(ui, &self.defs);
        }
    }

    /// Callback for option UI creation.
    pub fn render_options(&mut self, ui: &Ui) {
        // tracker settings
        ui.spacing();
        ui.text_disabled("Tracker");

        // tracker save chars
        ui.checkbox(
            "Save own characters between game sessions",
            &mut self.tracker.settings.save_chars,
        );

        // tracker hotkey
        let mut align = LeftAlign::with_margin(10.0);

        align.item(ui, || {
            ui.align_text_to_frame_padding();
            ui.text("Tracker Hotkey")
        });

        align.item(ui, || {
            let mut key_buffer = String::with_capacity(3);
            key_buffer.push_str(
                &self
                    .tracker
                    .settings
                    .hotkey
                    .map(|keycode| keycode.to_string())
                    .unwrap_or_default(),
            );

            ui.push_item_width(ui.calc_text_size("0000")[0]);
            if ui
                .input_text("##hotkey", &mut key_buffer)
                .chars_decimal(true)
                .build()
            {
                if key_buffer.is_empty() {
                    self.tracker.settings.hotkey = None;
                } else if let Ok(keycode) = key_buffer.parse() {
                    self.tracker.settings.hotkey = Some(keycode);
                }
            }
        });

        align.item(ui, || {
            let name = self
                .tracker
                .settings
                .hotkey
                .map(|keycode| util::keycode_to_name(keycode as u32))
                .flatten()
                .unwrap_or_default();
            ui.text(name);
        });

        // reminder settings
        ui.spacing();
        ui.text_disabled("Reminder");

        ui.checkbox(
            "Remind on encounter start",
            &mut self.reminder.settings.encounter_start,
        );
        ui.checkbox(
            "Remind on encounter end",
            &mut self.reminder.settings.encounter_end,
        );
        ui.checkbox(
            "Remind during encounter",
            &mut self.reminder.settings.during_encounter,
        );
        ui.checkbox(
            "Restrict reminders for encounters to Raids & Fractal CMs",
            &mut self.reminder.settings.only_bosses,
        );
        ui.checkbox(
            "Always remind when becoming Malnourished/Diminished",
            &mut self.reminder.settings.always_mal_dim,
        );

        // reminder duration
        let mut duration_buffer = String::with_capacity(5);
        duration_buffer.push_str(&self.reminder.settings.duration.as_millis().to_string());

        ui.push_item_width(ui.calc_text_size("000000")[0]);
        if ui
            .input_text("Reminder duration (ms)", &mut duration_buffer)
            .chars_decimal(true)
            .build()
        {
            if let Ok(num) = duration_buffer.parse() {
                self.reminder.settings.duration = Duration::from_millis(num);
            }
        }

        ui.spacing();
        ui.separator();
        ui.spacing();

        // reset button
        if ui.button("Reset to default") {
            self.tracker.reset_settings();
            self.reminder.reset_settings();
        }
    }

    /// Callback for ArcDPS option checkboxes.
    pub fn render_window_options(&mut self, ui: &Ui, option_name: Option<&str>) -> bool {
        if option_name.is_none() {
            ui.checkbox("Food Tracker", self.tracker.is_visible_mut());

            #[cfg(feature = "demo")]
            ui.checkbox("Food Demo", self.demo.is_visible_mut());

            #[cfg(feature = "log")]
            ui.checkbox("Food Debug Log", self.debug.is_visible_mut());
        }
        false
    }

    /// Handles a key event.
    pub fn key_event(&mut self, key: usize, down: bool, prev_down: bool) -> bool {
        // check for down
        if down && !prev_down {
            // check for hotkeys
            if Some(key) == self.tracker.settings.hotkey {
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
