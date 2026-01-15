//! Application state management

use super::{Action, Mode, Prompt};

/// The complete application state
#[derive(Debug)]
pub struct AppState {
    /// Current application mode
    pub mode: Mode,
    /// List of all prompts in the current view
    pub prompts: Vec<Prompt>,
    /// Currently selected prompt index
    pub selected_index: usize,
    /// Current folder path (None = root)
    pub current_folder: Option<String>,
    /// Currently active tag filter (None = show all)
    pub tag_filter: Option<String>,
    /// All available tags
    pub all_tags: Vec<String>,
    /// Whether safe mode is enabled
    pub safe_mode: bool,
    /// Whether the application should quit
    pub should_quit: bool,
    /// Whether there are unsaved changes
    pub has_unsaved_changes: bool,
    /// Current notification message
    pub notification: Option<Notification>,
    /// Whether help overlay is visible
    pub show_help: bool,
    /// Active popup/overlay
    pub active_popup: Option<PopupType>,
    /// Focus state (true = editor, false = list)
    pub editor_focused: bool,
    /// Text input buffer for rename operations
    pub input_buffer: String,
    /// Scroll offset for the prompt list
    pub list_scroll_offset: usize,
    /// Confirmation dialog state
    pub confirm_dialog: Option<ConfirmDialog>,
    /// Editor content (when in insert mode)
    pub editor_content: Option<String>,
    /// Editor scroll offset (for long content)
    pub editor_scroll_offset: usize,
}

impl AppState {
    /// Create a new application state with defaults
    pub fn new() -> Self {
        Self {
            mode: Mode::Normal,
            prompts: Vec::new(),
            selected_index: 0,
            current_folder: None,
            tag_filter: None,
            all_tags: Vec::new(),
            safe_mode: true,
            should_quit: false,
            has_unsaved_changes: false,
            notification: None,
            show_help: false,
            active_popup: None,
            editor_focused: false,
            input_buffer: String::new(),
            list_scroll_offset: 0,
            confirm_dialog: None,
            editor_content: None,
            editor_scroll_offset: 0,
        }
    }

    /// Get the currently selected prompt, if any
    pub fn selected_prompt(&self) -> Option<&Prompt> {
        self.prompts.get(self.selected_index)
    }

    /// Get a mutable reference to the currently selected prompt
    pub fn selected_prompt_mut(&mut self) -> Option<&mut Prompt> {
        self.prompts.get_mut(self.selected_index)
    }

    /// Move selection down
    pub fn select_next(&mut self) {
        if !self.prompts.is_empty() {
            self.selected_index = (self.selected_index + 1).min(self.prompts.len() - 1);
        }
    }

    /// Move selection up
    pub fn select_previous(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    /// Go to first prompt
    pub fn select_first(&mut self) {
        self.selected_index = 0;
        self.list_scroll_offset = 0;
    }

    /// Go to last prompt
    pub fn select_last(&mut self) {
        if !self.prompts.is_empty() {
            self.selected_index = self.prompts.len() - 1;
        }
    }

    /// Update scroll offset to keep selection visible
    /// `visible_height` is the number of items that can be displayed in the list
    pub fn ensure_visible(&mut self, visible_height: usize) {
        if visible_height == 0 {
            return;
        }

        // If selection is above the visible area, scroll up
        if self.selected_index < self.list_scroll_offset {
            self.list_scroll_offset = self.selected_index;
        }
        
        // If selection is below the visible area, scroll down
        if self.selected_index >= self.list_scroll_offset + visible_height {
            self.list_scroll_offset = self.selected_index - visible_height + 1;
        }
    }

    /// Set a notification message
    pub fn notify(&mut self, message: impl Into<String>, level: NotificationLevel) {
        self.notification = Some(Notification {
            message: message.into(),
            level,
        });
    }

    /// Clear the current notification
    pub fn clear_notification(&mut self) {
        self.notification = None;
    }

    /// Get prompt count for display
    pub fn prompt_count(&self) -> usize {
        self.prompts.len()
    }

    /// Check if there are any prompts
    pub fn has_prompts(&self) -> bool {
        !self.prompts.is_empty()
    }

    /// Start editing the current prompt
    pub fn start_editing(&mut self) {
        if let Some(prompt) = self.selected_prompt() {
            self.editor_content = Some(prompt.content.clone());
            self.editor_scroll_offset = 0;
        }
    }

    /// Stop editing and return the edited content if any
    pub fn stop_editing(&mut self) -> Option<String> {
        self.editor_content.take()
    }

    /// Get the current editor content for display
    pub fn get_editor_content(&self) -> Option<&str> {
        self.editor_content.as_deref()
    }

    /// Scroll editor up by n lines
    pub fn scroll_editor_up(&mut self, n: usize) {
        self.editor_scroll_offset = self.editor_scroll_offset.saturating_sub(n);
    }

    /// Scroll editor down by n lines
    pub fn scroll_editor_down(&mut self, n: usize, max_lines: usize, visible_height: usize) {
        let max_scroll = max_lines.saturating_sub(visible_height);
        self.editor_scroll_offset = (self.editor_scroll_offset + n).min(max_scroll);
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

/// A notification message to display to the user
#[derive(Debug, Clone)]
pub struct Notification {
    pub message: String,
    pub level: NotificationLevel,
}

/// Notification severity level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotificationLevel {
    Info,
    Success,
    Warning,
    Error,
}

/// Types of popups/overlays
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PopupType {
    /// Confirmation dialog
    Confirm { message: String },
    /// Tag selector
    TagSelector,
    /// Folder selector
    FolderSelector,
    /// Rename input
    RenameInput,
    /// Search overlay
    Search,
    /// Export options
    Export,
    /// Command confirmation (safe mode)
    CommandConfirm { commands: Vec<String> },
}

/// State for confirmation dialogs
#[derive(Debug, Clone)]
pub struct ConfirmDialog {
    /// Title of the dialog
    pub title: String,
    /// Message to display
    pub message: String,
    /// Whether "Yes" is selected (true) or "No" (false)
    pub yes_selected: bool,
    /// Action to execute if confirmed
    pub pending_action: PendingAction,
}

impl ConfirmDialog {
    /// Create a new confirmation dialog
    pub fn new(title: impl Into<String>, message: impl Into<String>, action: PendingAction) -> Self {
        Self {
            title: title.into(),
            message: message.into(),
            yes_selected: false, // Default to "No" for safety
            pending_action: action,
        }
    }

    /// Toggle selection between Yes and No
    pub fn toggle_selection(&mut self) {
        self.yes_selected = !self.yes_selected;
    }
}

/// Actions that require confirmation before execution
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PendingAction {
    /// Delete the prompt with the given name
    DeletePrompt { name: String },
    /// Permanently delete from archive
    PermanentDelete { name: String },
    /// Execute commands (safe mode confirmation)
    ExecuteCommands { commands: Vec<String> },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_navigation() {
        let mut state = AppState::new();
        state.prompts = vec![
            Prompt::with_content("First"),
            Prompt::with_content("Second"),
            Prompt::with_content("Third"),
        ];

        assert_eq!(state.selected_index, 0);

        state.select_next();
        assert_eq!(state.selected_index, 1);

        state.select_next();
        assert_eq!(state.selected_index, 2);

        state.select_next(); // Should stay at 2
        assert_eq!(state.selected_index, 2);

        state.select_previous();
        assert_eq!(state.selected_index, 1);

        state.select_first();
        assert_eq!(state.selected_index, 0);

        state.select_last();
        assert_eq!(state.selected_index, 2);
    }

    #[test]
    fn test_empty_navigation() {
        let mut state = AppState::new();
        
        state.select_next();
        assert_eq!(state.selected_index, 0);
        
        state.select_previous();
        assert_eq!(state.selected_index, 0);
        
        state.select_last();
        assert_eq!(state.selected_index, 0);
    }
}
