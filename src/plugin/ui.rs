use crate::{defs::Definitions, plugin::DEFINITIONS_FILE};

use super::Plugin;
use arc_util::{
    api::CoreColor,
    exports,
    settings::{HasSettings, Settings},
    ui::{render, Component, Hideable},
};
use arcdps::imgui::Ui;
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
        let grey = colors
            .core(CoreColor::MediumGrey)
            .unwrap_or([0.5, 0.5, 0.5, 1.0]);
        let green = colors
            .core(CoreColor::LightGreen)
            .unwrap_or([0.0, 1.0, 0.0, 1.0]);
        let yellow = colors
            .core(CoreColor::LightYellow)
            .unwrap_or([1.0, 1.0, 0.0, 1.0]);

        // use small padding
        let _style = render::small_padding(ui);

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

        // unofficial extras indicator
        ui.group(|| {
            ui.text("Unofficial extras (for subgroup updates):");
            ui.same_line();
            if self.extras {
                ui.text_colored(green, "Found");
            } else {
                ui.text_colored(yellow, "Missing");
            }
        });
        if ui.is_item_hovered() {
            ui.tooltip_text(
                "Unofficial extras allows for more frequent updates on player subgroups.",
            );
        }

        // reset buttons
        self.tracker.render_reset_buttons::<true>(ui);

        // reminder settings
        ui.spacing();
        ui.spacing();

        ui.text_colored(grey, "Reminder");

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
            ui.tooltip_text("Default boss list consists of Raids & Fractal CMs.\nYou can add more bosses in a custom definitions file.");
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
        let mut duration_buffer = String::with_capacity(5);
        duration_buffer.push_str(&self.reminder.settings.duration.as_millis().to_string());

        ui.set_next_item_width(render::ch_width(ui, 6));
        if ui
            .input_text("Reminder duration (ms)", &mut duration_buffer)
            .chars_decimal(true)
            .build()
        {
            if let Ok(num) = duration_buffer.parse() {
                self.reminder.settings.duration = Duration::from_millis(num);
            }
        }
        if ui.is_item_hovered() {
            ui.tooltip_text("How long the reminder is displayed on screen.");
        }

        ui.spacing();
        ui.spacing();
        ui.text_colored(grey, "Custom definitions");
        if ui.button("Reload definitions file") {
            if let Some(defs_path) = Settings::config_path(DEFINITIONS_FILE) {
                // try loading custom defs
                if self.defs.try_load(&defs_path).is_ok() {
                    #[cfg(feature = "log")]
                    self.debug.log(format!(
                        "Reloaded custom definitions from \"{}\"",
                        defs_path.display()
                    ));
                } else {
                    #[cfg(feature = "log")]
                    self.debug.log(format!(
                        "Failed to reload custom definitions from \"{}\"",
                        defs_path.display()
                    ));
                }
            }
        }

        ui.same_line_with_spacing(0.0, 5.0);
        if ui.button("Reset definitions") {
            self.defs = Definitions::with_defaults();
        }

        ui.spacing();
        ui.separator();
        ui.spacing();

        // reset button
        if ui.button("Reset to default") {
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

            #[cfg(feature = "log")]
            ui.checkbox("Food Debug Log", self.debug.visible_mut());
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
