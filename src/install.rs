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

/// Writes the menu entries; running it again just overwrites the same keys.
#[cfg(windows)]
pub fn install() -> Result<(), String> {
    use winreg::enums::HKEY_CURRENT_USER;
    use winreg::RegKey;

    let exe = std::env::current_exe().map_err(|e| format!("Cannot find the exe path: {e}"))?;
    let exe = exe.display();
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);

    for (key, arg) in TARGETS {
        let err = |e: std::io::Error| format!("Registry error at HKCU\\{key}: {e}");
        let (menu, _) = hkcu.create_subkey(key).map_err(err)?;
        menu.set_value("", &MENU_NAME).map_err(err)?;
        menu.set_value("Icon", &format!("\"{exe}\",0")).map_err(err)?;
        let (command, _) = menu.create_subkey("command").map_err(err)?;
        command.set_value("", &format!("\"{exe}\" \"{arg}\"")).map_err(err)?;
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
pub fn install() -> Result<(), String> {
    Err("Context menu integration works on Windows only".to_string())
}

#[cfg(not(windows))]
pub fn uninstall() -> Result<(), String> {
    Err("Context menu integration works on Windows only".to_string())
}
