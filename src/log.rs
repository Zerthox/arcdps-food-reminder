use crate::ui::Component;
use arcdps::imgui::{im_str, ChildWindow, ImString, Ui};
use chrono::Local;

/// Time format used for debug messages.
const FORMAT: &str = "%b %d %H:%M:%S.%3f";

/// Debug log component.
#[derive(Debug, Clone)]
pub struct DebugLog {
    contents: ImString,
    size: usize,
    clear_button_width: f32,
    copy_button_width: f32,
}

impl DebugLog {
    pub fn new() -> Self {
        Self {
            contents: ImString::default(),
            size: 1, // imgui string has an implicit null at the end
            clear_button_width: 60.0,
            copy_button_width: 60.0,
        }
    }

    /// Appends output to the debug log.
    pub fn log<S>(&mut self, output: S)
    where
        S: AsRef<str>,
    {
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
        ui.align_text_to_frame_padding();

        // time
        ui.text(format!("Time: {}", Local::now().format(FORMAT)));

        // get window size
        let [window_size_x, _] = ui.window_content_region_max();

        // clear button
        ui.same_line(window_size_x - self.clear_button_width - self.copy_button_width - 5.0);
        if ui.button(im_str!("Clear"), [0.0, 0.0]) {
            self.clear();
        }
        let [button_size_x, _] = ui.item_rect_size();
        self.clear_button_width = button_size_x;

        // copy button
        ui.same_line(window_size_x - self.copy_button_width);
        if ui.button(im_str!("Copy"), [0.0, 0.0]) {
            ui.set_clipboard_text(&self.contents);
        }
        let [button_size_x, _] = ui.item_rect_size();
        self.copy_button_width = button_size_x;

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
