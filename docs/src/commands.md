# Command Reference

This page provides detailed documentation for all `gith` commands.

## Repository Initialization

### `gith init [OPTIONS] [DIRECTORY]`

Initialize a new Git repository (identical to `git init`).

**Options:**
- `--bare` - Create a bare repository
- `-b, --initial-branch <BRANCH>` - Set initial branch name
- `--shared[=<PERMISSIONS>]` - Create shared repository
- `--gith` - Also initialize gith tracking

**Examples:**

```bash
# Standard Git initialization
gith init my-project

# Initialize with gith tracking enabled
gith init --gith my-project

# Create with custom initial branch
gith init -b main --gith my-project
```

### `gith init-tracking`

Enable gith tracking in an existing Git repository.

**What it creates:**
- `.gith/` directory for metadata storage
- `human_manifest.json` for tracking human-generated content
- Updates `.gitignore` to exclude gith metadata from version control
- `LICENSE-HUMAN` file with human-generated content license

**Requirements:**
- Must be run inside an existing Git repository
- Repository must have at least one commit

**Example:**

```bash
cd existing-git-repo
gith init-tracking
```

## Committing Changes

### `gith commit [OPTIONS] -m <MESSAGE>`

Create commits with optional human certification.

**Key Options:**
- `--human` - Mark commit as entirely human-generated
- `-a, --all` - Stage all modified files before committing
- `--allow-empty` - Allow empty commits
- `-m, --message <MSG>` - Commit message

**Standard Git Options:**
All standard `git commit` options are supported and forwarded to Git.

**Examples:**

```bash
# Regular commit (forwards to git)
gith commit -m "Fix bug in user authentication"

# Human-certified commit
gith commit --human -m "Implement custom sorting algorithm"

# Stage all changes and commit as human-generated
gith commit -a --human -m "Refactor data processing module"
```

**Human-Certified Commit Format:**

When using `--human`, gith automatically adds these trailers to your commit message:

```
Human-Flag: true
License: HUMAN-GENERATED, NO AI TRAINING
```

## Content Tracking

### `gith list-human [OPTIONS]`

View all human-flagged commits and files in your repository.

**Options:**
- `--commits-only` - Display only human-certified commits
- `--files-only` - Display only files from human-certified commits

**Example Output:**

```bash
$ gith list-human

Human-Generated Commits:
========================
abc123f - 2024-01-15 14:30:25 - Implement custom sorting algorithm
def456a - 2024-01-14 09:15:10 - Add user authentication system
ghi789b - 2024-01-13 16:45:33 - Create initial project structure

Human-Generated Files:
======================
src/sort.rs (from commit abc123f)
src/auth.rs (from commit def456a)
src/main.rs (from commit ghi789b)
README.md (from commit ghi789b)
```

**Filtering Output:**

```bash
# Show only commits
gith list-human --commits-only

# Show only files
gith list-human --files-only
```

## Standard Git Commands

All other Git commands work exactly as expected and are forwarded to the system's Git installation:

### File Operations
```bash
gith add file.txt           # Stage files
gith add .                  # Stage all changes
gith rm file.txt            # Remove files
gith mv old.txt new.txt     # Move/rename files
```

### Repository Status
```bash
gith status                 # Check repository status
gith diff                   # View unstaged changes
gith diff --staged          # View staged changes
gith log --oneline         # View commit history
```

### Branching
```bash
gith branch                 # List branches
gith branch feature-xyz     # Create new branch
gith checkout main          # Switch branches
gith checkout -b new-feature # Create and switch to new branch
gith merge feature-branch   # Merge branches
```

### Remote Operations
```bash
gith remote add origin <url>  # Add remote
gith push origin main         # Push to remote
gith pull                     # Pull from remote
gith fetch                    # Fetch from remote
gith clone <url>              # Clone repository
```

### Advanced Operations
```bash
gith rebase main              # Rebase current branch
gith reset --hard HEAD~1      # Reset to previous commit
gith stash                    # Stash changes
gith stash pop                # Apply stashed changes
```

## Command Compatibility

`gith` is designed as a drop-in replacement for Git:

- **Unknown commands** are automatically forwarded to the system's `git` command
- **All Git options and flags** are preserved and passed through
- **Exit codes** match Git's behavior exactly
- **Output formatting** is identical to Git's output

This means you can use `gith` anywhere you would use `git` without any changes to your workflow or scripts.
