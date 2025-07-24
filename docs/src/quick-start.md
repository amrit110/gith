# Quick Start

This guide will get you up and running with `gith` in just a few minutes.

## Initialize a New Repository with gith Tracking

Create a new repository with gith tracking enabled from the start:

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

## Add gith to an Existing Repository

Enable gith tracking in an existing Git repository:

```bash
# Navigate to your existing Git repository
cd existing-project

# Enable gith tracking
gith init-tracking

# This creates the necessary gith infrastructure while preserving your Git history
```

## Make Your First Human-Certified Commit

Create content and commit it with human certification:

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

## View Your Human-Generated Content

Check what content has been certified as human-generated:

```bash
# List all human-flagged commits and files
gith list-human

# Show only commits
gith list-human --commits-only

# Show only files
gith list-human --files-only
```

## Regular Git Commands

All standard Git commands work exactly as expected:

```bash
gith add file.txt           # Stage files
gith status                 # Check repository status
gith log --oneline         # View commit history
gith branch feature-xyz    # Create branches
gith push origin main      # Push to remote
gith pull                  # Pull from remote
gith diff                  # View changes
```

## Next Steps

Now that you have gith set up:

1. **Learn the commands**: Check out the [Commands](./commands.md) reference
2. **See examples**: Browse [practical examples](./examples.md) for common workflows
3. **Understand licensing**: Read about [human-generated content licensing](./licensing.md)
4. **Follow best practices**: Review our [guidelines](./best-practices.md) for effective usage
