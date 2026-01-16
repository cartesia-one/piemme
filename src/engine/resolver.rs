//! Prompt content resolution (references and commands)

use std::collections::HashSet;
use std::path::Path;

use super::commands::{execute_command_safe, find_commands, has_commands};
use super::references::{find_references, find_file_references, has_references, has_file_references};

/// Options for resolving prompt content
pub struct ResolveOptions {
    /// Maximum depth for recursive reference resolution
    pub max_depth: usize,
    /// Whether to execute commands
    pub execute_commands: bool,
}

impl Default for ResolveOptions {
    fn default() -> Self {
        Self {
            max_depth: 10,
            execute_commands: true,
        }
    }
}

/// Result of resolving a prompt
#[derive(Debug, Clone)]
pub struct ResolveResult {
    /// The resolved content
    pub content: String,
    /// Commands that would be executed
    pub commands: Vec<String>,
    /// References that were resolved
    pub references: Vec<String>,
    /// File references that were resolved
    pub file_references: Vec<String>,
    /// Whether there were circular references
    pub had_circular_refs: bool,
    /// Whether max depth was exceeded
    pub max_depth_exceeded: bool,
}

/// Resolve a prompt's content, replacing references and optionally executing commands
pub fn resolve_prompt<F>(content: &str, get_content: F, execute_cmds: bool) -> ResolveResult
where
    F: Fn(&str) -> Option<String>,
{
    // Use current working directory as base for file references
    let base_dir = std::env::current_dir().unwrap_or_default();
    resolve_prompt_with_base(content, get_content, execute_cmds, &base_dir)
}

/// Resolve a prompt's content with a specific base directory for file references
pub fn resolve_prompt_with_base<F>(content: &str, get_content: F, execute_cmds: bool, base_dir: &Path) -> ResolveResult
where
    F: Fn(&str) -> Option<String>,
{
    let mut visited = HashSet::new();
    let mut result = ResolveResult {
        content: content.to_string(),
        commands: Vec::new(),
        references: Vec::new(),
        file_references: Vec::new(),
        had_circular_refs: false,
        max_depth_exceeded: false,
    };

    // Resolve file references first (they don't have circular dependency issues)
    result.content = resolve_file_references(&result.content, base_dir, &mut result.file_references);

    // Resolve prompt references
    result.content = resolve_references_recursive(
        &result.content,
        &get_content,
        &mut visited,
        0,
        10,
        &mut result.references,
        &mut result.had_circular_refs,
        &mut result.max_depth_exceeded,
    );

    // Find commands
    let commands = find_commands(&result.content);
    result.commands = commands.iter().map(|c| c.command.clone()).collect();

    // Execute commands if requested
    if execute_cmds {
        result.content = resolve_commands_in_content(&result.content);
    }

    result
}

/// Resolve file references in content by reading file contents
fn resolve_file_references(content: &str, base_dir: &Path, resolved_files: &mut Vec<String>) -> String {
    if !has_file_references(content) {
        return content.to_string();
    }

    let file_refs = find_file_references(content);
    let mut result = content.to_string();

    // Process file references in reverse order to maintain correct positions
    for file_ref in file_refs.into_iter().rev() {
        let file_path = base_dir.join(&file_ref.path);
        
        if file_path.exists() && file_path.is_file() {
            match std::fs::read_to_string(&file_path) {
                Ok(file_content) => {
                    resolved_files.push(file_ref.path.clone());
                    result = result.replace(&file_ref.full_match, &file_content);
                }
                Err(e) => {
                    // If we can't read the file, replace with an error comment
                    let error_msg = format!("<!-- [FILE READ ERROR: {} - {}] -->", file_ref.path, e);
                    result = result.replace(&file_ref.full_match, &error_msg);
                }
            }
        } else {
            // File doesn't exist, replace with an error comment
            let error_msg = format!("<!-- [FILE NOT FOUND: {}] -->", file_ref.path);
            result = result.replace(&file_ref.full_match, &error_msg);
        }
    }

    result
}

/// Recursively resolve references in content
fn resolve_references_recursive<F>(
    content: &str,
    get_content: &F,
    visited: &mut HashSet<String>,
    depth: usize,
    max_depth: usize,
    resolved_refs: &mut Vec<String>,
    had_circular: &mut bool,
    max_exceeded: &mut bool,
) -> String
where
    F: Fn(&str) -> Option<String>,
{
    if depth >= max_depth {
        *max_exceeded = true;
        return content.to_string();
    }

    if !has_references(content) {
        return content.to_string();
    }

    let refs = find_references(content);
    let mut result = content.to_string();

    // Process references in reverse order to maintain correct positions
    for reference in refs.into_iter().rev() {
        if visited.contains(&reference.name) {
            // Circular reference detected
            *had_circular = true;
            let warning = format!("<!-- [CIRCULAR REFERENCE DETECTED: {}] -->", reference.name);
            result = result.replace(&reference.full_match, &warning);
            continue;
        }

        if let Some(ref_content) = get_content(&reference.name) {
            visited.insert(reference.name.clone());
            resolved_refs.push(reference.name.clone());

            // Recursively resolve the referenced content
            let resolved_content = resolve_references_recursive(
                &ref_content,
                get_content,
                visited,
                depth + 1,
                max_depth,
                resolved_refs,
                had_circular,
                max_exceeded,
            );

            result = result.replace(&reference.full_match, &resolved_content);
            visited.remove(&reference.name);
        }
        // If reference not found, leave it as-is (will show as invalid in highlighting)
    }

    result
}

/// Replace command placeholders with their output (public for use in command confirmation flow)
pub fn resolve_commands_in_content(content: &str) -> String {
    if !has_commands(content) {
        return content.to_string();
    }

    let commands = find_commands(content);
    let mut result = content.to_string();

    // Process in reverse order to maintain positions
    for cmd in commands.into_iter().rev() {
        let output = execute_command_safe(&cmd.command);
        result = result.replace(&cmd.full_match, &output);
    }

    result
}

/// Check if content needs resolution (has references, file references, or commands)
pub fn needs_resolution(content: &str) -> bool {
    has_references(content) || has_file_references(content) || has_commands(content)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mock_get_content(name: &str) -> Option<String> {
        match name {
            "greeting" => Some("Hello, World!".to_string()),
            "nested" => Some("Start [[greeting]] End".to_string()),
            "circular_a" => Some("A references [[circular_b]]".to_string()),
            "circular_b" => Some("B references [[circular_a]]".to_string()),
            _ => None,
        }
    }

    #[test]
    fn test_simple_reference() {
        let content = "Say [[greeting]]!";
        let result = resolve_prompt(content, mock_get_content, false);
        
        assert_eq!(result.content, "Say Hello, World!!".to_string());
        assert!(result.references.contains(&"greeting".to_string()));
    }

    #[test]
    fn test_nested_references() {
        let content = "Message: [[nested]]";
        let result = resolve_prompt(content, mock_get_content, false);
        
        assert_eq!(result.content, "Message: Start Hello, World! End");
    }

    #[test]
    fn test_circular_reference() {
        let content = "Check [[circular_a]]";
        let result = resolve_prompt(content, mock_get_content, false);
        
        assert!(result.had_circular_refs);
        assert!(result.content.contains("CIRCULAR REFERENCE DETECTED"));
    }

    #[test]
    fn test_invalid_reference() {
        let content = "Check [[nonexistent]]";
        let result = resolve_prompt(content, mock_get_content, false);
        
        // Invalid references are left as-is
        assert_eq!(result.content, "Check [[nonexistent]]");
    }

    #[test]
    fn test_needs_resolution() {
        assert!(needs_resolution("Has [[reference]]"));
        assert!(needs_resolution("Has {{command}}"));
        assert!(needs_resolution("Has [[ref]] and {{cmd}}"));
        assert!(!needs_resolution("Plain text"));
    }
}
