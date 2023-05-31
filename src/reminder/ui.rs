use super::{custom::CustomReminder, Reminder};
use arc_util::{
    colors::RED,
    ui::{render, Component},
};
use arcdps::{
    exports::{self, CoreColor},
    imgui::{self, Ui},
};
use imgui::Condition;
use std::time::{Duration, Instant};

// TODO: split component with custom text and add to arc_util

/// Font size used by the reminder.
const FONT_SIZE: f32 = 2.0;

impl Reminder {
    /// Checks if a trigger is currently active and resets it if necessary.
    fn update_trigger(trigger: &mut Option<Instant>, duration: Duration) -> bool {
        match trigger {
            Some(time) if Self::is_triggered(*time, duration) => true,
            Some(_) => {
                *trigger = None;
                false
            }
            None => false,
        }
    }

    /// Checks if a trigger is currently active.
    fn is_triggered(time: Instant, duration: Duration) -> bool {
        Instant::now().saturating_duration_since(time) <= duration
    }

    /// Helper to render text.
    fn render_text(ui: &Ui, text: &str) {
        // grab colors
        let colors = exports::colors();
        let red = colors.core(CoreColor::LightRed).unwrap_or(RED);

        // adjust cursor to center text
        let [cursor_x, cursor_y] = ui.cursor_pos();
        let [text_width, _] = ui.calc_text_size(text);
        let window_width = ui.window_content_region_width();
        ui.set_cursor_pos([cursor_x + 0.5 * (window_width - text_width), cursor_y]);

        // render text
        ui.text_colored(red, text);
    }

    /// Renders the custom reminder reset button.
    pub fn render_custom_reset(&mut self, ui: &Ui) {
        if render::reset_button(ui, "Reset custom", &mut self.custom_reset) {
            self.settings.custom = CustomReminder::defaults();
        }
    }
}

impl Component<()> for Reminder {
    fn render(&mut self, ui: &Ui, _: ()) {
        // update triggers
        let food = Self::update_trigger(&mut self.food_trigger, self.settings.duration);
        let util = Self::update_trigger(&mut self.util_trigger, self.settings.duration);
        self.custom_triggers
            .retain(|_, time| Self::is_triggered(*time, self.settings.duration));

        // check if any is triggered
        if food || util || !self.custom_triggers.is_empty() {
            // calculate window position
            let [screen_width, screen_height] = ui.io().display_size;

            // render "invisible" window with text
            imgui::Window::new("##food-reminder-reminder")
                .position(
                    [0.5 * screen_width, self.settings.position * screen_height],
                    Condition::Always,
                )
                .position_pivot([0.5, 0.5])
                .content_size([screen_width, 0.0])
                .always_auto_resize(true)
                .no_decoration()
                .draw_background(false)
                .no_inputs()
                .movable(false)
                .focus_on_appearing(false)
                .build(ui, || {
                    ui.set_window_font_scale(FONT_SIZE);

                    // food/util
                    match (food, util) {
                        (true, true) => Self::render_text(ui, "Food & Utility reminder!"),
                        (true, false) => Self::render_text(ui, "Food reminder!"),
                        (false, true) => Self::render_text(ui, "Utility reminder!"),
                        (false, false) => {}
                    }

                    // custom reminders
                    for id in self.custom_triggers.keys() {
                        if let Some(remind) = self.custom(*id) {
                            Self::render_text(ui, &format!("{} reminder!", remind.display_name()));
                        }
                    }
                });
        }
    }
}
