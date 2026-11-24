#![windows_subsystem = "windows"]

mod config;
mod install;
mod strings;
mod tree;
mod ui;

use arboard::Clipboard;
use std::env;
use std::path::{self, Path, PathBuf};
use std::process::ExitCode;

fn main() -> ExitCode {
    // No argument: the exe was started by hand, so set up (or remove) the Explorer menu entry.
    // With a path: Explorer started it from the menu, so copy the tree or the file path.
    let result = match env::args_os().nth(1) {
        None => setup(),
        Some(arg) => copy(&fix_drive_root(PathBuf::from(arg))),
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            ui::error(&message);
            ExitCode::from(1)
        }
    }
}

fn setup() -> Result<(), String> {
    if !install::is_installed() {
        let dir = install::install()?;
        ui::info(&strings::get(strings::SETUP_DONE, &[&dir.display()]));
        return Ok(());
    }
    // Already installed: offer an update from this exe (unless it is the installed copy), then removal.
    if !install::running_from_install_dir()
        && ui::ask(&strings::get(strings::SETUP_ASK_UPDATE, &[]))
    {
        install::install()?;
        ui::info(&strings::get(strings::SETUP_UPDATED, &[]));
        return Ok(());
    }
    if ui::ask(&strings::get(strings::SETUP_ASK_REMOVE, &[])) {
        install::uninstall()?;
        ui::info(&strings::get(strings::SETUP_REMOVED, &[]));
    }
    Ok(())
}

fn copy(path: &Path) -> Result<(), String> {
    let text = render(path)?;
    let mut clipboard = Clipboard::new()
        .map_err(|e| strings::get(strings::ERR_CLIPBOARD_OPEN, &[&e]))?;
    clipboard
        .set_text(text)
        .map_err(|e| strings::get(strings::ERR_CLIPBOARD_WRITE, &[&e]))
}

/// Explorer passes a drive root as `D:"` (the `\"` in `"D:\"` is parsed as a quote); restore `D:\`.
fn fix_drive_root(path: PathBuf) -> PathBuf {
    match path.to_string_lossy().strip_suffix('"') {
        Some(stripped) => PathBuf::from(format!("{stripped}\\")),
        None => path,
    }
}

/// Folder: its tree. File: its absolute path on one line.
fn render(path: &Path) -> Result<String, String> {
    if path.is_dir() {
        let config = config::load()?;
        tree::build_tree(path, &config)
    } else if path.is_file() {
        let absolute = path::absolute(path)
            .map_err(|e| strings::get(strings::ERR_RESOLVE, &[&path.display(), &e]))?;
        Ok(absolute.to_string_lossy().into_owned())
    } else {
        Err(strings::get(strings::ERR_PATH_NOT_FOUND, &[&path.display()]))
    }
}
