use crate::ui::Component;
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

    // button widths used for ui rendering
    toggle_button_width: f32,
    clear_button_width: f32,
    copy_button_width: f32,
}

impl DebugLog {
    pub fn new() -> Self {
        Self {
            active: true,
            contents: ImString::default(),
            size: 1, // imgui string has an implicit null at the end
            toggle_button_width: 60.0,
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
    type Props = ();

    fn create(_props: Self::Props) -> Self {
        Self::new()
    }

    fn render(&mut self, ui: &Ui) {
        // get window size
        let [window_width, _] = ui.window_content_region_max();

        // time
        ui.align_text_to_frame_padding();
        ui.text(format!("Time: {}", Local::now().format(FORMAT)));

        // activity toggle button
        ui.same_line(
            window_width
                - self.toggle_button_width
                - self.clear_button_width
                - self.clear_button_width
                - 5.0,
        );
        let toggle_button_text = if !self.active {
            im_str!("Enable")
        } else {
            im_str!("Disable")
        };
        if ui.button(toggle_button_text, [0.0, 0.0]) {
            self.active = !self.active;
        }
        self.toggle_button_width = ui.item_rect_size()[0];

        // copy button
        ui.same_line(window_width - self.copy_button_width - self.clear_button_width - 5.0);
        if ui.button(im_str!("Copy"), [0.0, 0.0]) {
            ui.set_clipboard_text(&self.contents);
        }
        self.copy_button_width = ui.item_rect_size()[0];

        // clear button
        ui.same_line(window_width - self.clear_button_width);
        if ui.button(im_str!("Clear"), [0.0, 0.0]) {
            self.clear();
        }
        self.clear_button_width = ui.item_rect_size()[0];

        ui.separator();

        // log contents
        ChildWindow::new(im_str!("food-reminder-log-scroller"))
            .scrollable(true)
            .horizontal_scrollbar(true)
            .build(ui, || {
                ui.text(&self.contents);
                ui.set_scroll_here_y_with_ratio(1.0);
            })
    }
}
