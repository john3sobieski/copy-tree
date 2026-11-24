//! UI texts live in the exe as STRINGTABLE resources (assets/strings.rc); Windows picks the language.

use std::fmt::Display;

// IDs must match assets/strings.rc.
pub const MENU_FILE: u32 = 101;
pub const MENU_FOLDER: u32 = 102;

pub const SETUP_DONE: u32 = 110;
pub const SETUP_ASK_UPDATE: u32 = 111;
pub const SETUP_UPDATED: u32 = 112;
pub const SETUP_ASK_REMOVE: u32 = 113;
pub const SETUP_REMOVED: u32 = 114;

pub const ERR_CLIPBOARD_OPEN: u32 = 120;
pub const ERR_CLIPBOARD_WRITE: u32 = 121;
pub const ERR_PATH_NOT_FOUND: u32 = 122;
pub const ERR_RESOLVE: u32 = 123;

pub const ERR_DEFAULT_CONFIG: u32 = 130;
pub const ERR_CONFIG_READ: u32 = 131;
pub const ERR_CONFIG_INVALID: u32 = 132;
pub const ERR_IGNORE_PATTERN: u32 = 133;
pub const ERR_IGNORE_LIST: u32 = 134;

pub const ERR_EXE_PATH: u32 = 140;
pub const ERR_LOCALAPPDATA: u32 = 141;
pub const ERR_CREATE_DIR: u32 = 142;
pub const ERR_COPY_EXE: u32 = 143;
pub const ERR_REGISTRY: u32 = 144;
pub const ERR_WINDOWS_ONLY: u32 = 145;

/// Text by ID; `{0}`, `{1}` in it are replaced with `args`.
pub fn get(id: u32, args: &[&dyn Display]) -> String {
    fill(&load(id), args)
}

/// Single pass, so a replaced value is never scanned for placeholders again.
fn fill(template: &str, args: &[&dyn Display]) -> String {
    let mut out = String::with_capacity(template.len());
    let mut rest = template;
    while let Some(start) = rest.find('{') {
        out.push_str(&rest[..start]);
        let tail = &rest[start..];
        let placeholder = tail.find('}').and_then(|end| {
            let index: usize = tail[1..end].parse().ok()?;
            args.get(index).map(|arg| (arg.to_string(), end + 1))
        });
        match placeholder {
            Some((value, consumed)) => {
                out.push_str(&value);
                rest = &tail[consumed..];
            }
            None => {
                out.push('{');
                rest = &tail[1..];
            }
        }
    }
    out.push_str(rest);
    out
}

#[cfg(windows)]
fn load(id: u32) -> String {
    use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;
    use windows_sys::Win32::UI::WindowsAndMessaging::LoadStringW;

    let mut buffer = [0u16; 1024];
    // SAFETY: a null module name means this exe; the buffer is valid for the given length.
    let len = unsafe {
        let module = GetModuleHandleW(std::ptr::null());
        LoadStringW(module, id, buffer.as_mut_ptr(), buffer.len() as i32)
    };
    if len > 0 {
        String::from_utf16_lossy(&buffer[..len as usize])
    } else {
        format!("[string {id} is missing]")
    }
}

/// Resources exist only in the Windows exe; elsewhere (dev builds) the ID is shown.
#[cfg(not(windows))]
fn load(id: u32) -> String {
    format!("[string {id}] {{0}} {{1}}")
}
