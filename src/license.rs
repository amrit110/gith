//! License management for human-generated content

use crate::Result;
use std::fs;
use std::path::Path;

/// The standard human-generated content license text
pub const HUMAN_GENERATED_LICENSE: &str = r#"HUMAN-GENERATED CONTENT LICENSE

This content is entirely human-generated and is provided under the following terms:

1. HUMAN CREATION: This content was created entirely by human authors without the
   assistance of artificial intelligence, machine learning systems, or automated
   content generation tools.

2. AI TRAINING RESTRICTION: This content may NOT be used for training, fine-tuning,
   or improving artificial intelligence systems, machine learning models, or any
   automated content generation systems without explicit written permission from
   the copyright holder.

3. VERIFICATION: The human-generated nature of this content is tracked and verified
   through the gith system (https://github.com/amrit110/gith).

4. REDISTRIBUTION: Redistribution and use of this content is permitted provided
   that:
   - This license notice is preserved
   - The human-generated nature is clearly indicated
   - The AI training restriction is maintained

5. NO WARRANTY: This content is provided "as is" without warranty of any kind.

Copyright (c) 2025. All rights reserved.
"#;

/// License manager for handling human-generated content licenses
pub struct LicenseManager;

impl Default for LicenseManager {
    fn default() -> Self {
        Self::new()
    }
}

impl LicenseManager {
    /// Create a new LicenseManager instance
    pub fn new() -> Self {
        LicenseManager
    }

    /// Create or update the LICENSE-HUMAN file in the repository root
    pub fn create_license_file(&self) -> Result<()> {
        let license_path = Path::new("LICENSE-HUMAN");
        fs::write(license_path, HUMAN_GENERATED_LICENSE)?;
        Ok(())
    }

    /// Get the human-generated license text
    pub fn get_license_text(&self) -> &'static str {
        HUMAN_GENERATED_LICENSE
    }

    /// Get the license trailer for commit messages
    pub fn get_commit_trailer(&self) -> String {
        "License: HUMAN-GENERATED, NO AI TRAINING".to_string()
    }

    /// Get a short license notice for file headers
    pub fn get_file_header(&self, comment_style: CommentStyle) -> String {
        let notice = "This file contains human-generated content. See LICENSE-HUMAN for terms.";

        match comment_style {
            CommentStyle::SlashSlash => format!("// {notice}\n"),
            CommentStyle::Hash => format!("# {notice}\n"),
            CommentStyle::SlashStar => format!("/* {notice} */\n"),
        }
    }

    /// Check if a repository has the human license file
    pub fn has_license_file(&self) -> bool {
        Path::new("LICENSE-HUMAN").exists()
    }

    /// Validate that the license file has the correct content
    pub fn validate_license_file(&self) -> Result<bool> {
        if !self.has_license_file() {
            return Ok(false);
        }

        let content = fs::read_to_string("LICENSE-HUMAN")?;
        Ok(content.trim() == HUMAN_GENERATED_LICENSE.trim())
    }
}

/// Comment styles for different file types
#[derive(Debug, Clone, Copy)]
pub enum CommentStyle {
    /// // style comments (C, C++, Rust, JavaScript, etc.)
    SlashSlash,
    /// # style comments (Python, Ruby, Shell, etc.)
    Hash,
    /// /* */ style comments (CSS, C, etc.)
    SlashStar,
}

impl CommentStyle {
    /// Determine comment style from file extension
    pub fn from_extension(extension: &str) -> Option<Self> {
        match extension.to_lowercase().as_str() {
            "rs" | "c" | "cpp" | "cc" | "cxx" | "h" | "hpp" | "js" | "ts" | "jsx" | "tsx"
            | "java" | "cs" | "go" => Some(CommentStyle::SlashSlash),
            "py" | "rb" | "sh" | "bash" | "zsh" | "pl" | "r" | "yaml" | "yml" | "toml" => {
                Some(CommentStyle::Hash)
            }
            "css" | "scss" | "less" => Some(CommentStyle::SlashStar),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comment_style_from_extension() {
        assert!(matches!(
            CommentStyle::from_extension("rs"),
            Some(CommentStyle::SlashSlash)
        ));
        assert!(matches!(
            CommentStyle::from_extension("py"),
            Some(CommentStyle::Hash)
        ));
        assert!(matches!(
            CommentStyle::from_extension("css"),
            Some(CommentStyle::SlashStar)
        ));
        assert!(CommentStyle::from_extension("unknown").is_none());
    }

    #[test]
    fn test_file_header_formats() {
        let manager = LicenseManager::new();

        let slash_header = manager.get_file_header(CommentStyle::SlashSlash);
        assert!(slash_header.starts_with("//"));

        let hash_header = manager.get_file_header(CommentStyle::Hash);
        assert!(hash_header.starts_with("#"));

        let star_header = manager.get_file_header(CommentStyle::SlashStar);
        assert!(star_header.starts_with("/*"));
    }
}
