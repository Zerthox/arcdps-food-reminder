use crate::ui::{
    align::RightAlign,
    window::{WindowProps, Windowed},
    Component,
};
use arcdps::imgui::{im_str, ChildWindow, ImString, Ui};
use chrono::Local;

/// Time format used for debug messages.
const FORMAT: &str = "%b %d %H:%M:%S.%3f";

/// Debug log component.
#[derive(Debug, Clone)]
pub struct DebugLog {
    /// Whether the log is active.
    active: bool,

    /// Current contents of the log.
    contents: ImString,

    /// Current size of contents string.
    size: usize,

    /// Alignment helper
    right_align: RightAlign,

    // button widths used for ui rendering
    toggle_width: f32,
    clear_button_width: f32,
    copy_button_width: f32,
}

impl DebugLog {
    /// Creates a new debug log.
    pub fn new() -> Self {
        Self {
            active: true,
            contents: ImString::default(),
            size: 1, // imgui string has an implicit null at the end
            right_align: RightAlign::new(),
            toggle_width: 60.0,
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

            // clear on overflow
            if let Some(new) = self.size.checked_add(line.len()) {
                self.size = new;
            } else {
                self.clear();
            }

            // append line
            self.contents.push_str(&line);
        }
    }

    /// Clears the debug log.
    pub fn clear(&mut self) {
        self.size = 1;
        self.contents.clear();
    }
}

impl Component for DebugLog {
    fn render(&mut self, ui: &Ui) {
        // time
        ui.align_text_to_frame_padding();
        ui.text(format!("Time: {}", Local::now().format(FORMAT)));

        // buttons from right to left
        let mut align = self.right_align.begin_render();

        // clear button
        align.next_item(ui, self.clear_button_width);
        if ui.button(im_str!("Clear"), [0.0, 0.0]) {
            self.clear();
        }
        self.clear_button_width = ui.item_rect_size()[0];

        // copy button
        align.next_item(ui, self.copy_button_width);
        if ui.button(im_str!("Copy"), [0.0, 0.0]) {
            ui.set_clipboard_text(&self.contents);
        }
        self.copy_button_width = ui.item_rect_size()[0];

        // activity toggle
        align.next_margin(10.0);
        align.next_item(ui, self.toggle_width);
        ui.checkbox(im_str!("Active"), &mut self.active);
        self.toggle_width = ui.item_rect_size()[0];

        ui.separator();

        // log contents
        ChildWindow::new(im_str!("##food-reminder-log-scroller"))
            .scrollable(true)
            .horizontal_scrollbar(true)
            .build(ui, || {
                ui.text(&self.contents);
                ui.set_scroll_here_y_with_ratio(1.0);
            })
    }
}

impl Windowed for DebugLog {
    fn props() -> WindowProps {
        WindowProps::new("Food Debug Log")
            .visible(true)
            .width(600.0)
            .height(300.0)
    }
}

impl Default for DebugLog {
    fn default() -> Self {
        Self::new()
    }
}
