//! Main application logic

use anyhow::Result;
use crossterm::event::{self, Event, KeyEventKind};
use std::time::Duration;
use tui_textarea::TextArea;

use crate::config::{archive_dir, config_path, index_path, prompts_dir, Config};
use crate::fs::{ensure_directories, load_all_prompts, save_prompt, delete_prompt, Index, IndexEntry};
use crate::models::{Action, AppState, ConfirmDialog, Mode, NotificationLevel, PendingAction, Prompt};
use crate::tui::{init_terminal, restore_terminal, Tui};
use crate::ui::{handle_key_event, render};

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
                if let Event::Key(key) = event::read()? {
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

                        // In Insert mode, let the editor handle most keys
                        if self.state.mode == Mode::Insert {
                            if let Some(ref mut editor) = self.editor {
                                // Check for special keys that exit insert mode or perform actions
                                let action = handle_key_event(key, &self.state);
                                match action {
                                    Action::ExitMode | Action::Save => {
                                        self.handle_action(action)?;
                                    }
                                    Action::Undo => {
                                        editor.undo();
                                    }
                                    Action::Redo => {
                                        editor.redo();
                                    }
                                    Action::OpenHelp => {
                                        self.handle_action(action)?;
                                    }
                                    Action::Quit => {
                                        self.handle_action(action)?;
                                    }
                                    Action::OpenReferencePopup => {
                                        self.handle_action(action)?;
                                    }
                                    _ => {
                                        // Let textarea handle the input
                                        editor.input(key);
                                    }
                                }
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
            Action::OpenFolder
            | Action::MoveToFolder
            | Action::OpenTagSelector
            | Action::OpenSearch
            | Action::QuickOpen
            | Action::QuickInsertReference
            | Action::Export => {
                self.state.notify("Feature not yet implemented", NotificationLevel::Warning);
            }
        }

        Ok(())
    }

    /// Enter insert mode
    fn enter_insert_mode(&mut self) {
        if let Some(prompt) = self.state.selected_prompt() {
            // Create textarea with current content
            let lines: Vec<String> = prompt.content.lines().map(String::from).collect();
            let mut textarea = TextArea::new(if lines.is_empty() {
                vec![String::new()]
            } else {
                lines
            });

            // Move cursor to end of file
            let line_count = textarea.lines().len();
            if line_count > 0 {
                let last_line_idx = line_count - 1;
                let last_line_len = textarea.lines()[last_line_idx].len();
                textarea.move_cursor(tui_textarea::CursorMove::Jump(
                    last_line_idx as u16,
                    last_line_len as u16,
                ));
            }

            self.editor = Some(textarea);
            self.state.mode = Mode::Insert;
            self.state.editor_focused = true;
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
        self.state.editor_focused = false;
        
        Ok(())
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

        // Enter insert mode with empty editor
        self.editor = Some(TextArea::new(vec![String::new()]));
        self.state.mode = Mode::Insert;
        self.state.editor_focused = true;

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
                    PendingAction::ExecuteCommands { commands: _ } => {
                        // TODO: Handle command execution confirmation
                        self.state.notify("Command execution not yet implemented", NotificationLevel::Warning);
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
            let content = if resolve {
                // Resolve references and commands
                let get_content = |name: &str| -> Option<String> {
                    self.state.prompts.iter().find(|p| p.name == name).map(|p| p.content.clone())
                };
                
                let result = crate::engine::resolve_prompt(&prompt.content, get_content, self.state.safe_mode);
                result.content
            } else {
                prompt.content.clone()
            };

            // Copy to clipboard
            match arboard::Clipboard::new() {
                Ok(mut clipboard) => {
                    if let Err(e) = clipboard.set_text(&content) {
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

    /// Open the reference popup (for CTRL+i in insert mode)
    fn open_reference_popup(&mut self) {
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
}
