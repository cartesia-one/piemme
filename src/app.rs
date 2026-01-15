//! Main application logic

use anyhow::Result;
use crossterm::event::{self, Event, KeyEventKind};
use std::time::Duration;

use crate::config::{archive_dir, config_path, index_path, prompts_dir, Config};
use crate::fs::{ensure_directories, load_all_prompts, save_prompt, delete_prompt, Index, IndexEntry};
use crate::models::{Action, AppState, Mode, NotificationLevel, Prompt};
use crate::tui::{init_terminal, restore_terminal, Tui};
use crate::ui::{handle_key_event, render};

/// The main application
pub struct App {
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
}

impl App {
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
        })
    }

    /// Run the main application loop
    pub fn run(&mut self) -> Result<()> {
        loop {
            // Draw UI
            self.terminal.draw(|frame| {
                render(frame, &self.state, &self.config, self.archived_count);
            })?;

            // Handle events
            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    // Only handle key press events (not release)
                    if key.kind == KeyEventKind::Press {
                        let action = handle_key_event(key, &self.state);
                        self.handle_action(action)?;
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
                    self.state.mode = Mode::Insert;
                    self.state.editor_focused = true;
                }
            }
            Action::ExitMode => {
                match self.state.mode {
                    Mode::Insert => {
                        // Save on exit from insert mode
                        self.save_current_prompt()?;
                        self.state.mode = Mode::Normal;
                    }
                    Mode::Archive | Mode::Folder | Mode::Preview => {
                        self.state.mode = Mode::Normal;
                        // Reload main prompts when exiting archive/folder
                        if matches!(self.state.mode, Mode::Archive | Mode::Folder) {
                            self.reload_prompts()?;
                        }
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
                self.delete_current_prompt()?;
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
            }
            Action::CloseOverlay => {
                self.state.show_help = false;
                self.state.active_popup = None;
            }

            // Application
            Action::Quit => {
                self.state.should_quit = true;
            }

            // Save
            Action::Save => {
                self.save_current_prompt()?;
            }

            // Rename prompt
            Action::RenamePrompt => {
                self.rename_current_prompt()?;
            }

            // Duplicate prompt
            Action::DuplicatePrompt => {
                self.duplicate_current_prompt()?;
            }

            // TODO: Implement these
            Action::OpenFolder
            | Action::MoveToFolder
            | Action::OpenTagSelector
            | Action::OpenSearch
            | Action::QuickOpen
            | Action::QuickInsertReference
            | Action::Undo
            | Action::Redo
            | Action::Export
            | Action::Confirm
            | Action::Cancel => {
                self.state.notify("Feature not yet implemented", NotificationLevel::Warning);
            }
        }

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

        // Add to list and select
        self.state.prompts.push(prompt);
        self.state.selected_index = self.state.prompts.len() - 1;

        // Enter insert mode
        self.state.mode = Mode::Insert;
        self.state.editor_focused = true;

        self.state.notify("Created new prompt", NotificationLevel::Success);

        Ok(())
    }

    /// Save the current prompt
    fn save_current_prompt(&mut self) -> Result<()> {
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

            // Get all existing names except current prompt
            let existing_names: Vec<&str> = self.state.prompts
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

            // Get all existing names for uniqueness check
            let existing_names: Vec<&str> = self.state.prompts
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

            // Add to list and select
            self.state.prompts.push(new_prompt.clone());
            self.state.selected_index = self.state.prompts.len() - 1;

            self.state.notify(format!("Duplicated as '{}'", new_prompt.name), NotificationLevel::Success);
        }

        Ok(())
    }

    /// Delete the current prompt
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

            // Move file back to prompts
            crate::fs::move_prompt(&name, &archive_dir()?, &prompts_dir()?)?;

            // Update index
            if let Some(entry) = self.index.entries.get_mut(&name) {
                entry.location = "prompts".to_string();
            }
            self.index.save(&index_path()?)?;

            // Remove from current list
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

        // Apply filter (reload prompts)
        // For now, just update notification
        if let Some(tag) = &self.state.tag_filter {
            self.state.notify(format!("Filtering by: {}", tag), NotificationLevel::Info);
        } else {
            self.state.notify("Showing all prompts", NotificationLevel::Info);
        }
    }

    /// Reload prompts from disk
    fn reload_prompts(&mut self) -> Result<()> {
        self.state.prompts = load_all_prompts(&prompts_dir()?)?;
        self.state.current_folder = None;
        
        if self.state.selected_index >= self.state.prompts.len() {
            self.state.selected_index = self.state.prompts.len().saturating_sub(1);
        }

        Ok(())
    }
}
