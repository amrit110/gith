//! Command-line interface for gith

use clap::{Parser, Subcommand};

/// Gith - A friendly Git wrapper for marking human-generated content
#[derive(Parser)]
#[command(name = "gith")]
#[command(about = "A friendly Git wrapper for marking human-generated content")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a git repository (mirrors git init)
    Init {
        /// Directory to initialize
        directory: Option<String>,
        /// Create a bare repository
        #[arg(long)]
        bare: bool,
        /// Set the name of the initial branch
        #[arg(short = 'b', long)]
        initial_branch: Option<String>,
        /// Create a shared repository
        #[arg(long)]
        shared: Option<String>,
        /// Specify template directory
        #[arg(long)]
        template: Option<String>,
        /// Set separate git directory
        #[arg(long)]
        separate_git_dir: Option<String>,
        /// Be quiet
        #[arg(short, long)]
        quiet: bool,
        /// Also initialize gith tracking in the repository
        #[arg(long)]
        gith: bool,
    },

    /// Initialize gith tracking in an existing repository
    InitTracking,

    /// Add files to the staging area (mirrors git add)
    Add {
        /// Files to add
        files: Vec<String>,
        /// Add all files
        #[arg(short = 'A', long)]
        all: bool,
    },

    /// Create a commit with human-generation flag
    Commit {
        /// Commit message
        #[arg(short, long)]
        message: Option<String>,

        /// Mark this commit as human-generated
        #[arg(long)]
        human: bool,

        /// Add all modified files before committing
        #[arg(short = 'a', long)]
        all: bool,

        /// Allow empty commits
        #[arg(long)]
        allow_empty: bool,
    },

    /// List all human-flagged commits and files
    ListHuman {
        /// Show only commits
        #[arg(long)]
        commits_only: bool,

        /// Show only files
        #[arg(long)]
        files_only: bool,
    },

    /// Show the status of the repository
    Status,

    /// Forward any other git command
    Git {
        /// Git command and arguments
        args: Vec<String>,
    },
}
