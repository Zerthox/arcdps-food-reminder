use std::ffi::CString;
use windows::{
    core::PSTR,
    Win32::UI::{
        Input::KeyboardAndMouse::{GetKeyNameTextA, MapVirtualKeyA},
        WindowsAndMessaging::MAPVK_VK_TO_VSC,
    },
};

/// Converts a keycode to the key's name.
pub fn keycode_to_name(keycode: u32) -> Option<String> {
    let scan_code = unsafe { MapVirtualKeyA(keycode, MAPVK_VK_TO_VSC) };

    const SIZE: usize = 32;
    let mut buffer = Vec::with_capacity(SIZE);

    let result = unsafe {
        GetKeyNameTextA(
            (scan_code << 16) as i32,
            PSTR(buffer.as_mut_ptr()),
            SIZE as i32,
        )
    };

    if result > 0 {
        unsafe { buffer.set_len(result as usize) }
        CString::new(buffer)
            .ok()
            .and_then(|string| string.into_string().ok())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_name() {
        assert_eq!(Some(String::from("ALT")), keycode_to_name(18));
        assert_eq!(Some(String::from("A")), keycode_to_name(65));
        assert_eq!(Some(String::from("F")), keycode_to_name(70));
    }
}
