//! Prompt list component

use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

use crate::config::Config;
use crate::models::AppState;

/// Render the prompt list
pub fn render_prompt_list(frame: &mut Frame, area: Rect, state: &AppState, config: &Config) {
    // Different title and styling for archive mode
    let (title, title_style, border_color) = if state.mode == crate::models::Mode::Archive {
        (
            format!(" 📦 Archived ({}) ", state.prompts.len()),
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            Color::Yellow,
        )
    } else {
        (
            format!(" Prompts ({}) ", state.prompts.len()),
            Style::default(),
            if state.editor_focused { Color::DarkGray } else { Color::Cyan },
        )
    };
    
    let border_style = Style::default().fg(border_color);

    // Handle empty list
    if state.prompts.is_empty() {
        let (empty_text, hint_text) = if state.mode == crate::models::Mode::Archive {
            ("  No archived prompts", "  Press 'Esc' to return")
        } else {
            ("  No prompts yet", "  Press 'n' to create")
        };
        
        let empty_msg = Paragraph::new(vec![
            Line::from(""),
            Line::from(empty_text),
            Line::from(""),
            Line::from(Span::styled(hint_text, Style::default().fg(Color::DarkGray))),
        ])
        .block(
            Block::default()
                .title(title)
                .title_style(title_style)
                .borders(Borders::ALL)
                .border_style(border_style),
        );
        
        frame.render_widget(empty_msg, area);
        return;
    }

    let items: Vec<ListItem> = state
        .prompts
        .iter()
        .enumerate()
        .map(|(idx, prompt)| {
            let mut spans = Vec::new();
            
            // Tag color indicator
            if let Some(first_tag) = prompt.tags.first() {
                let color = tag_color(config.get_tag_color(first_tag));
                spans.push(Span::styled("● ", Style::default().fg(color)));
            } else {
                spans.push(Span::raw("  "));
            }
            
            // Prompt name
            let name_style = if idx == state.selected_index && !state.editor_focused {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            
            spans.push(Span::styled(&prompt.name, name_style));
            
            ListItem::new(Line::from(spans))
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .title(title)
                .title_style(title_style)
                .borders(Borders::ALL)
                .border_style(border_style),
        )
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD),
        );

    let mut list_state = ListState::default();
    list_state.select(Some(state.selected_index));
    // Apply scroll offset from state
    *list_state.offset_mut() = state.list_scroll_offset;

    frame.render_stateful_widget(list, area, &mut list_state);
}

/// Convert a color name to a ratatui Color
fn tag_color(name: &str) -> Color {
    match name.to_lowercase().as_str() {
        "blue" => Color::Blue,
        "green" => Color::Green,
        "yellow" => Color::Yellow,
        "magenta" | "purple" => Color::Magenta,
        "cyan" => Color::Cyan,
        "red" => Color::Red,
        "white" => Color::White,
        "gray" | "grey" => Color::Gray,
        _ => Color::White,
    }
}
