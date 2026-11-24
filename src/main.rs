mod config;
mod tree;

use std::env;
use std::path::PathBuf;
use std::process::ExitCode;

fn main() -> ExitCode {
    let Some(arg) = env::args_os().nth(1) else {
        eprintln!("Usage: copytree <folder>");
        return ExitCode::from(2);
    };

    let path = PathBuf::from(arg);
    if !path.is_dir() {
        eprintln!("Not a folder: {}", path.display());
        return ExitCode::from(1);
    }

    let config = match config::load() {
        Ok(config) => config,
        Err(message) => {
            eprintln!("{message}");
            return ExitCode::from(1);
        }
    };

    // Debug mode for now: print the tree instead of copying it.
    match tree::build_tree(&path, &config) {
        Ok(text) => {
            print!("{text}");
            ExitCode::SUCCESS
        }
        Err(message) => {
            eprintln!("{message}");
            ExitCode::from(1)
        }
    }
}
