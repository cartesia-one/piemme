//! Application actions (commands that can be executed)

/// All possible user actions in the application
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    // Navigation
    /// Move selection down
    MoveDown,
    /// Move selection up
    MoveUp,
    /// Go to first item
    GoToFirst,
    /// Go to last item
    GoToLast,

    // Mode switching
    /// Enter insert mode
    EnterInsertMode,
    /// Exit current mode (return to normal)
    ExitMode,
    /// Toggle preview mode
    TogglePreview,
    /// Open archive view
    OpenArchive,
    /// Open folder selector
    OpenFolder,

    // Prompt management
    /// Create a new prompt
    NewPrompt,
    /// Rename the selected prompt
    RenamePrompt,
    /// Delete the selected prompt
    DeletePrompt,
    /// Duplicate the selected prompt
    DuplicatePrompt,
    /// Archive the selected prompt
    ArchivePrompt,
    /// Unarchive the selected prompt (in archive mode)
    UnarchivePrompt,
    /// Move prompt to a folder
    MoveToFolder,

    // Clipboard
    /// Copy rendered prompt to clipboard
    CopyRendered,
    /// Copy raw prompt to clipboard
    CopyRaw,

    // Tags
    /// Open tag selector for selected prompt
    OpenTagSelector,
    /// Filter by previous tag
    PreviousTagFilter,
    /// Filter by next tag
    NextTagFilter,

    // Search
    /// Open fuzzy search
    OpenSearch,
    /// Quick open (fuzzy find by name)
    QuickOpen,
    /// Quick insert reference (in insert mode)
    QuickInsertReference,

    // Editing
    /// Save current changes
    Save,
    /// Undo last edit
    Undo,
    /// Redo last undone edit
    Redo,

    // Export
    /// Open export dialog
    Export,

    // Settings
    /// Toggle safe mode
    ToggleSafeMode,

    // UI
    /// Toggle focus between list and editor
    ToggleFocus,
    /// Open help overlay
    OpenHelp,
    /// Scroll help up
    HelpScrollUp,
    /// Scroll help down
    HelpScrollDown,
    /// Close current overlay/popup
    CloseOverlay,
    /// Toggle selection in confirmation dialog (Yes/No)
    ToggleConfirmSelection,

    // Application
    /// Quit the application
    Quit,
    /// Confirm current dialog
    Confirm,
    /// Cancel current dialog
    Cancel,

    // Rename popup actions
    /// Open rename popup
    OpenRenamePopup,
    /// Confirm rename
    ConfirmRename,
    /// Cancel rename popup
    CancelRename,

    // Reference insertion popup actions (CTRL+i in insert mode)
    /// Open reference insertion popup
    OpenReferencePopup,
    /// Confirm reference selection
    ConfirmReference,
    /// Cancel reference popup
    CancelReference,
    /// Move up in reference popup
    ReferencePopupUp,
    /// Move down in reference popup
    ReferencePopupDown,

    // No action (used for unhandled keys)
    None,
}

impl Action {
    /// Check if this action requires confirmation
    pub fn requires_confirmation(&self) -> bool {
        matches!(
            self,
            Action::DeletePrompt | Action::Quit | Action::ArchivePrompt
        )
    }

    /// Check if this action modifies data
    pub fn is_destructive(&self) -> bool {
        matches!(
            self,
            Action::DeletePrompt | Action::ArchivePrompt | Action::UnarchivePrompt
        )
    }
}
