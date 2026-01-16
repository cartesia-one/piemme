//! Reference parsing and validation

use regex::Regex;
use std::sync::LazyLock;

/// A reference to another prompt or a file
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reference {
    /// The full match including brackets: [[name]] or [[file:path]]
    pub full_match: String,
    /// The prompt name being referenced (None for file references)
    pub name: String,
    /// Start position in the content
    pub start: usize,
    /// End position in the content
    pub end: usize,
    /// Whether this reference is valid (points to existing prompt)
    pub is_valid: bool,
    /// Whether this is a file reference
    pub is_file: bool,
}

/// A file reference
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileReference {
    /// The full match including brackets: [[file:path]]
    pub full_match: String,
    /// The file path being referenced
    pub path: String,
    /// Start position in the content
    pub start: usize,
    /// End position in the content
    pub end: usize,
    /// Whether this file exists and is readable
    pub is_valid: bool,
}

// Regex for matching [[prompt_name]] references
static REFERENCE_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\[\[([a-z0-9_]+)\]\]").expect("Invalid reference regex")
});

// Regex for matching [[file:path]] references
static FILE_REFERENCE_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\[\[file:([^\]]+)\]\]").expect("Invalid file reference regex")
});

/// Find all prompt references in content (excluding file references)
pub fn find_references(content: &str) -> Vec<Reference> {
    REFERENCE_REGEX
        .captures_iter(content)
        .map(|cap| {
            let full_match = cap.get(0).unwrap();
            let name_match = cap.get(1).unwrap();
            
            Reference {
                full_match: full_match.as_str().to_string(),
                name: name_match.as_str().to_string(),
                start: full_match.start(),
                end: full_match.end(),
                is_valid: false, // Will be set by validate_reference
                is_file: false,
            }
        })
        .collect()
}

/// Find all file references in content
pub fn find_file_references(content: &str) -> Vec<FileReference> {
    FILE_REFERENCE_REGEX
        .captures_iter(content)
        .map(|cap| {
            let full_match = cap.get(0).unwrap();
            let path_match = cap.get(1).unwrap();
            
            FileReference {
                full_match: full_match.as_str().to_string(),
                path: path_match.as_str().to_string(),
                start: full_match.start(),
                end: full_match.end(),
                is_valid: false, // Will be set by validate_file_reference
            }
        })
        .collect()
}

/// Validate a reference against existing prompt names
pub fn validate_reference(reference: &mut Reference, existing_names: &[&str]) {
    reference.is_valid = existing_names.contains(&reference.name.as_str());
}

/// Validate all references in content and return them
pub fn find_and_validate_references(content: &str, existing_names: &[&str]) -> Vec<Reference> {
    let mut refs = find_references(content);
    for r in &mut refs {
        validate_reference(r, existing_names);
    }
    refs
}

/// Validate a file reference (check if file exists)
pub fn validate_file_reference(reference: &mut FileReference) {
    use std::path::Path;
    let path = Path::new(&reference.path);
    reference.is_valid = path.exists() && path.is_file();
}

/// Validate all file references and return them
pub fn find_and_validate_file_references(content: &str) -> Vec<FileReference> {
    let mut refs = find_file_references(content);
    for r in &mut refs {
        validate_file_reference(r);
    }
    refs
}

/// Check if content contains any file references
pub fn has_file_references(content: &str) -> bool {
    FILE_REFERENCE_REGEX.is_match(content)
}

/// Read file content for a file reference
pub fn read_file_content(path: &str) -> std::io::Result<String> {
    std::fs::read_to_string(path)
}

/// Check if content contains any references
pub fn has_references(content: &str) -> bool {
    REFERENCE_REGEX.is_match(content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_references() {
        let content = "Hello [[world]] and [[test_prompt]]!";
        let refs = find_references(content);
        
        assert_eq!(refs.len(), 2);
        assert_eq!(refs[0].name, "world");
        assert_eq!(refs[0].full_match, "[[world]]");
        assert_eq!(refs[1].name, "test_prompt");
    }

    #[test]
    fn test_no_references() {
        let content = "Hello world without any references!";
        let refs = find_references(content);
        assert!(refs.is_empty());
    }

    #[test]
    fn test_invalid_reference_format() {
        // These should NOT match
        let content = "[[UPPER]] [[with space]] [[with-dash]]";
        let refs = find_references(content);
        assert!(refs.is_empty());
    }

    #[test]
    fn test_validate_references() {
        let content = "Check [[valid_ref]] and [[invalid_ref]]";
        let existing = vec!["valid_ref", "other"];
        
        let refs = find_and_validate_references(content, &existing);
        
        assert_eq!(refs.len(), 2);
        assert!(refs[0].is_valid);
        assert!(!refs[1].is_valid);
    }

    #[test]
    fn test_has_references() {
        assert!(has_references("Contains [[reference]]"));
        assert!(!has_references("No references here"));
    }

    #[test]
    fn test_find_file_references() {
        let content = "Check [[file:src/main.rs]] and [[file:README.md]]";
        let refs = find_file_references(content);
        
        assert_eq!(refs.len(), 2);
        assert_eq!(refs[0].path, "src/main.rs");
        assert_eq!(refs[0].full_match, "[[file:src/main.rs]]");
        assert_eq!(refs[1].path, "README.md");
    }

    #[test]
    fn test_no_file_references() {
        let content = "Hello world without any file references!";
        let refs = find_file_references(content);
        assert!(refs.is_empty());
    }

    #[test]
    fn test_has_file_references() {
        assert!(has_file_references("Contains [[file:test.txt]]"));
        assert!(!has_file_references("No file references here"));
        assert!(!has_file_references("Only [[prompt_ref]]"));
    }

    #[test]
    fn test_mixed_references() {
        let content = "[[prompt_ref]] and [[file:test.txt]]";
        let prompt_refs = find_references(content);
        let file_refs = find_file_references(content);
        
        assert_eq!(prompt_refs.len(), 1);
        assert_eq!(file_refs.len(), 1);
        assert_eq!(prompt_refs[0].name, "prompt_ref");
        assert_eq!(file_refs[0].path, "test.txt");
    }
}
