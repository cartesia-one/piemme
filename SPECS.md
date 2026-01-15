# Piemme - TUI Prompt Manager Specifications

## Overview

Piemme is a terminal-based user interface (TUI) application for managing, organizing, and composing reusable prompts. It provides a clean, minimal interface with vim-like keybindings for efficient prompt management.

---

## Technology Stack

| Component | Technology | Purpose |
|-----------|------------|---------|
| Language | Rust | Performance, safety, ecosystem |
| TUI Framework | `ratatui` | Terminal UI rendering |
| Terminal Backend | `crossterm` | Cross-platform terminal handling |
| Editor | `tui-textarea` | Multi-line text editing with undo/redo |
| Clipboard | `arboard` | Cross-platform clipboard access |
| Fuzzy Search | `nucleo` | Fast fuzzy matching |
| Markdown | `pulldown-cmark` | Parsing for syntax highlighting |
| Serialization | `serde` + `serde_yaml` | YAML frontmatter handling |
| File Watching | `notify` | External file change detection |
| UUID | `uuid` | Unique prompt identifiers |

---

## Data Storage

### Directory Structure

```
.piemme/
├── config.yaml              # User preferences, tag colors, settings
├── prompts/                  # Main prompts directory
│   ├── given_the_foll.md
│   ├── code_review.md
│   └── ...
├── archive/                  # Archived prompts (read-only view)
│   └── old_prompt.md
├── folders/                  # User-created organizational folders
│   ├── work/
│   │   └── meeting_notes.md
│   └── personal/
│       └── journal.md
└── .index.json              # Auto-generated cache for fast search
```

### Prompt File Format

Each prompt is stored as a Markdown file with YAML frontmatter:

```markdown
---
id: "550e8400-e29b-41d4-a716-446655440000"
tags: ["coding", "python"]
created: "2026-01-15T10:30:00Z"
modified: "2026-01-15T14:22:00Z"
---
Given the following number you must calculate the factorial...
```

### Metadata Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | UUID string | Yes | Unique identifier, auto-generated |
| `tags` | Array of strings | No | List of tag names |
| `created` | ISO 8601 timestamp | Yes | Creation date |
| `modified` | ISO 8601 timestamp | Yes | Last modification date |

### Config File Format (`config.yaml`)

```yaml
safe_mode: true
tag_colors:
  coding: "blue"
  writing: "green"
  work: "yellow"
  personal: "magenta"
default_export_format: "rendered"  # "rendered" or "raw"
```

---

## Naming Convention

### Auto-Generated Names

Prompt names are derived from the first line of content:

1. Take the first ~15-20 characters of content
2. Convert to lowercase
3. Replace spaces with underscores
4. Remove special characters (keep only `a-z`, `0-9`, `_`)
5. Truncate to create a short, readable name
6. If empty content, use `empty_prompt_<number>`

**Example:**
- Content: `"Given the following number you must...."`
- Name: `given_the_foll`

### Uniqueness Handling

If a generated name already exists:
1. Append `_1`, `_2`, etc. until unique
2. Example: `given_the_foll`, `given_the_foll_1`, `given_the_foll_2`

Names must be unique across ALL prompts (main, archived, and in folders).

---

## User Interface

### Layout

```
┌─ Piemme ─────────────────────────────────────────────────────────┐
│ 📁 /work                                     [🔒 Safe Mode: ON] │
├──────────────────┬───────────────────────────────────────────────┤
│ Prompts (12)     │ # given_the_foll                              │
│──────────────────│───────────────────────────────────────────────│
│ ● given_the_foll │ Given the following number you must           │
│   code_review    │ calculate the factorial using recursion.      │
│   explain_like   │                                               │
│   debug_this     │ Include:                                      │
│   ○ meeting_sum  │ - [[code_review]]                             │
│                  │ - Current files: {{ls -la}}                   │
│                  │                                               │
├──────────────────┴───────────────────────────────────────────────┤
│ [NORMAL] Tags: coding, python │ 42 prompts │ 3 archived │ 5 tags│
└──────────────────────────────────────────────────────────────────┘
```

### UI Components

1. **Title Bar**: Application name, current folder path, safe mode indicator
2. **Left Panel**: Scrollable list of prompts with tag color indicators
3. **Right Panel**: Editor/viewer for selected prompt content
4. **Status Bar**: Current mode, prompt tags, statistics

### Visual Indicators

- **Tag Colors**: Colored dot/bullet next to prompt name
- **Selected Prompt**: Highlighted background
- **Current Folder**: Shown in title bar with 📁 icon
- **Safe Mode**: 🔒 icon when ON, 🔓 when OFF
- **Archive View**: Different title/color scheme to distinguish
- **Mode Indicator**: `[NORMAL]`, `[INSERT]`, `[ARCHIVE]`, `[FOLDER]`

---

## Syntax Highlighting

### Color Scheme

| Element | Color | Description |
|---------|-------|-------------|
| `[[valid_ref]]` | Green | Valid reference to existing prompt |
| `[[invalid_ref]]` | Red | Reference to non-existent prompt |
| `{{command}}` | Yellow/Orange | Shell command (warning color) |
| Tags | Per-tag color | Configurable in config.yaml |
| Selection | Inverted | Selected text in editor |

---

## Application Modes

### 1. Normal Mode (Default)
- Navigate prompts
- Execute single-key commands
- View prompt content (read-only in right panel)

### 2. Insert Mode
- Edit prompt content in right panel
- Standard text editing keybindings
- Exit with `Esc`

### 3. Archive Mode
- View archived prompts (read-only)
- Limited actions: unarchive, delete
- Exit with `Esc`

### 4. Folder Mode
- View prompts within a specific folder
- All normal operations available
- Exit with `Esc` to return to root

### 5. Preview Mode
- Shows rendered output (references and commands resolved)
- Read-only view
- Exit with `Esc` or `p`

---

## Keybindings

### Global (All Modes)

| Key | Action |
|-----|--------|
| `?` | Open help overlay with all keybindings |
| `Ctrl+c` | Quit application (with confirmation if unsaved) |
| `q` | Quit application (with confirmation if unsaved) |

### Normal Mode

| Key | Action |
|-----|--------|
| `j` / `↓` | Select next prompt (down) |
| `k` / `↑` | Select previous prompt (up) |
| `g` | Go to first prompt |
| `G` | Go to last prompt |
| `Enter` / `i` | Enter insert mode (edit selected prompt) |
| `n` | Create new prompt |
| `r` | Rename selected prompt |
| `d` | Delete selected prompt (with confirmation) |
| `y` | Copy rendered prompt to clipboard |
| `p` | Toggle preview mode |
| `a` | Archive selected prompt |
| `A` | Open archive view |
| `t` | Tag selected prompt (opens tag selector) |
| `M` | Move prompt to folder |
| `O` | Open folder (shows folder selector) |
| `/` | Open fuzzy search |
| `Ctrl+p` | Quick open (fuzzy find prompt by name) |
| `[` | Filter by previous tag |
| `]` | Filter by next tag |
| `Tab` | Toggle focus between list and editor |
| `!` | Toggle safe mode |
| `e` | Export prompt |
| `Ctrl+d` | Duplicate selected prompt |

### Insert Mode

| Key | Action |
|-----|--------|
| `Esc` | Exit insert mode, save changes |
| `Ctrl+s` | Save changes (explicit) |
| `Ctrl+z` | Undo |
| `Ctrl+y` | Redo |
| `Ctrl+l` | Quick insert reference (fuzzy search prompts) |
| `Ctrl+←` | Move cursor word left |
| `Ctrl+→` | Move cursor word right |
| `Home` | Move to line start |
| `End` | Move to line end |
| `Ctrl+Home` | Move to document start |
| `Ctrl+End` | Move to document end |

### Archive Mode

| Key | Action |
|-----|--------|
| `j` / `↓` | Select next archived prompt |
| `k` / `↑` | Select previous archived prompt |
| `u` | Unarchive selected prompt (move back to main) |
| `Delete` | Permanently delete prompt (with confirmation) |
| `Esc` | Exit archive mode |

### Folder Mode

| Key | Action |
|-----|--------|
| `Esc` | Go back to parent/root folder |
| (All normal mode keys available) | |

### Popup/Selector Overlays

| Key | Action |
|-----|--------|
| `j` / `↓` | Next item |
| `k` / `↑` | Previous item |
| `Enter` | Select/confirm |
| `Esc` | Cancel/close |
| (typing) | Filter/fuzzy search |
| `Ctrl+n` | Create new (tag/folder) |

---

## Prompt Engine

### Reference Resolution (`[[]]`)

Prompts can include references to other prompts using `[[prompt_name]]` syntax.

**Behavior:**
- References are resolved **only when copying** to clipboard
- In editor, references display with syntax highlighting
- Valid references: green
- Invalid references: red

**Example:**
```markdown
# In editor:
Please review this code:
[[code_review]]

# After copy (clipboard content):
Please review this code:
You are an expert code reviewer. Analyze the following code...
```

### Circular Reference Protection

- Maximum resolution depth: 10 levels
- If circular reference detected: stop resolution, include warning comment
- Example output: `<!-- [CIRCULAR REFERENCE DETECTED: prompt_name] -->`

### Command Execution (`{{}}`)

Prompts can include shell commands using `{{command}}` syntax.

**Behavior:**
- Commands are executed **only when copying** to clipboard
- Command output replaces the `{{}}` block in copied text
- Commands run in the current working directory

**Safe Mode (Default: ON):**
- When ON: Show confirmation dialog before executing any commands
- Dialog lists all commands to be executed
- User must confirm with `y` or cancel with `n`/`Esc`

**Safe Mode OFF:**
- Commands execute immediately without confirmation
- Visual warning in status bar when safe mode is off

**Example:**
```markdown
# In editor:
Current directory contents:
{{ls -la}}

# After copy (with command executed):
Current directory contents:
total 24
drwxr-xr-x  5 user user 4096 Jan 15 10:00 .
drwxr-xr-x 20 user user 4096 Jan 15 09:00 ..
-rw-r--r--  1 user user  256 Jan 15 10:00 file.txt
```

### Fuzzy Helper for References

When typing `[[` in insert mode:
1. Popup appears with fuzzy-searchable list of all prompts
2. Continue typing to filter
3. `Enter` to insert selected reference
4. `Esc` to cancel
5. Popup shows prompt name and first line preview

---

## Features

### Fuzzy Search (`/` or `Ctrl+p`)

- Search across prompt names AND content
- Results ranked by relevance
- Shows matching snippet preview
- `Enter` to jump to selected prompt

### Export (`e`)

Export dialog with options:
1. **Copy Rendered** - Copy with all references/commands resolved
2. **Copy Raw** - Copy original markdown without resolution
3. **Save to File (Rendered)** - Save resolved content to external file
4. **Save to File (Raw)** - Save original markdown to external file

File export prompts for filename and location.

### Prompt Statistics

Displayed in status bar:
- Total prompt count
- Archived prompt count
- Tag count
- Current folder prompt count (when in folder)

### Tag Management

**Creating Tags:**
- Press `t` on a prompt
- Type new tag name or select existing
- `Ctrl+n` in tag selector to create new tag

**Tag Colors:**
- Defined in `config.yaml`
- Default colors assigned if not specified
- Colors cycle through palette for new tags

**Filtering by Tag:**
- `[` / `]` to cycle through tag filters
- Shows only prompts with selected tag
- "All" option to show all prompts

---

## Error Handling

### File Operations
- Show error notification for failed reads/writes
- Auto-save with backup before overwrite
- Detect external file changes and prompt for reload

### Invalid References
- Highlight in red
- Show warning in status bar
- Still allow copy (reference text included as-is)

### Command Failures
- Show error message with command output
- Include error in copied text as comment: `<!-- Command failed: ... -->`
- Log errors to `.piemme/error.log`

---

## Initialization

### First Run
1. Create `.piemme/` directory structure if not exists
2. Create default `config.yaml`
3. Show welcome message with keybinding hints

### Startup
1. Load config
2. Scan and index all prompts
3. Build tag list
4. Display main view with prompts sorted alphabetically

---

## Performance Considerations

- Index file (`.index.json`) for fast fuzzy search
- Lazy loading of prompt content
- Debounced search input
- Efficient re-rendering (only changed components)
- File watcher for external changes

---

## Future Considerations (Not in Initial Release)

- Variables/placeholders (`<<variable>>`)
- Favorites/starred prompts
- Sync/backup to cloud
- Import from other formats
- Prompt templates
- Version history
- Collaborative editing
