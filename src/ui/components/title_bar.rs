//! Title bar component

use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::models::AppState;

/// Render the title bar
pub fn render_title_bar(frame: &mut Frame, area: Rect, state: &AppState) {
    // Different title styling for archive mode
    let (title_text, title_style) = if state.mode == crate::models::Mode::Archive {
        (
            " Piemme - Archive ",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
    } else {
        (
            " Piemme ",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
    };

    let mut spans = vec![Span::styled(title_text, title_style)];

    // Archive mode indicator
    if state.mode == crate::models::Mode::Archive {
        spans.push(Span::styled(
            "📦 ",
            Style::default().fg(Color::Yellow),
        ));
    }

    // Current folder
    if let Some(folder) = &state.current_folder {
        spans.push(Span::raw(" 📁 /"));
        spans.push(Span::styled(
            folder,
            Style::default().fg(Color::Yellow),
        ));
    }

    // Spacer (we'll handle right-alignment differently)
    spans.push(Span::raw(" "));

    // Safe mode indicator
    let safe_mode_text = if state.safe_mode {
        "🔒 Safe Mode: ON"
    } else {
        "🔓 Safe Mode: OFF"
    };
    
    let safe_mode_style = if state.safe_mode {
        Style::default().fg(Color::Green)
    } else {
        Style::default().fg(Color::Red)
    };

    // For simplicity, just append (proper right-alignment would need width calculation)
    spans.push(Span::styled(safe_mode_text, safe_mode_style));

    let title_line = Line::from(spans);
    
    let paragraph = Paragraph::new(title_line)
        .block(Block::default().borders(Borders::BOTTOM));

    frame.render_widget(paragraph, area);
}
