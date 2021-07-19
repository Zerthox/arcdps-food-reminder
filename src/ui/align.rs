//! Helpers for UI alignment.

use arcdps::imgui::Ui;

/// Sets the cursor position to align the next item right.
pub fn right(ui: &Ui, child_width: f32, parent_width: f32) {
    let [cursor_x, cursor_y] = ui.cursor_pos();
    let scroll_x = ui.scroll_x();
    ui.set_cursor_pos([cursor_x + parent_width - child_width - scroll_x, cursor_y]);
}

/// Sets the cursor position to align the next item center.
pub fn center(ui: &Ui, child_width: f32, parent_width: f32) {
    let [cursor_x, cursor_y] = ui.cursor_pos();
    let scroll_x = ui.scroll_x();
    ui.set_cursor_pos([
        cursor_x + 0.5 * parent_width - 0.5 * child_width - scroll_x,
        cursor_y,
    ]);
}
