//! Helpers for UI alignment.

use arcdps::imgui::Ui;

/// Render state for left alignment.
#[derive(Debug, Clone, Copy)]
pub struct LeftAlign {
    default_margin: f32,
    accumulated: f32,
}

impl LeftAlign {
    /// Starts rendering items in left alignment.
    ///
    /// Items are passed from **left to right**.
    pub fn build() -> Self {
        Self::with_margin(5.0)
    }

    /// Starts rendering items in left alignment with a custom margin.
    ///
    /// Items are passed from **left to right**.
    pub fn with_margin(margin: f32) -> Self {
        Self {
            default_margin: margin,
            accumulated: f32::NAN, // placeholder to identify first render
        }
    }

    /// Renders the next item.
    ///
    /// Items are passed from **left to right**.
    pub fn item<F>(&mut self, ui: &Ui, render: F)
    where
        F: FnOnce(),
    {
        self.item_with_margin(ui, self.default_margin, render);
    }

    /// Renders the next item with a temporary margin override.
    ///
    /// Items are passed from **left to right**.
    pub fn item_with_margin<F>(&mut self, ui: &Ui, margin: f32, render: F)
    where
        F: FnOnce(),
    {
        // prepare
        if self.accumulated.is_nan() {
            // first render is normal
            self.accumulated = 0.0;
        } else {
            // successive renders on same line
            ui.same_line(self.accumulated);
        }

        // render item
        render();

        // update accumulated
        let [last_width, _] = ui.item_rect_size();
        self.accumulated += last_width + margin;
    }
}

/// Render state for right alignment.
#[derive(Debug, Clone, Copy)]
pub struct RightAlign {
    margin: f32,
    accumulated: f32,
}

impl RightAlign {
    /// Starts rendering items in right alignment.
    ///
    /// Items are passed from **right to left**.
    pub fn build() -> Self {
        Self::with_margin(5.0)
    }

    /// Starts rendering items in right alignment with a custom margin.
    ///
    /// Items are passed from **right to left**.
    pub fn with_margin(margin: f32) -> Self {
        Self {
            margin,
            accumulated: 0.0,
        }
    }

    /// Renders the next item.
    ///
    /// Items are passed from **right to left**.
    ///
    /// The item width will be used for alignment and updated with the correct width after render.
    /// It can be a guessed default on the first render.
    pub fn item<F>(&mut self, ui: &Ui, item_width: &mut f32, render: F)
    where
        F: FnOnce(),
    {
        self.item_with_margin(ui, self.margin, item_width, render)
    }

    /// Renders the next item with a temporary margin override.
    ///
    /// Items are passed from **right to left**.
    ///
    /// The item width will be used for alignment and updated with the correct width after render.
    /// It can be a guessed default on the first render.
    pub fn item_with_margin<F>(&mut self, ui: &Ui, margin: f32, item_width: &mut f32, render: F)
    where
        F: FnOnce(),
    {
        // prepare alignment
        let [window_x, _] = ui.window_content_region_max();
        ui.same_line(window_x - self.accumulated - *item_width);

        // render item
        render();

        // update item width & accumulated with actual size
        *item_width = ui.item_rect_size()[0];
        self.accumulated += *item_width + margin;
    }
}
