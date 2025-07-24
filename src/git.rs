//! Git operations wrapper

use crate::{GithError, Result};
use git2::{Oid, Repository, Signature, Time};
use std::process::{Command, Stdio};

/// Wrapper around Git operations
pub struct GitWrapper {
    repo: Repository,
}

/// Information about a commit
#[derive(Debug)]
pub struct CommitInfo {
    pub hash: String,
    pub subject: String,
    pub date: String,
    pub author: String,
}

impl GitWrapper {
    /// Create a new GitWrapper instance
    pub fn new() -> Result<Self> {
        let repo = Repository::open_from_env()
            .or_else(|_| Repository::discover("."))
            .map_err(|_| GithError::NotARepository)?;

        Ok(GitWrapper { repo })
    }

    /// Add all files to the staging area
    pub fn add_all(&self) -> Result<()> {
        self.run_git_command(&["add", "-A"])
    }

    /// Add specific files to the staging area
    pub fn add_files(&self, files: &[String]) -> Result<()> {
        let mut args = vec!["add"];
        for file in files {
            args.push(file);
        }
        self.run_git_command(&args)
    }

    /// Create a commit with optional human flag
    pub fn commit(&self, message: &str, human: bool, _allow_empty: bool) -> Result<String> {
        let enhanced_message = if human {
            format!("{message}\n\nHuman-Flag: true\nLicense: HUMAN-GENERATED, NO AI TRAINING")
        } else {
            message.to_string()
        };

        // Use git2 for creating the commit
        let signature = self.get_signature()?;
        let tree_id = {
            let mut index = self.repo.index()?;
            index.write()?;
            index.write_tree()?
        };
        let tree = self.repo.find_tree(tree_id)?;

        let parent_commit = match self.repo.head() {
            Ok(head) => Some(head.peel_to_commit()?),
            Err(_) => None, // First commit
        };

        let parents: Vec<&git2::Commit> = parent_commit.iter().collect();
        let commit_id = self.repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            &enhanced_message,
            &tree,
            &parents,
        )?;

        Ok(commit_id.to_string())
    }

    /// Get repository status
    pub fn status(&self) -> Result<()> {
        self.run_git_command(&["status"])
    }

    /// Forward command to git binary
    pub fn forward_to_git(&self, args: &[String]) -> Result<()> {
        let mut cmd = Command::new("git");
        cmd.args(args);
        cmd.stdout(Stdio::inherit());
        cmd.stderr(Stdio::inherit());

        let status = cmd.status()?;
        if !status.success() {
            return Err(GithError::GitError(format!(
                "Git command failed with exit code: {}",
                status.code().unwrap_or(-1)
            )));
        }

        Ok(())
    }

    /// Get information about a specific commit
    pub fn get_commit_info(&self, commit_hash: &str) -> Result<CommitInfo> {
        let oid = Oid::from_str(commit_hash)
            .map_err(|_| GithError::GitError("Invalid commit hash".to_string()))?;

        let commit = self.repo.find_commit(oid)?;

        let subject = commit.summary().unwrap_or("No subject").to_string();
        let author = commit.author();
        let author_name = author.name().unwrap_or("Unknown").to_string();
        let time = Time::new(commit.time().seconds(), 0);
        let date = format!("{}", time.seconds()); // Simplified date formatting

        Ok(CommitInfo {
            hash: commit_hash.to_string(),
            subject,
            date,
            author: author_name,
        })
    }

    /// Get all commits in the repository
    pub fn get_all_commits(&self) -> Result<Vec<String>> {
        let mut revwalk = self.repo.revwalk()?;
        revwalk.push_head()?;

        let mut commits = Vec::new();
        for oid in revwalk {
            let oid = oid?;
            commits.push(oid.to_string());
        }

        Ok(commits)
    }

    /// Check if a commit has human flag in its message
    pub fn has_human_flag(&self, commit_hash: &str) -> Result<bool> {
        let oid = Oid::from_str(commit_hash)
            .map_err(|_| GithError::GitError("Invalid commit hash".to_string()))?;

        let commit = self.repo.find_commit(oid)?;
        let message = commit.message().unwrap_or("");

        Ok(message.contains("Human-Flag: true"))
    }

    /// Run a git command using the system git binary
    fn run_git_command(&self, args: &[&str]) -> Result<()> {
        let mut cmd = Command::new("git");
        cmd.args(args);
        cmd.stdout(Stdio::inherit());
        cmd.stderr(Stdio::inherit());

        let status = cmd.status()?;
        if !status.success() {
            return Err(GithError::GitError(format!(
                "Git command failed with exit code: {}",
                status.code().unwrap_or(-1)
            )));
        }

        Ok(())
    }

    /// Get the git signature for commits
    fn get_signature(&self) -> Result<Signature> {
        let config = self.repo.config()?;
        let name = config
            .get_string("user.name")
            .map_err(|_| GithError::ConfigError("user.name not set".to_string()))?;
        let email = config
            .get_string("user.email")
            .map_err(|_| GithError::ConfigError("user.email not set".to_string()))?;

        Ok(Signature::now(&name, &email)?)
    }
}
