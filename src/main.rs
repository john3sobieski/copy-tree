mod config;
mod tree;

use arboard::Clipboard;
use std::env;
use std::path::{self, Path, PathBuf};
use std::process::ExitCode;

fn main() -> ExitCode {
    // `--print` writes the result to stdout instead of the clipboard (debug).
    let mut print_only = false;
    let mut target: Option<PathBuf> = None;
    for arg in env::args_os().skip(1) {
        if arg == "--print" {
            print_only = true;
        } else if target.is_none() {
            target = Some(PathBuf::from(arg));
        }
    }

    let Some(target) = target else {
        eprintln!("Usage: copytree [--print] <folder or file>");
        return ExitCode::from(2);
    };

    let text = match render(&target) {
        Ok(text) => text,
        Err(message) => {
            eprintln!("{message}");
            return ExitCode::from(1);
        }
    };

    if print_only {
        print!("{text}");
        return ExitCode::SUCCESS;
    }

    match copy_to_clipboard(&text) {
        Ok(()) => {
            println!("Copied to clipboard ({} lines)", text.lines().count());
            ExitCode::SUCCESS
        }
        Err(message) => {
            eprintln!("{message}");
            ExitCode::from(1)
        }
    }
}

/// Folder: its tree. File: its absolute path on one line.
fn render(path: &Path) -> Result<String, String> {
    if path.is_dir() {
        let config = config::load()?;
        tree::build_tree(path, &config)
    } else if path.is_file() {
        let absolute = path::absolute(path)
            .map_err(|e| format!("Cannot resolve {}: {e}", path.display()))?;
        Ok(absolute.to_string_lossy().into_owned())
    } else {
        Err(format!("Path not found: {}", path.display()))
    }
}

fn copy_to_clipboard(text: &str) -> Result<(), String> {
    let mut clipboard = Clipboard::new().map_err(|e| format!("Cannot open the clipboard: {e}"))?;
    clipboard
        .set_text(text.to_owned())
        .map_err(|e| format!("Cannot write to the clipboard: {e}"))
}
