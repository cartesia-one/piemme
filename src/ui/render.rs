//! Main rendering logic

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::config::Config;
use crate::models::AppState;

use super::components::{render_help_overlay, render_prompt_list, render_status_bar, render_title_bar};

/// Render the entire application
pub fn render(frame: &mut Frame, state: &AppState, config: &Config, archived_count: usize) {
    let size = frame.area();

    // Main layout: title bar, content, status bar
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),  // Title bar
            Constraint::Min(5),     // Content
            Constraint::Length(2),  // Status bar
        ])
        .split(size);

    // Render title bar
    render_title_bar(frame, main_chunks[0], state);

    // Content area: left panel (list) and right panel (editor)
    let content_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30),  // Prompt list
            Constraint::Percentage(70),  // Editor
        ])
        .split(main_chunks[1]);

    // Render prompt list
    render_prompt_list(frame, content_chunks[0], state, config);

    // Render editor/viewer
    render_editor(frame, content_chunks[1], state, config);

    // Render status bar
    render_status_bar(frame, main_chunks[2], state, archived_count);

    // Render help overlay if active
    if state.show_help {
        render_help_overlay(frame, size, state.mode);
    }
}

/// Render the editor/viewer panel
fn render_editor(frame: &mut Frame, area: Rect, state: &AppState, config: &Config) {
    let border_style = if state.editor_focused {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    // Collect all prompt names for reference validation
    let prompt_names: Vec<&str> = state.prompts.iter().map(|p| p.name.as_str()).collect();

    let (title, content) = if let Some(prompt) = state.selected_prompt() {
        let title = format!(" {} ", prompt.name);
        let content = highlight_content(&prompt.content, &prompt_names, config);
        (title, content)
    } else {
        (" No prompt selected ".to_string(), vec![Line::from("Select or create a prompt to get started")])
    };

    // Add mode indicator to title
    let full_title = if state.mode == crate::models::Mode::Insert {
        format!("{}[EDITING] ", title)
    } else if state.mode == crate::models::Mode::Preview {
        format!("{}[PREVIEW] ", title)
    } else {
        title
    };

    let paragraph = Paragraph::new(content)
        .block(
            Block::default()
                .title(full_title)
                .borders(Borders::ALL)
                .border_style(border_style),
        )
        .wrap(Wrap { trim: false });

    frame.render_widget(paragraph, area);
}

/// Apply syntax highlighting to content
fn highlight_content<'a>(content: &'a str, existing_prompts: &[&str], _config: &Config) -> Vec<Line<'a>> {
    content
        .lines()
        .map(|line| highlight_line(line, existing_prompts))
        .collect()
}

/// Highlight a single line of content
fn highlight_line<'a>(line: &'a str, existing_prompts: &[&str]) -> Line<'a> {
    let mut spans = Vec::new();
    let mut current_pos = 0;
    let line_bytes = line.as_bytes();

    while current_pos < line.len() {
        // Check for reference pattern [[...]]
        if current_pos + 1 < line.len() 
            && line_bytes[current_pos] == b'['
            && line_bytes[current_pos + 1] == b'[' 
        {
            if let Some(end) = find_closing_brackets(line, current_pos + 2) {
                let ref_name = &line[current_pos + 2..end];
                let full_ref = &line[current_pos..end + 2];
                
                // Determine if reference is valid
                let is_valid = existing_prompts.contains(&ref_name);
                let color = if is_valid { Color::Green } else { Color::Red };
                
                spans.push(Span::styled(
                    full_ref.to_string(),
                    Style::default().fg(color).add_modifier(Modifier::BOLD),
                ));
                
                current_pos = end + 2;
                continue;
            }
        }

        // Check for command pattern {{...}}
        if current_pos + 1 < line.len()
            && line_bytes[current_pos] == b'{'
            && line_bytes[current_pos + 1] == b'{'
        {
            if let Some(end) = find_closing_braces(line, current_pos + 2) {
                let full_cmd = &line[current_pos..end + 2];
                
                spans.push(Span::styled(
                    full_cmd.to_string(),
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
                ));
                
                current_pos = end + 2;
                continue;
            }
        }

        // Regular character - accumulate until we hit a special pattern
        let start = current_pos;
        while current_pos < line.len() {
            if (current_pos + 1 < line.len() && line_bytes[current_pos] == b'[' && line_bytes[current_pos + 1] == b'[')
                || (current_pos + 1 < line.len() && line_bytes[current_pos] == b'{' && line_bytes[current_pos + 1] == b'{')
            {
                break;
            }
            current_pos += 1;
        }
        
        if start < current_pos {
            spans.push(Span::raw(line[start..current_pos].to_string()));
        }
    }

    if spans.is_empty() {
        Line::from(line.to_string())
    } else {
        Line::from(spans)
    }
}

/// Find closing ]] for a reference
fn find_closing_brackets(s: &str, start: usize) -> Option<usize> {
    let bytes = s.as_bytes();
    let mut i = start;
    
    while i + 1 < bytes.len() {
        if bytes[i] == b']' && bytes[i + 1] == b']' {
            return Some(i);
        }
        i += 1;
    }
    
    None
}

/// Find closing }} for a command
fn find_closing_braces(s: &str, start: usize) -> Option<usize> {
    let bytes = s.as_bytes();
    let mut i = start;
    
    while i + 1 < bytes.len() {
        if bytes[i] == b'}' && bytes[i + 1] == b'}' {
            return Some(i);
        }
        i += 1;
    }
    
    None
}
