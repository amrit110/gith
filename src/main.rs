//! Main entry point for the gith CLI application

use clap::Parser;
use gith::{
    cli::{Cli, Commands},
    git::GitWrapper,
    manifest::ManifestManager,
    Result,
};
use std::env;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    // If no subcommand is provided, forward to git
    if args.len() > 1 && !is_gith_command(&args[1]) {
        let git_wrapper = GitWrapper::new()?;
        return git_wrapper.forward_to_git(&args[1..]);
    }

    let cli = Cli::parse();

    match cli.command {
        Commands::Init {
            directory,
            bare,
            initial_branch,
            shared,
            template,
            separate_git_dir,
            quiet,
        } => {
            // For git init, we don't need GitWrapper since there's no repo yet
            // Forward to git init with all the same arguments
            let mut git_args = vec!["init".to_string()];

            if let Some(dir) = directory {
                git_args.push(dir);
            }
            if bare {
                git_args.push("--bare".to_string());
            }
            if let Some(branch) = initial_branch {
                git_args.push("-b".to_string());
                git_args.push(branch);
            }
            if let Some(shared_val) = shared {
                git_args.push("--shared".to_string());
                git_args.push(shared_val);
            }
            if let Some(template_dir) = template {
                git_args.push("--template".to_string());
                git_args.push(template_dir);
            }
            if let Some(git_dir) = separate_git_dir {
                git_args.push("--separate-git-dir".to_string());
                git_args.push(git_dir);
            }
            if quiet {
                git_args.push("--quiet".to_string());
            }

            // Use system git command directly for init
            use std::process::{Command, Stdio};
            let mut cmd = Command::new("git");
            cmd.args(&git_args);
            cmd.stdout(Stdio::inherit());
            cmd.stderr(Stdio::inherit());

            let status = cmd.status()?;
            if !status.success() {
                return Err(gith::GithError::GitError(format!(
                    "Git init failed with exit code: {}",
                    status.code().unwrap_or(-1)
                )));
            }
        }

        Commands::GithInit => {
            let manifest_manager = ManifestManager::new()?;
            println!("Initializing gith tracking in current repository...");
            manifest_manager.init()?;
            println!("Gith tracking initialized successfully!");
        }

        // For all other commands, we need a git repository
        other_command => {
            let git_wrapper = GitWrapper::new()?;
            let manifest_manager = ManifestManager::new()?;

            match other_command {
                Commands::Add { files, all } => {
                    if all {
                        git_wrapper.add_all()?;
                    } else {
                        git_wrapper.add_files(&files)?;
                    }
                    println!("Files added to staging area");
                }

                Commands::Commit {
                    message,
                    human,
                    all,
                    allow_empty,
                } => {
                    let commit_msg = match message {
                        Some(msg) => msg,
                        None => {
                            return Err(gith::GithError::Other(
                                "Commit message is required. Use -m or --message".to_string(),
                            ));
                        }
                    };

                    if all {
                        git_wrapper.add_all()?;
                    }

                    let commit_hash = git_wrapper.commit(&commit_msg, human, allow_empty)?;

                    if human {
                        manifest_manager.add_human_commit(&commit_hash)?;
                        println!("Human-flagged commit created: {commit_hash}");
                    } else {
                        println!("Commit created: {commit_hash}");
                    }
                }

                Commands::ListHuman {
                    commits_only,
                    files_only,
                } => {
                    let human_data = manifest_manager.get_human_content()?;

                    if !commits_only && !human_data.files.is_empty() {
                        println!("Human-flagged files:");
                        for file in &human_data.files {
                            println!("  {file}");
                        }
                    }

                    if !files_only && !human_data.commits.is_empty() {
                        println!("Human-flagged commits:");
                        for commit in &human_data.commits {
                            let commit_info = git_wrapper.get_commit_info(&commit.hash)?;
                            println!(
                                "  {} - {} ({})",
                                &commit.hash[..8],
                                commit_info.subject,
                                commit_info.date
                            );
                        }
                    }
                }

                Commands::Status => {
                    git_wrapper.status()?;
                }

                Commands::Git { args } => {
                    git_wrapper.forward_to_git(&args)?;
                }

                // These cases are handled above
                Commands::Init { .. } | Commands::GithInit => unreachable!(),
            }
        }
    }

    Ok(())
}

/// Check if the given argument is a gith-specific command
fn is_gith_command(arg: &str) -> bool {
    matches!(
        arg,
        "init"
            | "gith-init"
            | "add"
            | "commit"
            | "list-human"
            | "status"
            | "git"
            | "--help"
            | "-h"
            | "--version"
            | "-V"
    )
}
