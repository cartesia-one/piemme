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
  - `notify` (file watching)
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
- [x] Position cursor at end of file when entering Insert mode

### 6.2 Text Editor Integration
- [x] Configure tui-textarea
- [x] Implement basic text input
- [x] Implement cursor movement
- [x] Implement `Ctrl+←`/`Ctrl+→` word movement
- [x] Implement `Home`/`End` line navigation
- [x] Implement `Ctrl+Home`/`Ctrl+End` document navigation

### 6.3 Edit Operations
- [x] Implement undo (`Ctrl+z`)
- [x] Implement redo (`Ctrl+y`)
- [x] Implement text selection (`Ctrl+a` for select all, `Shift+Arrow` for keyboard selection)
- [x] Implement copy/cut/paste within editor (`Ctrl+c`/`Ctrl+v`)

### 6.4 Mouse Support
- [x] Enable mouse capture in terminal
- [x] Handle mouse scroll events in editor
- [ ] Mouse click to position cursor (not supported by tui-textarea)
- [ ] Mouse drag to select text (not supported by tui-textarea)

### 6.5 Auto-Save
- [x] Save on exit from Insert mode
- [x] Implement explicit save (`Ctrl+s`)
- [x] Update modified timestamp
- [x] Update index after save

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
- [ ] Show confirmation dialog when safe mode ON
- [ ] List all commands to be executed
- [ ] Execute on confirm, cancel on reject
- [ ] Skip confirmation when safe mode OFF

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

## Phase 18: File Watching

### 18.1 Watch Setup
- [ ] Initialize file watcher on `.piemme/`
- [ ] Watch for file changes
- [ ] Watch for file additions
- [ ] Watch for file deletions

### 18.2 Change Handling
- [ ] Detect external modifications
- [ ] Prompt for reload if file changed
- [ ] Auto-refresh prompt list
- [ ] Update index on changes

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
- [ ] Lazy load prompt content
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
| 18 | File Watching | 6 |
| 19 | Testing | 10 |
| 20 | Polish | 9 |
| 21 | Release | 9 |
| **Total** | | **~200 tasks** |

---

## Suggested Development Order

1. **Foundation First**: Phases 1-3 (setup, file system, TUI scaffolding)
2. **Core Loop**: Phases 4, 6, 7 (navigation, editing, prompt management)
3. **Prompt Engine**: Phases 5, 8, 9 (highlighting, clipboard, preview)
4. **Organization**: Phases 10, 11, 12 (archive, folders, tags)
5. **Search & Export**: Phases 13, 14 (fuzzy search, export)
6. **Polish**: Phases 15-18 (help, popups, errors, file watching)
7. **Quality**: Phases 19-21 (testing, optimization, release)

---

## MVP Scope (Minimum Viable Product)

For a first working version, prioritize:

1. ✅ Phases 1-4: Basic TUI with navigation
2. ✅ Phase 6: Insert mode editing
3. ✅ Phase 7: Create, rename, delete prompts
4. ✅ Phase 8: Copy with reference resolution
5. ✅ Phase 5: Syntax highlighting (basic)
6. ✅ Phase 15: Help overlay

**MVP excludes**: Archive, folders, tags, fuzzy search, export, file watching, preview mode

Add these features incrementally after MVP is stable.
