//! Prompt file I/O operations

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

use crate::config::{archive_dir, folders_dir, prompts_dir};
use crate::models::prompt::{generate_name_from_content, make_unique_name, Prompt, PromptFrontmatter};

/// Load a prompt from a markdown file
pub fn load_prompt(path: &Path) -> Result<Prompt> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read prompt file: {}", path.display()))?;

    let (frontmatter, body) = parse_frontmatter(&content)?;

    let name = path
        .file_stem()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();

    Ok(Prompt {
        id: frontmatter.id,
        name,
        content: body,
        tags: frontmatter.tags,
        created: frontmatter.created,
        modified: frontmatter.modified,
    })
}

/// Save a prompt to a markdown file
pub fn save_prompt(prompt: &Prompt, dir: &Path) -> Result<PathBuf> {
    let path = dir.join(format!("{}.md", prompt.name));
    
    let frontmatter = prompt.frontmatter();
    let frontmatter_str = serde_yaml::to_string(&frontmatter)
        .with_context(|| "Failed to serialize frontmatter")?;

    let file_content = format!("---\n{}---\n{}", frontmatter_str, prompt.content);

    // Ensure directory exists
    std::fs::create_dir_all(dir)
        .with_context(|| format!("Failed to create directory: {}", dir.display()))?;

    std::fs::write(&path, file_content)
        .with_context(|| format!("Failed to write prompt file: {}", path.display()))?;

    Ok(path)
}

/// Load all prompts from a directory
pub fn load_all_prompts(dir: &Path) -> Result<Vec<Prompt>> {
    let files = super::directory::list_markdown_files(dir)?;
    
    let mut prompts = Vec::new();
    for file in files {
        match load_prompt(&file) {
            Ok(prompt) => prompts.push(prompt),
            Err(e) => {
                // Log error but continue loading other prompts
                eprintln!("Warning: Failed to load prompt {}: {}", file.display(), e);
            }
        }
    }

    // Sort by name
    prompts.sort_by(|a, b| a.name.cmp(&b.name));
    
    Ok(prompts)
}

/// Delete a prompt file
pub fn delete_prompt(name: &str, dir: &Path) -> Result<()> {
    let path = dir.join(format!("{}.md", name));
    
    if path.exists() {
        std::fs::remove_file(&path)
            .with_context(|| format!("Failed to delete prompt file: {}", path.display()))?;
    }
    
    Ok(())
}

/// Move a prompt to a different directory
pub fn move_prompt(name: &str, from_dir: &Path, to_dir: &Path) -> Result<PathBuf> {
    let from_path = from_dir.join(format!("{}.md", name));
    let to_path = to_dir.join(format!("{}.md", name));

    // Ensure destination directory exists
    std::fs::create_dir_all(to_dir)
        .with_context(|| format!("Failed to create directory: {}", to_dir.display()))?;

    std::fs::rename(&from_path, &to_path)
        .with_context(|| format!("Failed to move prompt from {} to {}", from_path.display(), to_path.display()))?;

    Ok(to_path)
}

/// Rename a prompt file
pub fn rename_prompt(old_name: &str, new_name: &str, dir: &Path) -> Result<PathBuf> {
    let old_path = dir.join(format!("{}.md", old_name));
    let new_path = dir.join(format!("{}.md", new_name));

    if new_path.exists() {
        anyhow::bail!("A prompt with name '{}' already exists", new_name);
    }

    std::fs::rename(&old_path, &new_path)
        .with_context(|| format!("Failed to rename prompt from {} to {}", old_path.display(), new_path.display()))?;

    Ok(new_path)
}

/// Parse YAML frontmatter from a markdown file
fn parse_frontmatter(content: &str) -> Result<(PromptFrontmatter, String)> {
    let content = content.trim();
    
    if !content.starts_with("---") {
        anyhow::bail!("Missing YAML frontmatter");
    }

    // Find the closing ---
    let rest = &content[3..];
    let end_pos = rest.find("\n---")
        .ok_or_else(|| anyhow::anyhow!("Invalid YAML frontmatter: missing closing ---"))?;

    let frontmatter_str = &rest[..end_pos].trim();
    let body = rest[end_pos + 4..].trim_start_matches('\n').to_string();

    let frontmatter: PromptFrontmatter = serde_yaml::from_str(frontmatter_str)
        .with_context(|| "Failed to parse YAML frontmatter")?;

    Ok((frontmatter, body))
}

/// Create a new prompt with a unique name
pub fn create_new_prompt(content: &str, existing_names: &[&str]) -> Prompt {
    let base_name = generate_name_from_content(content);
    let name = make_unique_name(&base_name, existing_names);
    
    let mut prompt = Prompt::with_content(content);
    prompt.name = name;
    
    prompt
}

/// Get all prompt names across all directories (for uniqueness checking)
pub fn get_all_prompt_names() -> Result<Vec<String>> {
    let mut names = Vec::new();

    // Main prompts directory
    for prompt in load_all_prompts(&prompts_dir()?)? {
        names.push(prompt.name);
    }

    // Archive directory
    for prompt in load_all_prompts(&archive_dir()?)? {
        names.push(prompt.name);
    }

    // Folders
    let folders_path = folders_dir()?;
    if folders_path.exists() {
        for entry in std::fs::read_dir(&folders_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                for prompt in load_all_prompts(&path)? {
                    names.push(prompt.name);
                }
            }
        }
    }

    Ok(names)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use uuid::Uuid;

    #[test]
    fn test_parse_frontmatter() {
        let content = r#"---
id: "550e8400-e29b-41d4-a716-446655440000"
tags: ["coding", "python"]
created: "2026-01-15T10:30:00Z"
modified: "2026-01-15T14:22:00Z"
---
This is the prompt content.
Multiple lines here.
"#;

        let (frontmatter, body) = parse_frontmatter(content).unwrap();
        assert_eq!(frontmatter.id, Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap());
        assert_eq!(frontmatter.tags, vec!["coding", "python"]);
        assert!(body.contains("This is the prompt content"));
    }

    #[test]
    fn test_save_and_load_prompt() {
        let dir = tempdir().unwrap();
        let dir_path = dir.path();

        let mut prompt = Prompt::with_content("Test prompt content");
        prompt.name = "test_prompt".to_string();
        prompt.add_tag("testing");

        save_prompt(&prompt, dir_path).unwrap();

        let loaded = load_prompt(&dir_path.join("test_prompt.md")).unwrap();
        assert_eq!(loaded.name, "test_prompt");
        assert_eq!(loaded.content, "Test prompt content");
        assert!(loaded.tags.contains(&"testing".to_string()));
    }

    #[test]
    fn test_create_new_prompt() {
        let existing = vec!["test_content_here"];
        let prompt = create_new_prompt("Test content here", &existing);
        assert_eq!(prompt.name, "test_content_here_1");
    }
}
