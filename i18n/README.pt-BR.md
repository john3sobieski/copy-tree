[English](../README.md) | [Русский](README.ru.md) | [Deutsch](README.de.md) | [Español](README.es.md) | **Português (Brasil)** | [简体中文](README.zh-CN.md) | [日本語](README.ja.md)

# CopyTree

Ferramenta do menu de contexto do Explorador de Arquivos do Windows: copia para a área de transferência a árvore de uma pasta ou o caminho completo de um arquivo. Útil para mostrar a estrutura de um projeto a uma IA.

- **Pasta** (ou espaço vazio dentro de uma pasta): copia uma visualização da pasta no estilo `tree`.
- **Arquivo**: copia o caminho absoluto em uma linha, por exemplo `C:\proj\src\main.rs`.

```
proj/
├── src/
│   ├── main.rs
│   └── tree.rs
├── Cargo.toml
└── README.md
```

Pastas ignoradas pela lista de ignorados ou por um `.gitignore` continuam aparecendo, marcadas como `node_modules/ [skipped]`, para que você veja que existem sem mostrar o conteúdo.

Itens do menu: "Copy folder tree" e "Copy file path" (copiar árvore da pasta / copiar caminho do arquivo). O menu e todas as caixas de diálogo seguem o idioma de exibição do Windows (inglês e russo já vêm incluídos, o inglês é o idioma de reserva). Em um Windows em português eles aparecerão em inglês.

## Instalação

1. Baixe o `copytree.exe` na página de Releases.
2. Dê um clique duplo. O exe se copia para `%LOCALAPPDATA%\CopyTree\` e adiciona os itens do menu. Não precisa de permissão de administrador.
3. Depois você pode mover ou apagar o exe baixado; o menu continua funcionando.

Feito para o Windows 10. No Windows 11, os itens devem aparecer em "Mostrar mais opções" (não testado).

## Atualizar e remover

Não há opções de linha de comando. Dê um clique duplo em um exe quando o CopyTree já estiver instalado:

- **Sim** em "Update it with this exe?" substitui o exe instalado. Suas configurações são mantidas.
- **Não** e depois **Sim** em "Remove CopyTree from the Explorer menu?" remove os itens e a pasta de instalação.

(Os textos das caixas de diálogo aparecem no idioma do Windows; em português, possivelmente em inglês.)

## Configurações

Arquivo: `%LOCALAPPDATA%\CopyTree\copytree.toml`, criado no primeiro uso com todas as configurações comentadas. Remova o `# ` do início de uma configuração para sobrescrevê-la. As mudanças valem no próximo clique. Apague o arquivo para restaurar os padrões.

| Configuração | Padrão | Significado |
|---|---|---|
| `max_depth` | `100` | Profundidade máxima de pastas (a pasta clicada é o nível 0). |
| `max_entries` | `1000000` | Número máximo de linhas; o restante é cortado com `... (truncated at N entries)`. |
| `use_gitignore` | `true` | Respeitar também os arquivos `.gitignore` encontrados dentro da pasta clicada. |
| `show_sizes` | `true` | Mostrar o tamanho após cada nome de arquivo, por exemplo `main.rs (1.2 KB)`. Pastas não têm tamanho. |
| `ignore` | veja abaixo | Nomes ou máscaras (como `*.pyc`) ignorados em qualquer profundidade. Pastas ignoradas aparecem como `name/ [skipped]`; arquivos ignorados ficam ocultos. Uma lista personalizada substitui a padrão. |

Lista `ignore` padrão:

- Controle de versão: `.git`, `.svn`, `.hg`
- Dependências e saída de build: `node_modules`, `target`, `dist`, `build`, `out`, `__pycache__`, `*.pyc`, `.venv`, `venv`, `.next`, `.nuxt`, `.gradle`
- Editores e IDEs: `.vs`, `.idea`, `.vscode`
- Caches: `.cache`, `.pytest_cache`, `.mypy_cache`
- Arquivos do sistema: `.DS_Store`, `Thumbs.db`, `desktop.ini`

Observações:

- A própria pasta clicada nunca é ignorada (clique com o botão direito em `node_modules` para ver o conteúdo).
- Pastas excluídas por um `.gitignore` também são marcadas como `[skipped]`.
- Primeiro vêm as pastas, depois os arquivos, ambos ordenados por nome. Arquivos ocultos são exibidos. Links simbólicos são listados, mas não seguidos.
- Se você selecionar vários arquivos, o Explorador executa o CopyTree uma vez por arquivo e só o último caminho fica na área de transferência.

## Traduções

Os itens do menu e as caixas de diálogo são recursos de texto dentro do exe (`assets/strings.rc`); o Windows escolhe o idioma que corresponde ao idioma de exibição do usuário. O texto do menu fica no registro como string indireta (`@exe,-ID`), então também acompanha uma troca de idioma sem reinstalar. A árvore em si (`[skipped]`, `... (truncated ...)`) é sempre em inglês.

Para adicionar um idioma, copie um bloco em `assets/strings.rc`, mude a linha `LANGUAGE` e traduza os textos. Os IDs devem permanecer iguais e coincidir com `src/strings.rs`.

As traduções deste README ficam na pasta [`i18n`](.).

## Compilação

Requer Rust 1.79+ no Windows.

```
cargo build --release
```

O exe fica em `target\release\copytree.exe`. O GitHub Actions também o compila a cada push e o envia como artefato.

## Créditos e licença

- Autor: john3sobieski
- Ideia: so1arch

[PolyForm Noncommercial License 1.0.0](../LICENSE.md): gratuita para uso não comercial.
