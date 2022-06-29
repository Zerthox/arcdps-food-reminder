pub mod settings;

use arc_util::{api::CoreColor, exports, ui::Component};
use arcdps::imgui::{self, Ui};
use imgui::Condition;
use settings::ReminderSettings;
use std::time::{Duration, Instant};

// TODO: alert component with custom text instead of this?

/// Font size used by the reminder.
const FONT_SIZE: f32 = 2.0;

/// Reminder UI component.
#[derive(Debug)]
pub struct Reminder {
    pub settings: ReminderSettings,
    food_trigger: Option<Instant>,
    util_trigger: Option<Instant>,
    reinf_trigger: Option<Instant>,
}

impl Reminder {
    /// Default duration used by the reminder.
    pub const DEFAULT_DURATION: Duration = Duration::from_secs(5);

    /// Creates a new reminder.
    pub const fn new() -> Self {
        Self {
            settings: ReminderSettings::new(),
            food_trigger: None,
            util_trigger: None,
            reinf_trigger: None,
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

    /// Triggers the reinforced reminder.
    pub fn trigger_reinforced(&mut self) {
        self.reinf_trigger = Some(Instant::now());
    }

    /// Checks if a trigger is currently active and resets it if necessary.
    fn check_trigger(trigger: &mut Option<Instant>, duration: Duration) -> bool {
        let now = Instant::now();
        match trigger {
            Some(time) if now.saturating_duration_since(*time) <= duration => true,
            Some(_) => {
                *trigger = None;
                false
            }
            None => false,
        }
    }

    /// Helper to render text.
    fn render_text(ui: &Ui, text: &str) {
        // grab colors
        let colors = exports::colors();
        let red = colors
            .core(CoreColor::LightRed)
            .unwrap_or([1.0, 0.0, 0.0, 1.0]);

        // adjust cursor to center text
        let [cursor_x, cursor_y] = ui.cursor_pos();
        let [text_width, _] = ui.calc_text_size(text);
        let window_width = ui.window_content_region_width();
        ui.set_cursor_pos([cursor_x + 0.5 * (window_width - text_width), cursor_y]);

        // render text
        ui.text_colored(red, text);
    }
}

impl Component<'_> for Reminder {
    type Props = ();

    fn render(&mut self, ui: &Ui, _props: &Self::Props) {
        // check for triggers
        let food = Self::check_trigger(&mut self.food_trigger, self.settings.duration);
        let util = Self::check_trigger(&mut self.util_trigger, self.settings.duration);
        let reinf = Self::check_trigger(&mut self.reinf_trigger, self.settings.duration);

        // check if any is triggered
        if food || util || reinf {
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
                    // font size
                    ui.set_window_font_scale(FONT_SIZE);

                    // render text
                    if food {
                        Self::render_text(ui, "Food reminder!");
                    }
                    if util {
                        Self::render_text(ui, "Utility reminder!");
                    }
                    if reinf {
                        Self::render_text(ui, "Reinforced reminder!");
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
