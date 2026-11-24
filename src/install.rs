//! Copies the exe to a fixed folder and adds/removes the Explorer menu entries under HKCU (no admin rights needed).

use crate::strings;
#[cfg(windows)]
use std::path::{Path, PathBuf};

#[cfg(windows)]
const EXE_NAME: &str = "copytree.exe";

// Registry key, the placeholder Explorer fills with the clicked path, and whether the entry is for files.
#[cfg(windows)]
const TARGETS: [(&str, &str, bool); 3] = [
    (r"Software\Classes\Directory\shell\CopyTree", "%1", false),
    (r"Software\Classes\Directory\Background\shell\CopyTree", "%V", false),
    (r"Software\Classes\*\shell\CopyTree", "%1", true),
];

/// Indirect string `@exe,-ID`: Explorer loads the menu text from the exe in the current display language.
#[cfg(windows)]
fn menu_label(exe: &str, is_file: bool) -> String {
    let id = if is_file { strings::MENU_FILE } else { strings::MENU_FOLDER };
    format!("@{exe},-{id}")
}

#[cfg(windows)]
fn command_line(exe: &str, arg: &str) -> String {
    format!("\"{exe}\" \"{arg}\"")
}

/// Fixed home of the installed exe, so the menu keeps working after the downloaded exe is moved.
#[cfg(windows)]
fn install_dir() -> Result<PathBuf, String> {
    let base = std::env::var_os("LOCALAPPDATA")
        .ok_or_else(|| strings::get(strings::ERR_LOCALAPPDATA, &[]))?;
    Ok(PathBuf::from(base).join("CopyTree"))
}

#[cfg(windows)]
fn same_path(a: &Path, b: &Path) -> bool {
    a.display().to_string().eq_ignore_ascii_case(&b.display().to_string())
}

/// True if the running exe is the installed copy itself.
#[cfg(windows)]
pub fn running_from_install_dir() -> bool {
    match (std::env::current_exe(), install_dir()) {
        (Ok(current), Ok(dir)) => same_path(&current, &dir.join(EXE_NAME)),
        _ => false,
    }
}

/// True if the installed exe exists and all entries point to it.
#[cfg(windows)]
pub fn is_installed() -> bool {
    use winreg::enums::HKEY_CURRENT_USER;
    use winreg::RegKey;

    let Ok(dir) = install_dir() else {
        return false;
    };
    let target = dir.join(EXE_NAME);
    if !target.is_file() {
        return false;
    }
    let exe = target.display().to_string();
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    TARGETS.iter().all(|(key, arg, _)| {
        hkcu.open_subkey(format!(r"{key}\command"))
            .and_then(|k| k.get_value::<String, _>(""))
            .is_ok_and(|value| value == command_line(&exe, arg))
    })
}

/// Copies the exe to the install folder and writes the entries; safe to repeat.
/// Returns the install folder.
#[cfg(windows)]
pub fn install() -> Result<PathBuf, String> {
    use winreg::enums::HKEY_CURRENT_USER;
    use winreg::RegKey;

    let current = std::env::current_exe()
        .map_err(|e| strings::get(strings::ERR_EXE_PATH, &[&e]))?;
    let dir = install_dir()?;
    let target = dir.join(EXE_NAME);

    if !same_path(&current, &target) {
        std::fs::create_dir_all(&dir)
            .map_err(|e| strings::get(strings::ERR_CREATE_DIR, &[&dir.display(), &e]))?;
        std::fs::copy(&current, &target)
            .map_err(|e| strings::get(strings::ERR_COPY_EXE, &[&target.display(), &e]))?;
    }

    let exe = target.display().to_string();
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    for (key, arg, is_file) in TARGETS {
        let err = |e: std::io::Error| strings::get(strings::ERR_REGISTRY, &[&key, &e]);
        let (menu, _) = hkcu.create_subkey(key).map_err(err)?;
        menu.set_value("MUIVerb", &menu_label(&exe, is_file)).map_err(err)?;
        // The text comes from MUIVerb now; drop the fixed text that older versions wrote here.
        let _ = menu.delete_value("");
        menu.set_value("Icon", &format!("\"{exe}\",0")).map_err(err)?;
        let (command, _) = menu.create_subkey("command").map_err(err)?;
        command.set_value("", &command_line(&exe, arg)).map_err(err)?;
    }
    Ok(dir)
}

/// Removes the entries and the install folder; missing keys are not an error.
#[cfg(windows)]
pub fn uninstall() -> Result<(), String> {
    use winreg::enums::HKEY_CURRENT_USER;
    use winreg::RegKey;

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    for (key, _, _) in TARGETS {
        match hkcu.delete_subkey_all(key) {
            Ok(()) => {}
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
            Err(e) => return Err(strings::get(strings::ERR_REGISTRY, &[&key, &e])),
        }
    }
    // A running exe cannot delete itself, so the folder stays if we run from it.
    if !running_from_install_dir() {
        if let Ok(dir) = install_dir() {
            let _ = std::fs::remove_dir_all(dir);
        }
    }
    Ok(())
}

#[cfg(not(windows))]
pub fn running_from_install_dir() -> bool {
    false
}

#[cfg(not(windows))]
pub fn is_installed() -> bool {
    false
}

#[cfg(not(windows))]
pub fn install() -> Result<std::path::PathBuf, String> {
    Err(strings::get(strings::ERR_WINDOWS_ONLY, &[]))
}

#[cfg(not(windows))]
pub fn uninstall() -> Result<(), String> {
    Err(strings::get(strings::ERR_WINDOWS_ONLY, &[]))
}
