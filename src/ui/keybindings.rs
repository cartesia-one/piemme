//! Keyboard event handling

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::models::{Action, AppState, Mode};

/// Handle a key event and return the corresponding action
pub fn handle_key_event(key: KeyEvent, state: &AppState) -> Action {
    // If a confirmation dialog is active, handle it first
    if state.confirm_dialog.is_some() {
        return handle_confirm_dialog(key);
    }

    // Global keybindings (work in all modes)
    match key.code {
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            return Action::Quit;
        }
        KeyCode::Char('?') => {
            return Action::OpenHelp;
        }
        _ => {}
    }

    // Mode-specific keybindings
    match state.mode {
        Mode::Normal => handle_normal_mode(key, state),
        Mode::Insert => handle_insert_mode(key),
        Mode::Archive => handle_archive_mode(key),
        Mode::Folder => handle_folder_mode(key, state),
        Mode::Preview => handle_preview_mode(key),
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
        KeyCode::Char('r') => Action::RenamePrompt,
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

        _ => Action::None,
    }
}

/// Handle keys in Insert mode
fn handle_insert_mode(key: KeyEvent) -> Action {
    match key.code {
        KeyCode::Esc => Action::ExitMode,
        KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => Action::Save,
        KeyCode::Char('z') if key.modifiers.contains(KeyModifiers::CONTROL) => Action::Undo,
        KeyCode::Char('y') if key.modifiers.contains(KeyModifiers::CONTROL) => Action::Redo,
        KeyCode::Char('l') if key.modifiers.contains(KeyModifiers::CONTROL) => Action::QuickInsertReference,
        _ => Action::None, // Let tui-textarea handle other keys
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
