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
    /// Returns whether the component is currently visible.
    fn is_visible(&self) -> bool;

    /// Returns a mutable reference to the component's visibility state.
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

    /// Sets the component's visibility state.
    fn set_visibility(&mut self, visible: bool) {
        *self.visibility() = visible;
    }
}
