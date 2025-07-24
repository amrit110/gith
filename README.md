# Gith - Human-Generated Content Git Wrapper

Gith is a friendly Git wrapper that helps you mark and track human-generated content in your repositories. It provides a simple way to flag commits and files as "entirely human-generated" (created without AI assistance) and manages associated licensing for AI training exclusion.

## Features

- **Human-flagged commits**: Mark commits as entirely human-generated
- **Content tracking**: Maintain a manifest of human-generated commits and files
- **License management**: Automatically handle licensing for AI training exclusion
- **Git compatibility**: Seamlessly forwards unrecognized commands to Git
- **Cross-platform**: Works on Windows, macOS, and Linux

## Installation

### From Source

```bash
git clone https://github.com/amrit110/gith.git
cd gith
cargo build --release
# Copy target/release/gith to your PATH
```

### Using Cargo

```bash
cargo install gith
```

## Quick Start

1. **Initialize a new Git repository (works exactly like git init):**
   ```bash
   gith init
   ```

2. **Initialize gith tracking in an existing repository:**
   ```bash
   gith gith-init
   ```

3. **Create a human-flagged commit:**
   ```bash
   gith add file.txt
   gith commit --human -m "Add my original implementation"
   ```

4. **List all human-flagged content:**
   ```bash
   gith list-human
   ```

## Commands

### `gith init [OPTIONS] [DIRECTORY]`
Initialize a Git repository (works exactly like `git init`). Supports all git init options:

Options:
- `--bare`: Create a bare repository
- `-b, --initial-branch <BRANCH>`: Set the name of the initial branch
- `--shared[=<PERMISSIONS>]`: Create a shared repository
- `--template <TEMPLATE_DIR>`: Specify template directory
- `--separate-git-dir <GIT_DIR>`: Set separate git directory
- `-q, --quiet`: Be quiet

### `gith gith-init`
Initialize gith tracking in the current Git repository. This creates:
- `.gith/` directory for tracking metadata
- `human_manifest.json` file for storing human-generation flags
- Updates `.gitignore` to exclude `.gith/` directory

### `gith commit [OPTIONS] -m <MESSAGE>`
Create a commit with optional human-generation flag.

Options:
- `--human`: Mark this commit as human-generated
- `-a, --all`: Add all modified files before committing
- `--allow-empty`: Allow empty commits
- `-m <MESSAGE>`: Commit message (required)

Example:
```bash
gith commit --human -m "Implement authentication system"
```

This adds the following trailer to the commit message:
```
Implement authentication system

Human-Flag: true
License: HUMAN-GENERATED, NO AI TRAINING
```

### `gith add [FILES...]`
Add files to the staging area (mirrors `git add`).

Options:
- `-A, --all`: Add all files

### `gith list-human`
List all human-flagged commits and files.

Options:
- `--commits-only`: Show only commits
- `--files-only`: Show only files

### `gith status`
Show repository status (forwards to `git status`).

### Other Git Commands
Any unrecognized command is forwarded directly to Git:
```bash
gith log --oneline    # Same as: git log --oneline
gith push origin main # Same as: git push origin main
```

## Human-Generated License

When you use `--human` flag, gith automatically applies the following license terms:

- Content is certified as entirely human-generated
- **Prohibits use for AI training** without explicit permission
- Allows redistribution with license preservation
- Tracked and verified through the gith system

The full license text is available in `LICENSE-HUMAN` file created in your repository.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

Human-generated content in this repository is additionally protected under the Human-Generated Content License - see [LICENSE-HUMAN](LICENSE-HUMAN) for details.
