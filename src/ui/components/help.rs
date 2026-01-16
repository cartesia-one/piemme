//! Help overlay component

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};

use crate::models::Mode;

/// Render the help overlay with scroll support
pub fn render_help_overlay(frame: &mut Frame, area: Rect, current_mode: Mode, scroll_offset: usize) {
    // Create a centered popup area
    let popup_area = centered_rect(70, 80, area);
    
    // Clear the background
    frame.render_widget(Clear, popup_area);
    
    let help_content = get_help_content(current_mode);
    let total_lines = help_content.len();
    
    // Calculate visible height (account for borders and title)
    let visible_height = popup_area.height.saturating_sub(2) as usize;
    
    // Add scroll indicator to title
    let scroll_indicator = if total_lines > visible_height {
        let current_pos = scroll_offset + 1;
        let max_pos = total_lines.saturating_sub(visible_height) + 1;
        format!(" [{}/{}]", current_pos.min(max_pos), max_pos)
    } else {
        String::new()
    };
    
    let title = format!(" Help (j/k to scroll, ? or Esc to close){} ", scroll_indicator);
    
    let paragraph = Paragraph::new(help_content)
        .block(
            Block::default()
                .title(title)
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .wrap(Wrap { trim: false })
        .style(Style::default().fg(Color::White))
        .scroll((scroll_offset as u16, 0));
    
    frame.render_widget(paragraph, popup_area);
}

/// Get the maximum scroll offset for help content
pub fn get_help_max_scroll(mode: Mode, visible_height: usize) -> usize {
    let content = get_help_content(mode);
    content.len().saturating_sub(visible_height)
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
        key_binding("q", "Quit application"),
        key_binding("Ctrl+y", "Copy rendered to clipboard"),
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
                key_binding("Enter / i", "Enter editor (Vim Normal)"),
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
                    Span::styled("Editor - Vim Normal Mode", Style::default().add_modifier(Modifier::BOLD | Modifier::UNDERLINED)),
                ]),
                Line::from(""),
                key_binding("Esc", "Exit editor (save & return)"),
                key_binding("i", "Enter Insert mode"),
                key_binding("I", "Insert at line start"),
                key_binding("a", "Append after cursor"),
                key_binding("A", "Append at line end"),
                key_binding("o", "Open line below"),
                key_binding("O", "Open line above"),
                Line::from(""),
                Line::from(vec![
                    Span::styled("Navigation", Style::default().add_modifier(Modifier::BOLD)),
                ]),
                key_binding("h/j/k/l", "Left/Down/Up/Right"),
                key_binding("w / b", "Word forward/backward"),
                key_binding("e", "End of word"),
                key_binding("0 / ^", "Line start / first char"),
                key_binding("$", "Line end"),
                key_binding("gg / G", "File start / end"),
                Line::from(""),
                Line::from(vec![
                    Span::styled("Editing", Style::default().add_modifier(Modifier::BOLD)),
                ]),
                key_binding("x", "Delete char"),
                key_binding("d", "Delete line"),
                key_binding("D", "Delete to end of line"),
                key_binding("c", "Change line"),
                key_binding("C", "Change to end of line"),
                key_binding("u", "Undo"),
                Line::from(""),
                Line::from(vec![
                    Span::styled("Clipboard", Style::default().add_modifier(Modifier::BOLD)),
                ]),
                key_binding("y", "Yank (copy) line"),
                key_binding("p / P", "Put after/before"),
                Line::from(""),
                Line::from(vec![
                    Span::styled("References", Style::default().add_modifier(Modifier::BOLD)),
                ]),
                key_binding("r / Ctrl+r", "Insert reference"),
                key_binding("Ctrl+f", "Insert file reference"),
                Line::from(""),
                Line::from(vec![
                    Span::styled("Visual Selection", Style::default().add_modifier(Modifier::BOLD)),
                ]),
                key_binding("v", "Visual mode (char)"),
                key_binding("V", "Visual line mode"),
                key_binding("Shift+Arrows", "Extend selection (hybrid)"),
                key_binding("Ctrl+a", "Select all"),
                Line::from(""),
                Line::from(vec![
                    Span::styled("Editor - Insert Mode", Style::default().add_modifier(Modifier::BOLD | Modifier::UNDERLINED)),
                ]),
                Line::from(""),
                key_binding("Esc", "Return to Vim Normal"),
                key_binding("Ctrl+s", "Save"),
                key_binding("Ctrl+z", "Undo"),
                key_binding("Ctrl+a", "Select all"),
                key_binding("Ctrl+c", "Copy selection"),
                key_binding("Ctrl+v", "Paste"),
                key_binding("Shift+Arrows", "Extend selection"),
                key_binding("Ctrl+r", "Insert reference"),
                key_binding("Ctrl+f", "Insert file reference"),
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
