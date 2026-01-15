//! Editor component wrapping tui-textarea

use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    Frame,
};
use tui_textarea::{CursorMove, Input, TextArea};

use crate::config::Config;

/// Editor component wrapping tui-textarea with syntax highlighting support
pub struct Editor<'a> {
    /// The underlying textarea
    textarea: TextArea<'a>,
    /// Whether the editor is focused
    focused: bool,
    /// The title to display
    title: String,
    /// Mode indicator (e.g., "[EDITING]", "[PREVIEW]")
    mode_indicator: Option<String>,
}

impl<'a> Editor<'a> {
    /// Create a new editor with the given content
    pub fn new(content: &str) -> Self {
        let lines: Vec<String> = content.lines().map(String::from).collect();
        let mut textarea = TextArea::new(if lines.is_empty() {
            vec![String::new()]
        } else {
            lines
        });

        // Configure default style
        textarea.set_cursor_line_style(Style::default());
        textarea.set_line_number_style(Style::default().fg(Color::DarkGray));

        Self {
            textarea,
            focused: false,
            title: String::new(),
            mode_indicator: None,
        }
    }

    /// Create an empty editor
    pub fn empty() -> Self {
        Self::new("")
    }

    /// Set whether the editor is focused
    pub fn set_focused(&mut self, focused: bool) {
        self.focused = focused;
    }

    /// Set the title
    pub fn set_title(&mut self, title: impl Into<String>) {
        self.title = title.into();
    }

    /// Set the mode indicator
    pub fn set_mode_indicator(&mut self, indicator: Option<String>) {
        self.mode_indicator = indicator;
    }

    /// Set the content
    pub fn set_content(&mut self, content: &str) {
        let lines: Vec<String> = content.lines().map(String::from).collect();
        self.textarea = TextArea::new(if lines.is_empty() {
            vec![String::new()]
        } else {
            lines
        });
        self.apply_styling();
    }

    /// Get the current content as a string
    pub fn content(&self) -> String {
        self.textarea.lines().join("\n")
    }

    /// Check if content has changed from original
    pub fn is_modified(&self, original: &str) -> bool {
        self.content() != original
    }

    /// Apply the current styling based on focus state
    fn apply_styling(&mut self) {
        let border_color = if self.focused {
            Color::Cyan
        } else {
            Color::DarkGray
        };

        let block = ratatui::widgets::Block::default()
            .title(self.full_title())
            .borders(ratatui::widgets::Borders::ALL)
            .border_style(Style::default().fg(border_color));

        self.textarea.set_block(block);

        // Cursor style
        if self.focused {
            self.textarea
                .set_cursor_style(Style::default().add_modifier(Modifier::REVERSED));
        } else {
            self.textarea.set_cursor_style(Style::default());
        }
    }

    /// Get the full title including mode indicator
    fn full_title(&self) -> String {
        let base_title = if self.title.is_empty() {
            " No prompt selected ".to_string()
        } else {
            format!(" {} ", self.title)
        };

        if let Some(indicator) = &self.mode_indicator {
            format!("{}{} ", base_title, indicator)
        } else {
            base_title
        }
    }

    /// Handle input and return true if the input was consumed
    pub fn handle_input(&mut self, input: Input) -> bool {
        if !self.focused {
            return false;
        }

        // Let textarea handle the input
        self.textarea.input(input);
        true
    }

    /// Move cursor to the start of the document
    pub fn move_to_start(&mut self) {
        self.textarea.move_cursor(CursorMove::Top);
        self.textarea.move_cursor(CursorMove::Head);
    }

    /// Move cursor to the end of the document
    pub fn move_to_end(&mut self) {
        self.textarea.move_cursor(CursorMove::Bottom);
        self.textarea.move_cursor(CursorMove::End);
    }

    /// Move cursor one word left
    pub fn move_word_left(&mut self) {
        self.textarea.move_cursor(CursorMove::WordBack);
    }

    /// Move cursor one word right
    pub fn move_word_right(&mut self) {
        self.textarea.move_cursor(CursorMove::WordForward);
    }

    /// Undo the last change
    pub fn undo(&mut self) {
        self.textarea.undo();
    }

    /// Redo the last undone change
    pub fn redo(&mut self) {
        self.textarea.redo();
    }

    /// Get cursor position (row, col)
    pub fn cursor(&self) -> (usize, usize) {
        self.textarea.cursor()
    }

    /// Scroll up by one line
    pub fn scroll_up(&mut self) {
        self.textarea.scroll((1, 0));
    }

    /// Scroll down by one line  
    pub fn scroll_down(&mut self) {
        self.textarea.scroll((-1, 0));
    }

    /// Render the editor in the given area
    pub fn render(&mut self, frame: &mut Frame, area: Rect, existing_prompts: &[&str], _config: &Config) {
        self.apply_styling();
        
        // Apply syntax highlighting to the content
        self.apply_syntax_highlighting(existing_prompts);
        
        frame.render_widget(&self.textarea, area);
    }

    /// Apply syntax highlighting for references and commands
    fn apply_syntax_highlighting(&mut self, existing_prompts: &[&str]) {
        let _lines: Vec<Line> = self
            .textarea
            .lines()
            .iter()
            .map(|line| highlight_line(line, existing_prompts))
            .collect();

        self.textarea.set_style(Style::default());
        
        // Note: tui-textarea doesn't directly support per-line styling in the same way
        // For now, we use basic styling and defer advanced highlighting to the view mode
        // The full highlighting is shown in Normal/Preview modes via the render_editor function
    }
}

/// Highlight a single line of content with references and commands
fn highlight_line<'a>(line: &'a str, existing_prompts: &[&str]) -> Line<'a> {
    use ratatui::text::Span;

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

                // Add warning indicator before commands
                spans.push(Span::styled(
                    "⚠ ",
                    Style::default().fg(Color::LightRed),
                ));
                spans.push(Span::styled(
                    full_cmd.to_string(),
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ));

                current_pos = end + 2;
                continue;
            }
        }

        // Regular character
        let start = current_pos;
        while current_pos < line.len() {
            if (current_pos + 1 < line.len()
                && line_bytes[current_pos] == b'['
                && line_bytes[current_pos + 1] == b'[')
                || (current_pos + 1 < line.len()
                    && line_bytes[current_pos] == b'{'
                    && line_bytes[current_pos + 1] == b'{')
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_editor_creation() {
        let editor = Editor::new("Hello\nWorld");
        assert_eq!(editor.content(), "Hello\nWorld");
    }

    #[test]
    fn test_editor_empty() {
        let editor = Editor::empty();
        assert_eq!(editor.content(), "");
    }

    #[test]
    fn test_editor_set_content() {
        let mut editor = Editor::new("Initial");
        editor.set_content("New content\nLine 2");
        assert_eq!(editor.content(), "New content\nLine 2");
    }

    #[test]
    fn test_editor_is_modified() {
        let editor = Editor::new("Original");
        assert!(!editor.is_modified("Original"));
        assert!(editor.is_modified("Different"));
    }
}
