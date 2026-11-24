//! Builds a `tree`-style text view of a folder.

use crate::config::Config;
use ignore::overrides::OverrideBuilder;
use ignore::WalkBuilder;
use std::cmp::Ordering;
use std::path::Path;

struct Entry {
    depth: usize,
    name: String,
    is_dir: bool,
}

/// Returns the folder tree as text, one entry per line, root first.
pub fn build_tree(root: &Path, config: &Config) -> Result<String, String> {
    // One entry more than allowed is collected, to know whether the output is cut.
    let entries = collect(root, config)?;
    let truncated = entries.len() > config.max_entries;
    let is_last = mark_last(&entries);

    let mut out = format!("{}/\n", root_name(root));
    // is_last flag of every ancestor of the current entry, used to draw the prefix.
    let mut ancestors: Vec<bool> = Vec::new();

    for (entry, &last) in entries.iter().zip(&is_last).take(config.max_entries) {
        ancestors.truncate(entry.depth - 1);
        for &ancestor_last in &ancestors {
            out.push_str(if ancestor_last { "    " } else { "│   " });
        }
        out.push_str(if last { "└── " } else { "├── " });
        out.push_str(&entry.name);
        if entry.is_dir {
            out.push('/');
        }
        out.push('\n');
        ancestors.push(last);
    }
    if truncated {
        out.push_str("... (truncated)\n");
    }
    Ok(out)
}

/// Name shown on the first line; falls back to the full path for `.` or a drive root.
fn root_name(root: &Path) -> String {
    let name_of = |p: &Path| p.file_name().map(|n| n.to_string_lossy().into_owned());
    name_of(root)
        .or_else(|| name_of(&root.canonicalize().ok()?))
        .unwrap_or_else(|| root.display().to_string())
}

/// Walks the folder in depth-first order, folders before files, applying the config filters.
fn collect(root: &Path, config: &Config) -> Result<Vec<Entry>, String> {
    // A "!" prefix turns an override glob into an ignore rule.
    let mut overrides = OverrideBuilder::new(root);
    for pattern in config.ignore.iter().map(|p| p.trim()).filter(|p| !p.is_empty()) {
        overrides
            .add(&format!("!{pattern}"))
            .map_err(|e| format!("Bad ignore pattern \"{pattern}\": {e}"))?;
    }
    let overrides = overrides.build().map_err(|e| format!("Bad ignore list: {e}"))?;

    let walker = WalkBuilder::new(root)
        .standard_filters(false)
        .git_ignore(config.use_gitignore)
        .git_exclude(config.use_gitignore)
        .require_git(false)
        .overrides(overrides)
        .max_depth(Some(config.max_depth))
        .sort_by_file_path(compare)
        .build();

    let entries = walker
        .filter_map(Result::ok)
        .filter(|e| e.depth() > 0)
        .take(config.max_entries.saturating_add(1))
        .map(|e| Entry {
            depth: e.depth(),
            name: e.file_name().to_string_lossy().into_owned(),
            is_dir: e.file_type().is_some_and(|t| t.is_dir()),
        })
        .collect();
    Ok(entries)
}

/// Folders first, then case-insensitive by name.
fn compare(a: &Path, b: &Path) -> Ordering {
    b.is_dir()
        .cmp(&a.is_dir())
        .then_with(|| {
            let (a, b) = (a.file_name(), b.file_name());
            let lower = |n: Option<&std::ffi::OsStr>| {
                n.map(|n| n.to_string_lossy().to_lowercase()).unwrap_or_default()
            };
            lower(a).cmp(&lower(b))
        })
        .then_with(|| a.cmp(b))
}

/// For every entry, whether it is the last child of its parent.
fn mark_last(entries: &[Entry]) -> Vec<bool> {
    let mut result = vec![false; entries.len()];
    // seen[d]: a later sibling exists at depth d under the current parent.
    let mut seen: Vec<bool> = Vec::new();

    for (i, entry) in entries.iter().enumerate().rev() {
        let d = entry.depth;
        if seen.len() <= d {
            seen.resize(d + 1, false);
        }
        result[i] = !seen[d];
        seen[d] = true;
        for deeper in seen.iter_mut().skip(d + 1) {
            *deeper = false;
        }
    }
    result
}
