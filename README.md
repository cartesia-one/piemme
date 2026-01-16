# piemme

vibecoded vibecoding prompt manager for vibecoders

## Demo

| Navigation & UI | Create & Edit |
|:---:|:---:|
| ![Navigation](gifs/output/01-navigation.gif) | ![Create Edit](gifs/output/02-create-edit.gif) |

| References Power | Commands Power |
|:---:|:---:|
| ![References](gifs/output/03-references.gif) | ![Commands](gifs/output/04-commands.gif) |

| Tags & Filtering | Folders & Archive |
|:---:|:---:|
| ![Tags](gifs/output/05-tags-filtering.gif) | ![Folders Archive](gifs/output/06-folders-archive.gif) |

| Search & Actions |
|:---:|
| ![Search](gifs/output/07-search-actions.gif) |

## Features

- **Vim-like Navigation**: Navigate with `j`/`k`, go to first/last with `g`/`G`
- **Prompt Management**: Create, edit, delete, and archive prompts
- **Reference System**: Include other prompts with `[[prompt_name]]` syntax
- **Command Execution**: Embed shell commands with `{{command}}` syntax
- **Safe Mode**: Confirmation before executing embedded commands
- **Tag System**: Organize prompts with colored tags
- **Folder Organization**: Group related prompts in folders
- **Syntax Highlighting**: Visual indicators for references and commands
- **Clipboard Integration**: Copy rendered or raw prompts to clipboard

## Installation

### Quick Install (Linux/macOS)

```bash
curl -fsSL https://raw.githubusercontent.com/yourusername/piemme/main/install.sh | bash
```

### Download Binary

Download the latest release for your platform from the [Releases](https://github.com/yourusername/piemme/releases) page:

| Platform | Download |
|----------|----------|
| Linux (x86_64) | `piemme-linux-x86_64` |
| Linux (ARM64) | `piemme-linux-aarch64` |
| macOS (Intel) | `piemme-macos-x86_64` |
| macOS (Apple Silicon) | `piemme-macos-aarch64` |
| Windows (x86_64) | `piemme-windows-x86_64.exe` |

After downloading, make the binary executable (Linux/macOS):

```bash
chmod +x piemme-*
mv piemme-* ~/.local/bin/piemme
```

### From Source

Requires Rust 1.85+ (edition 2024)

```bash
git clone https://github.com/yourusername/piemme.git
cd piemme
cargo build --release
```

The binary will be available at `target/release/piemme`.

### Cargo Install

```bash
cargo install --git https://github.com/yourusername/piemme.git
```

## Usage

```bash
# Run in any directory - piemme creates a .piemme folder
piemme
```

### Keybindings

#### Normal Mode

| Key | Action |
|-----|--------|
| `j` / `↓` | Move down |
| `k` / `↑` | Move up |
| `g` | Go to first |
| `G` | Go to last |
| `Enter` / `i` | Enter insert mode |
| `n` | New prompt |
| `d` | Delete prompt |
| `a` | Archive prompt |
| `A` | Open archive |
| `y` | Copy rendered to clipboard |
| `p` | Preview mode |
| `t` | Tag selector |
| `!` | Toggle safe mode |
| `?` | Help |
| `q` | Quit |

#### Insert Mode

| Key | Action |
|-----|--------|
| `Esc` | Exit (save) |
| `Ctrl+s` | Save |
| `Ctrl+z` | Undo |
| `Ctrl+y` | Redo |

## Prompt Format

Prompts are stored as Markdown files with YAML frontmatter:

```markdown
---
id: "550e8400-e29b-41d4-a716-446655440000"
tags: ["coding", "python"]
created: "2026-01-15T10:30:00Z"
modified: "2026-01-15T14:22:00Z"
---
Your prompt content here...

Include another prompt: [[other_prompt_name]]

Include command output: {{ls -la}}
```

## Directory Structure

```
.piemme/
├── config.yaml          # User preferences
├── prompts/             # Main prompts
├── archive/             # Archived prompts
├── folders/             # Organized folders
└── .index.json          # Search index
```

## Configuration

Edit `.piemme/config.yaml`:

```yaml
safe_mode: true
tag_colors:
  coding: "blue"
  writing: "green"
  work: "yellow"
default_export_format: "rendered"
```

## Development Status

This project is under active development. See [TASKS.md](TASKS.md) for the roadmap.

## License

MIT
