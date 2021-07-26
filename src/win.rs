#[cfg(not(target_os = "windows"))]
compile_error!("cannot compile for OS other than Windows");

mod bindings {
    windows::include_bindings!();
}

pub use bindings::Windows::Win32::Media::Multimedia::timeGetTime;
