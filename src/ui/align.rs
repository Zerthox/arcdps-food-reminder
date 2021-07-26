//! Helpers for UI alignment.

use arcdps::imgui::Ui;

/// Helper struct to align items right.
///
/// Memorizes previous items and aligns them from right to left.
#[derive(Debug, Clone, Copy)]
pub struct RightAlign {
    margin: f32,
    // TODO: memoize item widths here
}

impl RightAlign {
    /// Creates a new right align helper with the default margin.
    pub fn new() -> Self {
        Self::with_margin(5.0)
    }

    /// Creates a new right align helper with a custom margin.
    pub fn with_margin(margin: f32) -> Self {
        Self { margin }
    }

    /// Starts the next render cycle.
    pub fn begin_render(&mut self) -> RightAlignRender {
        RightAlignRender {
            margin: self.margin,
            temp_margin: None,
            accumulated: 0.0,
        }
    }
}

impl Default for RightAlign {
    fn default() -> Self {
        Self::new()
    }
}

/// Render state for a [`RightAlign`].
#[derive(Debug, Clone, Copy)]
pub struct RightAlignRender {
    margin: f32,
    temp_margin: Option<f32>,
    accumulated: f32,
}

impl RightAlignRender {
    /// Aligns the next item.
    ///
    /// Items are passed from **right to left**.
    ///
    /// The item width can be guessed on the first render and then read & saved for successive renders.
    pub fn next_item(&mut self, ui: &Ui, item_width: f32) {
        let [window_x, _] = ui.window_content_region_max();
        self.accumulated += item_width + self.temp_margin.take().unwrap_or(self.margin);
        ui.same_line(window_x - self.accumulated);
    }

    /// Sets the margin for the next item.
    ///
    /// Margin is reset to default after.
    pub fn next_margin(&mut self, margin: f32) {
        self.temp_margin = Some(margin);
    }
}
