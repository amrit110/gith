# gith: Track Human-Generated Content in Git

`gith` is a friendly Git wrapper that helps developers track and certify human-generated content in their repositories. It provides a seamless way to mark commits as entirely human-created while maintaining full Git compatibility.

## Why Use gith?

In an era of AI-assisted development, `gith` provides essential tools to:

- **🎯 Certify Human Work**: Mark commits as entirely human-generated with cryptographic verification
- **🛡️ Protect Your Content**: Apply licensing that excludes AI training without explicit permission
- **📋 Maintain Transparency**: Keep a clear manifest of human vs AI-assisted contributions
- **🔄 Stay Compatible**: Works as a drop-in replacement for Git with full command forwarding

## Installation

### Method 1: Install from Crates.io (Recommended)

```bash
cargo install gith
```

### Method 2: Install from Source

```bash
git clone https://github.com/amrit110/gith.git
cd gith
cargo build --release
sudo cp target/release/gith /usr/local/bin/
```

### Method 3: Download Pre-built Binaries

Visit the [GitHub releases page](https://github.com/amrit110/gith/releases) to download pre-built binaries for your platform.

## Quick Start

### Initialize a New Repository with gith Tracking

```bash
# Create a new repository with gith tracking enabled
mkdir my-project
cd my-project
gith init --gith

# Your repository now has:
# - Standard Git initialization
# - .gith/ directory for tracking metadata
# - human_manifest.json for human-generated content tracking
# - Updated .gitignore with gith-specific entries
```

### Add gith to an Existing Repository

```bash
# Navigate to your existing Git repository
cd existing-project

# Enable gith tracking
gith init-tracking

# This creates the necessary gith infrastructure while preserving your Git history
```

### Make Your First Human-Certified Commit

```bash
# Create some content
echo "console.log('Hello, human world!');" > app.js

# Stage the file
gith add app.js

# Create a human-certified commit
gith commit --human -m "Add initial application logic"

# The commit will automatically include:
# Human-Flag: true
# License: HUMAN-GENERATED, NO AI TRAINING
```

### View Your Human-Generated Content

```bash
# List all human-flagged commits and files
gith list-human

# Show only commits
gith list-human --commits-only

# Show only files
gith list-human --files-only
```

## Core Commands

### Repository Initialization

#### `gith init [OPTIONS] [DIRECTORY]`

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

#### `gith init-tracking`

Enable gith tracking in an existing Git repository.

**What it creates:**
- `.gith/` directory for metadata storage
- `human_manifest.json` for tracking human-generated content
- Updates `.gitignore` to exclude gith metadata from version control
- `LICENSE-HUMAN` file with human-generated content license

**Example:**

```bash
cd existing-git-repo
gith init-tracking
```

### Committing Changes

#### `gith commit [OPTIONS] -m <MESSAGE>`

Create commits with optional human certification.

**Key Options:**
- `--human` - Mark commit as entirely human-generated
- `-a, --all` - Stage all modified files before committing
- `--allow-empty` - Allow empty commits
- `-m, --message <MSG>` - Commit message

**Examples:**

```bash
# Regular commit (forwards to git)
gith commit -m "Fix bug in user authentication"

# Human-certified commit
gith commit --human -m "Implement custom sorting algorithm"

# Stage all changes and commit as human-generated
gith commit -a --human -m "Refactor data processing module"
```

When using `--human`, gith automatically adds these trailers to your commit message:

```
Human-Flag: true
License: HUMAN-GENERATED, NO AI TRAINING
```

### Content Tracking

#### `gith list-human [OPTIONS]`

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

### Standard Git Operations

All other Git commands work exactly as expected:

```bash
gith add file.txt           # Stage files
gith status                 # Check repository status
gith log --oneline         # View commit history
gith branch feature-xyz    # Create branches
gith push origin main      # Push to remote
gith pull                  # Pull from remote
gith diff                  # View changes
```

## Practical Examples

### Scenario 1: Starting a New Project

```bash
# Initialize project with gith tracking
mkdir awesome-app
cd awesome-app
gith init --gith

# Create initial files
echo "# Awesome App" > README.md
echo "print('Hello World')" > main.py

# Make human-certified commits
gith add README.md
gith commit --human -m "Add project documentation"

gith add main.py
gith commit --human -m "Implement core application logic"

# View your human-generated content
gith list-human
```

### Scenario 2: Mixed Development Workflow

```bash
# Human-generated feature
gith add feature.js
gith commit --human -m "Implement user authentication logic"

# AI-assisted bug fix (regular commit)
gith add bugfix.js
gith commit -m "Fix memory leak in data processing"

# Another human-generated feature
gith add api.js
gith commit --human -m "Design RESTful API endpoints"

# See what's certified as human-generated
gith list-human --commits-only
```

### Scenario 3: Team Collaboration

```bash
# Clone repository with gith tracking
git clone https://github.com/team/project.git
cd project

# Enable gith tracking if not already enabled
gith init-tracking

# Work on features
gith checkout -b feature/user-profiles
echo "const profiles = {};" > profiles.js
gith add profiles.js
gith commit --human -m "Create user profile management system"

# Push branch
gith push origin feature/user-profiles

# Create pull request through your preferred method
# The human certification will be preserved in the commit history
```

## Human-Generated Content License

When you use the `--human` flag, gith automatically applies a license that:

- ✅ **Certifies content as entirely human-generated**
- 🚫 **Prohibits AI training use** without explicit permission
- 📄 **Enables redistribution** with license preservation
- 🔍 **Provides verification** through gith's tracking system

The complete license text is available in the `LICENSE-HUMAN` file created in your repository.

### License Summary

```
HUMAN-GENERATED CONTENT LICENSE

This content was created entirely by humans without AI assistance.

PERMITTED:
- Use, copy, modify, distribute for any purpose
- Commercial and non-commercial use

PROHIBITED:
- Training AI models without explicit written permission
- Claiming AI generated this content

REQUIRED:
- Preserve this license notice in redistributions
- Maintain human-generation certification
```

## Best Practices

### 1. Be Honest About Human Generation

Only use `--human` flag for content that is genuinely created without AI assistance:

```bash
# ✅ Good - You wrote this logic yourself
gith commit --human -m "Implement binary search algorithm"

# ❌ Avoid - Don't flag AI-assisted work as human
gith commit --human -m "Add code generated by ChatGPT"
```

### 2. Consistent Workflow

Establish team conventions for when to use human certification:

```bash
# Example team policy: Human-flag core algorithms and business logic
gith commit --human -m "Implement payment processing logic"      # ✅
gith commit -m "Fix typo in comments"                           # Regular commit
gith commit --human -m "Design user authentication system"      # ✅
```

### 3. Documentation

Document your human-generation policy in your project:

```bash
echo "## Human-Generated Content Policy

This project uses gith to track human-generated content.

- Core business logic: Always human-flagged
- Bug fixes: Human-flagged if significant
- Documentation: Mixed (as appropriate)
- Tests: Mixed (as appropriate)

To view certified human content: \`gith list-human\`
" >> README.md

gith add README.md
gith commit --human -m "Add human-generation policy documentation"
```

## Integration with CI/CD

### GitHub Actions Example

Create `.github/workflows/verify-human-content.yml`:

```yaml
name: Verify Human Content
on: [push, pull_request]

jobs:
  verify:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Install gith
        run: cargo install gith
      - name: Verify human content tracking
        run: |
          gith list-human --commits-only
          echo "Human-certified commits verified ✅"
```

### Pre-commit Hook

Add to `.git/hooks/pre-commit`:

```bash
#!/bin/bash
# Remind developers about human certification

echo "💡 Tip: Use 'gith commit --human' to certify human-generated content"
echo "Current human-certified commits:"
gith list-human --commits-only | head -5
```

## Troubleshooting

### Common Issues

**Issue**: `gith init-tracking` fails with "not a git repository"
```bash
# Solution: Ensure you're in a Git repository
git status  # Should show repository status
gith init-tracking
```

**Issue**: Commands not recognized
```bash
# Solution: Ensure gith is in your PATH
which gith  # Should show gith location
echo $PATH  # Verify gith directory is included
```

**Issue**: Human manifest seems corrupted
```bash
# Solution: Reinitialize tracking (preserves Git history)
rm -rf .gith/
gith init-tracking
```

### Getting Help

- **Documentation**: This guide and inline help (`gith --help`)
- **Issues**: [GitHub Issues](https://github.com/amrit110/gith/issues)
- **Discussions**: [GitHub Discussions](https://github.com/amrit110/gith/discussions)

## Advanced Usage

### Custom License Text

You can customize the human-generated content license by editing `LICENSE-HUMAN` in your repository root after running `gith init-tracking`.

### Scripting with gith

```bash
#!/bin/bash
# Script to batch-commit human-generated files

for file in src/*.rs; do
    if [[ $file =~ "human_written" ]]; then
        gith add "$file"
        gith commit --human -m "Add human-written module: $(basename $file)"
    fi
done
```

### Integration with Other Tools

```bash
# Use with conventional commits
gith commit --human -m "feat: implement user authentication system"

# Combine with semantic versioning
gith commit --human -m "feat!: redesign API with breaking changes"
```

---

**Ready to start tracking your human-generated content?** Initialize your first gith repository and begin certifying your authentic contributions today!
