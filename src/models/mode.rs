//! Application mode definitions

/// The different modes the application can be in
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Mode {
    /// Normal mode - navigate prompts and execute commands
    #[default]
    Normal,
    /// Insert mode - edit prompt content
    Insert,
    /// Archive mode - view archived prompts
    Archive,
    /// Folder mode - view prompts within a folder
    Folder,
    /// Preview mode - view rendered prompt output
    Preview,
}

impl Mode {
    /// Get a display string for the mode
    pub fn as_str(&self) -> &'static str {
        match self {
            Mode::Normal => "NORMAL",
            Mode::Insert => "INSERT",
            Mode::Archive => "ARCHIVE",
            Mode::Folder => "FOLDER",
            Mode::Preview => "PREVIEW",
        }
    }

    /// Check if the mode allows editing
    pub fn is_editable(&self) -> bool {
        matches!(self, Mode::Insert)
    }

    /// Check if the mode is read-only
    pub fn is_read_only(&self) -> bool {
        matches!(self, Mode::Normal | Mode::Archive | Mode::Preview)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mode_as_str() {
        assert_eq!(Mode::Normal.as_str(), "NORMAL");
        assert_eq!(Mode::Insert.as_str(), "INSERT");
        assert_eq!(Mode::Archive.as_str(), "ARCHIVE");
        assert_eq!(Mode::Folder.as_str(), "FOLDER");
        assert_eq!(Mode::Preview.as_str(), "PREVIEW");
    }

    #[test]
    fn test_mode_is_editable() {
        assert!(!Mode::Normal.is_editable());
        assert!(Mode::Insert.is_editable());
        assert!(!Mode::Archive.is_editable());
        assert!(!Mode::Preview.is_editable());
    }

    #[test]
    fn test_mode_is_read_only() {
        assert!(Mode::Normal.is_read_only());
        assert!(!Mode::Insert.is_read_only());
        assert!(Mode::Archive.is_read_only());
        assert!(Mode::Preview.is_read_only());
    }
}
