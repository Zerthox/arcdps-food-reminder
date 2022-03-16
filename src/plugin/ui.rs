use super::Plugin;
use arc_util::{
    api::CoreColor,
    exports,
    settings::HasSettings,
    ui::{components::key_input, Component, Hideable},
};
use arcdps::imgui::Ui;
use arcdps_imgui::StyleVar;
use std::time::Duration;

impl Plugin {
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

    /// Callback for settings UI creation.
    pub fn render_settings(&mut self, ui: &Ui) {
        let colors = exports::colors();
        let green = match colors.core(CoreColor::LightGreen) {
            Some(vec) => vec.into(),
            None => [0.0, 1.0, 0.0, 1.0],
        };
        let yellow = match colors.core(CoreColor::LightYellow) {
            Some(vec) => vec.into(),
            None => [1.0, 1.0, 0.0, 1.0],
        };

        // use small padding similar to arc & other plugins
        let _style = ui.push_style_var(StyleVar::FramePadding([1.0, 1.0]));

        // tracker settings
        ui.spacing();
        ui.text_disabled("Tracker");

        // tracker save chars
        ui.checkbox(
            "Save own characters between game sessions",
            &mut self.tracker.settings.save_chars,
        );

        // tracker hotkey
        key_input(
            ui,
            "##hotkey",
            "Tracker Hotkey:",
            &mut self.tracker.settings.hotkey,
        );

        // unofficial extras indicator
        ui.text("Unofficial extras (for subgroup updates):");
        ui.same_line();
        if self.extras {
            ui.text_colored(green, "Found");
        } else {
            ui.text_colored(yellow, "Missing");
        }

        // reminder settings
        ui.spacing();
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
            if matches!(self.tracker.settings.hotkey, Some(hotkey) if hotkey as usize == key) {
                self.tracker.toggle_visibility();
                return false;
            }
        }
        true
    }
}
