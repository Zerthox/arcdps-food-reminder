use super::Plugin;
use crate::{
    combo_ui::render_enum_combo,
    data::{Definitions, LoadError},
    plugin::{ExtrasState, DEFINITIONS_FILE},
    reminder::custom::CustomReminder,
};
use arc_util::{
    settings::{HasSettings, Settings},
    ui::{action::Action, render, Component, Hideable},
};
use arcdps::{
    exports::{self, CoreColor},
    imgui::{InputTextFlags, Ui},
};
use log::{info, warn};
use std::time::Duration;

impl Plugin {
    /// Callback for standalone UI creation.
    pub fn render_windows(&mut self, ui: &Ui, not_loading: bool) {
        // reminder, log & demo render always
        self.reminder.render(ui, ());

        #[cfg(feature = "demo")]
        self.demo
            .render(ui, (&self.defs, self.reminder.all_custom()));

        // other ui renders conditionally
        let ui_settings = exports::ui_settings();
        if !ui_settings.hidden && (not_loading || ui_settings.draw_always) {
            self.tracker
                .render(ui, (&self.defs, self.reminder.all_custom()));
        }
    }

    /// Callback for settings UI creation.
    // TODO: split settings UI into components
    pub fn render_settings(&mut self, ui: &Ui) {
        let colors = exports::colors();
        let grey = colors
            .core(CoreColor::MediumGrey)
            .unwrap_or([0.5, 0.5, 0.5, 1.0]);
        let red = colors
            .core(CoreColor::LightRed)
            .unwrap_or([1.0, 0.0, 0.0, 1.0]);
        let green = colors
            .core(CoreColor::LightGreen)
            .unwrap_or([0.0, 1.0, 0.0, 1.0]);
        let yellow = colors
            .core(CoreColor::LightYellow)
            .unwrap_or([1.0, 1.0, 0.0, 1.0]);

        const SPACING: f32 = 5.0;

        // use small padding
        let _style = render::small_padding(ui);

        let input_width = render::ch_width(ui, 16);

        // tracker settings
        ui.spacing();
        ui.text_colored(grey, "Tracker");

        // tracker save chars
        ui.checkbox(
            "Save own characters between game sessions",
            &mut self.tracker.settings.save_chars,
        );

        // tracker hotkey
        render::input_key(
            ui,
            "##hotkey",
            "Tracker Hotkey:",
            &mut self.tracker.settings.hotkey,
        );

        ui.group(|| {
            ui.text("Unofficial extras:");
            ui.same_line();
            match self.extras {
                ExtrasState::Missing => ui.text_colored(yellow, "Missing"),
                ExtrasState::Incompatible => ui.text_colored(red, "Incompatible"),
                ExtrasState::Found => ui.text_colored(green, "Found"),
            }
        });
        if ui.is_item_hovered() {
            ui.tooltip_text(
                "Unofficial extras allows for more frequent updates on player subgroups.",
            );
        }

        // reset buttons
        self.tracker.render_reset_buttons(ui, true);

        // reminder settings
        ui.spacing();
        ui.spacing();
        ui.text_colored(grey, "Reminder");

        ui.checkbox("Remind for Food buff", &mut self.reminder.settings.food);
        ui.checkbox("Remind for Utility buff", &mut self.reminder.settings.util);

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
            "Restrict reminders for encounters to bosses",
            &mut self.reminder.settings.only_bosses,
        );
        if ui.is_item_hovered() {
            ui.tooltip_text("Only remind for the default & custom bosses set in Arc.");
        }

        ui.checkbox(
            "Always remind on Malnourished/Diminished",
            &mut self.reminder.settings.always_mal_dim,
        );
        if ui.is_item_hovered() {
            ui.tooltip_text(
                "Makes the reminder always trigger when Malnourished or Diminished is detected.",
            );
        }

        // reminder duration
        let mut dura = self.reminder.settings.duration.as_millis() as i32;
        ui.set_next_item_width(input_width);
        if ui
            .input_int("Duration (ms)", &mut dura)
            .step(100)
            .step_fast(1000)
            .build()
        {
            self.reminder.settings.duration = Duration::from_millis(dura.max(0) as u64);
        }
        if ui.is_item_hovered() {
            ui.tooltip_text("How long the reminder is displayed on screen.");
        }

        // reminder position
        let mut pos = self.reminder.settings.position * 100.0;
        ui.set_next_item_width(input_width);
        if render::input_float_with_format(
            "Position (%)",
            &mut pos,
            1.0,
            10.0,
            "%.1f",
            InputTextFlags::empty(),
        ) {
            self.reminder.settings.position = pos / 100.0;
        }
        if ui.is_item_hovered() {
            ui.tooltip_text("Vertical position of the reminder displayed on screen.");
        }

        // test button
        if ui.button("Test reminder") {
            self.reminder.trigger_all();
        }

        ui.spacing();
        ui.spacing();
        ui.text_colored(grey, "Custom reminders");

        let custom = &mut self.reminder.settings.custom;
        let mut action = Action::new();
        let len = custom.len();

        for (i, remind) in custom.iter_mut().enumerate() {
            ui.checkbox(format!("##custom-active-{i}"), &mut remind.active);

            let mut id = remind.id.try_into().unwrap_or(0);
            ui.same_line();
            ui.set_next_item_width(render::ch_width(ui, 7));
            if ui.input_int("##custom-id-{i}", &mut id).step(0).build() {
                remind.id = id.max(0) as u32;
            }

            ui.same_line();
            ui.set_next_item_width(input_width);
            ui.input_text("##custom-name-{i}", &mut remind.name).build();

            ui.same_line();
            ui.set_next_item_width(70.0);
            render_enum_combo(ui, format!("##custom-mode-{i}"), &mut remind.mode);

            // action buttons
            ui.same_line();
            action.render_buttons(ui, "custom-actions", i, len);
        }
        action.perform(custom);

        if ui.button("Add##custom") {
            custom.push(CustomReminder::empty());
        }
        ui.same_line();
        self.reminder.render_custom_reset(ui);

        ui.spacing();
        ui.spacing();

        ui.text_colored(grey, "Custom definitions");
        ui.text("Status:");
        ui.same_line();
        match self.defs_state {
            Ok(()) => ui.text_colored(green, "Loaded"),
            Err(LoadError::NotFound) => ui.text_colored(yellow, "Not found"),
            Err(LoadError::FailedToRead) => ui.text_colored(red, "Failed to read file"),
            Err(LoadError::InvalidJSON) => ui.text_colored(red, "Failed to parse JSON"),
        }

        if ui.button("Reload definitions file") {
            if let Some(defs_path) = Settings::config_path(DEFINITIONS_FILE) {
                // try loading custom defs
                self.defs_state = self.defs.try_load(&defs_path);

                if self.defs_state.is_ok() {
                    info!(
                        "Reloaded custom definitions from \"{}\"",
                        defs_path.display()
                    );
                } else {
                    warn!(
                        "Failed to reload custom definitions from \"{}\"",
                        defs_path.display()
                    );
                }
            }
        }

        ui.same_line_with_spacing(0.0, SPACING);
        if ui.button("Reset definitions") {
            self.defs = Definitions::with_defaults();
            self.defs_state = Err(LoadError::NotFound);
        }

        ui.spacing();
        ui.separator();
        ui.spacing();

        // reset button
        if render::reset_button(ui, "Reset to default", &mut self.reset_confirm) {
            self.tracker.reset_settings();
            self.reminder.reset_settings();
        }

        #[cfg(feature = "demo")]
        self.refresh_demo_settings();
    }

    /// Callback for ArcDPS option checkboxes.
    pub fn render_window_options(&mut self, ui: &Ui, option_name: Option<&str>) -> bool {
        if option_name.is_none() {
            ui.checkbox("Food Tracker", self.tracker.visible_mut());

            #[cfg(feature = "demo")]
            ui.checkbox("Food Demo", self.demo.visible_mut());
        }
        false
    }

    /// Handles a key event.
    pub fn key_event(&mut self, key: usize, down: bool, prev_down: bool) -> bool {
        // check for down
        if down && !prev_down {
            // check for hotkeys
            if matches!(self.tracker.settings.hotkey, Some(hotkey) if hotkey as usize == key) {
                self.tracker.toggle_visibility();
                return false;
            }
        }
        true
    }
}
