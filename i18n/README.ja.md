[English](../README.md) | [Русский](README.ru.md) | [Deutsch](README.de.md) | [Español](README.es.md) | [Português (Brasil)](README.pt-BR.md) | [简体中文](README.zh-CN.md) | **日本語**

# CopyTree

Windows エクスプローラーの右クリックツールです。フォルダーのツリー、またはファイルのフルパスをクリップボードにコピーします。プロジェクトの構成を AI に見せたいときに便利です。

- **フォルダー**（またはフォルダー内の空白部分）: `tree` 形式のフォルダー構成をコピーします。
- **ファイル**: 絶対パスを 1 行でコピーします。例: `C:\proj\src\main.rs`

```
proj/
├── src/
│   ├── main.rs
│   └── tree.rs
├── Cargo.toml
└── README.md
```

無視リストや `.gitignore` でスキップされたフォルダーも、`node_modules/ [skipped]` と印を付けて表示されます。中身は出さずに、存在だけを確認できます。

メニュー項目: "Copy folder tree"（フォルダーツリーをコピー）と "Copy file path"（ファイルパスをコピー）。メニューとすべてのダイアログは Windows の表示言語に従います（英語とロシア語を内蔵、フォールバックは英語）。そのため日本語版 Windows では英語で表示されます。

## インストール

1. Releases ページから `copytree.exe` をダウンロードします。
2. ダブルクリックします。exe は `%LOCALAPPDATA%\CopyTree\` に自分自身をコピーし、メニュー項目を追加します。管理者権限は不要です。
3. ダウンロードした exe は移動や削除をしても、メニューは引き続き動作します。

Windows 10 向けです。Windows 11 では「その他のオプションを確認」の中に表示されるはずです（未テスト）。

## 更新と削除

コマンドラインオプションはありません。CopyTree がインストール済みの状態で exe をダブルクリックします。

- "Update it with this exe?" で**はい**を選ぶと、インストール済みの exe が置き換わります。設定は保持されます。
- **いいえ**を選び、続く "Remove CopyTree from the Explorer menu?" で**はい**を選ぶと、メニュー項目とインストールフォルダーが削除されます。

（ダイアログの文言は Windows の表示言語に従うため、日本語版では英語になる場合があります。）

## 設定

ファイル: `%LOCALAPPDATA%\CopyTree\copytree.toml`。初回使用時に作成され、すべての設定がコメントアウトされています。設定を上書きするには、行頭の `# ` を削除します。変更は次のクリックから反映されます。ファイルを削除すると既定値に戻ります。

| 設定 | 既定値 | 意味 |
|---|---|---|
| `max_depth` | `100` | フォルダーの最大の深さ（クリックしたフォルダーがレベル 0）。 |
| `max_entries` | `1000000` | 最大行数。超えた分は `... (truncated at N entries)` で切り捨てられます。 |
| `use_gitignore` | `true` | クリックしたフォルダー内にある `.gitignore` ファイルも尊重します。 |
| `show_sizes` | `true` | 各ファイル名の後にサイズを表示します。例: `main.rs (1.2 KB)`。フォルダーにはサイズがありません。 |
| `ignore` | 下記参照 | どの深さでもスキップする名前またはマスク（`*.pyc` など）。スキップされたフォルダーは `name/ [skipped]` と表示され、スキップされたファイルは非表示になります。独自のリストは既定のリストを置き換えます。 |

`ignore` の既定リスト:

- バージョン管理: `.git`, `.svn`, `.hg`
- 依存関係とビルド出力: `node_modules`, `target`, `dist`, `build`, `out`, `__pycache__`, `*.pyc`, `.venv`, `venv`, `.next`, `.nuxt`, `.gradle`
- エディターと IDE: `.vs`, `.idea`, `.vscode`
- キャッシュ: `.cache`, `.pytest_cache`, `.mypy_cache`
- システムファイル: `.DS_Store`, `Thumbs.db`, `desktop.ini`

補足:

- クリックしたフォルダー自体は無視されません（`node_modules` を右クリックすれば中身を確認できます）。
- `.gitignore` で除外されたフォルダーにも `[skipped]` の印が付きます。
- フォルダーが先、ファイルが後で、それぞれ名前順です。隠しファイルも表示されます。シンボリックリンクは一覧に出ますが、たどりません。
- 複数のファイルを選択した場合、エクスプローラーはファイルごとに CopyTree を 1 回ずつ実行し、クリップボードには最後のパスだけが残ります。

## 翻訳

メニュー項目とダイアログは exe 内の文字列リソース（`assets/strings.rc`）です。Windows はユーザーの表示言語に合う言語を選びます。メニューの文言はレジストリに間接文字列（`@exe,-ID`）として保存されるため、再インストールなしで言語の変更にも追従します。ツリー自体（`[skipped]`、`... (truncated ...)`）は常に英語です。

言語を追加するには、`assets/strings.rc` のブロックをコピーし、`LANGUAGE` 行を変更して文言を翻訳します。ID は変えず、`src/strings.rs` と一致させてください。

この README の翻訳は [`i18n`](.) フォルダーにあります。

## ビルド

Windows 上で Rust 1.79 以上が必要です。

```
cargo build --release
```

exe は `target\release\copytree.exe` にできます。GitHub Actions も push のたびにビルドし、アーティファクトとしてアップロードします。

## クレジットとライセンス

- 作者: john3sobieski
- アイデア: so1arch

[PolyForm Noncommercial License 1.0.0](../LICENSE.md): 非商用利用は無料です。
