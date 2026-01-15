//! Help overlay component

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};

use crate::models::Mode;

/// Render the help overlay
pub fn render_help_overlay(frame: &mut Frame, area: Rect, current_mode: Mode) {
    // Create a centered popup area
    let popup_area = centered_rect(70, 80, area);
    
    // Clear the background
    frame.render_widget(Clear, popup_area);
    
    let help_content = get_help_content(current_mode);
    
    let paragraph = Paragraph::new(help_content)
        .block(
            Block::default()
                .title(" Help (press ? or Esc to close) ")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .wrap(Wrap { trim: false })
        .style(Style::default().fg(Color::White));
    
    frame.render_widget(paragraph, popup_area);
}

/// Create a centered rectangle
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

/// Get help content based on current mode
fn get_help_content(mode: Mode) -> Vec<Line<'static>> {
    let mut lines = vec![
        Line::from(vec![
            Span::styled("Global Keybindings", Style::default().add_modifier(Modifier::BOLD | Modifier::UNDERLINED)),
        ]),
        Line::from(""),
        key_binding("?", "Open/close this help"),
        key_binding("Ctrl+c / q", "Quit application"),
        Line::from(""),
    ];

    match mode {
        Mode::Normal | Mode::Folder => {
            lines.extend(vec![
                Line::from(vec![
                    Span::styled("Normal Mode", Style::default().add_modifier(Modifier::BOLD | Modifier::UNDERLINED)),
                ]),
                Line::from(""),
                key_binding("j / ↓", "Move down"),
                key_binding("k / ↑", "Move up"),
                key_binding("g", "Go to first prompt"),
                key_binding("G", "Go to last prompt"),
                key_binding("Enter / i", "Enter insert mode"),
                key_binding("n", "Create new prompt"),
                key_binding("r", "Rename prompt"),
                key_binding("d", "Delete prompt"),
                key_binding("Ctrl+d", "Duplicate prompt"),
                key_binding("y", "Copy rendered to clipboard"),
                key_binding("p", "Toggle preview mode"),
                key_binding("a", "Archive prompt"),
                key_binding("A", "Open archive view"),
                key_binding("t", "Open tag selector"),
                key_binding("M", "Move to folder"),
                key_binding("O", "Open folder"),
                key_binding("/", "Open search"),
                key_binding("Ctrl+p", "Quick open"),
                key_binding("[ / ]", "Cycle tag filter"),
                key_binding("Tab", "Toggle list/editor focus"),
                key_binding("!", "Toggle safe mode"),
                key_binding("e", "Export prompt"),
            ]);
        }
        Mode::Insert => {
            lines.extend(vec![
                Line::from(vec![
                    Span::styled("Insert Mode", Style::default().add_modifier(Modifier::BOLD | Modifier::UNDERLINED)),
                ]),
                Line::from(""),
                key_binding("Esc", "Exit insert mode (save)"),
                key_binding("Ctrl+s", "Save explicitly"),
                key_binding("Ctrl+z", "Undo"),
                key_binding("Ctrl+y", "Redo"),
                key_binding("Ctrl+l", "Quick insert reference"),
                key_binding("Ctrl+← / →", "Move word left/right"),
                key_binding("Home / End", "Line start/end"),
                key_binding("Ctrl+Home / End", "Document start/end"),
            ]);
        }
        Mode::Archive => {
            lines.extend(vec![
                Line::from(vec![
                    Span::styled("Archive Mode", Style::default().add_modifier(Modifier::BOLD | Modifier::UNDERLINED)),
                ]),
                Line::from(""),
                key_binding("j / ↓", "Move down"),
                key_binding("k / ↑", "Move up"),
                key_binding("u", "Unarchive prompt"),
                key_binding("Delete", "Permanently delete"),
                key_binding("Esc", "Exit archive mode"),
            ]);
        }
        Mode::Preview => {
            lines.extend(vec![
                Line::from(vec![
                    Span::styled("Preview Mode", Style::default().add_modifier(Modifier::BOLD | Modifier::UNDERLINED)),
                ]),
                Line::from(""),
                key_binding("Esc / p", "Exit preview mode"),
                key_binding("j / ↓", "Scroll down"),
                key_binding("k / ↑", "Scroll up"),
            ]);
        }
    }

    lines
}

/// Create a key binding line
fn key_binding(key: &'static str, description: &'static str) -> Line<'static> {
    Line::from(vec![
        Span::styled(
            format!("  {:15}", key),
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        ),
        Span::raw(description),
    ])
}
