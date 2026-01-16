# Piemme - Development Tasks

## Phase 1: Project Setup & Foundation

### 1.1 Project Initialization
- [x] Initialize Rust project with `cargo init`
- [x] Set up Cargo.toml with dependencies:
  - `ratatui` (TUI framework)
  - `crossterm` (terminal backend)
  - `tui-textarea` (editor component)
  - `arboard` (clipboard)
  - `nucleo` (fuzzy search)
  - `pulldown-cmark` (markdown parsing)
  - `serde`, `serde_yaml`, `serde_json` (serialization)
  - `uuid` (unique IDs)
  - `chrono` (timestamps)
  - `directories` (platform paths)
  - `anyhow` (error handling)
  - `thiserror` (custom errors)
- [x] Set up project structure (src/main.rs, modules)
- [x] Configure rustfmt.toml and clippy

### 1.2 Core Data Structures
- [x] Define `Prompt` struct (id, name, content, tags, timestamps)
- [x] Define `Config` struct (safe_mode, tag_colors, settings)
- [x] Define `AppState` struct (mode, selected_index, prompts, etc.)
- [x] Define `Mode` enum (Normal, Insert, Archive, Folder, Preview)
- [x] Define `Action` enum for all possible user actions
- [x] Implement serialization/deserialization for all structs

---

## Phase 2: File System Layer

### 2.1 Directory Management
- [x] Implement `.piemme/` directory initialization
- [x] Create subdirectories (prompts/, archive/, folders/)
- [x] Implement directory existence checks
- [x] Handle first-run setup

### 2.2 Config Management
- [x] Load config from `config.yaml`
- [x] Create default config if not exists
- [x] Implement config save
- [x] Validate config values

### 2.3 Prompt File Operations
- [x] Implement YAML frontmatter parsing
- [x] Implement YAML frontmatter writing
- [x] Load single prompt from file
- [x] Save single prompt to file
- [x] Load all prompts from directory
- [x] Handle file read/write errors gracefully

### 2.4 Prompt Name Generation
- [x] Implement auto-name generation from content
- [x] Implement uniqueness checking across all prompts
- [x] Implement suffix appending for duplicates
- [x] Handle empty content case (`empty_prompt_<n>`)

### 2.5 Index Management
- [x] Design index structure (`.index.json`)
- [x] Build index from prompts
- [x] Save index to file
- [x] Load index on startup
- [x] Update index on prompt changes

---

## Phase 3: TUI Framework Setup

### 3.1 Application Scaffolding
- [x] Set up main event loop
- [x] Initialize crossterm terminal
- [x] Set up panic handler (restore terminal on crash)
- [x] Implement clean shutdown

### 3.2 Basic Layout
- [x] Create main layout (title, left panel, right panel, status bar)
- [x] Implement responsive sizing
- [x] Add borders and styling

### 3.3 Component Architecture
- [x] Create `TitleBar` component
- [x] Create `PromptList` component
- [x] Create `Editor` component (wrapper around tui-textarea)
- [x] Create `StatusBar` component
- [x] Create `HelpOverlay` component
- [x] Create `Popup` component (for selectors)

---

## Phase 4: Core Navigation & Display

### 4.1 Prompt List
- [x] Display list of prompts
- [x] Implement scrolling
- [x] Highlight selected prompt
- [x] Show tag color indicators
- [x] Display prompt count in header

### 4.2 Navigation
- [x] Implement `j`/`↓` - move down
- [x] Implement `k`/`↑` - move up
- [x] Implement `g` - go to first
- [x] Implement `G` - go to last
- [x] Handle empty list edge case

### 4.3 Editor Display
- [x] Display selected prompt content
- [x] Show prompt name as header
- [x] Implement read-only view for Normal mode
- [x] Handle long content with scrolling

### 4.4 Status Bar
- [x] Display current mode
- [x] Display selected prompt tags
- [x] Display statistics (prompt count, archived, tags)
- [x] Display safe mode indicator

---

## Phase 5: Syntax Highlighting

### 5.1 Reference Highlighting
- [x] Parse content for `[[...]]` patterns
- [x] Validate references against existing prompts
- [x] Apply green color for valid references
- [x] Apply red color for invalid references

### 5.2 Command Highlighting
- [x] Parse content for `{{...}}` patterns
- [x] Apply yellow/orange color for commands
- [x] Add visual warning indicator

### 5.3 Integration
- [x] Apply highlighting in editor view
- [x] Apply highlighting in preview mode (N/A - preview shows resolved content)
- [x] Update highlighting on content change (done via re-render)

---

## Phase 6: Insert Mode & Editing

### 6.1 Mode Switching
- [x] Implement `Enter`/`i` to enter Insert mode
- [x] Implement `Esc` to exit Insert mode
- [x] Visual mode indicator update
- [x] Position cursor at start of file when entering editor

### 6.2 Vim-Style Editor Sub-Modes
- [x] Define `EditorMode` enum (VimNormal, VimInsert, VimVisual, VimVisualLine)
- [x] Add `editor_mode` to AppState
- [x] Enter editor in Vim Normal mode by default
- [x] Implement `i` to enter Vim Insert mode from Vim Normal
- [x] Implement `I` for insert at line start
- [x] Implement `a` for append after cursor
- [x] Implement `A` for append at end of line
- [x] Implement `o`/`O` for open line below/above
- [x] Implement `Esc` to exit Vim Insert back to Vim Normal
- [x] Implement `Esc` in Vim Normal to exit editor entirely
- [x] Implement `v` for Visual mode (character-wise)
- [x] Implement `V` for Visual Line mode

### 6.3 Vim Navigation (in Vim Normal/Visual)
- [x] Implement `h`/`j`/`k`/`l` cursor movement
- [x] Implement arrow keys for cursor movement
- [x] Implement `w`/`b`/`e` word navigation
- [x] Implement `0`/`^`/`$` line navigation
- [x] Implement `gg`/`G` file start/end
- [x] Implement `Home`/`End` line navigation

### 6.4 Vim Editing Commands
- [x] Implement `x` delete character
- [x] Implement `d` delete line (simplified dd)
- [x] Implement `D` delete to end of line
- [x] Implement `c` change line (simplified cc)
- [x] Implement `C` change to end of line
- [x] Implement `u` undo
- [x] Implement `Ctrl+r` redo

### 6.4.1 Operator-Pending Mode
- [x] Implement VimOperatorPending mode for d/c/y + motion combinations
- [x] Support `dw`/`db`/`de` delete word motions
- [x] Support `cw`/`cb`/`ce` change word motions
- [x] Support `yw`/`yb`/`ye` yank word motions
- [x] Support `d0`/`d$` delete to line boundaries
- [x] Support `c0`/`c$` change to line boundaries
- [x] Support `y0`/`y$` yank to line boundaries
- [x] Support `dd`/`cc`/`yy` for full line operations
- [x] Handle `dd` on last line edge case
- [x] Support `{`/`}` paragraph movements in operator-pending mode
- [x] Show operator-pending status in status bar (e.g., "d...", "c...", "y...")
- [x] Escape cancels operator-pending mode

### 6.5 Vim Clipboard Operations
- [x] Implement `y` yank (copy) line/selection
- [x] Implement `p` put (paste) after cursor
- [x] Implement `P` put (paste) before cursor
- [x] Integrate with system clipboard
- [x] Implement internal yank buffer for vim y/p operations (avoids clipboard errors)
- [x] System clipboard used only for Ctrl+C/Ctrl+V

### 6.5.1 Paragraph Navigation
- [x] Implement `{` move to previous paragraph boundary
- [x] Implement `}` move to next paragraph boundary
- [x] Support paragraph movements in Visual mode

### 6.6 Hybrid Editor Features
- [x] Support `Shift+Arrow` for selection in all editor modes
- [x] Support `Ctrl+a` for select all
- [x] Support `Ctrl+c`/`Ctrl+v` in Vim Insert mode
- [x] Support `Ctrl+z`/`Ctrl+y` in Vim Insert mode

### 6.7 Visual Feedback
- [x] Show editor sub-mode in status bar
- [x] Change editor border color based on sub-mode
- [x] Update help overlay with vim keybindings
- [x] Different cursor styles per mode

---

## Phase 7: Prompt Management

### 7.1 Create Prompt
- [x] Implement `n` - new prompt
- [x] Create file with default content
- [x] Generate unique name
- [x] Add to prompt list
- [x] Select new prompt
- [x] Enter Insert mode automatically

### 7.2 Rename Prompt
- [x] Implement `r` - rename prompt (auto-rename from content)
- [x] Show rename input popup
- [x] Validate new name (unique, valid chars)
- [x] Rename file on filesystem
- [ ] Update all references to old name (optional/future)

### 7.3 Delete Prompt
- [x] Implement `d` - delete prompt
- [x] Show confirmation dialog
- [x] Delete file from filesystem
- [x] Remove from prompt list
- [x] Update index

### 7.4 Duplicate Prompt
- [x] Implement `Ctrl+d` - duplicate
- [x] Create copy with new unique name
- [x] Copy content and tags
- [x] Select duplicated prompt

---

## Phase 8: Clipboard & Prompt Engine

### 8.1 Basic Copy
- [x] Implement `y` - copy to clipboard
- [x] Copy raw content (no resolution)
- [x] Show success notification
- [x] Implement `Ctrl+y` - copy rendered prompt to clipboard (global, all modes)

### 8.2 Reference Resolution
- [x] Parse all `[[...]]` references
- [x] Recursively resolve references
- [x] Implement circular reference detection
- [x] Implement max depth limit (10)
- [x] Insert warning comment for circular refs

### 8.3 Command Execution
- [x] Parse all `{{...}}` commands
- [x] Execute commands via shell
- [x] Capture command output
- [x] Handle command errors
- [x] Insert output or error message

### 8.4 Safe Mode
- [x] Implement `!` - toggle safe mode
- [x] Show safe mode status in UI
- [x] Store safe mode in config

### 8.5 Command Confirmation
- [x] Detect commands in content
- [x] Show confirmation dialog when safe mode ON
- [x] List all commands to be executed
- [x] Execute on confirm, cancel on reject
- [x] Skip confirmation when safe mode OFF

### 8.6 Copy with Resolution
- [x] Combine reference resolution and command execution
- [x] Produce final rendered content
- [x] Copy to clipboard

---

## Phase 9: Preview Mode

### 9.1 Preview Display
- [x] Implement `p` - toggle preview mode
- [x] Show rendered content (references resolved)
- [x] Execute commands for preview (disabled for safety in preview)
- [x] Display in read-only editor view

### 9.2 Preview UI
- [x] Different styling for preview mode (magenta border)
- [x] Mode indicator: `[PREVIEW]`
- [x] `Esc` or `p` to exit preview

---

## Phase 10: Archive System

### 10.1 Archive Prompt
- [x] Implement `a` - archive prompt
- [x] Move file to archive/ directory
- [x] Remove from main list
- [x] Update index

### 10.2 Archive View
- [x] Implement `A` - open archive
- [x] Switch to Archive mode
- [x] Display archived prompts
- [x] Different UI styling (indicate archive view)

### 10.3 Archive Operations
- [x] Implement `u` - unarchive prompt
- [x] Move file back to prompts/
- [x] Add to main list
- [x] Implement `Delete` - permanent delete
- [x] Show confirmation for delete
- [x] Implement `Esc` - exit archive view

---

## Phase 11: Folder System

### 11.1 Folder Navigation
- [x] Implement `O` - open folder selector
- [x] Display folder list
- [x] Navigate into selected folder
- [x] Display folder path in title bar

### 11.2 Folder Mode
- [x] Switch to Folder mode when in subfolder
- [x] Show only prompts in current folder
- [x] Implement `Esc` - go back to parent/root
- [x] All normal operations available

### 11.3 Move to Folder
- [x] Implement `M` - move prompt to folder
- [x] Show folder selector
- [x] Option to create new folder (`Ctrl+n`)
- [x] Move file to selected folder
- [x] Update prompt list

### 11.4 Folder Management
- [x] Create new folder
- [x] List existing folders
- [ ] Handle nested folders (optional/future)

---

## Phase 12: Tag System

### 12.1 Tag Selector
- [x] Implement `t` - open tag selector
- [x] Display existing tags
- [x] Fuzzy search tags
- [x] Create new tag (`Ctrl+n`)

### 12.2 Tag Assignment
- [x] Toggle tag on selected prompt
- [x] Update prompt metadata
- [x] Save changes to file

### 12.3 Tag Colors
- [x] Load tag colors from config
- [x] Assign default colors to new tags
- [x] Display colored indicators in prompt list

### 12.4 Tag Filtering
- [x] Implement `[` - previous tag filter
- [x] Implement `]` - next tag filter
- [x] Filter prompt list by selected tag
- [x] "All" option to show all prompts
- [x] Display active filter in status bar

---

## Phase 13: Fuzzy Search

### 13.1 Search Infrastructure
- [ ] Integrate nucleo fuzzy matcher
- [x] Build searchable index (names + content)
- [ ] Implement search ranking

### 13.2 Search UI
- [ ] Implement `/` - open search
- [ ] Implement `Ctrl+p` - quick open
- [ ] Display search input
- [ ] Display ranked results
- [ ] Show matching preview snippet

### 13.3 Search Actions
- [ ] Navigate results with `j`/`k`
- [ ] Jump to prompt on `Enter`
- [ ] Close search on `Esc`
- [ ] Update results as user types

### 13.4 Quick Insert Reference
- [x] Implement `Ctrl+r` in Insert mode (reference popup)
- [ ] Implement `Ctrl+l` in Insert mode (alternative binding)
- [x] Open prompt selector
- [x] Insert `[[selected_prompt]]` at cursor

---

## Phase 14: Export System

### 14.1 Export Dialog
- [ ] Implement `e` - open export dialog
- [ ] Display export options

### 14.2 Export Options
- [x] Copy Rendered - clipboard with resolution
- [x] Copy Raw - clipboard without resolution
- [ ] Save to File (Rendered)
- [ ] Save to File (Raw)

### 14.3 File Export
- [ ] Prompt for filename
- [ ] Prompt for location (or use default)
- [ ] Write content to file
- [x] Show success/error notification

---

## Phase 15: Help System

### 15.1 Help Overlay
- [x] Implement `?` - open help
- [x] Design help layout
- [x] List all keybindings by category
- [x] Show mode-specific bindings

### 15.2 Help Navigation
- [x] Scrollable help content
- [x] Close with `Esc` or `?`
- [x] Quick reference section

---

## Phase 16: Popup Components

### 16.1 Generic Popup
- [x] Create reusable popup component
- [ ] Support text input
- [ ] Support list selection
- [ ] Support fuzzy filtering

### 16.2 Confirmation Dialog
- [x] Create confirmation popup
- [x] Yes/No options
- [x] Custom message support

### 16.3 Notification System
- [x] Implement notification display
- [ ] Auto-dismiss after timeout
- [x] Support success/error/warning types

---

## Phase 17: Error Handling & Edge Cases

### 17.1 Error Handling
- [x] Handle file not found
- [ ] Handle permission denied
- [x] Handle corrupted YAML frontmatter
- [ ] Handle disk full
- [x] Display user-friendly error messages

### 17.2 Edge Cases
- [x] Empty prompt list
- [ ] Very long prompt names
- [ ] Very large prompt content
- [ ] Special characters in names
- [ ] Unicode content support

### 17.3 Logging
- [ ] Set up error logging to `.piemme/error.log`
- [ ] Log file operations
- [ ] Log command execution errors

---

## Phase 19: Testing

### 19.1 Unit Tests
- [ ] Test prompt name generation
- [ ] Test uniqueness checking
- [ ] Test YAML frontmatter parsing
- [ ] Test reference resolution
- [ ] Test circular reference detection
- [ ] Test fuzzy search

### 19.2 Integration Tests
- [ ] Test file operations
- [ ] Test config loading/saving
- [ ] Test prompt CRUD operations

### 19.3 Manual Testing
- [ ] Test all keybindings
- [ ] Test all modes
- [ ] Test edge cases
- [ ] Test on different terminal emulators

---

## Phase 20: Polish & Optimization

### 20.1 Performance
- [ ] Profile and optimize rendering
- [ ] Optimize search indexing
- [ ] Debounce search input

### 20.2 UX Polish
- [ ] Consistent color scheme
- [ ] Smooth scrolling
- [ ] Loading indicators
- [ ] Keyboard shortcut hints

### 20.3 Documentation
- [ ] Write README.md
- [ ] Document installation
- [ ] Document configuration
- [ ] Add usage examples

---

## Phase 21: Release Preparation

### 21.1 Build & Distribution
- [ ] Set up release build profile
- [ ] Create GitHub Actions CI/CD
- [ ] Build for Linux
- [ ] Build for macOS
- [ ] Build for Windows
- [ ] Create release binaries

### 21.2 Installation
- [ ] Create install script
- [ ] Document manual installation
- [ ] Consider package managers (cargo install, brew, etc.)

### 21.3 Final Testing
- [ ] Full feature test on all platforms
- [ ] Performance benchmarks
- [ ] Fix any remaining bugs

---

## Task Summary

| Phase | Description | Estimated Tasks |
|-------|-------------|-----------------|
| 1 | Project Setup | 4 |
| 2 | File System | 15 |
| 3 | TUI Framework | 10 |
| 4 | Navigation & Display | 15 |
| 5 | Syntax Highlighting | 8 |
| 6 | Insert Mode | 14 |
| 7 | Prompt Management | 14 |
| 8 | Clipboard & Engine | 16 |
| 9 | Preview Mode | 5 |
| 10 | Archive System | 11 |
| 11 | Folder System | 11 |
| 12 | Tag System | 12 |
| 13 | Fuzzy Search | 11 |
| 14 | Export System | 7 |
| 15 | Help System | 5 |
| 16 | Popup Components | 7 |
| 17 | Error Handling | 11 |
| 18 | Testing | 10 |
| 19 | Polish | 9 |
| 20 | Release | 9 |
| **Total** | | **~200 tasks** |

---

## Suggested Development Order

1. **Foundation First**: Phases 1-3 (setup, file system, TUI scaffolding)
2. **Core Loop**: Phases 4, 6, 7 (navigation, editing, prompt management)
3. **Prompt Engine**: Phases 5, 8, 9 (highlighting, clipboard, preview)
4. **Organization**: Phases 10, 11, 12 (archive, folders, tags)
5. **Search & Export**: Phases 13, 14 (fuzzy search, export)
6. **Polish**: Phases 15-17 (help, popups, errors)
7. **Quality**: Phases 18-20 (testing, optimization, release)

---

## MVP Scope (Minimum Viable Product)

For a first working version, prioritize:

1. ✅ Phases 1-4: Basic TUI with navigation
2. ✅ Phase 6: Insert mode editing
3. ✅ Phase 7: Create, rename, delete prompts
4. ✅ Phase 8: Copy with reference resolution
5. ✅ Phase 5: Syntax highlighting (basic)
6. ✅ Phase 15: Help overlay

**MVP excludes**: Archive, folders, tags, fuzzy search, export, preview mode

Add these features incrementally after MVP is stable.

---

## Bonus Features (Implemented)

### UI Improvements
- [x] Change title to lowercase "piemme" for consistent branding
- [x] Resizable left column width with `Ctrl+l` (increase) and `Ctrl+h` (decrease)
  - Default: 15%, Min: 15%, Max: 70%, Step: 5%

### Keybinding Improvements
- [x] `q` quits application from editor Vim Normal mode (in addition to Normal mode)
- [x] `r` and `Ctrl+r` both open reference insertion popup in editor Vim Normal mode
  - Previously `Ctrl+r` was redo, now `r` and `Ctrl+r` are unified for references

---

## Bug Fixes

### Help System
- [x] Fix `?` key triggering help overlay in Vim Insert mode
  - Now `?` only opens help when not in Insert mode (VimInsert sub-mode)
  - Allows typing `?` character in the editor