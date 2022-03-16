use arcdps_imgui::Ui;
use std::ffi::CString;
use windows::Win32::{
    Foundation::CHAR,
    UI::{
        Input::KeyboardAndMouse::{GetKeyNameTextA, MapVirtualKeyA, VkKeyScanA},
        WindowsAndMessaging::MAPVK_VK_TO_VSC,
    },
};

/// Renders a custom key input.
pub fn key_input(ui: &Ui, id: impl AsRef<str>, label: impl AsRef<str>, keycode: &mut Option<u32>) {
    const SPACING: f32 = 5.0;

    ui.text(label);

    let mut buffer = String::with_capacity(3);
    if let Some(keycode) = keycode {
        buffer.push_str(&keycode.to_string());
    }
    ui.same_line_with_spacing(0.0, SPACING);
    ui.push_item_width(ui.calc_text_size("0000")[0]);
    if ui
        .input_text(id, &mut buffer)
        .chars_uppercase(true)
        .chars_noblank(true)
        .build()
    {
        match buffer.len() {
            1 => {
                // read entered key name
                *keycode = Some(name_to_keycode(buffer.as_bytes()[0]));
            }
            2 | 3 => {
                // read entered keycode
                *keycode = buffer.parse().ok();
            }
            _ => {
                // reset to none
                *keycode = None;
            }
        }
    }

    // display key name
    let name = keycode
        .and_then(|keycode| keycode_to_name(keycode))
        .unwrap_or_default();
    ui.same_line_with_spacing(0.0, SPACING);
    ui.text(name);
}

/// Converts a key's name to its keycode.
pub fn name_to_keycode(name: u8) -> u32 {
    let result = unsafe { VkKeyScanA(CHAR(name)) } as u32;
    result & 0xff
}

/// Converts a keycode to the key's name.
pub fn keycode_to_name(keycode: u32) -> Option<String> {
    let scan_code = unsafe { MapVirtualKeyA(keycode, MAPVK_VK_TO_VSC) };
    let mut buffer = vec![0; 32];

    let result = unsafe { GetKeyNameTextA((scan_code << 16) as i32, &mut buffer) };

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
    fn name_to_code() {
        assert_eq!(65, name_to_keycode('A' as u8));
        assert_eq!(70, name_to_keycode('F' as u8));
    }

    #[test]
    fn code_to_name() {
        assert_eq!(Some(String::from("ALT")), keycode_to_name(18));
        assert_eq!(Some(String::from("A")), keycode_to_name(65));
        assert_eq!(Some(String::from("F")), keycode_to_name(70));
    }
}
