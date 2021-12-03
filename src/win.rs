use arcdps::imgui::ImString;
use windows::Win32::{
    Foundation::PSTR,
    UI::{
        Input::KeyboardAndMouse::{GetKeyNameTextA, MapVirtualKeyA},
        WindowsAndMessaging::MAPVK_VK_TO_VSC,
    },
};

pub fn keycode_to_name(keycode: u32) -> Option<ImString> {
    let scan_code = unsafe { MapVirtualKeyA(keycode, MAPVK_VK_TO_VSC) };

    const SIZE: i32 = 4;
    let mut buffer = ImString::with_capacity(SIZE as usize);

    let result = unsafe {
        GetKeyNameTextA(
            (scan_code << 16) as i32,
            PSTR(buffer.as_mut_ptr() as *mut u8),
            SIZE,
        )
    };

    if result > 0 {
        Some(buffer)
    } else {
        None
    }
}
