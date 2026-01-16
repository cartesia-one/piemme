//! Directory management

use anyhow::{Context, Result};
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use crate::config::{archive_dir, folders_dir, piemme_dir, prompts_dir};

/// Provide user-friendly error messages for I/O errors
fn format_io_error(err: &std::io::Error, path: &Path, operation: &str) -> String {
    match err.kind() {
        ErrorKind::PermissionDenied => {
            format!(
                "Permission denied: Cannot {} '{}'. Check directory permissions.",
                operation,
                path.display()
            )
        }
        ErrorKind::NotFound => {
            format!("Directory not found: '{}'", path.display())
        }
        ErrorKind::AlreadyExists => {
            format!("Directory already exists: '{}'", path.display())
        }
        _ => {
            format!(
                "Failed to {} '{}': {}",
                operation,
                path.display(),
                err
            )
        }
    }
}

/// Ensure all required directories exist
pub fn ensure_directories() -> Result<()> {
    let dirs = [
        piemme_dir()?,
        prompts_dir()?,
        archive_dir()?,
        folders_dir()?,
    ];

    for dir in dirs {
        std::fs::create_dir_all(&dir)
            .map_err(|e| anyhow::anyhow!(format_io_error(&e, &dir, "create")))?;
    }

    Ok(())
}

/// List all user-created folders
pub fn list_folders() -> Result<Vec<String>> {
    let folders_path = folders_dir()?;
    
    if !folders_path.exists() {
        return Ok(Vec::new());
    }

    let mut folders = Vec::new();

    for entry in std::fs::read_dir(&folders_path)
        .map_err(|e| anyhow::anyhow!(format_io_error(&e, &folders_path, "read")))?
    {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_dir() {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                folders.push(name.to_string());
            }
        }
    }

    folders.sort();
    Ok(folders)
}

/// Create a new folder
pub fn create_folder(name: &str) -> Result<PathBuf> {
    let folder_path = folders_dir()?.join(name);
    
    std::fs::create_dir_all(&folder_path)
        .map_err(|e| anyhow::anyhow!(format_io_error(&e, &folder_path, "create")))?;
    
    Ok(folder_path)
}

/// Check if a directory is empty
pub fn is_directory_empty(path: &Path) -> Result<bool> {
    if !path.exists() {
        return Ok(true);
    }
    
    let mut entries = std::fs::read_dir(path)
        .map_err(|e| anyhow::anyhow!(format_io_error(&e, path, "read")))?;
    
    Ok(entries.next().is_none())
}

/// Get all markdown files in a directory
pub fn list_markdown_files(dir: &Path) -> Result<Vec<PathBuf>> {
    if !dir.exists() {
        return Ok(Vec::new());
    }

    let mut files = Vec::new();

    for entry in std::fs::read_dir(dir)
        .map_err(|e| anyhow::anyhow!(format_io_error(&e, dir, "read")))?
    {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "md" {
                    files.push(path);
                }
            }
        }
    }

    files.sort();
    Ok(files)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_list_markdown_files() {
        let dir = tempdir().unwrap();
        let dir_path = dir.path();

        // Create some test files
        fs::write(dir_path.join("test1.md"), "content").unwrap();
        fs::write(dir_path.join("test2.md"), "content").unwrap();
        fs::write(dir_path.join("test3.txt"), "content").unwrap();

        let files = list_markdown_files(dir_path).unwrap();
        assert_eq!(files.len(), 2);
    }

    #[test]
    fn test_is_directory_empty() {
        let dir = tempdir().unwrap();
        let dir_path = dir.path();

        assert!(is_directory_empty(dir_path).unwrap());

        fs::write(dir_path.join("file.txt"), "content").unwrap();
        assert!(!is_directory_empty(dir_path).unwrap());
    }
}
