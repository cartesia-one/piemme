//! Main application logic

use anyhow::Result;
use crossterm::event::{self, Event, KeyEvent, KeyEventKind};
use std::time::Duration;
use tui_textarea::{CursorMove, TextArea};

use crate::config::{archive_dir, config_path, folders_dir, index_path, prompts_dir, Config};
use crate::fs::{ensure_directories, load_all_prompts, save_prompt, delete_prompt, Index, IndexEntry};
use crate::models::{Action, AppState, ConfirmDialog, EditorMode, FolderSelectorMode, FolderSelectorState, Mode, NotificationLevel, PendingAction, Prompt, TagSelectorState, VimOperator};
use crate::tui::{init_terminal, restore_terminal, Tui};
use crate::ui::{handle_key_event, render};

/// Execute a vim motion on the editor (free function to avoid borrow issues)
fn execute_vim_motion(editor: &mut TextArea, action: &Action) {
    match action {
        Action::VimWordForward => editor.move_cursor(CursorMove::WordForward),
        Action::VimWordBackward => editor.move_cursor(CursorMove::WordBack),
        Action::VimWordEnd => {
            editor.move_cursor(CursorMove::WordForward);
            editor.move_cursor(CursorMove::Back);
        }
        Action::VimLineStart => editor.move_cursor(CursorMove::Head),
        Action::VimFirstNonBlank => {
            editor.move_cursor(CursorMove::Head);
            let (row, _) = editor.cursor();
            if let Some(line) = editor.lines().get(row) {
                let first_non_blank = line.chars().take_while(|c| c.is_whitespace()).count();
                editor.move_cursor(CursorMove::Jump(row as u16, first_non_blank as u16));
            }
        }
        Action::VimLineEnd => editor.move_cursor(CursorMove::End),
        Action::VimLeft => editor.move_cursor(CursorMove::Back),
        Action::VimRight => editor.move_cursor(CursorMove::Forward),
        Action::VimUp => editor.move_cursor(CursorMove::Up),
        Action::VimDown => editor.move_cursor(CursorMove::Down),
        Action::VimGoToTop => {
            editor.move_cursor(CursorMove::Top);
            editor.move_cursor(CursorMove::Head);
        }
        Action::VimGoToBottom => {
            editor.move_cursor(CursorMove::Bottom);
            editor.move_cursor(CursorMove::Head);
        }
        Action::VimParagraphBackward => {
            move_to_paragraph_boundary(editor, false);
        }
        Action::VimParagraphForward => {
            move_to_paragraph_boundary(editor, true);
        }
        _ => {}
    }
}

/// Move cursor to paragraph boundary (free function)
fn move_to_paragraph_boundary(editor: &mut TextArea, forward: bool) {
    let lines = editor.lines();
    let (current_row, _) = editor.cursor();
    let total_lines = lines.len();

    if forward {
        // Move forward to next paragraph boundary (empty line after non-empty lines)
        let mut row = current_row + 1;
        let mut found_non_empty = false;
        
        while row < total_lines {
            let line = &lines[row];
            let is_empty = line.trim().is_empty();
            
            if !is_empty {
                found_non_empty = true;
            } else if found_non_empty {
                // Found empty line after non-empty content
                editor.move_cursor(CursorMove::Jump(row as u16, 0));
                return;
            }
            row += 1;
        }
        
        // No paragraph boundary found, go to end
        editor.move_cursor(CursorMove::Bottom);
        editor.move_cursor(CursorMove::Head);
    } else {
        // Move backward to previous paragraph boundary
        if current_row == 0 {
            return;
        }
        
        let mut row = current_row.saturating_sub(1);
        let mut found_non_empty = false;
        
        loop {
            let line = &lines[row];
            let is_empty = line.trim().is_empty();
            
            if !is_empty {
                found_non_empty = true;
            } else if found_non_empty {
                // Found empty line before non-empty content
                editor.move_cursor(CursorMove::Jump(row as u16, 0));
                return;
            }
            
            if row == 0 {
                break;
            }
            row -= 1;
        }
        
        // No paragraph boundary found, go to start
        editor.move_cursor(CursorMove::Top);
        editor.move_cursor(CursorMove::Head);
    }
}

/// The main application
pub struct App<'a> {
    /// Terminal instance
    terminal: Tui,
    /// Application state
    state: AppState,
    /// Configuration
    config: Config,
    /// Search index
    index: Index,
    /// Archived prompts count
    archived_count: usize,
    /// Text editor (created lazily when entering insert mode)
    editor: Option<TextArea<'a>>,
    /// All prompts (unfiltered) - used as source for tag filtering
    all_prompts: Vec<Prompt>,
}

impl<'a> App<'a> {
    /// Create a new application instance
    pub fn new() -> Result<Self> {
        // Initialize terminal
        let terminal = init_terminal()?;

        // Ensure directories exist
        ensure_directories()?;

        // Load configuration
        let config = Config::load_or_default(&config_path()?)?;

        // Load index
        let index = Index::load_or_new(&index_path()?)?;

        // Create initial state
        let mut state = AppState::new();
        state.safe_mode = config.safe_mode;

        // Load prompts
        let prompts = load_all_prompts(&prompts_dir()?)?;
        let all_prompts = prompts.clone();
        state.prompts = prompts;

        // Count archived prompts
        let archived_count = load_all_prompts(&archive_dir()?)?.len();

        // Collect all tags
        let mut all_tags: Vec<String> = state
            .prompts
            .iter()
            .flat_map(|p| p.tags.clone())
            .collect();
        all_tags.sort();
        all_tags.dedup();
        state.all_tags = all_tags;

        Ok(Self {
            terminal,
            state,
            config,
            index,
            archived_count,
            editor: None,
            all_prompts,
        })
    }

    /// Run the main application loop
    pub fn run(&mut self) -> Result<()> {
        loop {
            // Draw UI
            self.terminal.draw(|frame| {
                render(
                    frame,
                    &self.state,
                    &self.config,
                    self.archived_count,
                    self.editor.as_ref(),
                );
            })?;

            // Handle events
            if event::poll(Duration::from_millis(100))? {
                let evt = event::read()?;
                
                // Handle mouse events for text selection in Insert mode
                if let Event::Mouse(mouse_event) = evt {
                    if self.state.mode == Mode::Insert {
                        if let Some(ref mut editor) = self.editor {
                            // Convert mouse event to Input for tui-textarea
                            editor.input(mouse_event);
                        }
                    }
                    continue;
                }
                
                if let Event::Key(key) = evt {
                    // Only handle key press events (not release)
                    if key.kind == KeyEventKind::Press {
                        // Handle rename popup input
                        if self.state.rename_popup.is_some() {
                            let action = handle_key_event(key, &self.state);
                            match action {
                                Action::ConfirmRename => {
                                    self.handle_action(action)?;
                                }
                                Action::CancelRename => {
                                    self.state.rename_popup = None;
                                }
                                Action::None => {
                                    // Handle text input for rename popup
                                    self.handle_rename_popup_input(key);
                                }
                                _ => {}
                            }
                            continue;
                        }

                        // Handle reference popup input
                        if self.state.reference_popup.is_some() {
                            let action = handle_key_event(key, &self.state);
                            match action {
                                Action::ConfirmReference => {
                                    self.handle_action(action)?;
                                }
                                Action::CancelReference => {
                                    self.state.reference_popup = None;
                                }
                                Action::ReferencePopupUp => {
                                    if let Some(ref mut popup) = self.state.reference_popup {
                                        popup.select_previous();
                                    }
                                }
                                Action::ReferencePopupDown => {
                                    if let Some(ref mut popup) = self.state.reference_popup {
                                        popup.select_next();
                                    }
                                }
                                Action::None => {
                                    // Handle text input for filter
                                    self.handle_reference_popup_input(key);
                                }
                                _ => {}
                            }
                            continue;
                        }

                        // Handle tag selector input
                        if self.state.tag_selector.is_some() {
                            let action = handle_key_event(key, &self.state);
                            match action {
                                Action::ConfirmTagToggle => {
                                    self.handle_action(action)?;
                                }
                                Action::CancelTagSelector => {
                                    self.handle_action(action)?;
                                }
                                Action::TagSelectorUp | Action::TagSelectorDown => {
                                    self.handle_action(action)?;
                                }
                                Action::CreateNewTag => {
                                    self.handle_action(action)?;
                                }
                                Action::ConfirmNewTag => {
                                    self.handle_action(action)?;
                                }
                                Action::None => {
                                    // Handle text input for filter or new tag
                                    self.handle_tag_selector_input(key);
                                }
                                _ => {}
                            }
                            continue;
                        }

                        // Handle folder selector input
                        if self.state.folder_selector.is_some() {
                            let action = handle_key_event(key, &self.state);
                            match action {
                                Action::ConfirmFolderSelection => {
                                    self.handle_action(action)?;
                                }
                                Action::CancelFolderSelector => {
                                    self.handle_action(action)?;
                                }
                                Action::FolderSelectorUp | Action::FolderSelectorDown => {
                                    self.handle_action(action)?;
                                }
                                Action::CreateNewFolder => {
                                    self.handle_action(action)?;
                                }
                                Action::ConfirmNewFolder => {
                                    self.handle_action(action)?;
                                }
                                Action::None => {
                                    // Handle text input for filter or new folder
                                    self.handle_folder_selector_input(key);
                                }
                                _ => {}
                            }
                            continue;
                        }

                        // In Insert mode, handle vim-style sub-modes
                        if self.state.mode == Mode::Insert {
                            if let Some(ref mut editor) = self.editor {
                                let action = handle_key_event(key, &self.state);
                                self.handle_vim_editor_action(action, key)?;
                            }
                        } else {
                            let action = handle_key_event(key, &self.state);
                            self.handle_action(action)?;
                        }
                    }
                }
            }

            // Check if we should quit
            if self.state.should_quit {
                break;
            }
        }

        // Restore terminal
        restore_terminal()?;

        Ok(())
    }

    /// Handle an action
    fn handle_action(&mut self, action: Action) -> Result<()> {
        // Clear notification on any action
        self.state.clear_notification();

        // Default visible height estimate for scrolling
        // Will be updated during render, but we use a reasonable default here
        const DEFAULT_VISIBLE_HEIGHT: usize = 20;

        match action {
            Action::None => {}

            // Navigation
            Action::MoveDown => {
                self.state.select_next();
                self.state.ensure_visible(DEFAULT_VISIBLE_HEIGHT);
            }
            Action::MoveUp => {
                self.state.select_previous();
                self.state.ensure_visible(DEFAULT_VISIBLE_HEIGHT);
            }
            Action::GoToFirst => {
                self.state.select_first();
                // select_first already resets scroll offset
            }
            Action::GoToLast => {
                self.state.select_last();
                self.state.ensure_visible(DEFAULT_VISIBLE_HEIGHT);
            }

            // Mode switching
            Action::EnterInsertMode => {
                if self.state.has_prompts() {
                    self.enter_insert_mode();
                }
            }
            Action::ExitMode => {
                match self.state.mode {
                    Mode::Insert => {
                        // Save on exit from insert mode
                        self.exit_insert_mode()?;
                    }
                    Mode::Archive | Mode::Folder => {
                        // Reload main prompts when exiting archive/folder
                        self.reload_prompts()?;
                        self.state.mode = Mode::Normal;
                    }
                    Mode::Preview => {
                        self.state.mode = Mode::Normal;
                    }
                    Mode::Normal => {}
                }
                self.state.editor_focused = false;
            }
            Action::TogglePreview => {
                if self.state.mode == Mode::Preview {
                    self.state.mode = Mode::Normal;
                } else {
                    self.state.mode = Mode::Preview;
                }
            }
            Action::OpenArchive => {
                self.state.mode = Mode::Archive;
                self.state.prompts = load_all_prompts(&archive_dir()?)?;
                self.state.selected_index = 0;
            }

            // Prompt management
            Action::NewPrompt => {
                self.create_new_prompt()?;
            }
            Action::DeletePrompt => {
                self.request_delete_confirmation()?;
            }
            Action::ArchivePrompt => {
                self.archive_current_prompt()?;
            }
            Action::UnarchivePrompt => {
                self.unarchive_current_prompt()?;
            }

            // Clipboard
            Action::CopyRendered => {
                self.copy_to_clipboard(true)?;
            }
            Action::CopyRaw => {
                self.copy_to_clipboard(false)?;
            }

            // Tags
            Action::NextTagFilter => {
                self.cycle_tag_filter(true);
            }
            Action::PreviousTagFilter => {
                self.cycle_tag_filter(false);
            }

            // Settings
            Action::ToggleSafeMode => {
                self.state.safe_mode = !self.state.safe_mode;
                self.config.safe_mode = self.state.safe_mode;
                let msg = if self.state.safe_mode {
                    "Safe mode enabled"
                } else {
                    "Safe mode disabled - commands will execute without confirmation"
                };
                self.state.notify(msg, NotificationLevel::Info);
            }

            // UI
            Action::ToggleFocus => {
                self.state.editor_focused = !self.state.editor_focused;
            }
            Action::IncreaseLeftColumnWidth => {
                // Increase left column width by 5%, max 70%
                self.state.left_column_percent = (self.state.left_column_percent + 5).min(70);
            }
            Action::DecreaseLeftColumnWidth => {
                // Decrease left column width by 5%, min 15%
                self.state.left_column_percent = self.state.left_column_percent.saturating_sub(5).max(15);
            }
            Action::OpenHelp => {
                self.state.show_help = !self.state.show_help;
                // Reset scroll when opening help
                if self.state.show_help {
                    self.state.help_scroll_offset = 0;
                }
            }
            Action::HelpScrollUp => {
                self.state.help_scroll_offset = self.state.help_scroll_offset.saturating_sub(1);
            }
            Action::HelpScrollDown => {
                // Allow scrolling down, will be capped by max scroll during render
                self.state.help_scroll_offset = self.state.help_scroll_offset.saturating_add(1);
            }
            Action::CloseOverlay => {
                self.state.show_help = false;
                self.state.active_popup = None;
                self.state.confirm_dialog = None;
            }
            Action::ToggleConfirmSelection => {
                if let Some(dialog) = &mut self.state.confirm_dialog {
                    dialog.toggle_selection();
                }
            }

            // Application
            Action::Quit => {
                self.state.should_quit = true;
            }

            // Confirmation dialog actions
            Action::Confirm => {
                self.handle_confirm()?;
            }
            Action::Cancel => {
                self.state.confirm_dialog = None;
            }

            // Save
            Action::Save => {
                self.save_current_prompt()?;
            }

            // Rename prompt
            Action::RenamePrompt => {
                self.rename_current_prompt()?;
            }

            // Open rename popup
            Action::OpenRenamePopup => {
                self.open_rename_popup();
            }

            // Confirm rename from popup
            Action::ConfirmRename => {
                self.confirm_rename_popup()?;
            }

            // Cancel rename popup
            Action::CancelRename => {
                self.state.rename_popup = None;
            }

            // Open reference popup (CTRL+i in insert mode)
            Action::OpenReferencePopup => {
                self.open_reference_popup();
            }

            // Confirm reference selection
            Action::ConfirmReference => {
                self.confirm_reference_popup()?;
            }

            // Cancel reference popup
            Action::CancelReference => {
                self.state.reference_popup = None;
            }

            // Reference popup navigation (handled in run loop, but just in case)
            Action::ReferencePopupUp | Action::ReferencePopupDown => {
                // Handled in run loop
            }

            // Duplicate prompt
            Action::DuplicatePrompt => {
                self.duplicate_current_prompt()?;
            }

            // Undo/Redo are handled directly in the run loop for insert mode
            Action::Undo | Action::Redo => {
                // These are handled in the event loop when in insert mode
            }

            // TODO: Implement these
            Action::OpenSearch
            | Action::QuickOpen
            | Action::QuickInsertReference
            | Action::Export => {
                self.state.notify("Feature not yet implemented", NotificationLevel::Warning);
            }

            // Tag selector actions
            Action::OpenTagSelector => {
                self.open_tag_selector();
            }
            Action::ConfirmTagToggle => {
                self.toggle_tag()?;
            }
            Action::CancelTagSelector => {
                self.close_tag_selector()?;
            }
            Action::TagSelectorUp => {
                if let Some(ref mut selector) = self.state.tag_selector {
                    selector.select_previous();
                }
            }
            Action::TagSelectorDown => {
                if let Some(ref mut selector) = self.state.tag_selector {
                    selector.select_next();
                }
            }
            Action::CreateNewTag => {
                if let Some(ref mut selector) = self.state.tag_selector {
                    selector.start_creating_new();
                }
            }
            Action::ConfirmNewTag => {
                self.confirm_new_tag()?;
            }

            // Folder selector actions
            Action::OpenFolder => {
                self.open_folder_selector(FolderSelectorMode::Open)?;
            }
            Action::MoveToFolder => {
                if self.state.has_prompts() {
                    self.open_folder_selector(FolderSelectorMode::Move)?;
                }
            }
            Action::ConfirmFolderSelection => {
                self.confirm_folder_selection()?;
            }
            Action::CancelFolderSelector => {
                self.state.folder_selector = None;
            }
            Action::FolderSelectorUp => {
                if let Some(ref mut selector) = self.state.folder_selector {
                    selector.select_previous();
                }
            }
            Action::FolderSelectorDown => {
                if let Some(ref mut selector) = self.state.folder_selector {
                    selector.select_next();
                }
            }
            Action::CreateNewFolder => {
                if let Some(ref mut selector) = self.state.folder_selector {
                    selector.start_creating_new();
                }
            }
            Action::ConfirmNewFolder => {
                self.confirm_new_folder()?;
            }

            // Text selection/clipboard actions (handled in event loop for Insert mode)
            Action::SelectAll | Action::CopySelection | Action::Paste => {
                // These are handled directly in the event loop when in Insert mode
            }
            
            // Vim-style editor actions (handled in handle_vim_editor_action)
            Action::VimEnterInsert
            | Action::VimEnterInsertEnd
            | Action::VimEnterInsertStart
            | Action::VimOpenBelow
            | Action::VimOpenAbove
            | Action::VimExitToNormal
            | Action::VimEnterVisual
            | Action::VimEnterVisualLine
            | Action::VimLeft
            | Action::VimDown
            | Action::VimUp
            | Action::VimRight
            | Action::VimLineStart
            | Action::VimFirstNonBlank
            | Action::VimLineEnd
            | Action::VimWordForward
            | Action::VimWordBackward
            | Action::VimWordEnd
            | Action::VimGoToTop
            | Action::VimGoToBottom
            | Action::VimParagraphBackward
            | Action::VimParagraphForward
            | Action::VimDeleteChar
            | Action::VimDeleteToEnd
            | Action::VimDeleteLine
            | Action::VimChangeToEnd
            | Action::VimChangeLine
            | Action::VimYank
            | Action::VimPut
            | Action::VimPutBefore
            | Action::VimStartDelete
            | Action::VimStartChange
            | Action::VimStartYank
            | Action::ExtendSelection => {
                // These are handled in handle_vim_editor_action when in Insert mode
            }
        }

        Ok(())
    }

    /// Enter insert mode (starts in Vim Normal mode within the editor)
    fn enter_insert_mode(&mut self) {
        if let Some(prompt) = self.state.selected_prompt() {
            // Create textarea with current content
            let lines: Vec<String> = prompt.content.lines().map(String::from).collect();
            let mut textarea = TextArea::new(if lines.is_empty() {
                vec![String::new()]
            } else {
                lines
            });

            // Start cursor at beginning of file in vim normal mode
            textarea.move_cursor(CursorMove::Top);
            textarea.move_cursor(CursorMove::Head);

            self.editor = Some(textarea);
            self.state.mode = Mode::Insert;
            self.state.editor_mode = EditorMode::VimNormal; // Start in Vim Normal mode
            self.state.editor_focused = true;
            self.state.visual_anchor = None;
        }
    }

    /// Exit insert mode and save changes
    fn exit_insert_mode(&mut self) -> Result<()> {
        // Get content from editor and update prompt
        if let Some(editor) = self.editor.take() {
            let new_content = editor.lines().join("\n");
            
            if let Some(prompt) = self.state.selected_prompt_mut() {
                prompt.content = new_content;
                prompt.modified = chrono::Utc::now();
            }
            
            // Save to disk
            self.save_current_prompt()?;
        }
        
        self.state.mode = Mode::Normal;
        self.state.editor_mode = EditorMode::VimNormal; // Reset editor mode
        self.state.editor_focused = false;
        self.state.visual_anchor = None;
        
        Ok(())
    }

    /// Handle vim-style editor actions when in Insert mode
    fn handle_vim_editor_action(&mut self, action: Action, key: KeyEvent) -> Result<()> {
        // Handle actions that need to exit insert mode first (borrow-sensitive)
        match action {
            Action::ExitMode => {
                return self.exit_insert_mode();
            }
            Action::Save => {
                self.save_current_prompt()?;
                self.state.editor_mode = EditorMode::VimNormal;
                self.state.visual_anchor = None;
                return Ok(());
            }
            Action::OpenHelp => {
                self.state.show_help = !self.state.show_help;
                if self.state.show_help {
                    self.state.help_scroll_offset = 0;
                }
                return Ok(());
            }
            Action::OpenReferencePopup => {
                self.open_reference_popup();
                return Ok(());
            }
            Action::QuickInsertReference => {
                self.state.notify("Quick insert not yet implemented", NotificationLevel::Warning);
                return Ok(());
            }
            Action::Quit => {
                self.state.should_quit = true;
                return Ok(());
            }
            _ => {}
        }

        // Now handle actions that operate on the editor
        let editor = match self.editor.as_mut() {
            Some(e) => e,
            None => return Ok(()),
        };

        // Check if we're in operator-pending mode and handle motions
        if let EditorMode::VimOperatorPending(operator) = self.state.editor_mode {
            // Motion actions complete the operator
            let is_motion = matches!(
                action,
                Action::VimWordForward
                | Action::VimWordBackward
                | Action::VimWordEnd
                | Action::VimLineStart
                | Action::VimFirstNonBlank
                | Action::VimLineEnd
                | Action::VimLeft
                | Action::VimRight
                | Action::VimUp
                | Action::VimDown
                | Action::VimGoToTop
                | Action::VimGoToBottom
                | Action::VimParagraphBackward
                | Action::VimParagraphForward
            );

            if is_motion {
                // Start selection, perform motion, then execute operator
                editor.start_selection();
                execute_vim_motion(editor, &action);
                
                match operator {
                    VimOperator::Delete => {
                        // cut() does both yank and delete in one operation
                        editor.cut();
                        let yanked = editor.yank_text();
                        self.state.yank_buffer = yanked;
                    }
                    VimOperator::Change => {
                        editor.cut();
                        let yanked = editor.yank_text();
                        self.state.yank_buffer = yanked;
                        self.state.editor_mode = EditorMode::VimInsert;
                        return Ok(());
                    }
                    VimOperator::Yank => {
                        editor.copy();
                        let yanked = editor.yank_text();
                        self.state.yank_buffer = yanked;
                        // copy() already cancels selection
                    }
                }
                
                self.state.editor_mode = EditorMode::VimNormal;
                self.state.visual_anchor = None;
                return Ok(());
            }
            
            // Line operations (dd, cc, yy) - already handled in keybindings
            // Just fall through to handle them below
        }

        match action {
            // Enter operator-pending mode
            Action::VimStartDelete => {
                self.state.editor_mode = EditorMode::VimOperatorPending(VimOperator::Delete);
            }
            Action::VimStartChange => {
                self.state.editor_mode = EditorMode::VimOperatorPending(VimOperator::Change);
            }
            Action::VimStartYank => {
                self.state.editor_mode = EditorMode::VimOperatorPending(VimOperator::Yank);
            }
            
            // Enter Vim Insert mode
            Action::VimEnterInsert => {
                self.state.editor_mode = EditorMode::VimInsert;
                self.state.visual_anchor = None;
                editor.cancel_selection();
            }
            Action::VimEnterInsertStart => {
                editor.move_cursor(CursorMove::Head);
                self.state.editor_mode = EditorMode::VimInsert;
                self.state.visual_anchor = None;
                editor.cancel_selection();
            }
            Action::VimEnterInsertEnd => {
                editor.move_cursor(CursorMove::End);
                self.state.editor_mode = EditorMode::VimInsert;
                self.state.visual_anchor = None;
                editor.cancel_selection();
            }
            Action::VimOpenBelow => {
                editor.move_cursor(CursorMove::End);
                editor.insert_newline();
                self.state.editor_mode = EditorMode::VimInsert;
                self.state.visual_anchor = None;
                editor.cancel_selection();
            }
            Action::VimOpenAbove => {
                editor.move_cursor(CursorMove::Head);
                editor.insert_newline();
                editor.move_cursor(CursorMove::Up);
                self.state.editor_mode = EditorMode::VimInsert;
                self.state.visual_anchor = None;
                editor.cancel_selection();
            }
            
            // Exit to Vim Normal mode (from Insert, Visual, or Operator-pending)
            Action::VimExitToNormal => {
                self.state.editor_mode = EditorMode::VimNormal;
                self.state.visual_anchor = None;
                editor.cancel_selection();
            }
            
            // Visual modes
            Action::VimEnterVisual => {
                self.state.visual_anchor = Some(editor.cursor());
                self.state.editor_mode = EditorMode::VimVisual;
                editor.start_selection();
            }
            Action::VimEnterVisualLine => {
                self.state.visual_anchor = Some(editor.cursor());
                self.state.editor_mode = EditorMode::VimVisualLine;
                editor.move_cursor(CursorMove::Head);
                editor.start_selection();
                editor.move_cursor(CursorMove::End);
            }
            
            // Movement actions
            Action::VimLeft => {
                editor.move_cursor(CursorMove::Back);
            }
            Action::VimRight => {
                editor.move_cursor(CursorMove::Forward);
            }
            Action::VimUp => {
                editor.move_cursor(CursorMove::Up);
            }
            Action::VimDown => {
                editor.move_cursor(CursorMove::Down);
            }
            Action::VimLineStart => {
                editor.move_cursor(CursorMove::Head);
            }
            Action::VimFirstNonBlank => {
                editor.move_cursor(CursorMove::Head);
                let (row, _) = editor.cursor();
                if let Some(line) = editor.lines().get(row) {
                    let first_non_blank = line.chars().take_while(|c| c.is_whitespace()).count();
                    editor.move_cursor(CursorMove::Jump(row as u16, first_non_blank as u16));
                }
            }
            Action::VimLineEnd => {
                editor.move_cursor(CursorMove::End);
            }
            Action::VimWordForward => {
                editor.move_cursor(CursorMove::WordForward);
            }
            Action::VimWordBackward => {
                editor.move_cursor(CursorMove::WordBack);
            }
            Action::VimWordEnd => {
                editor.move_cursor(CursorMove::WordForward);
                editor.move_cursor(CursorMove::Back);
            }
            Action::VimGoToTop => {
                editor.move_cursor(CursorMove::Top);
                editor.move_cursor(CursorMove::Head);
            }
            Action::VimGoToBottom => {
                editor.move_cursor(CursorMove::Bottom);
                editor.move_cursor(CursorMove::Head);
            }
            // Paragraph movements
            Action::VimParagraphBackward => {
                move_to_paragraph_boundary(editor, false);
            }
            Action::VimParagraphForward => {
                move_to_paragraph_boundary(editor, true);
            }
            
            // Editing actions - use internal yank buffer
            Action::VimDeleteChar => {
                if self.state.editor_mode.is_visual() {
                    editor.cut();
                    let yanked = editor.yank_text();
                    self.state.yank_buffer = yanked;
                    self.state.editor_mode = EditorMode::VimNormal;
                    self.state.visual_anchor = None;
                } else {
                    editor.delete_char();
                }
            }
            Action::VimDeleteToEnd => {
                editor.start_selection();
                editor.move_cursor(CursorMove::End);
                editor.cut();
                let yanked = editor.yank_text();
                self.state.yank_buffer = yanked;
            }
            Action::VimDeleteLine => {
                if self.state.editor_mode.is_visual() {
                    // Visual mode - delete selection
                    editor.cut();
                    let yanked = editor.yank_text();
                    self.state.yank_buffer = yanked;
                    self.state.editor_mode = EditorMode::VimNormal;
                    self.state.visual_anchor = None;
                } else {
                    // Normal mode or operator-pending (dd) - delete entire line
                    let (current_row, _) = editor.cursor();
                    let total_lines = editor.lines().len();
                    let is_last_line = current_row == total_lines.saturating_sub(1);
                    
                    if is_last_line && total_lines > 1 {
                        // On last line with more than one line: go to end of previous line
                        // and select through end of current line
                        editor.move_cursor(CursorMove::Up);
                        editor.move_cursor(CursorMove::End);
                        editor.start_selection();
                        editor.move_cursor(CursorMove::Down);
                        editor.move_cursor(CursorMove::End);
                        editor.cut();
                        let yanked = editor.yank_text();
                        self.state.yank_buffer = yanked;
                    } else if is_last_line && total_lines == 1 {
                        // Only one line: select entire content
                        editor.move_cursor(CursorMove::Head);
                        editor.start_selection();
                        editor.move_cursor(CursorMove::End);
                        editor.cut();
                        let yanked = editor.yank_text();
                        self.state.yank_buffer = yanked + "\n";
                    } else {
                        // Normal case: select from head through next line start
                        editor.move_cursor(CursorMove::Head);
                        editor.start_selection();
                        editor.move_cursor(CursorMove::Down);
                        editor.cut();
                        let yanked = editor.yank_text();
                        self.state.yank_buffer = yanked;
                    }
                    self.state.editor_mode = EditorMode::VimNormal;
                }
            }
            Action::VimChangeToEnd => {
                editor.start_selection();
                editor.move_cursor(CursorMove::End);
                editor.cut();
                let yanked = editor.yank_text();
                self.state.yank_buffer = yanked;
                self.state.editor_mode = EditorMode::VimInsert;
                self.state.visual_anchor = None;
            }
            Action::VimChangeLine => {
                if self.state.editor_mode.is_visual() {
                    // Visual mode - change selection
                    editor.cut();
                    let yanked = editor.yank_text();
                    self.state.yank_buffer = yanked;
                    self.state.editor_mode = EditorMode::VimInsert;
                    self.state.visual_anchor = None;
                } else {
                    // Normal mode or operator-pending (cc) - change entire line (but keep indentation)
                    editor.move_cursor(CursorMove::Head);
                    editor.start_selection();
                    editor.move_cursor(CursorMove::End);
                    editor.cut();
                    let yanked = editor.yank_text();
                    self.state.yank_buffer = yanked;
                    self.state.editor_mode = EditorMode::VimInsert;
                }
            }
            
            // Yank actions - use internal yank buffer
            Action::VimYank => {
                if self.state.editor_mode.is_visual() {
                    // Visual mode - yank selection
                    editor.copy();
                    let yanked = editor.yank_text();
                    self.state.yank_buffer = yanked;
                    editor.cancel_selection();
                    self.state.editor_mode = EditorMode::VimNormal;
                    self.state.visual_anchor = None;
                } else {
                    // Normal mode or operator-pending (yy) - yank entire line
                    let (row, col) = editor.cursor();
                    editor.move_cursor(CursorMove::Head);
                    editor.start_selection();
                    editor.move_cursor(CursorMove::End);
                    editor.copy();
                    let yanked = editor.yank_text();
                    self.state.yank_buffer = yanked + "\n";
                    // copy() already cancels selection
                    editor.move_cursor(CursorMove::Jump(row as u16, col as u16));
                    self.state.editor_mode = EditorMode::VimNormal;
                }
            }
            
            // Put actions - use internal yank buffer
            Action::VimPut => {
                if !self.state.yank_buffer.is_empty() {
                    // Check if yanked text ends with newline (line-wise paste)
                    if self.state.yank_buffer.ends_with('\n') {
                        editor.move_cursor(CursorMove::End);
                        editor.insert_newline();
                        let text = self.state.yank_buffer.trim_end_matches('\n');
                        editor.insert_str(text);
                    } else {
                        editor.move_cursor(CursorMove::Forward);
                        editor.insert_str(&self.state.yank_buffer);
                    }
                }
            }
            Action::VimPutBefore => {
                if !self.state.yank_buffer.is_empty() {
                    if self.state.yank_buffer.ends_with('\n') {
                        editor.move_cursor(CursorMove::Head);
                        let text = self.state.yank_buffer.trim_end_matches('\n');
                        editor.insert_str(text);
                        editor.insert_newline();
                        editor.move_cursor(CursorMove::Up);
                    } else {
                        editor.insert_str(&self.state.yank_buffer);
                    }
                }
            }
            
            // Standard editing actions
            Action::Undo => {
                editor.undo();
            }
            Action::Redo => {
                editor.redo();
            }
            Action::SelectAll => {
                editor.select_all();
                self.state.editor_mode = EditorMode::VimVisual;
                self.state.visual_anchor = Some((0, 0));
            }
            Action::CopySelection => {
                // Use system clipboard for Ctrl+C
                editor.copy();
                if let Ok(mut cb) = arboard::Clipboard::new() {
                    let _ = cb.set_text(&editor.yank_text());
                }
            }
            Action::Paste => {
                // Use system clipboard for Ctrl+V
                if let Ok(mut cb) = arboard::Clipboard::new() {
                    if let Ok(text) = cb.get_text() {
                        editor.insert_str(&text);
                    }
                }
            }
            
            // Hybrid: Extend selection with Shift+Arrow
            Action::ExtendSelection => {
                if !self.state.editor_mode.is_visual() {
                    self.state.visual_anchor = Some(editor.cursor());
                    self.state.editor_mode = EditorMode::VimVisual;
                    editor.start_selection();
                }
                editor.input(key);
            }
            
            Action::None => {
                if self.state.editor_mode == EditorMode::VimInsert {
                    editor.input(key);
                }
            }
            
            _ => {}
        }
        
        Ok(())
    }

    /// Copy yank buffer to system clipboard
    fn copy_yank_to_clipboard(&mut self, editor: &TextArea) {
        let yanked = editor.yank_text();
        if !yanked.is_empty() {
            match arboard::Clipboard::new() {
                Ok(mut clipboard) => {
                    if let Err(e) = clipboard.set_text(&yanked) {
                        self.state.notify(format!("Clipboard error: {}", e), NotificationLevel::Error);
                    }
                }
                Err(e) => {
                    self.state.notify(format!("Clipboard error: {}", e), NotificationLevel::Error);
                }
            }
        }
    }

    /// Paste from system clipboard
    fn paste_from_clipboard(&mut self, editor: &mut TextArea) {
        match arboard::Clipboard::new() {
            Ok(mut clipboard) => {
                match clipboard.get_text() {
                    Ok(text) => {
                        editor.insert_str(&text);
                    }
                    Err(e) => {
                        self.state.notify(format!("Clipboard error: {}", e), NotificationLevel::Error);
                    }
                }
            }
            Err(e) => {
                self.state.notify(format!("Clipboard error: {}", e), NotificationLevel::Error);
            }
        }
    }

    /// Create a new prompt
    fn create_new_prompt(&mut self) -> Result<()> {
        let existing_names: Vec<&str> = self.state.prompts.iter().map(|p| p.name.as_str()).collect();
        
        let mut prompt = Prompt::new();
        prompt.name = crate::models::prompt::make_unique_name("new_prompt", &existing_names);
        prompt.content = String::new();

        // Save to disk
        save_prompt(&prompt, &prompts_dir()?)?;

        // Update index
        let entry = IndexEntry::from_prompt(&prompt, "prompts");
        self.index.upsert(entry);
        self.index.save(&index_path()?)?;

        // Add to both lists
        self.all_prompts.push(prompt.clone());
        self.state.prompts.push(prompt);
        self.state.selected_index = self.state.prompts.len() - 1;

        // Enter insert mode with empty editor (start in Vim Insert for new prompts)
        self.editor = Some(TextArea::new(vec![String::new()]));
        self.state.mode = Mode::Insert;
        self.state.editor_mode = EditorMode::VimInsert; // New prompt goes directly to insert
        self.state.editor_focused = true;
        self.state.visual_anchor = None;

        self.state.notify("Created new prompt", NotificationLevel::Success);

        Ok(())
    }

    /// Save the current prompt
    fn save_current_prompt(&mut self) -> Result<()> {
        // If we have an active editor, update the prompt content first
        if let Some(ref editor) = self.editor {
            let new_content = editor.lines().join("\n");
            if let Some(prompt) = self.state.selected_prompt_mut() {
                prompt.content = new_content.clone();
                prompt.modified = chrono::Utc::now();
                
                // Also update in all_prompts
                if let Some(all_prompt) = self.all_prompts.iter_mut().find(|p| p.id == prompt.id) {
                    all_prompt.content = new_content;
                    all_prompt.modified = prompt.modified;
                }
            }
        }

        if let Some(prompt) = self.state.selected_prompt() {
            save_prompt(prompt, &prompts_dir()?)?;

            // Update index
            let entry = IndexEntry::from_prompt(prompt, "prompts");
            self.index.upsert(entry);
            self.index.save(&index_path()?)?;

            self.state.notify("Saved", NotificationLevel::Success);
        }

        Ok(())
    }

    /// Rename the current prompt
    /// For now, this auto-renames based on content (regenerates name from first line)
    fn rename_current_prompt(&mut self) -> Result<()> {
        if self.state.mode != Mode::Normal {
            return Ok(());
        }

        if let Some(prompt) = self.state.selected_prompt() {
            let old_name = prompt.name.clone();
            let content = prompt.content.clone();

            // Generate new name from content
            let base_name = crate::models::prompt::generate_name_from_content(&content);
            
            if base_name.is_empty() {
                self.state.notify("Cannot rename: prompt has no content", NotificationLevel::Warning);
                return Ok(());
            }

            // Check if new name would be the same
            if base_name == old_name {
                self.state.notify("Name unchanged", NotificationLevel::Info);
                return Ok(());
            }

            // Get all existing names except current prompt (use all_prompts for uniqueness)
            let existing_names: Vec<&str> = self.all_prompts
                .iter()
                .filter(|p| p.name != old_name)
                .map(|p| p.name.as_str())
                .collect();

            // Make the name unique
            let new_name = crate::models::prompt::make_unique_name(&base_name, &existing_names);

            // Rename file on disk
            let dir = prompts_dir()?;
            crate::fs::rename_prompt(&old_name, &new_name, &dir)?;

            // Update index
            self.index.remove(&old_name);
            
            // Update the prompt in state
            if let Some(prompt) = self.state.selected_prompt_mut() {
                prompt.name = new_name.clone();
                
                // Re-add to index with new name
                let entry = IndexEntry::from_prompt(prompt, "prompts");
                self.index.upsert(entry);
            }
            
            // Also update in all_prompts
            if let Some(prompt) = self.all_prompts.iter_mut().find(|p| p.name == old_name) {
                prompt.name = new_name.clone();
            }
            
            self.index.save(&index_path()?)?;

            self.state.notify(format!("Renamed '{}' to '{}'", old_name, new_name), NotificationLevel::Success);
        }

        Ok(())
    }

    /// Duplicate the current prompt
    fn duplicate_current_prompt(&mut self) -> Result<()> {
        if !self.state.has_prompts() {
            return Ok(());
        }

        if let Some(prompt) = self.state.selected_prompt() {
            // Clone the prompt data
            let content = prompt.content.clone();
            let tags = prompt.tags.clone();

            // Get all existing names for uniqueness check (from all_prompts to be safe)
            let existing_names: Vec<&str> = self.all_prompts
                .iter()
                .map(|p| p.name.as_str())
                .collect();

            // Create new prompt with same content
            let mut new_prompt = crate::models::Prompt::with_content(&content);
            new_prompt.tags = tags;
            
            // Generate a unique name based on original
            let base_name = crate::models::prompt::generate_name_from_content(&content);
            new_prompt.name = crate::models::prompt::make_unique_name(&base_name, &existing_names);

            // Save to disk
            save_prompt(&new_prompt, &prompts_dir()?)?;

            // Update index
            let entry = IndexEntry::from_prompt(&new_prompt, "prompts");
            self.index.upsert(entry);
            self.index.save(&index_path()?)?;

            // Add to both lists
            self.all_prompts.push(new_prompt.clone());
            self.state.prompts.push(new_prompt.clone());
            self.state.selected_index = self.state.prompts.len() - 1;

            self.state.notify(format!("Duplicated as '{}'", new_prompt.name), NotificationLevel::Success);
        }

        Ok(())
    }

    /// Request confirmation before deleting
    fn request_delete_confirmation(&mut self) -> Result<()> {
        if let Some(prompt) = self.state.selected_prompt() {
            let name = prompt.name.clone();
            let is_archive = self.state.mode == Mode::Archive;
            
            let (title, message, action) = if is_archive {
                (
                    "Permanently Delete",
                    format!("Are you sure you want to permanently delete '{}'?\n\nThis cannot be undone.", name),
                    PendingAction::PermanentDelete { name },
                )
            } else {
                (
                    "Delete Prompt",
                    format!("Are you sure you want to delete '{}'?", name),
                    PendingAction::DeletePrompt { name },
                )
            };
            
            self.state.confirm_dialog = Some(ConfirmDialog::new(title, message, action));
        }
        
        Ok(())
    }

    /// Handle confirmation of a pending action
    fn handle_confirm(&mut self) -> Result<()> {
        if let Some(dialog) = self.state.confirm_dialog.take() {
            if dialog.yes_selected {
                match dialog.pending_action {
                    PendingAction::DeletePrompt { name } | PendingAction::PermanentDelete { name } => {
                        self.execute_delete(&name)?;
                    }
                    PendingAction::ExecuteCommandsAndCopy { content_with_refs, .. } => {
                        // User confirmed: execute commands and copy to clipboard
                        let final_content = crate::engine::resolve_commands_in_content(&content_with_refs);
                        self.copy_text_to_clipboard(&final_content)?;
                    }
                }
            }
            // Dialog is already taken (consumed), no need to clear
        }
        
        Ok(())
    }

    /// Execute the actual deletion
    fn execute_delete(&mut self, name: &str) -> Result<()> {
        // Determine directory based on mode
        let dir = match self.state.mode {
            Mode::Archive => archive_dir()?,
            _ => prompts_dir()?,
        };

        // Delete from disk
        delete_prompt(name, &dir)?;

        // Remove from index
        self.index.remove(name);
        self.index.save(&index_path()?)?;

        // Remove from the filtered list
        if let Some(pos) = self.state.prompts.iter().position(|p| p.name == name) {
            self.state.prompts.remove(pos);
            
            // Adjust selection
            if self.state.selected_index >= self.state.prompts.len() && self.state.selected_index > 0 {
                self.state.selected_index -= 1;
            }
        }

        // Also remove from all_prompts
        if let Some(pos) = self.all_prompts.iter().position(|p| p.name == name) {
            self.all_prompts.remove(pos);
        }

        if self.state.mode == Mode::Archive {
            self.archived_count = self.archived_count.saturating_sub(1);
        }

        self.state.notify(format!("Deleted '{}'", name), NotificationLevel::Success);
        
        Ok(())
    }

    /// Delete the current prompt (kept for backwards compatibility, now uses confirmation)
    fn delete_current_prompt(&mut self) -> Result<()> {
        if let Some(prompt) = self.state.selected_prompt() {
            let name = prompt.name.clone();

            // Determine directory based on mode
            let dir = match self.state.mode {
                Mode::Archive => archive_dir()?,
                _ => prompts_dir()?,
            };

            // Delete from disk
            delete_prompt(&name, &dir)?;

            // Remove from index
            self.index.remove(&name);
            self.index.save(&index_path()?)?;

            // Remove from list
            self.state.prompts.remove(self.state.selected_index);

            // Adjust selection
            if self.state.selected_index >= self.state.prompts.len() && self.state.selected_index > 0 {
                self.state.selected_index -= 1;
            }

            if self.state.mode == Mode::Archive {
                self.archived_count = self.archived_count.saturating_sub(1);
            }

            self.state.notify(format!("Deleted '{}'", name), NotificationLevel::Success);
        }

        Ok(())
    }

    /// Archive the current prompt
    fn archive_current_prompt(&mut self) -> Result<()> {
        if self.state.mode != Mode::Normal {
            return Ok(());
        }

        if let Some(prompt) = self.state.selected_prompt() {
            let name = prompt.name.clone();

            // Move file to archive
            crate::fs::move_prompt(&name, &prompts_dir()?, &archive_dir()?)?;

            // Update index
            if let Some(entry) = self.index.entries.get_mut(&name) {
                entry.location = "archive".to_string();
            }
            self.index.save(&index_path()?)?;

            // Remove from current list
            self.state.prompts.remove(self.state.selected_index);

            // Also remove from all_prompts
            if let Some(pos) = self.all_prompts.iter().position(|p| p.name == name) {
                self.all_prompts.remove(pos);
            }

            // Adjust selection
            if self.state.selected_index >= self.state.prompts.len() && self.state.selected_index > 0 {
                self.state.selected_index -= 1;
            }

            self.archived_count += 1;

            self.state.notify(format!("Archived '{}'", name), NotificationLevel::Success);
        }

        Ok(())
    }

    /// Unarchive the current prompt
    fn unarchive_current_prompt(&mut self) -> Result<()> {
        if self.state.mode != Mode::Archive {
            return Ok(());
        }

        if let Some(prompt) = self.state.selected_prompt() {
            let name = prompt.name.clone();
            let unarchived_prompt = prompt.clone();

            // Move file back to prompts
            crate::fs::move_prompt(&name, &archive_dir()?, &prompts_dir()?)?;

            // Update index
            if let Some(entry) = self.index.entries.get_mut(&name) {
                entry.location = "prompts".to_string();
            }
            self.index.save(&index_path()?)?;

            // Add to all_prompts (will be added when returning to Normal mode via reload)
            self.all_prompts.push(unarchived_prompt);

            // Remove from current list (archive view)
            self.state.prompts.remove(self.state.selected_index);

            // Adjust selection
            if self.state.selected_index >= self.state.prompts.len() && self.state.selected_index > 0 {
                self.state.selected_index -= 1;
            }

            self.archived_count = self.archived_count.saturating_sub(1);

            self.state.notify(format!("Unarchived '{}'", name), NotificationLevel::Success);
        }

        Ok(())
    }

    /// Copy prompt content to clipboard
    fn copy_to_clipboard(&mut self, resolve: bool) -> Result<()> {
        if let Some(prompt) = self.state.selected_prompt() {
            if resolve {
                // Resolve references first (always safe)
                let get_content = |name: &str| -> Option<String> {
                    self.state.prompts.iter().find(|p| p.name == name).map(|p| p.content.clone())
                };
                
                // Resolve references but don't execute commands yet
                let result = crate::engine::resolve_prompt(&prompt.content, get_content, false);
                
                // Check if there are commands to execute
                if !result.commands.is_empty() && self.state.safe_mode {
                    // Safe mode ON: show confirmation dialog with list of commands
                    let cmd_list = result.commands.iter()
                        .enumerate()
                        .map(|(i, c)| format!("{}. {}", i + 1, c))
                        .collect::<Vec<_>>()
                        .join("\n");
                    
                    let message = format!(
                        "The following commands will be executed:\n\n{}\n\nProceed?",
                        cmd_list
                    );
                    
                    let action = PendingAction::ExecuteCommandsAndCopy {
                        commands: result.commands,
                        content_with_refs: result.content,
                    };
                    
                    self.state.confirm_dialog = Some(ConfirmDialog::new(
                        "Execute Commands?",
                        message,
                        action,
                    ));
                    return Ok(());
                }
                
                // Either no commands, or safe mode is OFF - execute immediately
                let final_content = if !result.commands.is_empty() {
                    crate::engine::resolve_commands_in_content(&result.content)
                } else {
                    result.content
                };
                
                self.copy_text_to_clipboard(&final_content)?;
            } else {
                // Raw copy - no resolution
                let content = prompt.content.clone();
                self.copy_text_to_clipboard(&content)?;
            }
        }

        Ok(())
    }

    /// Actually copy text to the system clipboard
    fn copy_text_to_clipboard(&mut self, content: &str) -> Result<()> {
        match arboard::Clipboard::new() {
            Ok(mut clipboard) => {
                if let Err(e) = clipboard.set_text(content) {
                    self.state.notify(format!("Clipboard error: {}", e), NotificationLevel::Error);
                } else {
                    // On Linux, clipboard content is owned by the application.
                    // We need to keep the clipboard alive briefly for clipboard managers to capture it.
                    std::thread::sleep(Duration::from_millis(100));
                    self.state.notify("Copied to clipboard", NotificationLevel::Success);
                }
            }
            Err(e) => {
                self.state.notify(format!("Clipboard error: {}", e), NotificationLevel::Error);
            }
        }
        Ok(())
    }

    /// Cycle through tag filters
    fn cycle_tag_filter(&mut self, forward: bool) {
        if self.state.all_tags.is_empty() {
            return;
        }

        match &self.state.tag_filter {
            None => {
                // Start filtering by first/last tag
                let tag = if forward {
                    self.state.all_tags.first()
                } else {
                    self.state.all_tags.last()
                };
                self.state.tag_filter = tag.cloned();
            }
            Some(current) => {
                if let Some(pos) = self.state.all_tags.iter().position(|t| t == current) {
                    if forward {
                        if pos + 1 < self.state.all_tags.len() {
                            self.state.tag_filter = Some(self.state.all_tags[pos + 1].clone());
                        } else {
                            self.state.tag_filter = None; // Wrap to "all"
                        }
                    } else {
                        if pos > 0 {
                            self.state.tag_filter = Some(self.state.all_tags[pos - 1].clone());
                        } else {
                            self.state.tag_filter = None; // Wrap to "all"
                        }
                    }
                }
            }
        }

        // Apply the filter to the prompt list
        self.apply_tag_filter();

        // Show notification
        if let Some(tag) = &self.state.tag_filter {
            self.state.notify(format!("Filtering by: {}", tag), NotificationLevel::Info);
        } else {
            self.state.notify("Showing all prompts", NotificationLevel::Info);
        }
    }

    /// Apply the current tag filter to the prompt list
    fn apply_tag_filter(&mut self) {
        match &self.state.tag_filter {
            None => {
                // Show all prompts
                self.state.prompts = self.all_prompts.clone();
            }
            Some(tag) => {
                // Filter prompts that have this tag
                self.state.prompts = self.all_prompts
                    .iter()
                    .filter(|p| p.tags.contains(tag))
                    .cloned()
                    .collect();
            }
        }
        
        // Reset selection if it's out of bounds
        if self.state.selected_index >= self.state.prompts.len() {
            self.state.selected_index = self.state.prompts.len().saturating_sub(1);
        }
    }

    /// Reload prompts from disk
    fn reload_prompts(&mut self) -> Result<()> {
        let prompts = load_all_prompts(&prompts_dir()?)?;
        self.all_prompts = prompts.clone();
        self.state.prompts = prompts;
        self.state.current_folder = None;
        self.state.tag_filter = None;  // Reset filter when reloading
        
        // Re-collect all tags
        let mut all_tags: Vec<String> = self.all_prompts
            .iter()
            .flat_map(|p| p.tags.clone())
            .collect();
        all_tags.sort();
        all_tags.dedup();
        self.state.all_tags = all_tags;
        
        if self.state.selected_index >= self.state.prompts.len() {
            self.state.selected_index = self.state.prompts.len().saturating_sub(1);
        }

        Ok(())
    }

    /// Open the rename popup
    fn open_rename_popup(&mut self) {
        if self.state.mode != Mode::Normal {
            return;
        }

        if let Some(prompt) = self.state.selected_prompt() {
            let popup = crate::models::RenamePopupState::new(prompt.name.clone());
            self.state.rename_popup = Some(popup);
        }
    }

    /// Handle text input in rename popup
    fn handle_rename_popup_input(&mut self, key: crossterm::event::KeyEvent) {
        use crossterm::event::KeyCode;

        if let Some(ref mut popup) = self.state.rename_popup {
            match key.code {
                KeyCode::Char(c) => {
                    popup.input.push(c);
                    self.validate_rename_input();
                }
                KeyCode::Backspace => {
                    popup.input.pop();
                    self.validate_rename_input();
                }
                _ => {}
            }
        }
    }

    /// Validate the current rename input
    fn validate_rename_input(&mut self) {
        if let Some(ref mut popup) = self.state.rename_popup {
            let input = &popup.input;

            // Check if name is valid format
            if !crate::models::prompt::is_valid_name(input) {
                popup.is_valid = false;
                popup.error_message = Some("Invalid characters (use a-z, 0-9, _)".to_string());
                return;
            }

            // Check if name is unique
            let existing_names: Vec<&str> = self.all_prompts
                .iter()
                .map(|p| p.name.as_str())
                .collect();

            if !crate::models::prompt::is_name_unique(input, &existing_names, Some(&popup.original_name)) {
                popup.is_valid = false;
                popup.error_message = Some("Name already exists".to_string());
                return;
            }

            popup.is_valid = true;
            popup.error_message = None;
        }
    }

    /// Confirm rename from popup
    fn confirm_rename_popup(&mut self) -> Result<()> {
        let popup = match self.state.rename_popup.take() {
            Some(p) => p,
            None => return Ok(()),
        };

        if !popup.is_valid {
            // Put the popup back if not valid
            self.state.rename_popup = Some(popup);
            return Ok(());
        }

        let old_name = popup.original_name;
        let new_name = popup.input;

        if old_name == new_name {
            self.state.notify("Name unchanged", NotificationLevel::Info);
            return Ok(());
        }

        // Rename file on disk
        let dir = prompts_dir()?;
        crate::fs::rename_prompt(&old_name, &new_name, &dir)?;

        // Update index
        self.index.remove(&old_name);

        // Update the prompt in state
        if let Some(prompt) = self.state.selected_prompt_mut() {
            prompt.name = new_name.clone();

            // Re-add to index with new name
            let entry = IndexEntry::from_prompt(prompt, "prompts");
            self.index.upsert(entry);
        }

        // Also update in all_prompts
        if let Some(prompt) = self.all_prompts.iter_mut().find(|p| p.name == old_name) {
            prompt.name = new_name.clone();
        }

        self.index.save(&index_path()?)?;

        self.state.notify(format!("Renamed '{}' to '{}'", old_name, new_name), NotificationLevel::Success);

        Ok(())
    }

    /// Open the reference popup (for r in vim normal mode or Ctrl+r in insert mode)
    fn open_reference_popup(&mut self) {
        // Only allow reference popup in Insert mode (editor is active)
        if self.state.mode != Mode::Insert {
            return;
        }

        let all_names: Vec<String> = self.all_prompts
            .iter()
            .map(|p| p.name.clone())
            .collect();

        let popup = crate::models::ReferencePopupState::new(all_names);
        self.state.reference_popup = Some(popup);
    }

    /// Handle text input in reference popup
    fn handle_reference_popup_input(&mut self, key: crossterm::event::KeyEvent) {
        use crossterm::event::KeyCode;

        let all_names: Vec<String> = self.all_prompts
            .iter()
            .map(|p| p.name.clone())
            .collect();

        if let Some(ref mut popup) = self.state.reference_popup {
            match key.code {
                KeyCode::Char(c) => {
                    popup.filter.push(c);
                    popup.update_filter(&all_names);
                }
                KeyCode::Backspace => {
                    popup.filter.pop();
                    popup.update_filter(&all_names);
                }
                _ => {}
            }
        }
    }

    /// Confirm reference selection and insert into editor
    fn confirm_reference_popup(&mut self) -> Result<()> {
        let popup = match self.state.reference_popup.take() {
            Some(p) => p,
            None => return Ok(()),
        };

        let selected_name = match popup.selected_name() {
            Some(name) => name.to_string(),
            None => {
                self.state.notify("No prompt selected", NotificationLevel::Warning);
                return Ok(());
            }
        };

        // Insert the reference into the editor
        if let Some(ref mut editor) = self.editor {
            let reference = format!("[[{}]]", selected_name);
            editor.insert_str(&reference);
        }

        self.state.notify(format!("Inserted [[{}]]", selected_name), NotificationLevel::Success);

        Ok(())
    }

    /// Open the tag selector popup
    fn open_tag_selector(&mut self) {
        if self.state.mode != Mode::Normal {
            return;
        }

        if let Some(prompt) = self.state.selected_prompt() {
            let prompt_tags = prompt.tags.clone();
            let all_tags = self.state.all_tags.clone();
            let selector = TagSelectorState::new(all_tags, prompt_tags);
            self.state.tag_selector = Some(selector);
        }
    }

    /// Handle text input in tag selector popup
    fn handle_tag_selector_input(&mut self, key: crossterm::event::KeyEvent) {
        use crossterm::event::KeyCode;

        if let Some(ref mut selector) = self.state.tag_selector {
            if selector.creating_new {
                match key.code {
                    KeyCode::Char(c) => {
                        selector.new_tag_input.push(c);
                    }
                    KeyCode::Backspace => {
                        selector.new_tag_input.pop();
                    }
                    _ => {}
                }
            } else {
                match key.code {
                    KeyCode::Char(c) => {
                        selector.filter.push(c);
                        selector.update_filter();
                    }
                    KeyCode::Backspace => {
                        selector.filter.pop();
                        selector.update_filter();
                    }
                    _ => {}
                }
            }
        }
    }

    /// Toggle the selected tag on the current prompt
    fn toggle_tag(&mut self) -> Result<()> {
        let (tag, added) = {
            if let Some(ref mut selector) = self.state.tag_selector {
                match selector.toggle_selected_tag() {
                    Some(result) => result,
                    None => return Ok(()),
                }
            } else {
                return Ok(());
            }
        };

        // Update the prompt's tags
        if let Some(prompt) = self.state.selected_prompt_mut() {
            if added {
                if !prompt.tags.contains(&tag) {
                    prompt.tags.push(tag.clone());
                }
            } else {
                prompt.tags.retain(|t| t != &tag);
            }
        }

        // Also update in all_prompts
        if let Some(prompt) = self.state.selected_prompt() {
            let id = prompt.id;
            let new_tags = prompt.tags.clone();
            if let Some(all_prompt) = self.all_prompts.iter_mut().find(|p| p.id == id) {
                all_prompt.tags = new_tags;
            }
        }

        let msg = if added {
            format!("Added tag: {}", tag)
        } else {
            format!("Removed tag: {}", tag)
        };
        self.state.notify(msg, NotificationLevel::Success);

        Ok(())
    }

    /// Close the tag selector and save changes
    fn close_tag_selector(&mut self) -> Result<()> {
        if let Some(selector) = self.state.tag_selector.take() {
            // Update the prompt's tags from the selector's state
            if let Some(prompt) = self.state.selected_prompt_mut() {
                prompt.tags = selector.prompt_tags.clone();
                prompt.modified = chrono::Utc::now();
            }

            // Update all_prompts as well
            if let Some(prompt) = self.state.selected_prompt() {
                let id = prompt.id;
                let new_tags = prompt.tags.clone();
                if let Some(all_prompt) = self.all_prompts.iter_mut().find(|p| p.id == id) {
                    all_prompt.tags = new_tags;
                }
            }

            // Update all_tags in state
            let mut all_tags: Vec<String> = self.all_prompts
                .iter()
                .flat_map(|p| p.tags.clone())
                .collect();
            all_tags.extend(selector.all_tags);
            all_tags.sort();
            all_tags.dedup();
            self.state.all_tags = all_tags;

            // Save the prompt
            self.save_current_prompt()?;
        }
        Ok(())
    }

    /// Confirm new tag creation
    fn confirm_new_tag(&mut self) -> Result<()> {
        let new_tag = {
            if let Some(ref mut selector) = self.state.tag_selector {
                selector.confirm_new_tag()
            } else {
                None
            }
        };

        if let Some(tag) = new_tag {
            // Add to all_tags if not present
            if !self.state.all_tags.contains(&tag) {
                self.state.all_tags.push(tag.clone());
                self.state.all_tags.sort();
            }

            self.state.notify(format!("Created and assigned tag: {}", tag), NotificationLevel::Success);
        }

        Ok(())
    }

    /// Open the folder selector popup
    fn open_folder_selector(&mut self, mode: FolderSelectorMode) -> Result<()> {
        let folders = crate::fs::list_folders()?;
        let selector = FolderSelectorState::new(folders, mode);
        self.state.folder_selector = Some(selector);
        Ok(())
    }

    /// Handle text input in folder selector popup
    fn handle_folder_selector_input(&mut self, key: crossterm::event::KeyEvent) {
        use crossterm::event::KeyCode;

        if let Some(ref mut selector) = self.state.folder_selector {
            if selector.creating_new {
                match key.code {
                    KeyCode::Char(c) => {
                        selector.new_folder_input.push(c);
                    }
                    KeyCode::Backspace => {
                        selector.new_folder_input.pop();
                    }
                    _ => {}
                }
            } else {
                match key.code {
                    KeyCode::Char(c) => {
                        selector.filter.push(c);
                        selector.update_filter();
                    }
                    KeyCode::Backspace => {
                        selector.filter.pop();
                        selector.update_filter();
                    }
                    _ => {}
                }
            }
        }
    }

    /// Confirm folder selection (open folder or move prompt)
    fn confirm_folder_selection(&mut self) -> Result<()> {
        let selector = match self.state.folder_selector.take() {
            Some(s) => s,
            None => return Ok(()),
        };

        let selected = selector.selected_folder().map(|s| s.to_string());

        match selector.mode {
            FolderSelectorMode::Open => {
                // Open the folder (or go to root)
                match selected {
                    None => {
                        // Go to root - reload prompts from main prompts directory
                        self.state.current_folder = None;
                        self.state.mode = Mode::Normal;
                        self.reload_prompts()?;
                        self.state.notify("Viewing all prompts", NotificationLevel::Info);
                    }
                    Some(folder) => {
                        // Load prompts from selected folder
                        let folder_path = folders_dir()?.join(&folder);
                        if folder_path.exists() {
                            let prompts = load_all_prompts(&folder_path)?;
                            self.state.prompts = prompts;
                            self.state.current_folder = Some(folder.clone());
                            self.state.mode = Mode::Folder;
                            self.state.selected_index = 0;
                            self.state.notify(format!("Opened folder: {}", folder), NotificationLevel::Info);
                        } else {
                            self.state.notify(format!("Folder not found: {}", folder), NotificationLevel::Error);
                        }
                    }
                }
            }
            FolderSelectorMode::Move => {
                // Move the current prompt to the selected folder
                if let Some(prompt) = self.state.selected_prompt() {
                    let name = prompt.name.clone();
                    let source_dir = match &self.state.current_folder {
                        None => prompts_dir()?,
                        Some(folder) => folders_dir()?.join(folder),
                    };

                    let dest_dir = match selected {
                        None => prompts_dir()?,
                        Some(ref folder) => folders_dir()?.join(folder),
                    };

                    if source_dir != dest_dir {
                        crate::fs::move_prompt(&name, &source_dir, &dest_dir)?;

                        // Update index location
                        let location = match &selected {
                            None => "prompts".to_string(),
                            Some(folder) => format!("folders/{}", folder),
                        };
                        if let Some(entry) = self.index.entries.get_mut(&name) {
                            entry.location = location;
                        }
                        self.index.save(&index_path()?)?;

                        // Remove from current list
                        self.state.prompts.remove(self.state.selected_index);
                        if self.state.selected_index >= self.state.prompts.len() && self.state.selected_index > 0 {
                            self.state.selected_index -= 1;
                        }

                        // Also update/remove from all_prompts
                        if let Some(pos) = self.all_prompts.iter().position(|p| p.name == name) {
                            self.all_prompts.remove(pos);
                        }

                        let dest_name = match selected {
                            None => "root".to_string(),
                            Some(folder) => folder,
                        };
                        self.state.notify(format!("Moved '{}' to {}", name, dest_name), NotificationLevel::Success);
                    }
                }
            }
        }

        Ok(())
    }

    /// Confirm new folder creation
    fn confirm_new_folder(&mut self) -> Result<()> {
        let (new_folder, mode) = {
            if let Some(ref mut selector) = self.state.folder_selector {
                (selector.confirm_new_folder(), selector.mode)
            } else {
                return Ok(());
            }
        };

        if let Some(folder) = new_folder {
            // Create the folder on disk
            crate::fs::create_folder(&folder)?;
            self.state.notify(format!("Created folder: {}", folder), NotificationLevel::Success);

            // If we're in Move mode, move the prompt there immediately
            if mode == FolderSelectorMode::Move {
                if let Some(prompt) = self.state.selected_prompt() {
                    let name = prompt.name.clone();
                    let source_dir = match &self.state.current_folder {
                        None => prompts_dir()?,
                        Some(f) => folders_dir()?.join(f),
                    };
                    let dest_dir = folders_dir()?.join(&folder);

                    crate::fs::move_prompt(&name, &source_dir, &dest_dir)?;

                    // Update index
                    let location = format!("folders/{}", folder);
                    if let Some(entry) = self.index.entries.get_mut(&name) {
                        entry.location = location;
                    }
                    self.index.save(&index_path()?)?;

                    // Remove from current list
                    self.state.prompts.remove(self.state.selected_index);
                    if self.state.selected_index >= self.state.prompts.len() && self.state.selected_index > 0 {
                        self.state.selected_index -= 1;
                    }

                    // Also remove from all_prompts
                    if let Some(pos) = self.all_prompts.iter().position(|p| p.name == name) {
                        self.all_prompts.remove(pos);
                    }

                    self.state.notify(format!("Moved '{}' to {}", name, folder), NotificationLevel::Success);
                    self.state.folder_selector = None;
                }
            }
        }

        Ok(())
    }
}
