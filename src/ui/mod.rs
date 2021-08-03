pub mod align;
pub mod render;
pub mod window;

use arcdps::imgui::Ui;

/// Interface for UI components.
pub trait Component {
    /// Renders the component.
    fn render(&mut self, ui: &Ui);
}

/// Interface for hideable UI components.
pub trait Hideable
where
    Self: Component,
{
    /// Returns the component's visibility state.
    fn visibility(&mut self) -> &mut bool;

    /// Hides the component.
    fn hide(&mut self) {
        *self.visibility() = false;
    }

    /// Shows the component.
    fn show(&mut self) {
        *self.visibility() = true;
    }

    /// Toggles the component's visibility.
    fn toggle_visibility(&mut self) {
        let shown = self.visibility();
        *shown = !*shown;
    }
}
