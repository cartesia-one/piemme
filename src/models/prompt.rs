//! Prompt data structure and operations

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A prompt with its metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prompt {
    /// Unique identifier for the prompt
    pub id: Uuid,
    /// Auto-generated name derived from content
    #[serde(skip)]
    pub name: String,
    /// The actual prompt content (markdown)
    #[serde(skip)]
    pub content: String,
    /// Tags associated with this prompt
    #[serde(default)]
    pub tags: Vec<String>,
    /// When the prompt was created
    pub created: DateTime<Utc>,
    /// When the prompt was last modified
    pub modified: DateTime<Utc>,
}

impl Prompt {
    /// Create a new prompt with default values
    pub fn new() -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name: String::new(),
            content: String::new(),
            tags: Vec::new(),
            created: now,
            modified: now,
        }
    }

    /// Create a new prompt with the given content
    pub fn with_content(content: impl Into<String>) -> Self {
        let mut prompt = Self::new();
        prompt.content = content.into();
        prompt.name = generate_name_from_content(&prompt.content);
        prompt
    }

    /// Update the prompt content and refresh the modified timestamp
    pub fn set_content(&mut self, content: impl Into<String>) {
        self.content = content.into();
        self.modified = Utc::now();
    }

    /// Add a tag to the prompt
    pub fn add_tag(&mut self, tag: impl Into<String>) {
        let tag = tag.into();
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
            self.modified = Utc::now();
        }
    }

    /// Remove a tag from the prompt
    pub fn remove_tag(&mut self, tag: &str) -> bool {
        if let Some(pos) = self.tags.iter().position(|t| t == tag) {
            self.tags.remove(pos);
            self.modified = Utc::now();
            true
        } else {
            false
        }
    }

    /// Check if the prompt has a specific tag
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }

    /// Get the first line of content (for preview)
    pub fn first_line(&self) -> &str {
        self.content.lines().next().unwrap_or("")
    }

    /// Get the YAML frontmatter for this prompt
    pub fn frontmatter(&self) -> PromptFrontmatter {
        PromptFrontmatter {
            id: self.id,
            tags: self.tags.clone(),
            created: self.created,
            modified: self.modified,
        }
    }
}

impl Default for Prompt {
    fn default() -> Self {
        Self::new()
    }
}

/// The YAML frontmatter portion of a prompt file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptFrontmatter {
    pub id: Uuid,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    pub created: DateTime<Utc>,
    pub modified: DateTime<Utc>,
}

/// Generate a prompt name from its content
///
/// Rules:
/// 1. Take the first ~15-20 characters of content
/// 2. Convert to lowercase
/// 3. Replace spaces with underscores
/// 4. Remove special characters (keep only a-z, 0-9, _)
/// 5. Truncate to create a short, readable name
pub fn generate_name_from_content(content: &str) -> String {
    let first_line = content.lines().next().unwrap_or("");

    if first_line.trim().is_empty() {
        return String::new();
    }

    // Take first ~20 chars, convert to lowercase
    let name: String = first_line
        .chars()
        .take(20)
        .collect::<String>()
        .to_lowercase()
        // Replace spaces and common separators with underscores
        .chars()
        .map(|c| if c.is_whitespace() || c == '-' { '_' } else { c })
        .collect::<String>()
        // Keep only alphanumeric and underscores
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '_')
        .collect();

    // Remove leading/trailing underscores and collapse multiple underscores
    let mut result = String::new();
    let mut prev_underscore = true; // Start true to skip leading underscores
    
    for c in name.chars() {
        if c == '_' {
            if !prev_underscore {
                result.push(c);
                prev_underscore = true;
            }
        } else {
            result.push(c);
            prev_underscore = false;
        }
    }

    // Remove trailing underscore
    if result.ends_with('_') {
        result.pop();
    }

    result
}

/// Make a name unique by appending a numeric suffix
pub fn make_unique_name(base_name: &str, existing_names: &[&str]) -> String {
    if base_name.is_empty() {
        // Handle empty content case
        let mut n = 1;
        loop {
            let name = format!("empty_prompt_{}", n);
            if !existing_names.contains(&name.as_str()) {
                return name;
            }
            n += 1;
        }
    }

    if !existing_names.contains(&base_name) {
        return base_name.to_string();
    }

    let mut suffix = 1;
    loop {
        let name = format!("{}_{}", base_name, suffix);
        if !existing_names.contains(&name.as_str()) {
            return name;
        }
        suffix += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_name_basic() {
        assert_eq!(
            generate_name_from_content("Given the following number you must"),
            "given_the_following"
        );
    }

    #[test]
    fn test_generate_name_with_special_chars() {
        assert_eq!(
            generate_name_from_content("Hello, World! How are you?"),
            "hello_world_how_ar"
        );
    }

    #[test]
    fn test_generate_name_empty() {
        assert_eq!(generate_name_from_content(""), "");
        assert_eq!(generate_name_from_content("   "), "");
    }

    #[test]
    fn test_generate_name_short() {
        assert_eq!(generate_name_from_content("Hi"), "hi");
    }

    #[test]
    fn test_make_unique_name() {
        let existing = vec!["test", "test_1", "test_2"];
        assert_eq!(make_unique_name("test", &existing), "test_3");
    }

    #[test]
    fn test_make_unique_name_no_conflict() {
        let existing = vec!["other", "names"];
        assert_eq!(make_unique_name("test", &existing), "test");
    }

    #[test]
    fn test_make_unique_name_empty() {
        let existing = vec!["empty_prompt_1"];
        assert_eq!(make_unique_name("", &existing), "empty_prompt_2");
    }

    #[test]
    fn test_prompt_tags() {
        let mut prompt = Prompt::new();
        
        prompt.add_tag("coding");
        assert!(prompt.has_tag("coding"));
        
        prompt.add_tag("coding"); // duplicate
        assert_eq!(prompt.tags.len(), 1);
        
        assert!(prompt.remove_tag("coding"));
        assert!(!prompt.has_tag("coding"));
        
        assert!(!prompt.remove_tag("nonexistent"));
    }
}
