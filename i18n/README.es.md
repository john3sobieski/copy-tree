[English](../README.md) | [Русский](README.ru.md) | [Deutsch](README.de.md) | **Español** | [Português (Brasil)](README.pt-BR.md) | [简体中文](README.zh-CN.md) | [日本語](README.ja.md)

# CopyTree

Herramienta del menú contextual del Explorador de Windows: copia al portapapeles el árbol de una carpeta o la ruta completa de un archivo. Útil para mostrar la estructura de un proyecto a una IA.

- **Carpeta** (o espacio vacío dentro de una carpeta): copia una vista de la carpeta al estilo `tree`.
- **Archivo**: copia su ruta absoluta en una sola línea, p. ej. `C:\proj\src\main.rs`.

```
proj/
├── src/
│   ├── main.rs
│   └── tree.rs
├── Cargo.toml
└── README.md
```

Las carpetas omitidas por la lista de ignorados o por un `.gitignore` se muestran igualmente con la marca `node_modules/ [skipped]`, para ver que existen sin mostrar su contenido.

Entradas del menú: «Copy folder tree» y «Copy file path» (copiar árbol de carpeta / copiar ruta de archivo). El menú y todos los diálogos siguen el idioma de Windows (inglés y ruso vienen incluidos, el inglés es el idioma de respaldo). En un Windows en español aparecerán en inglés.

## Instalación

1. Descarga `copytree.exe` desde la página de Releases.
2. Haz doble clic. El exe se copia a `%LOCALAPPDATA%\CopyTree\` y añade las entradas del menú. No se necesitan permisos de administrador.
3. Después puedes mover o borrar el exe descargado; el menú sigue funcionando.

Diseñado para Windows 10. En Windows 11 las entradas deberían aparecer en «Mostrar más opciones» (no probado).

## Actualizar y desinstalar

No hay opciones de línea de comandos. Haz doble clic en un exe cuando CopyTree ya esté instalado:

- **Sí** en «Update it with this exe?» reemplaza el exe instalado. Tus ajustes se conservan.
- **No** y luego **Sí** en «Remove CopyTree from the Explorer menu?» elimina las entradas y la carpeta de instalación.

(Los textos de los diálogos aparecen en el idioma de Windows; en español, posiblemente en inglés.)

## Ajustes

Archivo: `%LOCALAPPDATA%\CopyTree\copytree.toml`, se crea en el primer uso con todos los ajustes comentados. Quita el `# ` inicial de un ajuste para sobrescribirlo. Los cambios se aplican en el siguiente clic. Borra el archivo para restaurar los valores por defecto.

| Ajuste | Por defecto | Significado |
|---|---|---|
| `max_depth` | `100` | Profundidad máxima de carpetas (la carpeta pulsada es el nivel 0). |
| `max_entries` | `1000000` | Número máximo de líneas; el resto se corta con `... (truncated at N entries)`. |
| `use_gitignore` | `true` | Respetar también los archivos `.gitignore` dentro de la carpeta pulsada. |
| `show_sizes` | `true` | Mostrar el tamaño tras cada nombre de archivo, p. ej. `main.rs (1.2 KB)`. Las carpetas no tienen tamaño. |
| `ignore` | ver abajo | Nombres o máscaras (como `*.pyc`) omitidos a cualquier profundidad. Las carpetas omitidas se listan como `name/ [skipped]`; los archivos omitidos se ocultan. Una lista propia reemplaza la de por defecto. |

Lista `ignore` por defecto:

- Control de versiones: `.git`, `.svn`, `.hg`
- Dependencias y resultados de compilación: `node_modules`, `target`, `dist`, `build`, `out`, `__pycache__`, `*.pyc`, `.venv`, `venv`, `.next`, `.nuxt`, `.gradle`
- Editores e IDE: `.vs`, `.idea`, `.vscode`
- Cachés: `.cache`, `.pytest_cache`, `.mypy_cache`
- Archivos del sistema: `.DS_Store`, `Thumbs.db`, `desktop.ini`

Notas:

- La carpeta pulsada nunca se ignora (haz clic derecho en `node_modules` para ver su contenido).
- Las carpetas excluidas por un `.gitignore` también se marcan como `[skipped]`.
- Primero van las carpetas y luego los archivos, ambos ordenados por nombre. Los archivos ocultos se muestran. Los enlaces simbólicos se listan, pero no se siguen.
- Si seleccionas varios archivos, el Explorador ejecuta CopyTree una vez por archivo y en el portapapeles solo queda la última ruta.

## Traducciones

Las entradas del menú y los diálogos son recursos de cadenas dentro del exe (`assets/strings.rc`); Windows elige el idioma que coincide con el idioma de pantalla del usuario. El texto del menú se guarda en el registro como cadena indirecta (`@exe,-ID`), así que también sigue un cambio de idioma sin reinstalar. El árbol en sí (`[skipped]`, `... (truncated ...)`) siempre está en inglés.

Para añadir un idioma, copia un bloque de `assets/strings.rc`, cambia la línea `LANGUAGE` y traduce los textos. Los ID deben seguir igual y coincidir con `src/strings.rs`.

Las traducciones de este README están en la carpeta [`i18n`](.).

## Compilación

Requiere Rust 1.79+ en Windows.

```
cargo build --release
```

El exe queda en `target\release\copytree.exe`. GitHub Actions también lo compila en cada push y lo sube como artefacto.

## Créditos y licencia

- Autor: john3sobieski
- Idea: so1arch

[PolyForm Noncommercial License 1.0.0](../LICENSE.md): gratis para uso no comercial.
