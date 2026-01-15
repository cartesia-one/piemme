//! Reference parsing and validation

use regex::Regex;
use std::sync::LazyLock;

/// A reference to another prompt
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reference {
    /// The full match including brackets: [[name]]
    pub full_match: String,
    /// The prompt name being referenced
    pub name: String,
    /// Start position in the content
    pub start: usize,
    /// End position in the content
    pub end: usize,
    /// Whether this reference is valid (points to existing prompt)
    pub is_valid: bool,
}

// Regex for matching [[prompt_name]] references
static REFERENCE_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\[\[([a-z0-9_]+)\]\]").expect("Invalid reference regex")
});

/// Find all references in content
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
}
