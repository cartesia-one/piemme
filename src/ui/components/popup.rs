//! Reusable popup components

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::models::{FilePickerState, FolderSelectorState, ReferencePopupState, RenamePopupState, SearchPopupState, TagSelectorState};

// Cursor indicator for input fields  
const CURSOR_INDICATOR: &str = "_";

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
        .with_size(50, 50)
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
            Constraint::Length(1),  // Error/status
            Constraint::Min(1),     // Hints
        ])
        .margin(1)
        .split(inner);

    // Label
    let label = Paragraph::new("Enter new name:");
    frame.render_widget(label, chunks[0]);

    // Input box - exactly like tag selector which works
    let input_block = Block::default()
        .borders(Borders::ALL)
        .border_style(if state.is_valid {
            Style::default().fg(Color::Green)
        } else {
            Style::default().fg(Color::Red)
        });

    let input_text = Paragraph::new(format!("{}{}", state.input, CURSOR_INDICATOR))
        .style(Style::default().fg(Color::White))
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
    let hints = Paragraph::new(Span::styled(
        "Enter: confirm | Esc: cancel | Valid: a-z, 0-9, _",
        Style::default().fg(Color::DarkGray),
    ));
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
        Paragraph::new(format!("{}{}", state.filter, CURSOR_INDICATOR))
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

/// Render the file picker popup (fuzzy finder for files)
pub fn render_file_picker(frame: &mut Frame, area: Rect, state: &FilePickerState) {
    let config = PopupConfig::new("Insert File [[file:path]]")
        .with_size(70, 60)
        .with_border_color(Color::Blue);

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
        Paragraph::new("Type to filter files...")
            .style(Style::default().fg(Color::DarkGray))
            .block(filter_block)
    } else {
        Paragraph::new(format!("{}{}", state.filter, CURSOR_INDICATOR))
            .style(Style::default().fg(Color::White))
            .block(filter_block)
    };
    frame.render_widget(filter_text, chunks[0]);

    // Results list - calculate visible height
    let results_inner_height = chunks[1].height.saturating_sub(2) as usize; // Account for borders

    // Create items with scroll offset
    // IMPORTANT: enumerate() is called BEFORE skip(), which means the indices (i) are absolute positions
    // in the filtered_files list, not relative to the visible window. This is correct because
    // state.selected_index is also an absolute index into filtered_files.
    // Example: If scroll_offset=5 and selected_index=7, enumerate().skip(5) yields [(5,item5), (6,item6), (7,item7), ...]
    // and the comparison i==7 correctly highlights item7.
    let items: Vec<ListItem> = state
        .filtered_files
        .iter()
        .enumerate()
        .skip(state.scroll_offset)
        .take(results_inner_height)
        .map(|(i, file)| {
            let style = if i == state.selected_index {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Blue)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            ListItem::new(Line::from(Span::styled(file.clone(), style)))
        })
        .collect();

    let list_title = if state.filtered_files.is_empty() {
        if state.filter.is_empty() {
            " No files found ".to_string()
        } else {
            " No matches ".to_string()
        }
    } else {
        format!(" Files ({}) ", state.filtered_files.len())
    };

    let list = List::new(items)
        .block(
            Block::default()
                .title(list_title)
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

/// Render the tag selector popup
pub fn render_tag_selector(frame: &mut Frame, area: Rect, state: &TagSelectorState) {
    let config = PopupConfig::new("Manage Tags")
        .with_size(50, 50)
        .with_border_color(Color::Yellow);

    let popup_area = centered_rect(config.width_percent, config.height_percent, area);

    // Clear the background
    frame.render_widget(Clear, popup_area);

    // Create layout
    let block = Block::default()
        .title(format!(" {} ", config.title))
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(config.border_color));

    let inner = block.inner(popup_area);
    frame.render_widget(block, popup_area);

    if state.creating_new {
        // Show new tag input
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),  // Label
                Constraint::Length(3),  // Input box
                Constraint::Min(1),     // Spacer
                Constraint::Length(1),  // Hints
            ])
            .margin(1)
            .split(inner);

        let label = Paragraph::new("Enter new tag name:");
        frame.render_widget(label, chunks[0]);

        let input_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Yellow));

        let input_text = Paragraph::new(format!("{}{}", state.new_tag_input, CURSOR_INDICATOR))
            .style(Style::default().fg(Color::White))
            .block(input_block);
        frame.render_widget(input_text, chunks[1]);

        let hints = Paragraph::new(Span::styled(
            "Enter: create | Esc: cancel",
            Style::default().fg(Color::DarkGray),
        ));
        frame.render_widget(hints, chunks[3]);
    } else {
        // Show tag list
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),  // Filter input
                Constraint::Min(3),     // Tag list
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
            Paragraph::new(format!("{}{}", state.filter, CURSOR_INDICATOR))
                .style(Style::default().fg(Color::White))
                .block(filter_block)
        };
        frame.render_widget(filter_text, chunks[0]);

        // Tag list
        let items: Vec<ListItem> = state
            .filtered_tags
            .iter()
            .enumerate()
            .map(|(i, tag)| {
                let is_assigned = state.is_tag_assigned(tag);
                let checkbox = if is_assigned { "[✓] " } else { "[ ] " };
                
                let style = if i == state.selected_index {
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::Yellow)
                        .add_modifier(Modifier::BOLD)
                } else if is_assigned {
                    Style::default().fg(Color::Green)
                } else {
                    Style::default().fg(Color::White)
                };
                ListItem::new(Line::from(Span::styled(format!("{}{}", checkbox, tag), style)))
            })
            .collect();

        let list = List::new(items)
            .block(
                Block::default()
                    .title(format!(" Tags ({}) ", state.filtered_tags.len()))
                    .borders(Borders::ALL),
            );
        frame.render_widget(list, chunks[1]);

        // Hints
        let hints = Paragraph::new(Span::styled(
            "↑↓: navigate | Enter/Space: toggle | Ctrl+n: new | Esc: done",
            Style::default().fg(Color::DarkGray),
        ));
        frame.render_widget(hints, chunks[2]);
    }
}

/// Render the folder selector popup
pub fn render_folder_selector(frame: &mut Frame, area: Rect, state: &FolderSelectorState) {
    let title = match state.mode {
        crate::models::FolderSelectorMode::Open => "Open Folder",
        crate::models::FolderSelectorMode::Move => "Move to Folder",
    };
    
    let config = PopupConfig::new(title)
        .with_size(50, 50)
        .with_border_color(Color::Magenta);

    let popup_area = centered_rect(config.width_percent, config.height_percent, area);

    // Clear the background
    frame.render_widget(Clear, popup_area);

    // Create layout
    let block = Block::default()
        .title(format!(" {} ", config.title))
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(config.border_color));

    let inner = block.inner(popup_area);
    frame.render_widget(block, popup_area);

    if state.creating_new {
        // Show new folder input
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),  // Label
                Constraint::Length(3),  // Input box
                Constraint::Min(1),     // Spacer
                Constraint::Length(1),  // Hints
            ])
            .margin(1)
            .split(inner);

        let label = Paragraph::new("Enter new folder name:");
        frame.render_widget(label, chunks[0]);

        let input_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Magenta));

        let input_text = Paragraph::new(format!("{}{}", state.new_folder_input, CURSOR_INDICATOR))
            .style(Style::default().fg(Color::White))
            .block(input_block);
        frame.render_widget(input_text, chunks[1]);

        let hints = Paragraph::new(Span::styled(
            "Enter: create | Esc: cancel",
            Style::default().fg(Color::DarkGray),
        ));
        frame.render_widget(hints, chunks[3]);
    } else {
        // Show folder list
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),  // Filter input
                Constraint::Min(3),     // Folder list
                Constraint::Length(1),  // Hints
            ])
            .margin(1)
            .split(inner);

        // Filter input
        let filter_block = Block::default()
            .title(" Filter ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Magenta));

        let filter_text = if state.filter.is_empty() {
            Paragraph::new("Type to filter...")
                .style(Style::default().fg(Color::DarkGray))
                .block(filter_block)
        } else {
            Paragraph::new(format!("{}{}", state.filter, CURSOR_INDICATOR))
                .style(Style::default().fg(Color::White))
                .block(filter_block)
        };
        frame.render_widget(filter_text, chunks[0]);

        // Folder list
        let items: Vec<ListItem> = state
            .filtered_folders
            .iter()
            .enumerate()
            .map(|(i, folder)| {
                let icon = if folder == "(root)" { "📁 " } else { "📂 " };
                
                let style = if i == state.selected_index {
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::Magenta)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White)
                };
                ListItem::new(Line::from(Span::styled(format!("{}{}", icon, folder), style)))
            })
            .collect();

        let list = List::new(items)
            .block(
                Block::default()
                    .title(format!(" Folders ({}) ", state.filtered_folders.len()))
                    .borders(Borders::ALL),
            );
        frame.render_widget(list, chunks[1]);

        // Hints
        let hints = Paragraph::new(Span::styled(
            "↑↓: navigate | Enter: select | Ctrl+n: new folder | Esc: cancel",
            Style::default().fg(Color::DarkGray),
        ));
        frame.render_widget(hints, chunks[2]);
    }
}

/// Render the search popup (fuzzy finder for prompts)
pub fn render_search_popup(frame: &mut Frame, area: Rect, state: &SearchPopupState) {
    let config = PopupConfig::new("Search Prompts")
        .with_size(70, 60)
        .with_border_color(Color::Green);

    let popup_area = centered_rect(config.width_percent, config.height_percent, area);

    // Clear the background
    frame.render_widget(Clear, popup_area);

    // Create layout for search input and results
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
            Constraint::Length(3),  // Search input
            Constraint::Min(3),     // Results list
            Constraint::Length(1),  // Hints
        ])
        .margin(1)
        .split(inner);

    // Search input
    let search_block = Block::default()
        .title(" Search ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow));

    let search_text = if state.query.is_empty() {
        Paragraph::new("Type to search by name or content...")
            .style(Style::default().fg(Color::DarkGray))
            .block(search_block)
    } else {
        Paragraph::new(format!("{}{}", state.query, CURSOR_INDICATOR))
            .style(Style::default().fg(Color::White))
            .block(search_block)
    };
    frame.render_widget(search_text, chunks[0]);

    // Results list - calculate visible height
    let results_inner_height = chunks[1].height.saturating_sub(2) as usize; // Account for borders

    // Create items with scroll offset
    let items: Vec<ListItem> = state
        .results
        .iter()
        .enumerate()
        .skip(state.scroll_offset)
        .take(results_inner_height)
        .map(|(i, result)| {
            let is_selected = i == state.selected_index;
            
            // Build the name with match highlights
            let name_spans = build_highlighted_spans(&result.name, &result.name_match_indices, is_selected);
            
            // Build preview line
            let preview_style = if is_selected {
                Style::default().fg(Color::Black).bg(Color::Green)
            } else {
                Style::default().fg(Color::DarkGray)
            };
            
            // Truncate preview to fit
            let preview = if result.preview.len() > 60 {
                format!("  {}...", &result.preview[..57])
            } else {
                format!("  {}", result.preview)
            };
            
            let content = vec![
                Line::from(name_spans),
                Line::from(Span::styled(preview, preview_style)),
            ];
            
            ListItem::new(content)
        })
        .collect();

    let results_title = if state.results.is_empty() {
        if state.query.is_empty() {
            " Results ".to_string()
        } else {
            " No matches ".to_string()
        }
    } else {
        format!(" Results ({}) ", state.results.len())
    };

    let list = List::new(items)
        .block(
            Block::default()
                .title(results_title)
                .borders(Borders::ALL),
        );
    frame.render_widget(list, chunks[1]);

    // Hints
    let hints = Paragraph::new(Span::styled(
        "↑↓: navigate | Enter: jump to prompt | Esc: cancel",
        Style::default().fg(Color::DarkGray),
    ));
    frame.render_widget(hints, chunks[2]);
}

/// Build spans with highlighted matching characters
fn build_highlighted_spans(text: &str, match_indices: &[usize], is_selected: bool) -> Vec<Span<'static>> {
    let base_style = if is_selected {
        Style::default().fg(Color::Black).bg(Color::Green)
    } else {
        Style::default().fg(Color::White)
    };
    
    let highlight_style = if is_selected {
        Style::default()
            .fg(Color::Yellow)
            .bg(Color::Green)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD)
    };
    
    let mut spans = Vec::new();
    let mut last_idx = 0;
    
    for &idx in match_indices {
        if idx >= text.len() {
            continue;
        }
        
        // Add non-matching text before this match
        if idx > last_idx {
            spans.push(Span::styled(text[last_idx..idx].to_string(), base_style));
        }
        
        // Add the matching character
        let end_idx = (idx + 1).min(text.len());
        spans.push(Span::styled(text[idx..end_idx].to_string(), highlight_style));
        last_idx = end_idx;
    }
    
    // Add any remaining text
    if last_idx < text.len() {
        spans.push(Span::styled(text[last_idx..].to_string(), base_style));
    }
    
    if spans.is_empty() {
        spans.push(Span::styled(text.to_string(), base_style));
    }
    
    spans
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
