use crate::{
    arc_util::{api::CoreColor, exports},
    ui::Component,
};
use arcdps::imgui::{im_str, Condition, ImStr, Ui, Window};
use std::time::{Duration, Instant};

// TODO: alert component with custom text instead of this?

/// Default duration used by the reminder.
pub const DEFAULT_DURATION: Duration = Duration::from_secs(5);

/// Font size used by the reminder.
const FONT_SIZE: f32 = 2.0;

/// Reminder UI component.
#[derive(Debug)]
pub struct Reminder {
    duration: Duration,
    food_trigger: Option<Instant>,
    util_trigger: Option<Instant>,
}

impl Reminder {
    /// Creates a new reminder with the default duration.
    pub fn new() -> Self {
        Self::with_duration(DEFAULT_DURATION)
    }

    /// Creates a new reminder with a custom duration.
    pub fn with_duration(duration: Duration) -> Self {
        Self {
            duration,
            food_trigger: None,
            util_trigger: None,
        }
    }

    /// Triggers the food reminder.
    pub fn trigger_food(&mut self) {
        self.food_trigger = Some(Instant::now());
    }

    /// Triggers the utility reminder.
    pub fn trigger_util(&mut self) {
        self.util_trigger = Some(Instant::now());
    }

    /// Helper to render text.
    fn render_text(ui: &Ui, text: &ImStr) {
        // grab colors
        let colors = exports::colors();
        let red = colors
            .core(CoreColor::LightRed)
            .map(|vec| vec.into())
            .unwrap_or([1.0, 0.0, 0.0, 1.0]);

        // adjust cursor to center text
        let [cursor_x, cursor_y] = ui.cursor_pos();
        let [text_width, _] = ui.calc_text_size(text, false, 0.0);
        let window_width = ui.window_content_region_width();
        ui.set_cursor_pos([cursor_x + 0.5 * (window_width - text_width), cursor_y]);

        // render text
        ui.text_colored(red, text);
    }
}

impl Component for Reminder {
    fn render(&mut self, ui: &Ui) {
        let now = Instant::now();

        // check for food trigger
        let food = match self.food_trigger {
            Some(time) if now.saturating_duration_since(time) <= self.duration => true,
            Some(_) => {
                self.food_trigger = None;
                false
            }
            None => false,
        };

        // check for util trigger
        let util = match self.util_trigger {
            Some(time) if now.saturating_duration_since(time) <= self.duration => true,
            Some(_) => {
                self.util_trigger = None;
                false
            }
            None => false,
        };

        // check if either is triggered
        if food || util {
            // calculate window position
            let [screen_width, screen_height] = ui.io().display_size;

            // render "invisible" window with text
            Window::new(im_str!("##food-reminder-reminder"))
                .position([0.5 * screen_width, 0.2 * screen_height], Condition::Always)
                .position_pivot([0.5, 0.5])
                .content_size([screen_width, 0.0])
                .always_auto_resize(true)
                .no_decoration()
                .draw_background(false)
                .no_inputs()
                .movable(false)
                .focus_on_appearing(false)
                .build(ui, || {
                    // font size
                    ui.set_window_font_scale(FONT_SIZE);

                    // render food text
                    if food {
                        Self::render_text(ui, im_str!("Food reminder!"));
                    }

                    // render util text
                    if util {
                        Self::render_text(ui, im_str!("Utility reminder!"));
                    }
                });
        }
    }
}

impl Default for Reminder {
    fn default() -> Self {
        Self::new()
    }
}
