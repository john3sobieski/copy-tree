[English](../README.md) | [Русский](README.ru.md) | **Deutsch** | [Español](README.es.md) | [Português (Brasil)](README.pt-BR.md) | [简体中文](README.zh-CN.md) | [日本語](README.ja.md)

# CopyTree

Rechtsklick-Werkzeug für den Windows-Explorer: kopiert den Verzeichnisbaum eines Ordners oder den vollständigen Pfad einer Datei in die Zwischenablage. Praktisch, um einer KI die Projektstruktur zu zeigen.

- **Ordner** (oder leere Fläche in einem Ordner): kopiert eine Ansicht des Ordners im Stil von `tree`.
- **Datei**: kopiert ihren absoluten Pfad in einer Zeile, z. B. `C:\proj\src\main.rs`.

```
proj/
├── src/
│   ├── main.rs
│   └── tree.rs
├── Cargo.toml
└── README.md
```

Ordner, die durch die Ignorierliste oder eine `.gitignore` übersprungen werden, erscheinen trotzdem mit der Markierung `node_modules/ [skipped]`. So sieht man, dass sie existieren, ohne ihren Inhalt.

Menüeinträge: „Copy folder tree“ und „Copy file path“ (Ordnerbaum kopieren / Dateipfad kopieren). Menü und alle Dialoge richten sich nach der Anzeigesprache von Windows (Englisch und Russisch sind eingebaut, Englisch ist die Rückfallsprache). Auf einem deutschen Windows erscheinen sie daher auf Englisch.

## Installation

1. Lade `copytree.exe` von der Releases-Seite herunter.
2. Doppelklicke die Datei. Die exe kopiert sich nach `%LOCALAPPDATA%\CopyTree\` und fügt die Menüeinträge hinzu. Administratorrechte sind nicht nötig.
3. Die heruntergeladene exe kann danach verschoben oder gelöscht werden, das Menü funktioniert weiter.

Ausgelegt für Windows 10. Unter Windows 11 sollten die Einträge unter „Weitere Optionen anzeigen“ erscheinen (nicht getestet).

## Aktualisieren und entfernen

Es gibt keine Kommandozeilenoptionen. Doppelklicke eine exe, wenn CopyTree bereits installiert ist:

- **Ja** bei „Update it with this exe?“ ersetzt die installierte exe. Deine Einstellungen bleiben erhalten.
- **Nein**, danach **Ja** bei „Remove CopyTree from the Explorer menu?“ entfernt die Einträge und den Installationsordner.

(Die Dialogtexte erscheinen in der Windows-Anzeigesprache, also auf Deutsch ggf. auf Englisch.)

## Einstellungen

Datei: `%LOCALAPPDATA%\CopyTree\copytree.toml`, wird bei der ersten Nutzung erstellt, alle Einstellungen sind auskommentiert. Entferne das führende `# ` einer Einstellung, um sie zu überschreiben. Änderungen gelten ab dem nächsten Klick. Lösche die Datei, um die Standardwerte wiederherzustellen.

| Einstellung | Standard | Bedeutung |
|---|---|---|
| `max_depth` | `100` | Maximale Ordnertiefe (der angeklickte Ordner ist Ebene 0). |
| `max_entries` | `1000000` | Maximale Zeilenzahl; der Rest wird mit `... (truncated at N entries)` abgeschnitten. |
| `use_gitignore` | `true` | Zusätzlich `.gitignore`-Dateien im angeklickten Ordner beachten. |
| `show_sizes` | `true` | Dateigröße hinter jedem Namen anzeigen, z. B. `main.rs (1.2 KB)`. Ordner haben keine Größe. |
| `ignore` | siehe unten | Namen oder Masken (wie `*.pyc`), die in jeder Tiefe übersprungen werden. Übersprungene Ordner werden als `name/ [skipped]` aufgeführt, übersprungene Dateien sind ausgeblendet. Eine eigene Liste ersetzt die Standardliste. |

Standardliste für `ignore`:

- Versionsverwaltung: `.git`, `.svn`, `.hg`
- Abhängigkeiten und Build-Ausgaben: `node_modules`, `target`, `dist`, `build`, `out`, `__pycache__`, `*.pyc`, `.venv`, `venv`, `.next`, `.nuxt`, `.gradle`
- Editoren und IDEs: `.vs`, `.idea`, `.vscode`
- Caches: `.cache`, `.pytest_cache`, `.mypy_cache`
- Systemdateien: `.DS_Store`, `Thumbs.db`, `desktop.ini`

Hinweise:

- Der angeklickte Ordner selbst wird nie ignoriert (Rechtsklick auf `node_modules` zeigt dessen Inhalt).
- Ordner, die durch eine `.gitignore` ausgeschlossen sind, werden ebenfalls mit `[skipped]` markiert.
- Zuerst kommen Ordner, dann Dateien, jeweils nach Name sortiert. Versteckte Dateien werden angezeigt. Symlinks werden aufgelistet, aber nicht verfolgt.
- Bei mehreren ausgewählten Dateien startet der Explorer CopyTree einmal pro Datei, und nur der letzte Pfad bleibt in der Zwischenablage.

## Übersetzungen

Menüeinträge und Dialoge sind Zeichenfolgenressourcen in der exe (`assets/strings.rc`); Windows wählt die Sprache passend zur Anzeigesprache des Benutzers. Der Menütext steht in der Registry als indirekte Zeichenfolge (`@exe,-ID`) und folgt daher auch einem Sprachwechsel ohne Neuinstallation. Der Baum selbst (`[skipped]`, `... (truncated ...)`) ist immer auf Englisch.

Um eine Sprache hinzuzufügen, kopiere einen Block in `assets/strings.rc`, ändere die `LANGUAGE`-Zeile und übersetze die Texte. Die IDs müssen gleich bleiben und zu `src/strings.rs` passen.

Übersetzungen dieser README liegen im Ordner [`i18n`](.).

## Build

Benötigt Rust 1.79+ unter Windows.

```
cargo build --release
```

Die exe liegt unter `target\release\copytree.exe`. GitHub Actions baut sie ebenfalls bei jedem Push und lädt sie als Artefakt hoch.

## Credits und Lizenz

- Autor: john3sobieski
- Idee: so1arch

[PolyForm Noncommercial License 1.0.0](../LICENSE.md): kostenlos für nichtkommerzielle Nutzung.
