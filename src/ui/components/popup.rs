//! Reusable popup components

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};

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
