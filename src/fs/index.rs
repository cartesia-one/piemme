//! Search index management

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use uuid::Uuid;

/// The search index structure
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Index {
    /// Version of the index format
    pub version: u32,
    /// When the index was last updated
    pub updated: Option<DateTime<Utc>>,
    /// Index entries by prompt name
    pub entries: HashMap<String, IndexEntry>,
}

/// An entry in the search index
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexEntry {
    /// Prompt ID
    pub id: Uuid,
    /// Prompt name
    pub name: String,
    /// First line of content (for preview)
    pub preview: String,
    /// Full content (for searching)
    pub content: String,
    /// Tags
    pub tags: Vec<String>,
    /// Location: "prompts", "archive", or folder path
    pub location: String,
    /// Last modified timestamp
    pub modified: DateTime<Utc>,
}

impl Index {
    /// Create a new empty index
    pub fn new() -> Self {
        Self {
            version: 1,
            updated: Some(Utc::now()),
            entries: HashMap::new(),
        }
    }

    /// Load index from file
    pub fn load(path: &Path) -> Result<Self> {
        let contents = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read index file: {}", path.display()))?;
        
        let index: Index = serde_json::from_str(&contents)
            .with_context(|| "Failed to parse index file")?;
        
        Ok(index)
    }

    /// Load index from file, or create new if doesn't exist
    pub fn load_or_new(path: &Path) -> Result<Self> {
        if path.exists() {
            Self::load(path)
        } else {
            Ok(Self::new())
        }
    }

    /// Save index to file
    pub fn save(&self, path: &Path) -> Result<()> {
        let contents = serde_json::to_string_pretty(self)
            .with_context(|| "Failed to serialize index")?;
        
        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        std::fs::write(path, contents)
            .with_context(|| format!("Failed to write index file: {}", path.display()))?;
        
        Ok(())
    }

    /// Add or update an entry
    pub fn upsert(&mut self, entry: IndexEntry) {
        self.entries.insert(entry.name.clone(), entry);
        self.updated = Some(Utc::now());
    }

    /// Remove an entry by name
    pub fn remove(&mut self, name: &str) -> Option<IndexEntry> {
        let entry = self.entries.remove(name);
        if entry.is_some() {
            self.updated = Some(Utc::now());
        }
        entry
    }

    /// Get an entry by name
    pub fn get(&self, name: &str) -> Option<&IndexEntry> {
        self.entries.get(name)
    }

    /// Get all entries
    pub fn all_entries(&self) -> impl Iterator<Item = &IndexEntry> {
        self.entries.values()
    }

    /// Get all prompt names
    pub fn all_names(&self) -> impl Iterator<Item = &str> {
        self.entries.keys().map(|s| s.as_str())
    }

    /// Get all unique tags
    pub fn all_tags(&self) -> Vec<String> {
        let mut tags: Vec<String> = self.entries
            .values()
            .flat_map(|e| e.tags.iter().cloned())
            .collect();
        
        tags.sort();
        tags.dedup();
        tags
    }

    /// Search entries by query (simple substring match)
    pub fn search(&self, query: &str) -> Vec<&IndexEntry> {
        let query_lower = query.to_lowercase();
        
        self.entries
            .values()
            .filter(|entry| {
                entry.name.to_lowercase().contains(&query_lower)
                    || entry.content.to_lowercase().contains(&query_lower)
            })
            .collect()
    }

    /// Filter entries by tag
    pub fn filter_by_tag(&self, tag: &str) -> Vec<&IndexEntry> {
        self.entries
            .values()
            .filter(|entry| entry.tags.contains(&tag.to_string()))
            .collect()
    }

    /// Filter entries by location
    pub fn filter_by_location(&self, location: &str) -> Vec<&IndexEntry> {
        self.entries
            .values()
            .filter(|entry| entry.location == location)
            .collect()
    }
}

impl IndexEntry {
    /// Create a new index entry from a prompt
    pub fn from_prompt(prompt: &crate::models::Prompt, location: &str) -> Self {
        Self {
            id: prompt.id,
            name: prompt.name.clone(),
            preview: prompt.first_line().to_string(),
            content: prompt.content.clone(),
            tags: prompt.tags.clone(),
            location: location.to_string(),
            modified: prompt.modified,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_index_operations() {
        let mut index = Index::new();
        
        let entry = IndexEntry {
            id: Uuid::new_v4(),
            name: "test_prompt".to_string(),
            preview: "This is a test".to_string(),
            content: "This is a test prompt content".to_string(),
            tags: vec!["coding".to_string(), "test".to_string()],
            location: "prompts".to_string(),
            modified: Utc::now(),
        };
        
        index.upsert(entry.clone());
        assert_eq!(index.entries.len(), 1);
        
        let retrieved = index.get("test_prompt").unwrap();
        assert_eq!(retrieved.name, "test_prompt");
        
        let removed = index.remove("test_prompt");
        assert!(removed.is_some());
        assert!(index.entries.is_empty());
    }

    #[test]
    fn test_index_search() {
        let mut index = Index::new();
        
        index.upsert(IndexEntry {
            id: Uuid::new_v4(),
            name: "coding_tips".to_string(),
            preview: "Tips for coding".to_string(),
            content: "Tips for coding in Python".to_string(),
            tags: vec!["coding".to_string()],
            location: "prompts".to_string(),
            modified: Utc::now(),
        });
        
        index.upsert(IndexEntry {
            id: Uuid::new_v4(),
            name: "writing_guide".to_string(),
            preview: "Writing guide".to_string(),
            content: "A guide for technical writing".to_string(),
            tags: vec!["writing".to_string()],
            location: "prompts".to_string(),
            modified: Utc::now(),
        });
        
        let results = index.search("coding");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "coding_tips");
        
        let results = index.search("guide");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "writing_guide");
    }

    #[test]
    fn test_index_save_load() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("index.json");
        
        let mut index = Index::new();
        index.upsert(IndexEntry {
            id: Uuid::new_v4(),
            name: "test".to_string(),
            preview: "Test".to_string(),
            content: "Test content".to_string(),
            tags: vec![],
            location: "prompts".to_string(),
            modified: Utc::now(),
        });
        
        index.save(&path).unwrap();
        
        let loaded = Index::load(&path).unwrap();
        assert_eq!(loaded.entries.len(), 1);
        assert!(loaded.get("test").is_some());
    }
}
