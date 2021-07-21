pub mod align;
pub mod window;

use arcdps::imgui::Ui;

/// Interface for UI components.
pub trait Component {
    type Props;

    /// Creates a new component.
    fn create(props: Self::Props) -> Self;

    /// Renders the component.
    fn render(&mut self, ui: &Ui);
}
