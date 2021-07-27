pub mod align;
pub mod window;

use arcdps::imgui::Ui;

/// Interface for UI components.
pub trait Component {
    /// Renders the component.
    fn render(&mut self, ui: &Ui);
}
