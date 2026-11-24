# CopyTree

Windows Explorer right-click tool: copy a folder's tree or a file's full path to the clipboard. Handy for showing a project structure to an AI.

- **Folder** (or empty space inside a folder): copies a `tree`-style view of the folder.
- **File**: copies its absolute path on one line, e.g. `C:\proj\src\main.rs`.

```
proj/
├── src/
│   ├── main.rs
│   └── tree.rs
├── Cargo.toml
└── README.md
```

Menu entries: "Copy folder tree" and "Copy file path" (Russian text on a Russian Windows UI).

## Install

1. Download `copytree.exe` from the Releases page.
2. Double-click it. The exe copies itself to `%LOCALAPPDATA%\CopyTree\` and adds the menu entries. No admin rights needed.
3. You can now move or delete the downloaded exe, the menu keeps working.

Designed for Windows 10. On Windows 11 the entries should appear under "Show more options" (not tested).

## Update and remove

There are no command-line options. Double-click an exe when CopyTree is already installed:

- **Yes** on "Update it with this exe?" replaces the installed exe. Your settings are kept.
- **No**, then **Yes** on "Remove CopyTree from the Explorer menu?" removes the entries and the install folder.

## Settings

File: `%LOCALAPPDATA%\CopyTree\copytree.toml`, created on first use with every setting commented out. Remove the leading `# ` from a setting to override it. Changes apply on the next click. Delete the file to restore the defaults.

| Setting | Default | Meaning |
|---|---|---|
| `max_depth` | `100` | Maximum folder depth (the clicked folder is level 0). |
| `max_entries` | `1000000` | Maximum number of lines; the rest is cut with `... (truncated at N entries)`. |
| `use_gitignore` | `true` | Also respect `.gitignore` files found inside the clicked folder. |
| `ignore` | see below | Names or masks (like `*.pyc`) skipped at any depth. A custom list replaces the default one. |

Default `ignore` list:

- Version control: `.git`, `.svn`, `.hg`
- Dependencies and build output: `node_modules`, `target`, `dist`, `build`, `out`, `__pycache__`, `*.pyc`, `.venv`, `venv`, `.next`, `.nuxt`, `.gradle`
- Editors and IDEs: `.vs`, `.idea`, `.vscode`
- Caches: `.cache`, `.pytest_cache`, `.mypy_cache`
- System files: `.DS_Store`, `Thumbs.db`, `desktop.ini`

Notes:

- The clicked folder itself is never ignored (right-click `node_modules` to see its content).
- Folders come first, then files, both sorted by name. Hidden files are shown. Symlinks are listed, not followed.
- If you select several files, Explorer runs CopyTree once per file and only the last path stays in the clipboard.

## Build

Needs Rust 1.79+ on Windows.

```
cargo build --release
```

The exe is `target\release\copytree.exe`. GitHub Actions also builds it on every push and uploads it as an artifact.

## Credits and license

- Author: john3sobieski
- Idea: solarch

[PolyForm Noncommercial License 1.0.0](LICENSE): free for noncommercial use.
