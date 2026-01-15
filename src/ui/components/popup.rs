//! Reusable popup components

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::models::{ReferencePopupState, RenamePopupState};

/// Configuration for a popup
pub struct PopupConfig {
    /// Title displayed at the top
    pub title: String,
    /// Border color
    pub border_color: Color,
    /// Width as percentage of screen (0-100)
    pub width_percent: u16,
    /// Height as percentage of screen (0-100)
    pub height_percent: u16,
}

impl Default for PopupConfig {
    fn default() -> Self {
        Self {
            title: String::new(),
            border_color: Color::Cyan,
            width_percent: 50,
            height_percent: 30,
        }
    }
}

impl PopupConfig {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            ..Default::default()
        }
    }

    pub fn with_size(mut self, width: u16, height: u16) -> Self {
        self.width_percent = width;
        self.height_percent = height;
        self
    }

    pub fn with_border_color(mut self, color: Color) -> Self {
        self.border_color = color;
        self
    }
}

/// Render a generic popup frame and return the inner content area
pub fn render_popup_frame(frame: &mut Frame, area: Rect, config: &PopupConfig) -> Rect {
    let popup_area = centered_rect(config.width_percent, config.height_percent, area);
    
    // Clear the background
    frame.render_widget(Clear, popup_area);
    
    // Render the border block
    let block = Block::default()
        .title(format!(" {} ", config.title))
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(config.border_color));
    
    let inner = block.inner(popup_area);
    frame.render_widget(block, popup_area);
    
    inner
}

/// Render a confirmation dialog
pub fn render_confirm_dialog(
    frame: &mut Frame,
    area: Rect,
    title: &str,
    message: &str,
    confirm_selected: bool,
) {
    let config = PopupConfig::new(title)
        .with_size(50, 25)
        .with_border_color(Color::Yellow);
    
    let popup_area = centered_rect(config.width_percent, config.height_percent, area);
    
    // Clear the background
    frame.render_widget(Clear, popup_area);
    
    // Create layout for message and buttons
    let block = Block::default()
        .title(format!(" {} ", config.title))
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(config.border_color));
    
    let inner = block.inner(popup_area);
    frame.render_widget(block, popup_area);
    
    // Split into message area and button area
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),     // Message
            Constraint::Length(3),  // Buttons
        ])
        .margin(1)
        .split(inner);
    
    // Render message
    let message_paragraph = Paragraph::new(message)
        .wrap(Wrap { trim: false })
        .alignment(Alignment::Center);
    frame.render_widget(message_paragraph, chunks[0]);
    
    // Render buttons
    let buttons = render_dialog_buttons(confirm_selected);
    let buttons_paragraph = Paragraph::new(buttons)
        .alignment(Alignment::Center);
    frame.render_widget(buttons_paragraph, chunks[1]);
}

/// Render the Yes/No buttons for confirmation dialogs
fn render_dialog_buttons(confirm_selected: bool) -> Vec<Line<'static>> {
    let yes_style = if confirm_selected {
        Style::default()
            .fg(Color::Black)
            .bg(Color::Green)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Green)
    };
    
    let no_style = if !confirm_selected {
        Style::default()
            .fg(Color::Black)
            .bg(Color::Red)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Red)
    };
    
    vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("  [ Yes ] ", yes_style),
            Span::raw("   "),
            Span::styled(" [ No ]  ", no_style),
        ]),
    ]
}

/// Create a centered rectangle
pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
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

/// Render the rename popup with validation
pub fn render_rename_popup(frame: &mut Frame, area: Rect, state: &RenamePopupState) {
    let config = PopupConfig::new("Rename Prompt")
        .with_size(50, 20)
        .with_border_color(if state.is_valid { Color::Cyan } else { Color::Red });

    let popup_area = centered_rect(config.width_percent, config.height_percent, area);

    // Clear the background
    frame.render_widget(Clear, popup_area);

    // Create layout for input and hints
    let block = Block::default()
        .title(format!(" {} ", config.title))
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(config.border_color));

    let inner = block.inner(popup_area);
    frame.render_widget(block, popup_area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),  // Label
            Constraint::Length(3),  // Input box
            Constraint::Length(2),  // Error/status
            Constraint::Min(1),     // Hints
        ])
        .margin(1)
        .split(inner);

    // Label
    let label = Paragraph::new("Enter new name:");
    frame.render_widget(label, chunks[0]);

    // Input box
    let input_style = if state.is_valid {
        Style::default().fg(Color::White)
    } else {
        Style::default().fg(Color::Red)
    };

    let input_block = Block::default()
        .borders(Borders::ALL)
        .border_style(if state.is_valid {
            Style::default().fg(Color::Green)
        } else {
            Style::default().fg(Color::Red)
        });

    let input_text = Paragraph::new(format!("{}_", state.input))
        .style(input_style)
        .block(input_block);
    frame.render_widget(input_text, chunks[1]);

    // Error message or validation status
    let status = if let Some(ref err) = state.error_message {
        Paragraph::new(err.as_str()).style(Style::default().fg(Color::Red))
    } else if state.is_valid {
        Paragraph::new("✓ Valid name").style(Style::default().fg(Color::Green))
    } else {
        Paragraph::new("").style(Style::default())
    };
    frame.render_widget(status, chunks[2]);

    // Hints
    let hints = Paragraph::new(vec![
        Line::from(Span::styled(
            "Enter: confirm | Esc: cancel",
            Style::default().fg(Color::DarkGray),
        )),
        Line::from(Span::styled(
            "Valid: a-z, 0-9, _ (no leading/trailing _)",
            Style::default().fg(Color::DarkGray),
        )),
    ]);
    frame.render_widget(hints, chunks[3]);
}

/// Render the reference insertion popup (fuzzy finder for prompts)
pub fn render_reference_popup(frame: &mut Frame, area: Rect, state: &ReferencePopupState) {
    let config = PopupConfig::new("Insert Reference [[prompt]]")
        .with_size(60, 50)
        .with_border_color(Color::Cyan);

    let popup_area = centered_rect(config.width_percent, config.height_percent, area);

    // Clear the background
    frame.render_widget(Clear, popup_area);

    // Create layout for filter input and list
    let block = Block::default()
        .title(format!(" {} ", config.title))
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(config.border_color));

    let inner = block.inner(popup_area);
    frame.render_widget(block, popup_area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Filter input
            Constraint::Min(3),     // Results list
            Constraint::Length(1),  // Hints
        ])
        .margin(1)
        .split(inner);

    // Filter input
    let filter_block = Block::default()
        .title(" Filter ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow));

    let filter_text = if state.filter.is_empty() {
        Paragraph::new("Type to filter...")
            .style(Style::default().fg(Color::DarkGray))
            .block(filter_block)
    } else {
        Paragraph::new(format!("{}_", state.filter))
            .style(Style::default().fg(Color::White))
            .block(filter_block)
    };
    frame.render_widget(filter_text, chunks[0]);

    // Results list
    let items: Vec<ListItem> = state
        .filtered_names
        .iter()
        .enumerate()
        .map(|(i, name)| {
            let style = if i == state.selected_index {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            ListItem::new(Line::from(Span::styled(name.clone(), style)))
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .title(format!(" Results ({}) ", state.filtered_names.len()))
                .borders(Borders::ALL),
        );
    frame.render_widget(list, chunks[1]);

    // Hints
    let hints = Paragraph::new(Span::styled(
        "↑↓: navigate | Enter: insert | Esc: cancel",
        Style::default().fg(Color::DarkGray),
    ));
    frame.render_widget(hints, chunks[2]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_popup_config_defaults() {
        let config = PopupConfig::default();
        assert_eq!(config.width_percent, 50);
        assert_eq!(config.height_percent, 30);
        assert_eq!(config.border_color, Color::Cyan);
    }

    #[test]
    fn test_popup_config_builder() {
        let config = PopupConfig::new("Test")
            .with_size(60, 40)
            .with_border_color(Color::Red);
        
        assert_eq!(config.title, "Test");
        assert_eq!(config.width_percent, 60);
        assert_eq!(config.height_percent, 40);
        assert_eq!(config.border_color, Color::Red);
    }
}
