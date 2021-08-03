//! Helpers for rendering specific items.

use arcdps::imgui::{sys, ImStr};

/// Renders a right-click context menu for the last item.
pub fn item_context_menu<F>(str_id: &ImStr, contents: F)
where
    F: FnOnce(),
{
    if unsafe {
        sys::igBeginPopupContextItem(
            str_id.as_ptr(),
            sys::ImGuiPopupFlags_MouseButtonRight as i32,
        )
    } {
        contents();
        unsafe { sys::igEndPopup() };
    }
}
