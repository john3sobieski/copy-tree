//! Adds or removes the Explorer context menu entries under HKCU (no admin rights needed).

#[cfg(windows)]
const MENU_NAME: &str = "CopyTree";

// Registry key and the placeholder Explorer fills with the clicked path.
#[cfg(windows)]
const TARGETS: [(&str, &str); 3] = [
    (r"Software\Classes\Directory\shell\CopyTree", "%1"),
    (r"Software\Classes\Directory\Background\shell\CopyTree", "%V"),
    (r"Software\Classes\*\shell\CopyTree", "%1"),
];

#[cfg(windows)]
fn command_line(exe: &str, arg: &str) -> String {
    format!("\"{exe}\" \"{arg}\"")
}

/// True if all entries exist and point to this exe (false after the exe was moved).
#[cfg(windows)]
pub fn is_installed() -> bool {
    use winreg::enums::HKEY_CURRENT_USER;
    use winreg::RegKey;

    let Ok(exe) = std::env::current_exe() else {
        return false;
    };
    let exe = exe.display().to_string();
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    TARGETS.iter().all(|(key, arg)| {
        hkcu.open_subkey(format!(r"{key}\command"))
            .and_then(|k| k.get_value::<String, _>(""))
            .is_ok_and(|value| value == command_line(&exe, arg))
    })
}

/// Writes the menu entries; running it again just overwrites the same keys.
#[cfg(windows)]
pub fn install() -> Result<(), String> {
    use winreg::enums::HKEY_CURRENT_USER;
    use winreg::RegKey;

    let exe = std::env::current_exe().map_err(|e| format!("Cannot find the exe path: {e}"))?;
    let exe = exe.display().to_string();
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);

    for (key, arg) in TARGETS {
        let err = |e: std::io::Error| format!("Registry error at HKCU\\{key}: {e}");
        let (menu, _) = hkcu.create_subkey(key).map_err(err)?;
        menu.set_value("", &MENU_NAME).map_err(err)?;
        menu.set_value("Icon", &format!("\"{exe}\",0")).map_err(err)?;
        let (command, _) = menu.create_subkey("command").map_err(err)?;
        command.set_value("", &command_line(&exe, arg)).map_err(err)?;
    }
    Ok(())
}

/// Removes the menu entries; missing keys are not an error.
#[cfg(windows)]
pub fn uninstall() -> Result<(), String> {
    use winreg::enums::HKEY_CURRENT_USER;
    use winreg::RegKey;

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    for (key, _) in TARGETS {
        match hkcu.delete_subkey_all(key) {
            Ok(()) => {}
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
            Err(e) => return Err(format!("Registry error at HKCU\\{key}: {e}")),
        }
    }
    Ok(())
}

#[cfg(not(windows))]
pub fn is_installed() -> bool {
    false
}

#[cfg(not(windows))]
pub fn install() -> Result<(), String> {
    Err("Context menu integration works on Windows only".to_string())
}

#[cfg(not(windows))]
pub fn uninstall() -> Result<(), String> {
    Err("Context menu integration works on Windows only".to_string())
}
