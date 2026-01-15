//! Status bar component

use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::models::{AppState, EditorMode, NotificationLevel};

/// Render the status bar
pub fn render_status_bar(frame: &mut Frame, area: Rect, state: &AppState, archived_count: usize) {
    let mut spans = Vec::new();

    // Mode indicator with vim editor sub-mode
    if state.mode == crate::models::Mode::Insert {
        // Show vim-style mode indicator when in editor
        let (mode_text, mode_color) = match state.editor_mode {
            EditorMode::VimNormal => ("NORMAL", Color::Blue),
            EditorMode::VimInsert => ("INSERT", Color::Green),
            EditorMode::VimVisual => ("VISUAL", Color::Magenta),
            EditorMode::VimVisualLine => ("V-LINE", Color::Magenta),
        };
        
        spans.push(Span::styled(
            format!(" [{}] ", mode_text),
            Style::default()
                .fg(Color::Black)
                .bg(mode_color)
                .add_modifier(Modifier::BOLD),
        ));
        
        // Add "EDITING" indicator
        spans.push(Span::styled(
            " EDITING ",
            Style::default()
                .fg(Color::Black)
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        ));
    } else {
        let mode_color = match state.mode {
            crate::models::Mode::Normal => Color::Blue,
            crate::models::Mode::Insert => Color::Green, // Won't reach here
            crate::models::Mode::Archive => Color::Yellow,
            crate::models::Mode::Folder => Color::Magenta,
            crate::models::Mode::Preview => Color::Cyan,
        };
        
        spans.push(Span::styled(
            format!(" [{}] ", state.mode.as_str()),
            Style::default()
                .fg(Color::Black)
                .bg(mode_color)
                .add_modifier(Modifier::BOLD),
        ));
    }
    spans.push(Span::raw(" "));

    // Current prompt tags
    if let Some(prompt) = state.selected_prompt() {
        if !prompt.tags.is_empty() {
            spans.push(Span::raw("Tags: "));
            let tags_str = prompt.tags.join(", ");
            spans.push(Span::styled(tags_str, Style::default().fg(Color::Yellow)));
            spans.push(Span::raw(" │ "));
        }
    }

    // Statistics
    spans.push(Span::styled(
        format!("{} prompts", state.prompts.len()),
        Style::default().fg(Color::Cyan),
    ));
    spans.push(Span::raw(" │ "));
    
    spans.push(Span::styled(
        format!("{} archived", archived_count),
        Style::default().fg(Color::DarkGray),
    ));
    spans.push(Span::raw(" │ "));
    
    spans.push(Span::styled(
        format!("{} tags", state.all_tags.len()),
        Style::default().fg(Color::Magenta),
    ));

    // Tag filter indicator
    if let Some(tag) = &state.tag_filter {
        spans.push(Span::raw(" │ "));
        spans.push(Span::styled(
            format!("Filter: {}", tag),
            Style::default().fg(Color::Yellow).add_modifier(Modifier::ITALIC),
        ));
    }

    // Notification (if any)
    if let Some(notification) = &state.notification {
        spans.push(Span::raw(" │ "));
        let notif_color = match notification.level {
            NotificationLevel::Info => Color::White,
            NotificationLevel::Success => Color::Green,
            NotificationLevel::Warning => Color::Yellow,
            NotificationLevel::Error => Color::Red,
        };
        spans.push(Span::styled(&notification.message, Style::default().fg(notif_color)));
    }

    let line = Line::from(spans);
    let paragraph = Paragraph::new(line)
        .block(Block::default().borders(Borders::TOP));

    frame.render_widget(paragraph, area);
}
