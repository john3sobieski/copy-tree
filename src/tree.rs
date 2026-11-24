//! Builds a `tree`-style text view of a folder.

use crate::config::Config;
use crate::strings;
use ignore::gitignore::{Gitignore, GitignoreBuilder};
use ignore::overrides::{Override, OverrideBuilder};
use std::fs;
use std::path::{Path, PathBuf};

struct Entry {
    depth: usize,
    name: String,
    is_dir: bool,
    /// Folder excluded by the ignore list or a .gitignore: listed, but its content is not.
    skipped: bool,
    size: Option<u64>,
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
            if entry.skipped {
                out.push_str(" [skipped]");
            }
        }
        if let Some(size) = entry.size {
            out.push_str(&format!(" ({})", format_size(size)));
        }
        out.push('\n');
        ancestors.push(last);
    }
    if truncated {
        // Show the active limit, so a stale config is easy to spot.
        out.push_str(&format!("... (truncated at {} entries)\n", config.max_entries));
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
            .map_err(|e| strings::get(strings::ERR_IGNORE_PATTERN, &[&pattern, &e]))?;
    }
    let ignore_list = overrides
        .build()
        .map_err(|e| strings::get(strings::ERR_IGNORE_LIST, &[&e]))?;

    let mut walk = Walk {
        config,
        ignore_list,
        entries: Vec::new(),
        // One entry more than allowed, to know whether the output is cut.
        limit: config.max_entries.saturating_add(1),
    };
    walk.visit(root, 0, &mut Vec::new());
    Ok(walk.entries)
}

struct Walk<'a> {
    config: &'a Config,
    ignore_list: Override,
    entries: Vec<Entry>,
    limit: usize,
}

struct Child {
    name: String,
    sort_key: String,
    path: PathBuf,
    is_dir: bool,
    size: Option<u64>,
}

impl Walk<'_> {
    /// Adds the children of `dir` (at `depth`) and descends into the folders that are not skipped.
    /// `git` holds the .gitignore rules of the folders above, the closest one last.
    fn visit(&mut self, dir: &Path, depth: usize, git: &mut Vec<Gitignore>) {
        if depth >= self.config.max_depth {
            return;
        }
        // Unreadable folders (no access) are left empty instead of failing the whole run.
        let Ok(read_dir) = fs::read_dir(dir) else {
            return;
        };
        let show_sizes = self.config.show_sizes;
        let mut children: Vec<Child> = read_dir
            .filter_map(Result::ok)
            .map(|e| {
                // Symlinks are not followed: they count as files, so there are no loops.
                let is_dir = e.file_type().is_ok_and(|t| t.is_dir());
                let name = e.file_name().to_string_lossy().into_owned();
                // Sizes cost one extra stat per file, so they are read only when enabled.
                let size = if show_sizes && !is_dir {
                    e.metadata().ok().map(|m| m.len())
                } else {
                    None
                };
                Child { sort_key: name.to_lowercase(), name, path: e.path(), is_dir, size }
            })
            .collect();
        // Folders first, then case-insensitive by name.
        children.sort_by(|a, b| {
            b.is_dir
                .cmp(&a.is_dir)
                .then_with(|| a.sort_key.cmp(&b.sort_key))
                .then_with(|| a.name.cmp(&b.name))
        });

        let mut pushed = false;
        if self.config.use_gitignore {
            if let Some(gitignore) = load_gitignore(dir) {
                git.push(gitignore);
                pushed = true;
            }
        }
        for child in children {
            if self.entries.len() >= self.limit {
                break;
            }
            let skipped = self.is_skipped(&child.path, child.is_dir, git);
            // Skipped files stay hidden, skipped folders are marked.
            if skipped && !child.is_dir {
                continue;
            }
            self.entries.push(Entry {
                depth: depth + 1,
                name: child.name,
                is_dir: child.is_dir,
                skipped,
                size: child.size,
            });
            if child.is_dir && !skipped {
                self.visit(&child.path, depth + 1, git);
            }
        }
        if pushed {
            git.pop();
        }
    }

    /// True if the ignore list or a .gitignore excludes the path (the closest .gitignore wins).
    fn is_skipped(&self, path: &Path, is_dir: bool, git: &[Gitignore]) -> bool {
        if self.ignore_list.matched(path, is_dir).is_ignore() {
            return true;
        }
        for gitignore in git.iter().rev() {
            let found = gitignore.matched(path, is_dir);
            if found.is_ignore() {
                return true;
            }
            if found.is_whitelist() {
                return false;
            }
        }
        false
    }
}

/// Rules of one folder from `.git/info/exclude` and `.gitignore`, or None if it has none.
fn load_gitignore(dir: &Path) -> Option<Gitignore> {
    let mut builder = GitignoreBuilder::new(dir);
    // A missing file is fine. Rules added later win, so `.gitignore` beats `info/exclude`.
    let _ = builder.add(dir.join(".git").join("info").join("exclude"));
    let _ = builder.add(dir.join(".gitignore"));
    builder.build().ok().filter(|g| !g.is_empty())
}

/// Size with 1024-based units, e.g. "532 B" or "1.5 KB".
fn format_size(bytes: u64) -> String {
    const UNITS: [&str; 4] = ["KB", "MB", "GB", "TB"];
    if bytes < 1024 {
        return format!("{bytes} B");
    }
    let mut value = bytes as f64 / 1024.0;
    let mut unit = 0;
    // Compare the rounded value, so 1023.99 KB becomes "1.0 MB", not "1024.0 KB".
    while (value * 10.0).round() >= 10240.0 && unit < UNITS.len() - 1 {
        value /= 1024.0;
        unit += 1;
    }
    format!("{value:.1} {}", UNITS[unit])
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
