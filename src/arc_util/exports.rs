//! ArcDPS exports.

use arcdps::{e6 as ui_settings_raw, e7 as modifiers_raw};

// FIXME: these do not return what we want

/// Current UI settings.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct UISettings {
    pub hidden: bool,
    pub draw_always: bool,
    pub modifiers_move_lock: bool,
    pub modifiers_click_block: bool,
    pub close_with_esc: bool,
}

/// Returns the UI settings.
pub fn get_ui_settings() -> UISettings {
    let raw = unsafe { ui_settings_raw() };
    UISettings {
        hidden: raw & 1 == 1,
        draw_always: (raw >> 1) & 1 == 1,
        modifiers_move_lock: (raw >> 2) & 1 == 1,
        modifiers_click_block: (raw >> 3) & 1 == 1,
        close_with_esc: (raw >> 4) & 1 == 1,
    }
}

/// Set of modifier keybinds as virtual key ids.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Modifiers {
    pub modifier1: u16,
    pub modifier2: u16,
    pub modifier_multi: u16,
}

/// Returns the modifier keybinds
pub fn get_modifiers() -> Modifiers {
    let raw = unsafe { modifiers_raw() };
    Modifiers {
        modifier1: raw as u16,
        modifier2: (raw >> 16) as u16,
        modifier_multi: (raw >> 32) as u16,
    }
}
