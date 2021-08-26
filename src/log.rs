use crate::{
    settings::HasSettings,
    ui::{
        align::RightAlign,
        window::{WindowProps, Windowed},
        Component,
    },
};
use arcdps::imgui::{im_str, ChildWindow, ImString, Ui};
use chrono::Local;

/// Time format used for debug messages.
const FORMAT: &str = "%b %d %H:%M:%S.%3f";

/// Debug log component.
#[derive(Debug, Clone)]
pub struct DebugLog {
    /// Current contents of the log.
    contents: ImString,

    /// Whether the log is active.
    active: bool,

    /// Last maximum scroll position.
    last_scroll_max: f32,

    // button widths used for ui rendering
    activity_toggle_width: f32,
    clear_button_width: f32,
    copy_button_width: f32,
}

impl DebugLog {
    /// Creates a new debug log.
    pub fn new() -> Self {
        Self {
            contents: ImString::default(),
            active: true,
            last_scroll_max: 0.0,
            activity_toggle_width: 60.0,
            clear_button_width: 60.0,
            copy_button_width: 60.0,
        }
    }

    /// Appends output to the debug log.
    pub fn log<S>(&mut self, output: S)
    where
        S: AsRef<str>,
    {
        if self.active {
            // generate line
            let now = Local::now();
            let line = format!("{}: {}\n", now.format(FORMAT), output.as_ref());

            // append line
            self.contents.push_str(&line);
        }
    }

    /// Clears the debug log.
    pub fn clear(&mut self) {
        self.contents.clear();
    }
}

impl Default for DebugLog {
    fn default() -> Self {
        Self::new()
    }
}

impl Component for DebugLog {
    fn render(&mut self, ui: &Ui) {
        // time
        ui.align_text_to_frame_padding();
        ui.text(format!("Time: {}", Local::now().format(FORMAT)));

        // buttons from right to left
        let mut align = RightAlign::build();

        // clear button
        let contents = &mut self.contents;
        align.item(ui, &mut self.clear_button_width, || {
            if ui.button(im_str!("Clear"), [0.0, 0.0]) {
                contents.clear();
            }
        });

        // copy button
        align.item(ui, &mut self.copy_button_width, || {
            if ui.button(im_str!("Copy"), [0.0, 0.0]) {
                ui.set_clipboard_text(contents);
            }
        });

        // activity toggle
        let active = &mut self.active;
        align.item_with_margin(ui, 10.0, &mut self.activity_toggle_width, || {
            ui.checkbox(im_str!("Active"), active);
        });

        ui.separator();

        // log contents
        ChildWindow::new(im_str!("##food-reminder-log-scroller"))
            .scrollable(true)
            .horizontal_scrollbar(true)
            .build(ui, || {
                // render text
                ui.text(&self.contents);

                // perform auto scroll
                if ui.scroll_y() == self.last_scroll_max {
                    ui.set_scroll_here_y_with_ratio(1.0);
                }

                // update last max
                self.last_scroll_max = ui.scroll_max_y();
            })
    }
}

impl Windowed for DebugLog {
    fn window_props() -> WindowProps {
        WindowProps::new("Food Debug Log")
            .visible(true)
            .width(600.0)
            .height(300.0)
    }
}

impl HasSettings for DebugLog {
    type Settings = ();
    fn settings_name() -> &'static str {
        "log"
    }
    fn get_settings(&self) -> Self::Settings {}
    fn load_settings(&mut self, _: Self::Settings) {}
}
