# Piemme - Development Tasks

## Phase 1: Project Setup & Foundation

### 1.1 Project Initialization
- [ ] Initialize Rust project with `cargo init`
- [ ] Set up Cargo.toml with dependencies:
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
- [ ] Set up project structure (src/main.rs, modules)
- [ ] Configure rustfmt.toml and clippy

### 1.2 Core Data Structures
- [ ] Define `Prompt` struct (id, name, content, tags, timestamps)
- [ ] Define `Config` struct (safe_mode, tag_colors, settings)
- [ ] Define `AppState` struct (mode, selected_index, prompts, etc.)
- [ ] Define `Mode` enum (Normal, Insert, Archive, Folder, Preview)
- [ ] Define `Action` enum for all possible user actions
- [ ] Implement serialization/deserialization for all structs

---

## Phase 2: File System Layer

### 2.1 Directory Management
- [ ] Implement `.piemme/` directory initialization
- [ ] Create subdirectories (prompts/, archive/, folders/)
- [ ] Implement directory existence checks
- [ ] Handle first-run setup

### 2.2 Config Management
- [ ] Load config from `config.yaml`
- [ ] Create default config if not exists
- [ ] Implement config save
- [ ] Validate config values

### 2.3 Prompt File Operations
- [ ] Implement YAML frontmatter parsing
- [ ] Implement YAML frontmatter writing
- [ ] Load single prompt from file
- [ ] Save single prompt to file
- [ ] Load all prompts from directory
- [ ] Handle file read/write errors gracefully

### 2.4 Prompt Name Generation
- [ ] Implement auto-name generation from content
- [ ] Implement uniqueness checking across all prompts
- [ ] Implement suffix appending for duplicates
- [ ] Handle empty content case (`empty_prompt_<n>`)

### 2.5 Index Management
- [ ] Design index structure (`.index.json`)
- [ ] Build index from prompts
- [ ] Save index to file
- [ ] Load index on startup
- [ ] Update index on prompt changes

---

## Phase 3: TUI Framework Setup

### 3.1 Application Scaffolding
- [ ] Set up main event loop
- [ ] Initialize crossterm terminal
- [ ] Set up panic handler (restore terminal on crash)
- [ ] Implement clean shutdown

### 3.2 Basic Layout
- [ ] Create main layout (title, left panel, right panel, status bar)
- [ ] Implement responsive sizing
- [ ] Add borders and styling

### 3.3 Component Architecture
- [ ] Create `TitleBar` component
- [ ] Create `PromptList` component
- [ ] Create `Editor` component (wrapper around tui-textarea)
- [ ] Create `StatusBar` component
- [ ] Create `HelpOverlay` component
- [ ] Create `Popup` component (for selectors)

---

## Phase 4: Core Navigation & Display

### 4.1 Prompt List
- [ ] Display list of prompts
- [ ] Implement scrolling
- [ ] Highlight selected prompt
- [ ] Show tag color indicators
- [ ] Display prompt count in header

### 4.2 Navigation
- [ ] Implement `j`/`↓` - move down
- [ ] Implement `k`/`↑` - move up
- [ ] Implement `g` - go to first
- [ ] Implement `G` - go to last
- [ ] Handle empty list edge case

### 4.3 Editor Display
- [ ] Display selected prompt content
- [ ] Show prompt name as header
- [ ] Implement read-only view for Normal mode
- [ ] Handle long content with scrolling

### 4.4 Status Bar
- [ ] Display current mode
- [ ] Display selected prompt tags
- [ ] Display statistics (prompt count, archived, tags)
- [ ] Display safe mode indicator

---

## Phase 5: Syntax Highlighting

### 5.1 Reference Highlighting
- [ ] Parse content for `[[...]]` patterns
- [ ] Validate references against existing prompts
- [ ] Apply green color for valid references
- [ ] Apply red color for invalid references

### 5.2 Command Highlighting
- [ ] Parse content for `{{...}}` patterns
- [ ] Apply yellow/orange color for commands
- [ ] Add visual warning indicator

### 5.3 Integration
- [ ] Apply highlighting in editor view
- [ ] Apply highlighting in preview mode
- [ ] Update highlighting on content change

---

## Phase 6: Insert Mode & Editing

### 6.1 Mode Switching
- [ ] Implement `Enter`/`i` to enter Insert mode
- [ ] Implement `Esc` to exit Insert mode
- [ ] Visual mode indicator update

### 6.2 Text Editor Integration
- [ ] Configure tui-textarea
- [ ] Implement basic text input
- [ ] Implement cursor movement
- [ ] Implement `Ctrl+←`/`Ctrl+→` word movement
- [ ] Implement `Home`/`End` line navigation
- [ ] Implement `Ctrl+Home`/`Ctrl+End` document navigation

### 6.3 Edit Operations
- [ ] Implement undo (`Ctrl+z`)
- [ ] Implement redo (`Ctrl+y`)
- [ ] Implement text selection
- [ ] Implement copy/cut/paste within editor

### 6.4 Auto-Save
- [ ] Save on exit from Insert mode
- [ ] Implement explicit save (`Ctrl+s`)
- [ ] Update modified timestamp
- [ ] Update index after save

---

## Phase 7: Prompt Management

### 7.1 Create Prompt
- [ ] Implement `n` - new prompt
- [ ] Create file with default content
- [ ] Generate unique name
- [ ] Add to prompt list
- [ ] Select new prompt
- [ ] Enter Insert mode automatically

### 7.2 Rename Prompt
- [ ] Implement `r` - rename prompt
- [ ] Show rename input popup
- [ ] Validate new name (unique, valid chars)
- [ ] Rename file on filesystem
- [ ] Update all references to old name (optional/future)

### 7.3 Delete Prompt
- [ ] Implement `d` - delete prompt
- [ ] Show confirmation dialog
- [ ] Delete file from filesystem
- [ ] Remove from prompt list
- [ ] Update index

### 7.4 Duplicate Prompt
- [ ] Implement `Ctrl+d` - duplicate
- [ ] Create copy with new unique name
- [ ] Copy content and tags
- [ ] Select duplicated prompt

---

## Phase 8: Clipboard & Prompt Engine

### 8.1 Basic Copy
- [ ] Implement `y` - copy to clipboard
- [ ] Copy raw content (no resolution)
- [ ] Show success notification

### 8.2 Reference Resolution
- [ ] Parse all `[[...]]` references
- [ ] Recursively resolve references
- [ ] Implement circular reference detection
- [ ] Implement max depth limit (10)
- [ ] Insert warning comment for circular refs

### 8.3 Command Execution
- [ ] Parse all `{{...}}` commands
- [ ] Execute commands via shell
- [ ] Capture command output
- [ ] Handle command errors
- [ ] Insert output or error message

### 8.4 Safe Mode
- [ ] Implement `!` - toggle safe mode
- [ ] Show safe mode status in UI
- [ ] Store safe mode in config

### 8.5 Command Confirmation
- [ ] Detect commands in content
- [ ] Show confirmation dialog when safe mode ON
- [ ] List all commands to be executed
- [ ] Execute on confirm, cancel on reject
- [ ] Skip confirmation when safe mode OFF

### 8.6 Copy with Resolution
- [ ] Combine reference resolution and command execution
- [ ] Produce final rendered content
- [ ] Copy to clipboard

---

## Phase 9: Preview Mode

### 9.1 Preview Display
- [ ] Implement `p` - toggle preview mode
- [ ] Show rendered content (references resolved)
- [ ] Execute commands for preview
- [ ] Display in read-only editor view

### 9.2 Preview UI
- [ ] Different styling for preview mode
- [ ] Mode indicator: `[PREVIEW]`
- [ ] `Esc` or `p` to exit preview

---

## Phase 10: Archive System

### 10.1 Archive Prompt
- [ ] Implement `a` - archive prompt
- [ ] Move file to archive/ directory
- [ ] Remove from main list
- [ ] Update index

### 10.2 Archive View
- [ ] Implement `A` - open archive
- [ ] Switch to Archive mode
- [ ] Display archived prompts
- [ ] Different UI styling (indicate archive view)

### 10.3 Archive Operations
- [ ] Implement `u` - unarchive prompt
- [ ] Move file back to prompts/
- [ ] Add to main list
- [ ] Implement `Delete` - permanent delete
- [ ] Show confirmation for delete
- [ ] Implement `Esc` - exit archive view

---

## Phase 11: Folder System

### 11.1 Folder Navigation
- [ ] Implement `O` - open folder selector
- [ ] Display folder list
- [ ] Navigate into selected folder
- [ ] Display folder path in title bar

### 11.2 Folder Mode
- [ ] Switch to Folder mode when in subfolder
- [ ] Show only prompts in current folder
- [ ] Implement `Esc` - go back to parent/root
- [ ] All normal operations available

### 11.3 Move to Folder
- [ ] Implement `M` - move prompt to folder
- [ ] Show folder selector
- [ ] Option to create new folder (`Ctrl+n`)
- [ ] Move file to selected folder
- [ ] Update prompt list

### 11.4 Folder Management
- [ ] Create new folder
- [ ] List existing folders
- [ ] Handle nested folders (optional/future)

---

## Phase 12: Tag System

### 12.1 Tag Selector
- [ ] Implement `t` - open tag selector
- [ ] Display existing tags
- [ ] Fuzzy search tags
- [ ] Create new tag (`Ctrl+n`)

### 12.2 Tag Assignment
- [ ] Toggle tag on selected prompt
- [ ] Update prompt metadata
- [ ] Save changes to file

### 12.3 Tag Colors
- [ ] Load tag colors from config
- [ ] Assign default colors to new tags
- [ ] Display colored indicators in prompt list

### 12.4 Tag Filtering
- [ ] Implement `[` - previous tag filter
- [ ] Implement `]` - next tag filter
- [ ] Filter prompt list by selected tag
- [ ] "All" option to show all prompts
- [ ] Display active filter in status bar

---

## Phase 13: Fuzzy Search

### 13.1 Search Infrastructure
- [ ] Integrate nucleo fuzzy matcher
- [ ] Build searchable index (names + content)
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
- [ ] Implement `Ctrl+l` in Insert mode
- [ ] Open prompt selector
- [ ] Insert `[[selected_prompt]]` at cursor

---

## Phase 14: Export System

### 14.1 Export Dialog
- [ ] Implement `e` - open export dialog
- [ ] Display export options

### 14.2 Export Options
- [ ] Copy Rendered - clipboard with resolution
- [ ] Copy Raw - clipboard without resolution
- [ ] Save to File (Rendered)
- [ ] Save to File (Raw)

### 14.3 File Export
- [ ] Prompt for filename
- [ ] Prompt for location (or use default)
- [ ] Write content to file
- [ ] Show success/error notification

---

## Phase 15: Help System

### 15.1 Help Overlay
- [ ] Implement `?` - open help
- [ ] Design help layout
- [ ] List all keybindings by category
- [ ] Show mode-specific bindings

### 15.2 Help Navigation
- [ ] Scrollable help content
- [ ] Close with `Esc` or `?`
- [ ] Quick reference section

---

## Phase 16: Popup Components

### 16.1 Generic Popup
- [ ] Create reusable popup component
- [ ] Support text input
- [ ] Support list selection
- [ ] Support fuzzy filtering

### 16.2 Confirmation Dialog
- [ ] Create confirmation popup
- [ ] Yes/No options
- [ ] Custom message support

### 16.3 Notification System
- [ ] Implement notification display
- [ ] Auto-dismiss after timeout
- [ ] Support success/error/warning types

---

## Phase 17: Error Handling & Edge Cases

### 17.1 Error Handling
- [ ] Handle file not found
- [ ] Handle permission denied
- [ ] Handle corrupted YAML frontmatter
- [ ] Handle disk full
- [ ] Display user-friendly error messages

### 17.2 Edge Cases
- [ ] Empty prompt list
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
