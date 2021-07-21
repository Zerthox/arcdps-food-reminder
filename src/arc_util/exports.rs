//! ArcDPS exports.

use super::{api::CoreColor, game::Profession};
use arcdps::{e5 as e5_colors, e6 as e6_ui_settings, e7 as e7_modifiers};
use std::{
    mem::{self, MaybeUninit},
    slice,
};

/// A color stored in an [`ImVec4`].
pub type ColorVec = [f32; 4];

/// The array of color arrays returned by ArcDPS.
type RawColorArray = [*mut [f32; 4]; 5];

/// Current color settings.
///
/// This holds color information in memory until dropped.
/// Use the associated methods to access individual colors.
#[derive(Debug, Clone, PartialEq)]
pub struct Colors {
    pub raw: RawColorArray,
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
    unsafe fn read_color(&self, first_index: usize, second_index: usize) -> Option<ColorVec> {
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
    pub fn get_core(&self, color: CoreColor) -> Option<ColorVec> {
        unsafe { self.read_color(0, color as usize) }
    }

    /// Returns the base color for a specific [`Profession`].
    ///
    /// This will return [`None`] if ArcDPS did not yield the requested color when the [`Colors`] struct was retrieved.
    pub fn get_prof_base(&self, prof: Profession) -> Option<ColorVec> {
        unsafe { self.read_color(1, prof as usize) }
    }

    /// Returns the highlight color for a specific [`Profession`].
    ///
    /// This will return [`None`] if ArcDPS did not yield the requested color when the [`Colors`] struct was retrieved.
    pub fn get_prof_highlight(&self, prof: Profession) -> Option<ColorVec> {
        unsafe { self.read_color(2, prof as usize) }
    }

    /// Returns the base color for a specific subgroup.
    ///
    /// This will return [`None`] if ArcDPS did not yield the requested color when the [`Colors`] struct was retrieved.
    /// Also returns [`None`] if the subgroup is out of the game subgroup range.
    pub fn get_sub_base(&self, sub: usize) -> Option<ColorVec> {
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
    pub fn get_sub_highlight(&self, sub: usize) -> Option<ColorVec> {
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
    // greaka's bindings currently use a wrongly sized type, we have to transmute the function again
    // TODO: remove once bindings are fixed
    type FuncType = unsafe fn(*mut RawColorArray);
    let transmuted = unsafe { mem::transmute::<_, FuncType>(e5_colors as *const ()) };

    // zeroing this is important!
    // we cannot trust arc to write the whole array
    let mut colors = MaybeUninit::<RawColorArray>::zeroed();
    unsafe { transmuted(colors.as_mut_ptr()) };
    Colors {
        raw: unsafe { colors.assume_init() },
    }
}

// TODO: do we want these to lazily evaluate via functions as well?

/// Set of UI settings.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct UISettings {
    pub hidden: bool,
    pub draw_always: bool,
    pub modifiers_move_lock: bool,
    pub modifiers_click_block: bool,
    pub close_with_esc: bool,
}

/// Returns the UI settings from ArcDPS.
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

/// Returns the modifier keybinds from ArcDPS.
pub fn get_modifiers() -> Modifiers {
    let raw = unsafe { e7_modifiers() };
    Modifiers {
        modifier1: raw as u16,
        modifier2: (raw >> 16) as u16,
        modifier_multi: (raw >> 32) as u16,
    }
}
