mod bindings {
    windows::include_bindings!();
}

pub use bindings::Windows::Win32::Media::Multimedia::timeGetTime;
