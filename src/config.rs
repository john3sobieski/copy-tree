//! Settings from `copytree.toml` next to the exe; a commented-out template is created if it is missing.

use crate::strings;
use serde::Deserialize;
use std::path::PathBuf;
use std::{env, fs};

const DEFAULT_TOML: &str = include_str!("../copytree.default.toml");
pub const FILE_NAME: &str = "copytree.toml";

pub struct Config {
    pub max_depth: usize,
    pub max_entries: usize,
    pub use_gitignore: bool,
    pub show_sizes: bool,
    pub ignore: Vec<String>,
}

/// Defaults file layout: every key is required, so a broken default fails loudly.
#[derive(Deserialize)]
struct DefaultConfig {
    max_depth: usize,
    max_entries: usize,
    use_gitignore: bool,
    show_sizes: bool,
    ignore: Vec<String>,
}

/// User file layout: a missing key falls back to the default, a typo in a key is an error.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct UserConfig {
    max_depth: Option<usize>,
    max_entries: Option<usize>,
    use_gitignore: Option<bool>,
    show_sizes: Option<bool>,
    ignore: Option<Vec<String>>,
}

pub fn load() -> Result<Config, String> {
    let defaults: DefaultConfig = toml::from_str(DEFAULT_TOML)
        .map_err(|e| strings::get(strings::ERR_DEFAULT_CONFIG, &[&e]))?;
    let mut config = Config {
        max_depth: defaults.max_depth,
        max_entries: defaults.max_entries,
        use_gitignore: defaults.use_gitignore,
        show_sizes: defaults.show_sizes,
        ignore: defaults.ignore,
    };

    let path = config_path()?;
    if !path.exists() {
        // Best effort: the folder may be read-only, the defaults still work.
        let _ = fs::write(&path, template());
        return Ok(config);
    }

    let text = fs::read_to_string(&path)
        .map_err(|e| strings::get(strings::ERR_CONFIG_READ, &[&path.display(), &e]))?;
    let user: UserConfig = toml::from_str(text.trim_start_matches('\u{feff}'))
        .map_err(|e| strings::get(strings::ERR_CONFIG_INVALID, &[&path.display(), &e]))?;

    if let Some(v) = user.max_depth {
        config.max_depth = v;
    }
    if let Some(v) = user.max_entries {
        config.max_entries = v;
    }
    if let Some(v) = user.use_gitignore {
        config.use_gitignore = v;
    }
    if let Some(v) = user.show_sizes {
        config.show_sizes = v;
    }
    if let Some(v) = user.ignore {
        config.ignore = v;
    }
    Ok(config)
}

/// Template for a new config file: all settings commented out, so built-in defaults keep applying after updates.
fn template() -> String {
    let mut out = String::from(
        "# Every setting below is commented out, so the built-in defaults apply (and follow app updates).\n\
         # To override one, remove the leading \"# \" from its line(s) and edit the value.\n\n",
    );
    for line in DEFAULT_TOML.lines() {
        let trimmed = line.trim_start();
        if !trimmed.is_empty() && !trimmed.starts_with('#') {
            out.push_str("# ");
        }
        out.push_str(line);
        out.push('\n');
    }
    out
}

fn config_path() -> Result<PathBuf, String> {
    let exe = env::current_exe().map_err(|e| strings::get(strings::ERR_EXE_PATH, &[&e]))?;
    Ok(exe.with_file_name(FILE_NAME))
}
