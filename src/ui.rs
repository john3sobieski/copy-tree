//! Message boxes for errors and setup questions (no console window is ever shown).

#[cfg(windows)]
use windows_sys::Win32::UI::WindowsAndMessaging::{
    MessageBoxW, IDYES, MB_ICONERROR, MB_ICONINFORMATION, MB_ICONQUESTION, MB_OK, MB_YESNO,
};

#[cfg(windows)]
fn show(text: &str, flags: u32) -> i32 {
    let wide = |s: &str| -> Vec<u16> { s.encode_utf16().chain(std::iter::once(0)).collect() };
    let (text, title) = (wide(text), wide("CopyTree"));
    // SAFETY: both buffers are NUL-terminated and outlive the call.
    unsafe { MessageBoxW(std::ptr::null_mut(), text.as_ptr(), title.as_ptr(), flags) }
}

#[cfg(windows)]
pub fn error(text: &str) {
    show(text, MB_OK | MB_ICONERROR);
}

#[cfg(windows)]
pub fn info(text: &str) {
    show(text, MB_OK | MB_ICONINFORMATION);
}

/// Yes/No question; returns true on Yes.
#[cfg(windows)]
pub fn ask(text: &str) -> bool {
    show(text, MB_YESNO | MB_ICONQUESTION) == IDYES
}

#[cfg(not(windows))]
pub fn error(text: &str) {
    eprintln!("{text}");
}

#[cfg(not(windows))]
pub fn info(text: &str) {
    println!("{text}");
}

#[cfg(not(windows))]
pub fn ask(_text: &str) -> bool {
    false
}
