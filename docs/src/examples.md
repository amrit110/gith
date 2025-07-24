# Practical Examples

This page provides real-world examples of how to use `gith` effectively in different development scenarios.

## Scenario 1: Starting a New Project

Complete workflow for initializing a new project with gith tracking:

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

**Expected Output:**
```
Human-Generated Commits:
========================
a1b2c3d - 2024-01-15 14:30:25 - Implement core application logic
e4f5g6h - 2024-01-15 14:25:10 - Add project documentation

Human-Generated Files:
======================
main.py (from commit a1b2c3d)
README.md (from commit e4f5g6h)
```

## Scenario 2: Mixed Development Workflow

Example of working with both human-generated and AI-assisted content:

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

**Output shows only human-certified commits:**
```
Human-Generated Commits:
========================
x9y8z7w - 2024-01-15 16:45:30 - Design RESTful API endpoints
a1b2c3d - 2024-01-15 15:20:15 - Implement user authentication logic
```

## Scenario 3: Team Collaboration

Working in a team environment with gith tracking:

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

## Scenario 4: Converting Existing Repository

Adding gith tracking to an existing project:

```bash
# Navigate to existing repository
cd my-existing-project

# Check current state
git log --oneline -5
git status

# Enable gith tracking
gith init-tracking

# From this point forward, selectively mark human-generated commits
echo "// New human-written function" >> utils.js
gith add utils.js
gith commit --human -m "Add utility function for data validation"

# Regular commits continue as normal
gith commit -m "Update dependencies and fix warnings"
```

## Scenario 5: Large Feature Development

Breaking down a large feature into tracked commits:

```bash
# Start feature branch
gith checkout -b feature/payment-system

# Human-designed architecture
touch payment_processor.py
gith add payment_processor.py
gith commit --human -m "Design payment processor architecture"

# Human-implemented core logic
# ... implement core payment logic ...
gith add payment_processor.py
gith commit --human -m "Implement core payment processing logic"

# AI-assisted error handling
# ... add error handling with AI assistance ...
gith add payment_processor.py
gith commit -m "Add comprehensive error handling and validation"

# Human-written tests
touch test_payments.py
# ... write tests manually ...
gith add test_payments.py
gith commit --human -m "Add comprehensive payment processor tests"

# Review what was human-generated in this feature
gith list-human --commits-only
```

## Scenario 6: Code Review and Verification

Using gith during code review process:

```bash
# Reviewer checks human-generated content
gith list-human --commits-only

# Review specific human-certified files
gith list-human --files-only

# Check specific commit details
git show abc123f  # Shows commit with human-flag trailer

# Verify licensing is properly applied
cat LICENSE-HUMAN
```

## Scenario 7: Automated Workflow Integration

Using gith in scripts and automation:

```bash
#!/bin/bash
# Script to batch-commit human-generated files

for file in src/*.rs; do
    if [[ $file =~ "human_written" ]]; then
        gith add "$file"
        gith commit --human -m "Add human-written module: $(basename $file)"
    fi
done

# Generate report of human-generated content
echo "=== Human-Generated Content Report ===" > human_report.txt
gith list-human >> human_report.txt
```

## Scenario 8: Integration with Conventional Commits

Combining gith with conventional commit standards:

```bash
# Human-generated features
gith commit --human -m "feat: implement user authentication system"
gith commit --human -m "feat: add payment processing module"

# AI-assisted fixes
gith commit -m "fix: resolve memory leak in data processing"
gith commit -m "docs: update API documentation with examples"

# Human-designed breaking changes
gith commit --human -m "feat!: redesign API with breaking changes"
```

## Scenario 9: Maintaining Project History

Tracking human contributions over time:

```bash
# Generate monthly report
echo "## Human Contributions - $(date +%B\ %Y)" > monthly_report.md
echo "" >> monthly_report.md
gith list-human --commits-only >> monthly_report.md

# Check human contribution ratio
total_commits=$(git rev-list --count HEAD)
human_commits=$(gith list-human --commits-only | grep -v "Human-Generated Commits" | grep -v "=" | wc -l)
echo "Human-generated: $human_commits of $total_commits commits"
```

## Scenario 10: Multi-Repository Project

Managing gith across multiple related repositories:

```bash
# Setup script for multi-repo project
for repo in frontend backend shared-utils; do
    cd $repo
    gith init-tracking
    echo "Enabled gith tracking in $repo"
    cd ..
done

# Commit human-generated shared utilities
cd shared-utils
gith add validation.js
gith commit --human -m "Create shared validation utilities"
cd ..

# Use in other repositories
cd frontend
# ... implement using shared utils ...
gith commit -m "Integrate shared validation utilities"
```

These examples demonstrate how `gith` integrates seamlessly into various development workflows while providing clear tracking of human-generated content.
