//! ArcDPS exports.

use super::{api::CoreColor, game::Profession};
use arcdps::{
    e0 as e0_config_path, e5 as e5_colors, e6 as e6_ui_settings, e7 as e7_modifiers,
    imgui::sys::ImVec4,
};
use std::{ffi::OsString, mem::MaybeUninit, os::windows::prelude::*, path::PathBuf, slice};

/// Retrieves the config path from ArcDPS.
pub fn get_config_path() -> Option<PathBuf> {
    let ptr = unsafe { e0_config_path() };
    if !ptr.is_null() {
        // calculate length
        let mut len = 0;
        while unsafe { *ptr.offset(len) } != 0 {
            len += 1;
        }

        // transform data
        let slice = unsafe { slice::from_raw_parts(ptr, len as usize) };
        let string = OsString::from_wide(slice);
        Some(PathBuf::from(string))
    } else {
        None
    }
}

/// The array of color arrays returned by ArcDPS.
type RawColorArray = [*mut ImVec4; 5];

/// Current color settings.
///
/// Use the associated functions to access individual colors.
///
/// This holds pointers to color information in memory until dropped.
#[derive(Debug, Clone, PartialEq)]
pub struct Colors {
    raw: RawColorArray,
}

impl Colors {
    /// Reads a color from the raw color array.
    ///
    /// The first index is the index of the subarray.
    /// The second index is the index of the color within the subarray.
    ///
    /// This will return [`None`] if the pointer retrieved from ArcDPS is null or was not initialized.
    ///
    /// This is unsafe since indexing the raw color array is only valid with specific indices.
    /// Incorrect indices may cause undefined behavior.
    unsafe fn read_color(&self, first_index: usize, second_index: usize) -> Option<ImVec4> {
        let ptr = self.raw[first_index];
        if !ptr.is_null() {
            // we do no need the full length slice
            let slice = slice::from_raw_parts(ptr, second_index + 1);
            Some(slice[second_index])
        } else {
            None
        }
    }

    /// Returns the base color for a specific [`CoreColor`].
    ///
    /// This will return [`None`] if ArcDPS did not yield the requested color when the [`Colors`] struct was retrieved.
    pub fn get_core(&self, color: CoreColor) -> Option<ImVec4> {
        unsafe { self.read_color(0, color as usize) }
    }

    /// Returns the base color for a specific [`Profession`].
    ///
    /// This will return [`None`] if ArcDPS did not yield the requested color when the [`Colors`] struct was retrieved.
    pub fn get_prof_base(&self, prof: Profession) -> Option<ImVec4> {
        unsafe { self.read_color(1, prof as usize) }
    }

    /// Returns the highlight color for a specific [`Profession`].
    ///
    /// This will return [`None`] if ArcDPS did not yield the requested color when the [`Colors`] struct was retrieved.
    pub fn get_prof_highlight(&self, prof: Profession) -> Option<ImVec4> {
        unsafe { self.read_color(2, prof as usize) }
    }

    /// Returns the base color for a specific subgroup.
    ///
    /// This will return [`None`] if ArcDPS did not yield the requested color when the [`Colors`] struct was retrieved.
    /// Also returns [`None`] if the subgroup is out of the game subgroup range.
    pub fn get_sub_base(&self, sub: usize) -> Option<ImVec4> {
        // range check
        if sub != 0 && sub <= 15 {
            unsafe { self.read_color(3, sub) }
        } else {
            None
        }
    }

    /// Returns the highlight color for a specific subgroup.
    ///
    /// This will return [`None`] if ArcDPS did not yield the requested color when the [`Colors`] struct was retrieved.
    /// Also returns [`None`] if the subgroup is out of the game subgroup range.
    pub fn get_sub_highlight(&self, sub: usize) -> Option<ImVec4> {
        // range check
        if sub != 0 && sub <= 15 {
            unsafe { self.read_color(4, sub) }
        } else {
            None
        }
    }
}

/// Retrieves the color settings from ArcDPS.
pub fn get_colors() -> Colors {
    // zeroing this is important for our null pointer checks later
    let mut colors = MaybeUninit::<RawColorArray>::zeroed();
    unsafe { e5_colors(colors.as_mut_ptr()) };
    Colors {
        raw: unsafe { colors.assume_init() },
    }
}

/// Set of UI settings.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct UISettings {
    pub hidden: bool,
    pub draw_always: bool,
    pub modifiers_move_lock: bool,
    pub modifiers_click_block: bool,
    pub close_with_esc: bool,
}

/// Retrieves the UI settings from ArcDPS.
pub fn get_ui_settings() -> UISettings {
    let raw = unsafe { e6_ui_settings() };
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

/// Retrieves the modifier keybinds from ArcDPS.
pub fn get_modifiers() -> Modifiers {
    let raw = unsafe { e7_modifiers() };
    Modifiers {
        modifier1: raw as u16,
        modifier2: (raw >> 16) as u16,
        modifier_multi: (raw >> 32) as u16,
    }
}
