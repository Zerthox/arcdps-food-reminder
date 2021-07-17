use crate::ui::Component;
use arcdps::imgui::{im_str, ChildWindow, ImString, Ui};
use chrono::Local;

/// Time format used for debug messages.
const FORMAT: &str = "%b %d %H:%M:%S.%3f";

/// Debug log component.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct DebugLog {
    contents: ImString,
}

impl DebugLog {
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
        Self::default()
    }

    fn render(&mut self, ui: &Ui) {
        ui.text(format!("Time: {}", Local::now().format(FORMAT)));
        ui.separator();
        ChildWindow::new(im_str!("food-reminder-log-scroller"))
            .scrollable(true)
            .horizontal_scrollbar(true)
            .build(ui, || {
                ui.text(&self.contents);
                ui.set_scroll_here_y_with_ratio(1.0);
            })
    }
}
