# gith

**A Git wrapper for tracking human-generated content**

`gith` helps you mark and track content created entirely by humans in your repositories. Flag commits as human-generated, manage licensing for AI training exclusion, and maintain a clear record of authentic human contributions.

## Why gith?

In an AI-assisted development world, `gith` provides a way to:
- **Certify human work**: Mark commits as entirely human-generated
- **Protect your content**: Automatically apply licensing that excludes AI training
- **Maintain transparency**: Keep a clear manifest of human vs AI-assisted contributions
- **Stay compatible**: Works seamlessly with existing Git workflows

## Features

- ✨ **Human-flagged commits** - Mark commits as entirely human-generated
- 📋 **Content tracking** - Maintain a manifest of human-generated commits and files
- 🛡️ **License management** - Automatically handle licensing for AI training exclusion
- 🔄 **Git compatibility** - Seamlessly forwards unrecognized commands to Git
- 🌍 **Cross-platform** - Works on Windows, macOS, and Linux

## Installation

### Option 1: Using Cargo

```bash
cargo install gith
```

### Option 2: From Source

```bash
git clone https://github.com/amrit110/gith.git
cd gith
cargo build --release
# Copy target/release/gith to your PATH
```

## Quick Start

**Start fresh** - Initialize a new Git repository with gith tracking:
```bash
gith init --gith
```

**Add to existing** - Enable gith tracking in your current repo:
```bash
gith init-tracking
```

**Make your first human-certified commit**:
```bash
gith add file.txt
gith commit --human -m "Add my original implementation"
```

**View your human-generated content**:
```bash
gith list-human
```

## Command Reference

### Core Commands

**`gith init [OPTIONS] [DIRECTORY]`**
Initialize a new Git repository (identical to `git init`)

Common options:
- `--bare` - Create a bare repository
- `-b, --initial-branch <BRANCH>` - Set initial branch name
- `--shared[=<PERMISSIONS>]` - Create shared repository

---

**`gith init-tracking`**
Enable gith tracking in an existing Git repository

Creates:
- `.gith/` directory for metadata
- `human_manifest.json` for tracking flags
- Updates `.gitignore` appropriately

**`gith init --gith [DIRECTORY]`**
Initialize a new Git repository with gith tracking enabled

Combines `git init` functionality with automatic gith tracking setup

---

**`gith commit [OPTIONS] -m <MESSAGE>`**
Create commits with optional human certification

Key options:
- `--human` - Mark as entirely human-generated
- `-a, --all` - Stage all modified files
- `--allow-empty` - Allow empty commits

Example human-certified commit:
```bash
gith commit --human -m "Implement authentication system"
```

Adds these trailers automatically:
```
Human-Flag: true
License: HUMAN-GENERATED, NO AI TRAINING
```

---

**`gith list-human`**
View all human-flagged content

Options:
- `--commits-only` - Show commits only
- `--files-only` - Show files only

### Standard Git Commands

All other commands work exactly like Git:
```bash
gith add file.txt     # Stage files
gith status          # Check status
gith log --oneline   # View history
gith push origin main # Push changes
```

## Human-Generated Content Protection

When you use the `--human` flag, `gith` automatically applies licensing that:

- 🔒 **Certifies content as entirely human-generated**
- 🚫 **Prohibits AI training use** without explicit permission
- 📄 **Enables redistribution** with license preservation
- ✅ **Provides verification** through the gith tracking system

The complete license text is available in the `LICENSE-HUMAN` file created in your repository.

---

## License

**gith tool**: MIT License - see [LICENSE](LICENSE)
**Human-generated content**: Human-Generated Content License - see [LICENSE-HUMAN](LICENSE-HUMAN)
