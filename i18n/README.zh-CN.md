[English](../README.md) | [Русский](README.ru.md) | [Deutsch](README.de.md) | [Español](README.es.md) | [Português (Brasil)](README.pt-BR.md) | **简体中文** | [日本語](README.ja.md)

# CopyTree

Windows 资源管理器右键工具：把文件夹的目录树或文件的完整路径复制到剪贴板。适合把项目结构展示给 AI。

- **文件夹**（或文件夹内的空白处）：复制 `tree` 风格的文件夹目录树。
- **文件**：在一行内复制其绝对路径，例如 `C:\proj\src\main.rs`。

```
proj/
├── src/
│   ├── main.rs
│   └── tree.rs
├── Cargo.toml
└── README.md
```

被忽略列表或 `.gitignore` 跳过的文件夹仍会列出，并标记为 `node_modules/ [skipped]`，这样你能看到它们存在，但不会展开其内容。

菜单项："Copy folder tree"（复制文件夹目录树）和 "Copy file path"（复制文件路径）。菜单和所有对话框跟随 Windows 显示语言（内置英语和俄语，英语为后备语言）。因此在中文 Windows 上会显示英文。

## 安装

1. 从 Releases 页面下载 `copytree.exe`。
2. 双击运行。exe 会把自己复制到 `%LOCALAPPDATA%\CopyTree\` 并添加菜单项。无需管理员权限。
3. 之后可以移动或删除下载的 exe，菜单仍然可用。

为 Windows 10 设计。在 Windows 11 上，菜单项应出现在"显示更多选项"中（未测试）。

## 更新与卸载

没有命令行选项。CopyTree 已安装时，双击任意 exe：

- 在 "Update it with this exe?" 中选**是**，会替换已安装的 exe，设置保持不变。
- 选**否**，然后在 "Remove CopyTree from the Explorer menu?" 中选**是**，会移除菜单项和安装文件夹。

（对话框文字使用 Windows 显示语言，在中文系统上可能显示为英文。）

## 设置

文件：`%LOCALAPPDATA%\CopyTree\copytree.toml`，首次使用时创建，所有设置均已注释。删除某项设置开头的 `#` 即可覆盖它。更改在下一次点击时生效。删除该文件即可恢复默认值。

| 设置 | 默认值 | 含义 |
|---|---|---|
| `max_depth` | `100` | 最大文件夹深度（被点击的文件夹为第 0 层）。 |
| `max_entries` | `1000000` | 最大行数；超出部分会被截断，并显示 `... (truncated at N entries)`。 |
| `use_gitignore` | `true` | 同时遵守被点击文件夹内的 `.gitignore` 文件。 |
| `show_sizes` | `true` | 在每个文件名后显示文件大小，例如 `main.rs (1.2 KB)`。文件夹没有大小。 |
| `ignore` | 见下文 | 在任意深度被跳过的名称或通配符（如 `*.pyc`）。被跳过的文件夹显示为 `name/ [skipped]`，被跳过的文件会被隐藏。自定义列表会替换默认列表。 |

默认 `ignore` 列表：

- 版本控制：`.git`、`.svn`、`.hg`
- 依赖与构建输出：`node_modules`、`target`、`dist`、`build`、`out`、`__pycache__`、`*.pyc`、`.venv`、`venv`、`.next`、`.nuxt`、`.gradle`
- 编辑器与 IDE：`.vs`、`.idea`、`.vscode`
- 缓存：`.cache`、`.pytest_cache`、`.mypy_cache`
- 系统文件：`.DS_Store`、`Thumbs.db`、`desktop.ini`

说明：

- 被点击的文件夹本身永远不会被忽略（右键点击 `node_modules` 可查看其内容）。
- 被 `.gitignore` 排除的文件夹同样标记为 `[skipped]`。
- 先列文件夹，再列文件，各自按名称排序。隐藏文件会显示。符号链接只列出，不跟随。
- 如果选择多个文件，资源管理器会对每个文件各运行一次 CopyTree，剪贴板中只保留最后一个路径。

## 翻译

菜单项和对话框是 exe 内的字符串资源（`assets/strings.rc`）；Windows 会选择与用户显示语言匹配的语言。菜单文字以间接字符串（`@exe,-ID`）的形式存放在注册表中，因此切换语言后无需重新安装也会跟随变化。目录树本身（`[skipped]`、`... (truncated ...)`）始终为英文。

要添加语言，请复制 `assets/strings.rc` 中的一个块，修改 `LANGUAGE` 行并翻译文字。ID 必须保持不变，并与 `src/strings.rs` 一致。

本 README 的翻译位于 [`i18n`](.) 文件夹。

## 构建

需要在 Windows 上安装 Rust 1.79+。

```
cargo build --release
```

exe 位于 `target\release\copytree.exe`。GitHub Actions 也会在每次 push 时构建它，并作为构建产物上传。

## 作者与许可证

- 作者：john3sobieski
- 创意：so1arch

[PolyForm Noncommercial License 1.0.0](../LICENSE.md)：非商业用途免费。
