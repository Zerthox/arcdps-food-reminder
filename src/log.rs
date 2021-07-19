use crate::ui::Component;
use arcdps::imgui::{im_str, ChildWindow, ImString, Ui};
use chrono::Local;

/// Time format used for debug messages.
const FORMAT: &str = "%b %d %H:%M:%S.%3f";

/// Debug log component.
#[derive(Debug, Clone)]
pub struct DebugLog {
    contents: ImString,
    copy_button_width: f32,
}

impl DebugLog {
    pub fn new() -> Self {
        Self {
            contents: ImString::default(),
            copy_button_width: 120.0,
        }
    }

    /// Appends output to the debug log.
    pub fn log<S>(&mut self, output: S)
    where
        S: AsRef<str>,
    {
        let now = Local::now();
        self.contents
            .push_str(&format!("{}: {}\n", now.format(FORMAT), output.as_ref()));
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

        // copy button
        let [window_size_x, _] = ui.window_content_region_max();
        ui.same_line(window_size_x - self.copy_button_width);
        if ui.button(im_str!("Copy to clipboard"), [0.0, 0.0]) {
            ui.set_clipboard_text(&self.contents);
        }

        // update button width
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
