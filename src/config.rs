//! Configuration management

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Whether safe mode is enabled (confirms before running commands)
    #[serde(default = "default_safe_mode")]
    pub safe_mode: bool,

    /// Tag colors mapping (tag name -> color name)
    #[serde(default)]
    pub tag_colors: HashMap<String, String>,

    /// Default export format ("rendered" or "raw")
    #[serde(default = "default_export_format")]
    pub default_export_format: String,
}

fn default_safe_mode() -> bool {
    true
}

fn default_export_format() -> String {
    "rendered".to_string()
}

impl Config {
    /// Create a new config with default values
    pub fn new() -> Self {
        Self {
            safe_mode: true,
            tag_colors: HashMap::new(),
            default_export_format: "rendered".to_string(),
        }
    }

    /// Load config from a file path
    pub fn load(path: &Path) -> Result<Self> {
        let contents = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read config file: {}", path.display()))?;
        
        let config: Config = serde_yaml::from_str(&contents)
            .with_context(|| "Failed to parse config file")?;
        
        Ok(config)
    }

    /// Load config from the default location, or create a default config
    pub fn load_or_default(path: &Path) -> Result<Self> {
        if path.exists() {
            Self::load(path)
        } else {
            Ok(Self::new())
        }
    }

    /// Save config to a file path
    pub fn save(&self, path: &Path) -> Result<()> {
        let contents = serde_yaml::to_string(self)
            .with_context(|| "Failed to serialize config")?;
        
        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create config directory: {}", parent.display()))?;
        }
        
        std::fs::write(path, contents)
            .with_context(|| format!("Failed to write config file: {}", path.display()))?;
        
        Ok(())
    }

    /// Get the color for a tag, or a default color if not set
    pub fn get_tag_color(&self, tag: &str) -> &str {
        self.tag_colors
            .get(tag)
            .map(|s| s.as_str())
            .unwrap_or_else(|| Self::default_color_for_tag(tag))
    }

    /// Set the color for a tag
    pub fn set_tag_color(&mut self, tag: impl Into<String>, color: impl Into<String>) {
        self.tag_colors.insert(tag.into(), color.into());
    }

    /// Get a default color based on tag name (deterministic)
    fn default_color_for_tag(tag: &str) -> &'static str {
        const COLORS: &[&str] = &["blue", "green", "yellow", "magenta", "cyan", "red"];
        let hash: usize = tag.bytes().map(|b| b as usize).sum();
        COLORS[hash % COLORS.len()]
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

/// Get the piemme configuration directory path
pub fn piemme_dir() -> Result<PathBuf> {
    // Use current directory's .piemme folder
    let current_dir = std::env::current_dir()
        .with_context(|| "Failed to get current directory")?;
    
    Ok(current_dir.join(".piemme"))
}

/// Get the path to the config file
pub fn config_path() -> Result<PathBuf> {
    Ok(piemme_dir()?.join("config.yaml"))
}

/// Get the path to the prompts directory
pub fn prompts_dir() -> Result<PathBuf> {
    Ok(piemme_dir()?.join("prompts"))
}

/// Get the path to the archive directory
pub fn archive_dir() -> Result<PathBuf> {
    Ok(piemme_dir()?.join("archive"))
}

/// Get the path to the folders directory
pub fn folders_dir() -> Result<PathBuf> {
    Ok(piemme_dir()?.join("folders"))
}

/// Get the path to the index file
pub fn index_path() -> Result<PathBuf> {
    Ok(piemme_dir()?.join(".index.json"))
}

/// Get the path to the error log file
pub fn error_log_path() -> Result<PathBuf> {
    Ok(piemme_dir()?.join("error.log"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::new();
        assert!(config.safe_mode);
        assert!(config.tag_colors.is_empty());
        assert_eq!(config.default_export_format, "rendered");
    }

    #[test]
    fn test_tag_color() {
        let mut config = Config::new();
        
        // Test default color (deterministic based on tag name)
        let color1 = config.get_tag_color("coding");
        let color2 = config.get_tag_color("coding");
        assert_eq!(color1, color2);
        
        // Test custom color
        config.set_tag_color("coding", "blue");
        assert_eq!(config.get_tag_color("coding"), "blue");
    }

    #[test]
    fn test_yaml_serialization() {
        let mut config = Config::new();
        config.set_tag_color("coding", "blue");
        config.set_tag_color("writing", "green");
        
        let yaml = serde_yaml::to_string(&config).unwrap();
        let parsed: Config = serde_yaml::from_str(&yaml).unwrap();
        
        assert_eq!(parsed.safe_mode, config.safe_mode);
        assert_eq!(parsed.get_tag_color("coding"), "blue");
        assert_eq!(parsed.get_tag_color("writing"), "green");
    }
}
