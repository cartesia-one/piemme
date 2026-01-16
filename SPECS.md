# Piemme - TUI Prompt Manager Specifications

## Overview

Piemme is a terminal-based user interface (TUI) application for managing, organizing, and composing reusable prompts. It provides a clean, minimal interface with vim-like keybindings for efficient prompt management.

---

## Demo GIFs

GIF demos are located in `gifs/output/` and showcase all major features:

| GIF | Features Demonstrated |
|-----|----------------------|
| `01-navigation.gif` | Basic navigation (j/k/g/G), panel resize (Ctrl+l/h), help overlay |
| `02-create-edit.gif` | Create prompts (n), vim editing (i/o/dd/u), save with Esc |
| `03-references.gif` | `[[reference]]` syntax, Ctrl+r insertion, preview mode, prompt composition |
| `04-commands.gif` | `{{command}}` syntax, safe mode toggle (!), command execution on copy |
| `05-tags-filtering.gif` | Tag selector (t), create tags (Ctrl+n), filter cycling ([/]) |
| `06-folders-archive.gif` | Folder navigation (O), move to folder (M), archive (a/A/u) |
| `07-search-actions.gif` | Fuzzy search (/), quick open (Ctrl+p), rename (r), delete (d), duplicate (Ctrl+d) |

To regenerate GIFs: `cd gifs && ./generate-all.sh` (requires [vhs](https://github.com/charmbracelet/vhs))

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
┌─ piemme ─────────────────────────────────────────────────────────┐
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

## Mouse Support

Mouse support is limited in the editor due to terminal constraints:

- **Mouse Scroll**: Scroll the editor content up/down with scroll wheel

Note: Mouse click/drag text selection is not supported by the underlying editor library.
Use keyboard shortcuts for text selection instead:
- `Shift+Arrow keys`: Select text character by character
- `Shift+Ctrl+Arrow keys`: Select text word by word  
- `Shift+Home/End`: Select to beginning/end of line
- `Ctrl+a`: Select all text

After selecting text, use `Ctrl+c` to copy and `Ctrl+v` to paste.

---

## Application Modes

### 1. Normal Mode (Default)
- Navigate prompts
- Execute single-key commands
- View prompt content (read-only in right panel)

### 2. Insert Mode (Editor Mode)
- Opens the editor for the selected prompt
- **Starts in Vim Normal mode** (not typing mode)
- Contains three sub-modes:

#### 2a. Vim Normal Mode (Editor Default)
- Navigate within the editor using vim-style movements
- Press `i` to enter Vim Insert mode for typing
- Press `Esc` to exit editor and save

#### 2b. Vim Insert Mode
- Actually type and edit text
- Press `Esc` to return to Vim Normal mode
- Standard editor shortcuts (Ctrl+C, Ctrl+V, etc.) work here

#### 2c. Vim Visual Mode
- Select text using vim movements
- Enter with `v` (character) or `V` (line)
- Operate on selection with `d`, `y`, `c`

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
| `?` | Open help overlay (except when typing in Insert mode) |
| `Ctrl+c` | Quit application (with confirmation if unsaved) |
| `q` | Quit application (with confirmation if unsaved) |
| `Ctrl+y` | Copy rendered prompt to clipboard (overrides vim-style y) |
| `Ctrl+l` | Increase left column width (Normal mode) |
| `Ctrl+h` | Decrease left column width (Normal mode) |

### Normal Mode

| Key | Action |
|-----|--------|
| `j` / `↓` | Select next prompt (down) |
| `k` / `↑` | Select previous prompt (up) |
| `g` | Go to first prompt |
| `G` | Go to last prompt |
| `Enter` / `i` | Enter editor (Vim Normal mode) |
| `n` | Create new prompt (enters Vim Insert mode directly) |
| `r` | Rename selected prompt (opens rename popup with validation) |
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
| `Ctrl+d` | Duplicate selected prompt |

### Insert Mode (Editor)

The editor uses a **hybrid Vim/normal editing model**. When you press Enter on a prompt, you enter the editor in **Vim Normal mode** - you need to press `i` to start typing.

#### Editor - Vim Normal Mode

| Key | Action |
|-----|--------|
| `Esc` | Exit editor (save and return to Normal mode) |
| `q` | Quit application (from Vim Normal mode) |
| `i` | Enter Vim Insert mode (at cursor) |
| `I` | Enter Vim Insert mode (at line start) |
| `a` | Append after cursor |
| `A` | Append at end of line |
| `o` | Open new line below and insert |
| `O` | Open new line above and insert |
| `h` / `←` | Move cursor left |
| `j` / `↓` | Move cursor down |
| `k` / `↑` | Move cursor up |
| `l` / `→` | Move cursor right |
| `w` | Move to next word |
| `b` | Move to previous word |
| `e` | Move to end of word |
| `0` / `Home` | Move to line start |
| `^` | Move to first non-blank character |
| `$` / `End` | Move to line end |
| `gg` | Move to file start |
| `G` | Move to file end |
| `x` / `Delete` | Delete character under cursor |
| `d` | Start delete operator (waits for motion: `dw`, `d$`, `d{`, etc.) |
| `dd` | Delete entire line |
| `D` | Delete to end of line |
| `c` | Start change operator (waits for motion: `cw`, `c$`, `c{`, etc.) |
| `cc` | Change entire line |
| `C` | Change to end of line |
| `u` | Undo |
| `y` | Start yank operator (waits for motion: `yw`, `y$`, `y{`, etc.) |
| `yy` | Yank (copy) current line to internal buffer |
| `p` | Put (paste) from internal buffer after cursor |
| `P` | Put (paste) from internal buffer before cursor |
| `{` | Move to previous paragraph (empty line) |
| `}` | Move to next paragraph (empty line) |
| `v` | Enter Visual mode (character-wise) |
| `V` | Enter Visual Line mode |
| `Shift+Arrow` | Extend selection (hybrid) |
| `r` / `Ctrl+r` | Open reference insertion popup |
| `Ctrl+f` | Open file picker popup |
| `?` | Open help |

**Note on Vim Operators:**
The `d`, `c`, and `y` keys enter "operator-pending" mode, waiting for a motion:
- `dw` - delete word
- `cw` - change word (delete and enter insert mode)
- `yw` - yank word
- `d$` - delete to end of line
- `d{` - delete to previous paragraph
- `y}` - yank to next paragraph
- And many more combinations...

**Note on Yank/Paste:**
Vim-style `y`/`p`/`P` operations use an internal buffer (not the system clipboard).
Use `Ctrl+c`/`Ctrl+v` in Insert mode to interact with the system clipboard.
Use `Ctrl+y` from any mode to copy the rendered prompt to the system clipboard.

#### Editor - Vim Insert Mode

| Key | Action |
|-----|--------|
| `Esc` | Return to Vim Normal mode |
| `Ctrl+s` | Save changes |
| `Ctrl+z` | Undo |
| `Ctrl+a` | Select all text |
| `Ctrl+c` | Copy selected text to system clipboard |
| `Ctrl+v` | Paste from system clipboard |
| `Ctrl+r` | Open reference insertion popup |
| `Ctrl+f` | Open file picker popup |
| `Shift+Arrow` | Extend text selection (hybrid) |
| (typing) | Insert text normally |

#### Editor - Visual Mode

| Key | Action |
|-----|--------|
| `Esc` | Exit to Vim Normal mode |
| `h/j/k/l` or arrows | Extend selection |
| `w/b/e/0/$` | Extend selection by word/line |
| `{/}` | Extend selection by paragraph |
| `d` / `x` | Delete selection (saves to internal buffer) |
| `c` | Change selection (delete and enter Insert) |
| `y` | Yank (copy) selection to internal buffer |
| `v` | Toggle Visual mode off |
| `V` | Switch to Visual Line mode |
| `Ctrl+a` | Select all |

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

### Rename Popup

When pressing `r` on a prompt, a rename popup appears:
- The popup shows the current name initially
- Type to modify the name (characters are appended)
- Valid characters: `a-z`, `0-9`, `_`
- The input box turns red if the name is invalid (invalid characters or already exists)
- Press `Enter` to confirm, `Esc` to cancel

### Reference Insertion Popup

When pressing `Ctrl+r` in Insert mode:
- A fuzzy finder popup appears with all prompt names
- Type to filter the list
- Use `↑`/`↓` to navigate
- Press `Enter` to insert `[[selected_prompt]]` at cursor position
- Press `Esc` to cancel

### File Picker Popup

When pressing `Ctrl+f` in Insert mode:
- A fuzzy finder popup appears with files from current directory
- Files are listed recursively (up to 3 levels deep)
- Hidden directories and build artifacts (`.git`, `target`, `node_modules`, etc.) are excluded
- Type to filter by file path
- Use `↑`/`↓` to navigate
- Press `Enter` to insert `[[file:path/to/file]]` at cursor position
- Press `Esc` to cancel

---

## Editor Behavior (Insert Mode)

The editor uses a **hybrid Vim/normal editing model** that combines:
- Vim-style modal editing (Normal/Insert/Visual modes within the editor)
- Standard editor shortcuts (Ctrl+C, Ctrl+V, Shift+Arrow selection)

### Entering the Editor

| From | Action | Result |
|------|--------|--------|
| Normal Mode | Press `Enter` or `i` | Enter editor in **Vim Normal mode** (cursor at start) |
| Normal Mode | Press `n` (new prompt) | Enter editor in **Vim Insert mode** (ready to type) |

### Editor Sub-Modes

```
┌─────────────────────────────────────────────────────────────┐
│                      EDITOR FLOW                             │
│                                                              │
│   Normal Mode ──Enter/i──► Vim Normal ◄──Esc──┐             │
│       ▲                        │               │             │
│       │                        i               │             │
│      Esc                       ▼               │             │
│       │                   Vim Insert ──────────┤             │
│       │                        │               │             │
│       │                       Esc              │             │
│       │                        ▼               │             │
│       └────────────────── Vim Normal           │             │
│                               │                │             │
│                              v/V               │             │
│                               ▼                │             │
│                          Vim Visual ───────────┘             │
└─────────────────────────────────────────────────────────────┘
```

### Visual Feedback

- **Border color** changes based on editor sub-mode:
  - Blue: Vim Normal mode
  - Green: Vim Insert mode  
  - Magenta: Vim Visual mode
- **Status bar** shows current sub-mode: `[NORMAL] EDITING`, `[INSERT] EDITING`, `[VISUAL] EDITING`
- **Cursor style** differs per mode (block for Normal, line for Insert)

### Hybrid Features

The editor supports both Vim and traditional editing paradigms:

| Feature | Vim Style | Traditional Style |
|---------|-----------|-------------------|
| Selection | `v` Visual mode + movements | `Shift+Arrow` keys |
| Select All | (manual) | `Ctrl+a` |
| Copy | `y` (yank to internal buffer) | `Ctrl+c` (to system clipboard) |
| Paste | `p` (put from internal buffer) | `Ctrl+v` (from system clipboard) |
| Undo | `u` | `Ctrl+z` |
| Copy Rendered Prompt | N/A | `Ctrl+y` (global, all modes) |

All changes are auto-saved when exiting the editor (pressing `Esc` in Vim Normal mode).

---

## Prompt Engine

### Reference Resolution (`[[]]`)

Prompts can include references to other prompts or local files.

**Prompt References:**
- Use `[[prompt_name]]` syntax to reference other prompts
- References are resolved **only when copying** to clipboard
- In editor, references display with syntax highlighting
- Valid references: green
- Invalid references: red

**File References:**
- Use `[[file:path/to/file]]` syntax to embed file content
- Files are resolved **only when copying** to clipboard  
- File paths are relative to current working directory
- In editor, file references display with syntax highlighting
- Valid files (exist and readable): green
- Invalid files (missing or unreadable): red
- On error, replaced with comment: `<!-- [FILE ERROR: path - error message] -->`

**Example:**
```markdown
# In editor:
Please review this code:
[[code_review]]

Source file:
[[file:src/main.rs]]

# After copy (clipboard content):
Please review this code:
You are an expert code reviewer. Analyze the following code...

Source file:
fn main() {
    println!("Hello, world!");
}
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

---

## Future Considerations (Not in Initial Release)

- Variables/placeholders (`<<variable>>`)
- Favorites/starred prompts
- Sync/backup to cloud
- Import from other formats
- Prompt templates
- Version history
- Collaborative editing
