//! Application state management

use super::{Mode, Prompt};

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
    }

    /// Go to last prompt
    pub fn select_last(&mut self) {
        if !self.prompts.is_empty() {
            self.selected_index = self.prompts.len() - 1;
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
