//! Keyboard event handling

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::models::{Action, AppState, EditorMode, Mode};

/// Handle a key event and return the corresponding action
pub fn handle_key_event(key: KeyEvent, state: &AppState) -> Action {
    // If a confirmation dialog is active, handle it first
    if state.confirm_dialog.is_some() {
        return handle_confirm_dialog(key);
    }

    // If rename popup is active, handle it
    if state.rename_popup.is_some() {
        return handle_rename_popup(key);
    }

    // If reference popup is active, handle it
    if state.reference_popup.is_some() {
        return handle_reference_popup(key);
    }

    // If tag selector is active, handle it
    if state.tag_selector.is_some() {
        return handle_tag_selector(key, state);
    }

    // If folder selector is active, handle it
    if state.folder_selector.is_some() {
        return handle_folder_selector(key, state);
    }

    // If help is open, handle help-specific keybindings
    if state.show_help {
        return handle_help_keys(key);
    }

    // Global keybindings (work in all modes except Insert for some keys)
    match key.code {
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            // In Insert mode, Ctrl+C is for copying, not quitting
            if state.mode != Mode::Insert {
                return Action::Quit;
            }
        }
        KeyCode::Char('?') => {
            return Action::OpenHelp;
        }
        _ => {}
    }

    // Mode-specific keybindings
    match state.mode {
        Mode::Normal => handle_normal_mode(key, state),
        Mode::Insert => handle_insert_mode(key, state),
        Mode::Archive => handle_archive_mode(key),
        Mode::Folder => handle_folder_mode(key, state),
        Mode::Preview => handle_preview_mode(key),
    }
}

/// Handle keys when help overlay is open
fn handle_help_keys(key: KeyEvent) -> Action {
    match key.code {
        KeyCode::Esc | KeyCode::Char('?') | KeyCode::Char('q') => Action::OpenHelp, // Toggle off
        KeyCode::Char('j') | KeyCode::Down => Action::HelpScrollDown,
        KeyCode::Char('k') | KeyCode::Up => Action::HelpScrollUp,
        _ => Action::None,
    }
}

/// Handle keys when a confirmation dialog is active
fn handle_confirm_dialog(key: KeyEvent) -> Action {
    match key.code {
        // Toggle between Yes/No
        KeyCode::Left | KeyCode::Right | KeyCode::Char('h') | KeyCode::Char('l') | KeyCode::Tab => {
            Action::ToggleConfirmSelection
        }
        // Confirm selection
        KeyCode::Enter => Action::Confirm,
        // Cancel (always cancels, regardless of selection)
        KeyCode::Esc | KeyCode::Char('n') | KeyCode::Char('N') => Action::Cancel,
        // Quick confirm with 'y'
        KeyCode::Char('y') | KeyCode::Char('Y') => Action::Confirm,
        _ => Action::None,
    }
}

/// Handle keys in Normal mode
fn handle_normal_mode(key: KeyEvent, _state: &AppState) -> Action {
    // Check for popups/overlays first
    // (would be handled here)

    match key.code {
        // Navigation
        KeyCode::Char('j') | KeyCode::Down => Action::MoveDown,
        KeyCode::Char('k') | KeyCode::Up => Action::MoveUp,
        KeyCode::Char('g') => Action::GoToFirst,
        KeyCode::Char('G') => Action::GoToLast,

        // Mode switching
        KeyCode::Enter | KeyCode::Char('i') => Action::EnterInsertMode,
        KeyCode::Char('p') if key.modifiers.contains(KeyModifiers::CONTROL) => Action::QuickOpen,
        KeyCode::Char('p') => Action::TogglePreview,
        KeyCode::Char('A') => Action::OpenArchive,
        KeyCode::Char('O') => Action::OpenFolder,

        // Prompt management
        KeyCode::Char('n') => Action::NewPrompt,
        KeyCode::Char('r') => Action::OpenRenamePopup,
        KeyCode::Char('d') if key.modifiers.contains(KeyModifiers::CONTROL) => Action::DuplicatePrompt,
        KeyCode::Char('d') => Action::DeletePrompt,
        KeyCode::Char('a') => Action::ArchivePrompt,

        // Clipboard
        KeyCode::Char('y') => Action::CopyRendered,

        // Tags
        KeyCode::Char('t') => Action::OpenTagSelector,
        KeyCode::Char('[') => Action::PreviousTagFilter,
        KeyCode::Char(']') => Action::NextTagFilter,

        // Search
        KeyCode::Char('/') => Action::OpenSearch,

        // Other
        KeyCode::Tab => Action::ToggleFocus,
        KeyCode::Char('!') => Action::ToggleSafeMode,
        KeyCode::Char('e') => Action::Export,
        KeyCode::Char('M') => Action::MoveToFolder,
        KeyCode::Char('q') => Action::Quit,
        
        // Column resize
        KeyCode::Char('l') if key.modifiers.contains(KeyModifiers::CONTROL) => Action::IncreaseLeftColumnWidth,
        KeyCode::Char('h') if key.modifiers.contains(KeyModifiers::CONTROL) => Action::DecreaseLeftColumnWidth,

        _ => Action::None,
    }
}

/// Handle keys in Insert mode (dispatch to appropriate vim sub-mode handler)
fn handle_insert_mode(key: KeyEvent, state: &AppState) -> Action {
    match state.editor_mode {
        EditorMode::VimNormal => handle_vim_normal_mode(key),
        EditorMode::VimInsert => handle_vim_insert_mode(key),
        EditorMode::VimVisual | EditorMode::VimVisualLine => handle_vim_visual_mode(key, state),
        EditorMode::VimOperatorPending(_) => handle_vim_operator_pending_mode(key, state),
    }
}

/// Handle keys in Vim Normal mode (within the editor)
fn handle_vim_normal_mode(key: KeyEvent) -> Action {
    match key.code {
        // Exit editor entirely
        KeyCode::Esc => Action::ExitMode,
        // Quit application from editor normal mode
        KeyCode::Char('q') => Action::Quit,
        
        // Save
        KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => Action::Save,
        
        // Enter Vim Insert mode
        KeyCode::Char('i') => Action::VimEnterInsert,
        KeyCode::Char('I') => Action::VimEnterInsertStart,
        KeyCode::Char('a') => Action::VimEnterInsert, // Will move cursor right first in app.rs
        KeyCode::Char('A') => Action::VimEnterInsertEnd,
        KeyCode::Char('o') => Action::VimOpenBelow,
        KeyCode::Char('O') => Action::VimOpenAbove,
        
        // Visual modes
        KeyCode::Char('v') => Action::VimEnterVisual,
        KeyCode::Char('V') => Action::VimEnterVisualLine,
        
        // Hybrid: Shift+Arrow for selection (check BEFORE plain arrows)
        KeyCode::Left if key.modifiers.contains(KeyModifiers::SHIFT) => Action::ExtendSelection,
        KeyCode::Right if key.modifiers.contains(KeyModifiers::SHIFT) => Action::ExtendSelection,
        KeyCode::Up if key.modifiers.contains(KeyModifiers::SHIFT) => Action::ExtendSelection,
        KeyCode::Down if key.modifiers.contains(KeyModifiers::SHIFT) => Action::ExtendSelection,
        
        // Movement (plain arrows)
        KeyCode::Char('h') | KeyCode::Left => Action::VimLeft,
        KeyCode::Char('j') | KeyCode::Down => Action::VimDown,
        KeyCode::Char('k') | KeyCode::Up => Action::VimUp,
        KeyCode::Char('l') | KeyCode::Right => Action::VimRight,
        KeyCode::Char('0') | KeyCode::Home => Action::VimLineStart,
        KeyCode::Char('^') => Action::VimFirstNonBlank,
        KeyCode::Char('$') | KeyCode::End => Action::VimLineEnd,
        KeyCode::Char('w') => Action::VimWordForward,
        KeyCode::Char('b') => Action::VimWordBackward,
        KeyCode::Char('e') => Action::VimWordEnd,
        KeyCode::Char('g') => Action::VimGoToTop,
        KeyCode::Char('G') => Action::VimGoToBottom,
        // Paragraph movements
        KeyCode::Char('{') => Action::VimParagraphBackward,
        KeyCode::Char('}') => Action::VimParagraphForward,
        
        // Editing - start operator-pending mode for d, c, y
        KeyCode::Char('x') | KeyCode::Delete => Action::VimDeleteChar,
        KeyCode::Char('D') => Action::VimDeleteToEnd,
        KeyCode::Char('C') => Action::VimChangeToEnd,
        KeyCode::Char('d') => Action::VimStartDelete,  // Enter operator-pending mode
        KeyCode::Char('c') => Action::VimStartChange,  // Enter operator-pending mode
        
        // Yank (vim style) - enter operator-pending mode
        KeyCode::Char('y') => Action::VimStartYank,
        KeyCode::Char('p') => Action::VimPut,
        KeyCode::Char('P') => Action::VimPutBefore,
        
        // Undo/Redo
        KeyCode::Char('u') => Action::Undo,
        
        // Open reference popup (both r and Ctrl+r open reference popup)
        KeyCode::Char('r') if key.modifiers.contains(KeyModifiers::CONTROL) => Action::OpenReferencePopup,
        KeyCode::Char('r') => Action::OpenReferencePopup,
        
        // Help
        KeyCode::Char('?') => Action::OpenHelp,
        
        _ => Action::None,
    }
}

/// Handle keys in Vim Insert mode (actual text editing)
fn handle_vim_insert_mode(key: KeyEvent) -> Action {
    match key.code {
        // Exit to Vim Normal mode
        KeyCode::Esc => Action::VimExitToNormal,
        
        // Save (also exits to normal)
        KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => Action::Save,
        
        // Standard editor shortcuts that work in insert mode
        KeyCode::Char('z') if key.modifiers.contains(KeyModifiers::CONTROL) => Action::Undo,
        KeyCode::Char('y') if key.modifiers.contains(KeyModifiers::CONTROL) => Action::Redo,
        KeyCode::Char('a') if key.modifiers.contains(KeyModifiers::CONTROL) => Action::SelectAll,
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => Action::CopySelection,
        KeyCode::Char('v') if key.modifiers.contains(KeyModifiers::CONTROL) => Action::Paste,
        KeyCode::Char('l') if key.modifiers.contains(KeyModifiers::CONTROL) => Action::QuickInsertReference,
        KeyCode::Char('r') if key.modifiers.contains(KeyModifiers::CONTROL) => Action::OpenReferencePopup,
        
        // Hybrid: Shift+Arrow for selection while in insert mode
        KeyCode::Left if key.modifiers.contains(KeyModifiers::SHIFT) => Action::ExtendSelection,
        KeyCode::Right if key.modifiers.contains(KeyModifiers::SHIFT) => Action::ExtendSelection,
        KeyCode::Up if key.modifiers.contains(KeyModifiers::SHIFT) => Action::ExtendSelection,
        KeyCode::Down if key.modifiers.contains(KeyModifiers::SHIFT) => Action::ExtendSelection,
        
        // Help
        KeyCode::Char('?') if key.modifiers.contains(KeyModifiers::CONTROL) => Action::OpenHelp,
        
        // All other keys handled by tui-textarea
        _ => Action::None,
    }
}

/// Handle keys in Vim Visual modes
fn handle_vim_visual_mode(key: KeyEvent, state: &AppState) -> Action {
    match key.code {
        // Exit visual mode
        KeyCode::Esc => Action::VimExitToNormal,
        
        // Ctrl+A to select all
        KeyCode::Char('a') if key.modifiers.contains(KeyModifiers::CONTROL) => Action::SelectAll,
        
        // Hybrid: Shift+Arrow continues selection (check BEFORE plain arrows)
        KeyCode::Left if key.modifiers.contains(KeyModifiers::SHIFT) => Action::VimLeft,
        KeyCode::Right if key.modifiers.contains(KeyModifiers::SHIFT) => Action::VimRight,
        KeyCode::Up if key.modifiers.contains(KeyModifiers::SHIFT) => Action::VimUp,
        KeyCode::Down if key.modifiers.contains(KeyModifiers::SHIFT) => Action::VimDown,
        
        // Movement (extends selection)
        KeyCode::Char('h') | KeyCode::Left => Action::VimLeft,
        KeyCode::Char('j') | KeyCode::Down => Action::VimDown,
        KeyCode::Char('k') | KeyCode::Up => Action::VimUp,
        KeyCode::Char('l') | KeyCode::Right => Action::VimRight,
        KeyCode::Char('0') | KeyCode::Home => Action::VimLineStart,
        KeyCode::Char('^') => Action::VimFirstNonBlank,
        KeyCode::Char('$') | KeyCode::End => Action::VimLineEnd,
        KeyCode::Char('w') => Action::VimWordForward,
        KeyCode::Char('b') => Action::VimWordBackward,
        KeyCode::Char('e') => Action::VimWordEnd,
        KeyCode::Char('g') => Action::VimGoToTop,
        KeyCode::Char('G') => Action::VimGoToBottom,
        // Paragraph movements
        KeyCode::Char('{') => Action::VimParagraphBackward,
        KeyCode::Char('}') => Action::VimParagraphForward,
        
        // Actions on selection
        KeyCode::Char('d') | KeyCode::Char('x') => Action::VimDeleteChar, // Delete selection
        KeyCode::Char('c') => Action::VimChangeLine, // Change selection
        KeyCode::Char('y') => Action::VimYank, // Yank selection
        
        // Switch visual modes
        KeyCode::Char('v') => {
            if state.editor_mode == EditorMode::VimVisual {
                Action::VimExitToNormal // Toggle off
            } else {
                Action::VimEnterVisual // Switch to char-wise
            }
        }
        KeyCode::Char('V') => {
            if state.editor_mode == EditorMode::VimVisualLine {
                Action::VimExitToNormal // Toggle off
            } else {
                Action::VimEnterVisualLine // Switch to line-wise
            }
        }
        
        _ => Action::None,
    }
}

/// Handle keys in Vim Operator-pending mode (after d, c, y)
fn handle_vim_operator_pending_mode(key: KeyEvent, state: &AppState) -> Action {
    use crate::models::VimOperator;
    
    let operator = match state.editor_mode.pending_operator() {
        Some(op) => op,
        None => return Action::VimExitToNormal,
    };
    
    match key.code {
        // Cancel operator
        KeyCode::Esc => Action::VimExitToNormal,
        
        // Double key for line operation (dd, cc, yy)
        KeyCode::Char('d') if operator == VimOperator::Delete => Action::VimDeleteLine,
        KeyCode::Char('c') if operator == VimOperator::Change => Action::VimChangeLine,
        KeyCode::Char('y') if operator == VimOperator::Yank => Action::VimYank,
        
        // Motion keys - these will be combined with the operator in app.rs
        KeyCode::Char('w') => Action::VimWordForward,
        KeyCode::Char('b') => Action::VimWordBackward,
        KeyCode::Char('e') => Action::VimWordEnd,
        KeyCode::Char('0') | KeyCode::Home => Action::VimLineStart,
        KeyCode::Char('^') => Action::VimFirstNonBlank,
        KeyCode::Char('$') | KeyCode::End => Action::VimLineEnd,
        KeyCode::Char('h') | KeyCode::Left => Action::VimLeft,
        KeyCode::Char('j') | KeyCode::Down => Action::VimDown,
        KeyCode::Char('k') | KeyCode::Up => Action::VimUp,
        KeyCode::Char('l') | KeyCode::Right => Action::VimRight,
        KeyCode::Char('g') => Action::VimGoToTop,
        KeyCode::Char('G') => Action::VimGoToBottom,
        KeyCode::Char('{') => Action::VimParagraphBackward,
        KeyCode::Char('}') => Action::VimParagraphForward,
        
        // Invalid key - cancel operator
        _ => Action::VimExitToNormal,
    }
}

/// Handle keys in Archive mode
fn handle_archive_mode(key: KeyEvent) -> Action {
    match key.code {
        KeyCode::Char('j') | KeyCode::Down => Action::MoveDown,
        KeyCode::Char('k') | KeyCode::Up => Action::MoveUp,
        KeyCode::Char('u') => Action::UnarchivePrompt,
        KeyCode::Delete => Action::DeletePrompt,
        KeyCode::Esc => Action::ExitMode,
        _ => Action::None,
    }
}

/// Handle keys in Folder mode
fn handle_folder_mode(key: KeyEvent, state: &AppState) -> Action {
    match key.code {
        KeyCode::Esc => Action::ExitMode,
        // All other keys work like normal mode
        _ => handle_normal_mode(key, state),
    }
}

/// Handle keys in Preview mode
fn handle_preview_mode(key: KeyEvent) -> Action {
    match key.code {
        KeyCode::Esc | KeyCode::Char('p') => Action::ExitMode,
        KeyCode::Char('j') | KeyCode::Down => Action::MoveDown,
        KeyCode::Char('k') | KeyCode::Up => Action::MoveUp,
        _ => Action::None,
    }
}

/// Handle keys when rename popup is active
fn handle_rename_popup(key: KeyEvent) -> Action {
    match key.code {
        KeyCode::Enter => Action::ConfirmRename,
        KeyCode::Esc => Action::CancelRename,
        // Other keys are handled directly by the popup input handling in app.rs
        _ => Action::None,
    }
}

/// Handle keys when reference popup is active
fn handle_reference_popup(key: KeyEvent) -> Action {
    match key.code {
        KeyCode::Enter => Action::ConfirmReference,
        KeyCode::Esc => Action::CancelReference,
        KeyCode::Up | KeyCode::Char('k') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            Action::ReferencePopupUp
        }
        KeyCode::Down | KeyCode::Char('j') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            Action::ReferencePopupDown
        }
        KeyCode::Up => Action::ReferencePopupUp,
        KeyCode::Down => Action::ReferencePopupDown,
        // Other keys are handled directly by the popup input handling in app.rs
        _ => Action::None,
    }
}

/// Handle keys when tag selector is active
fn handle_tag_selector(key: KeyEvent, state: &AppState) -> Action {
    // Check if we're in "new tag" creation mode
    if let Some(ref selector) = state.tag_selector {
        if selector.creating_new {
            return match key.code {
                KeyCode::Enter => Action::ConfirmNewTag,
                KeyCode::Esc => Action::CancelTagSelector,
                // Other keys handled in app.rs for text input
                _ => Action::None,
            };
        }
    }

    match key.code {
        KeyCode::Enter | KeyCode::Char(' ') => Action::ConfirmTagToggle,
        KeyCode::Esc => Action::CancelTagSelector,
        KeyCode::Up | KeyCode::Char('k') => Action::TagSelectorUp,
        KeyCode::Down | KeyCode::Char('j') => Action::TagSelectorDown,
        KeyCode::Char('n') if key.modifiers.contains(KeyModifiers::CONTROL) => Action::CreateNewTag,
        // Other keys are handled directly by the popup input handling in app.rs
        _ => Action::None,
    }
}

/// Handle keys when folder selector is active
fn handle_folder_selector(key: KeyEvent, state: &AppState) -> Action {
    // Check if we're in "new folder" creation mode
    if let Some(ref selector) = state.folder_selector {
        if selector.creating_new {
            return match key.code {
                KeyCode::Enter => Action::ConfirmNewFolder,
                KeyCode::Esc => Action::CancelFolderSelector,
                // Other keys handled in app.rs for text input
                _ => Action::None,
            };
        }
    }

    match key.code {
        KeyCode::Enter => Action::ConfirmFolderSelection,
        KeyCode::Esc => Action::CancelFolderSelector,
        KeyCode::Up | KeyCode::Char('k') => Action::FolderSelectorUp,
        KeyCode::Down | KeyCode::Char('j') => Action::FolderSelectorDown,
        KeyCode::Char('n') if key.modifiers.contains(KeyModifiers::CONTROL) => Action::CreateNewFolder,
        // Other keys are handled directly by the popup input handling in app.rs
        _ => Action::None,
    }
}
