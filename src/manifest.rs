//! Manifest management for tracking human-generated content

use crate::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

/// Represents a human-flagged commit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumanCommit {
    pub hash: String,
    pub timestamp: DateTime<Utc>,
    pub message: String,
}

/// Represents a human-flagged file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumanFile {
    pub path: String,
    pub timestamp: DateTime<Utc>,
    pub commit_hash: Option<String>,
}

/// Main manifest structure
#[derive(Debug, Serialize, Deserialize)]
pub struct HumanManifest {
    pub version: String,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub commits: Vec<HumanCommit>,
    pub files: Vec<HumanFile>,
}

impl Default for HumanManifest {
    fn default() -> Self {
        let now = Utc::now();
        HumanManifest {
            version: "1.0".to_string(),
            created: now,
            updated: now,
            commits: Vec::new(),
            files: Vec::new(),
        }
    }
}

/// Simplified structure for returning human content
#[derive(Debug)]
pub struct HumanContent {
    pub commits: Vec<HumanCommit>,
    pub files: Vec<String>,
}

/// Manages the human-generation manifest
pub struct ManifestManager {
    manifest_path: PathBuf,
    gith_dir: PathBuf,
}

impl ManifestManager {
    /// Create a new ManifestManager instance
    pub fn new() -> Result<Self> {
        let current_dir = std::env::current_dir()?;
        let gith_dir = current_dir.join(".gith");
        let manifest_path = gith_dir.join("human_manifest.json");

        Ok(ManifestManager {
            manifest_path,
            gith_dir,
        })
    }

    /// Initialize gith in the current repository
    pub fn init(&self) -> Result<()> {
        // Create .gith directory if it doesn't exist
        if !self.gith_dir.exists() {
            fs::create_dir_all(&self.gith_dir)?;
        }

        // Create manifest file if it doesn't exist
        if !self.manifest_path.exists() {
            let manifest = HumanManifest::default();
            self.save_manifest(&manifest)?;
        }

        // Create .gitignore entry for .gith if needed
        self.update_gitignore()?;

        Ok(())
    }

    /// Add a human-flagged commit to the manifest
    pub fn add_human_commit(&self, commit_hash: &str) -> Result<()> {
        let mut manifest = self.load_manifest()?;

        // Check if commit already exists
        if manifest.commits.iter().any(|c| c.hash == commit_hash) {
            return Ok(());
        }

        let human_commit = HumanCommit {
            hash: commit_hash.to_string(),
            timestamp: Utc::now(),
            message: String::new(), // Will be populated from git if needed
        };

        manifest.commits.push(human_commit);
        manifest.updated = Utc::now();

        self.save_manifest(&manifest)?;
        Ok(())
    }

    /// Add a human-flagged file to the manifest
    pub fn add_human_file(&self, file_path: &str, commit_hash: Option<String>) -> Result<()> {
        let mut manifest = self.load_manifest()?;

        // Remove existing entry for this file
        manifest.files.retain(|f| f.path != file_path);

        let human_file = HumanFile {
            path: file_path.to_string(),
            timestamp: Utc::now(),
            commit_hash,
        };

        manifest.files.push(human_file);
        manifest.updated = Utc::now();

        self.save_manifest(&manifest)?;
        Ok(())
    }

    /// Get all human-flagged content
    pub fn get_human_content(&self) -> Result<HumanContent> {
        let manifest = self.load_manifest().unwrap_or_default();

        let files: Vec<String> = manifest.files.iter().map(|f| f.path.clone()).collect();

        Ok(HumanContent {
            commits: manifest.commits,
            files,
        })
    }

    /// Remove a commit from the manifest
    pub fn remove_human_commit(&self, commit_hash: &str) -> Result<()> {
        let mut manifest = self.load_manifest()?;
        manifest.commits.retain(|c| c.hash != commit_hash);
        manifest.updated = Utc::now();
        self.save_manifest(&manifest)?;
        Ok(())
    }

    /// Remove a file from the manifest
    pub fn remove_human_file(&self, file_path: &str) -> Result<()> {
        let mut manifest = self.load_manifest()?;
        manifest.files.retain(|f| f.path != file_path);
        manifest.updated = Utc::now();
        self.save_manifest(&manifest)?;
        Ok(())
    }

    /// Check if a commit is marked as human-generated
    pub fn is_human_commit(&self, commit_hash: &str) -> Result<bool> {
        let manifest = self.load_manifest().unwrap_or_default();
        Ok(manifest.commits.iter().any(|c| c.hash == commit_hash))
    }

    /// Check if a file is marked as human-generated
    pub fn is_human_file(&self, file_path: &str) -> Result<bool> {
        let manifest = self.load_manifest().unwrap_or_default();
        Ok(manifest.files.iter().any(|f| f.path == file_path))
    }

    /// Load the manifest from disk
    fn load_manifest(&self) -> Result<HumanManifest> {
        if !self.manifest_path.exists() {
            return Ok(HumanManifest::default());
        }

        let content = fs::read_to_string(&self.manifest_path)?;
        let manifest: HumanManifest = serde_json::from_str(&content)?;
        Ok(manifest)
    }

    /// Save the manifest to disk
    fn save_manifest(&self, manifest: &HumanManifest) -> Result<()> {
        // Ensure .gith directory exists
        fs::create_dir_all(&self.gith_dir)?;

        let content = serde_json::to_string_pretty(manifest)?;
        fs::write(&self.manifest_path, content)?;
        Ok(())
    }

    /// Update .gitignore to include .gith directory
    fn update_gitignore(&self) -> Result<()> {
        let gitignore_path = Path::new(".gitignore");
        let gith_entry = ".gith/";

        if gitignore_path.exists() {
            let content = fs::read_to_string(gitignore_path)?;
            if content.contains(gith_entry) {
                return Ok(());
            }

            let new_content = if content.ends_with('\n') {
                format!("{}{}\n", content, gith_entry)
            } else {
                format!("{}\n{}\n", content, gith_entry)
            };

            fs::write(gitignore_path, new_content)?;
        } else {
            fs::write(gitignore_path, format!("{}\n", gith_entry))?;
        }

        Ok(())
    }
}
