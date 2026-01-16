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
    /// Close search popup
    CloseSearch,
    /// Move up in search results
    SearchUp,
    /// Move down in search results
    SearchDown,
    /// Confirm search selection (jump to prompt)
    ConfirmSearch,

    // Editing
    /// Save current changes
    Save,
    /// Undo last edit
    Undo,
    /// Redo last undone edit
    Redo,
    /// Select all text in editor
    SelectAll,
    /// Copy selected text to clipboard (without rendering)
    CopySelection,
    /// Paste from clipboard
    Paste,

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
    /// Increase left column width
    IncreaseLeftColumnWidth,
    /// Decrease left column width
    DecreaseLeftColumnWidth,
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

    // Tag selector actions
    /// Confirm tag toggle/selection
    ConfirmTagToggle,
    /// Cancel tag selector
    CancelTagSelector,
    /// Move up in tag selector
    TagSelectorUp,
    /// Move down in tag selector
    TagSelectorDown,
    /// Create new tag in selector
    CreateNewTag,
    /// Confirm new tag creation
    ConfirmNewTag,

    // Folder selector actions
    /// Confirm folder selection
    ConfirmFolderSelection,
    /// Cancel folder selector
    CancelFolderSelector,
    /// Move up in folder selector
    FolderSelectorUp,
    /// Move down in folder selector
    FolderSelectorDown,
    /// Create new folder in selector
    CreateNewFolder,
    /// Confirm new folder creation
    ConfirmNewFolder,

    // Vim-style editor actions
    /// Enter Vim Insert mode (from Vim Normal)
    VimEnterInsert,
    /// Enter Vim Insert mode at end of line (A)
    VimEnterInsertEnd,
    /// Enter Vim Insert mode at start of line (I)
    VimEnterInsertStart,
    /// Open new line below and enter insert (o)
    VimOpenBelow,
    /// Open new line above and enter insert (O)
    VimOpenAbove,
    /// Exit to Vim Normal mode (Esc in insert/visual)
    VimExitToNormal,
    /// Enter Visual mode (v)
    VimEnterVisual,
    /// Enter Visual Line mode (V)
    VimEnterVisualLine,
    /// Vim move cursor left (h)
    VimLeft,
    /// Vim move cursor down (j)
    VimDown,
    /// Vim move cursor up (k)
    VimUp,
    /// Vim move cursor right (l)
    VimRight,
    /// Vim move to start of line (0)
    VimLineStart,
    /// Vim move to first non-blank (^)
    VimFirstNonBlank,
    /// Vim move to end of line ($)
    VimLineEnd,
    /// Vim move word forward (w)
    VimWordForward,
    /// Vim move word backward (b)
    VimWordBackward,
    /// Vim move to end of word (e)
    VimWordEnd,
    /// Vim move to start of file (gg)
    VimGoToTop,
    /// Vim move to end of file (G)
    VimGoToBottom,
    /// Vim move to previous paragraph ({)
    VimParagraphBackward,
    /// Vim move to next paragraph (})
    VimParagraphForward,
    /// Vim delete character under cursor (x)
    VimDeleteChar,
    /// Vim delete to end of line (D)
    VimDeleteToEnd,
    /// Vim delete entire line (dd)
    VimDeleteLine,
    /// Vim change to end of line (C)
    VimChangeToEnd,
    /// Vim change entire line (cc)
    VimChangeLine,
    /// Vim yank (copy) selection/line (y/yy)
    VimYank,
    /// Vim put (paste) after cursor (p)
    VimPut,
    /// Vim put (paste) before cursor (P)
    VimPutBefore,
    /// Enter operator-pending mode for delete (d)
    VimStartDelete,
    /// Enter operator-pending mode for change (c)
    VimStartChange,
    /// Enter operator-pending mode for yank (y)
    VimStartYank,
    /// Extend selection with Shift+Arrow (hybrid mode)
    ExtendSelection,

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
