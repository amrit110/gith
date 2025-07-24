# Integration with CI/CD and Tooling

This page covers how to integrate `gith` with various development tools, CI/CD systems, and workflows.

## GitHub Actions

### Basic Verification Workflow

Create `.github/workflows/verify-human-content.yml`:

```yaml
name: Verify Human Content
on: [push, pull_request]

jobs:
  verify:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0  # Need full history for gith
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Install gith
        run: cargo install gith
      - name: Verify human content tracking
        run: |
          if [ -d ".gith" ]; then
            gith list-human --commits-only
            echo "✅ Human-certified commits verified"
          else
            echo "ℹ️  No gith tracking in this repository"
          fi
```

### PR Reporting

Add human content reports to pull requests:

```yaml
name: Human Content Report
on: pull_request

jobs:
  report:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0
      - name: Install gith
        run: cargo install gith
      - name: Generate human content report
        run: |
          if [ -d ".gith" ]; then
            echo "## Human-Certified Content in PR" >> $GITHUB_STEP_SUMMARY
            echo "" >> $GITHUB_STEP_SUMMARY
            echo "### Commits" >> $GITHUB_STEP_SUMMARY
            echo "\`\`\`" >> $GITHUB_STEP_SUMMARY
            gith list-human --commits-only | head -10 >> $GITHUB_STEP_SUMMARY
            echo "\`\`\`" >> $GITHUB_STEP_SUMMARY
            echo "" >> $GITHUB_STEP_SUMMARY
            echo "### Files" >> $GITHUB_STEP_SUMMARY
            echo "\`\`\`" >> $GITHUB_STEP_SUMMARY
            gith list-human --files-only | head -20 >> $GITHUB_STEP_SUMMARY
            echo "\`\`\`" >> $GITHUB_STEP_SUMMARY
          fi
```

### Release Automation

Include human content in release notes:

```yaml
name: Create Release
on:
  push:
    tags: ['v*']

jobs:
  release:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0
      - name: Install gith
        run: cargo install gith
      - name: Generate release notes
        run: |
          echo "## Human-Generated Content" > release_notes.md
          echo "" >> release_notes.md
          if [ -d ".gith" ]; then
            human_commits=$(gith list-human --commits-only | grep -v "Human-Generated Commits" | grep -v "=" | wc -l)
            total_commits=$(git rev-list --count HEAD)
            echo "- Human-certified commits: $human_commits of $total_commits" >> release_notes.md
            echo "- Percentage human-generated: $(( human_commits * 100 / total_commits ))%" >> release_notes.md
          fi
      - name: Create Release
        uses: actions/create-release@v1
        with:
          tag_name: ${{ github.ref }}
          release_name: Release ${{ github.ref }}
          body_path: release_notes.md
```

## GitLab CI

### Basic Pipeline

Create `.gitlab-ci.yml`:

```yaml
stages:
  - verify
  - report

verify-human-content:
  stage: verify
  image: rust:latest
  before_script:
    - cargo install gith
  script:
    - |
      if [ -d ".gith" ]; then
        echo "Verifying human-certified content..."
        gith list-human --commits-only
        echo "✅ Verification complete"
      else
        echo "ℹ️  No gith tracking detected"
      fi
  only:
    - merge_requests
    - main

human-content-report:
  stage: report
  image: rust:latest
  before_script:
    - cargo install gith
  script:
    - |
      if [ -d ".gith" ]; then
        echo "## Human Content Report" > report.md
        echo "" >> report.md
        total=$(git rev-list --count HEAD)
        human=$(gith list-human --commits-only | grep -v "Human-Generated Commits" | grep -v "=" | wc -l)
        echo "- Total commits: $total" >> report.md
        echo "- Human-certified: $human" >> report.md
        echo "- Percentage: $(( human * 100 / total ))%" >> report.md
        cat report.md
      fi
  artifacts:
    reports:
      junit: report.md
  only:
    - main
```

## Pre-commit Hooks

### Installation and Setup

Install pre-commit and configure for gith:

```bash
# Install pre-commit
pip install pre-commit

# Create .pre-commit-config.yaml
cat > .pre-commit-config.yaml << 'EOF'
repos:
  - repo: local
    hooks:
      - id: gith-human-reminder
        name: Remind about human certification
        entry: scripts/pre-commit-gith.sh
        language: script
        stages: [pre-commit]
        always_run: true
EOF

# Create the hook script
mkdir -p scripts
cat > scripts/pre-commit-gith.sh << 'EOF'
#!/bin/bash
echo "💡 Tip: Use 'gith commit --human' to certify human-generated content"

if [ -d ".gith" ]; then
    echo "Recent human-certified commits:"
    gith list-human --commits-only | head -3
fi
echo ""
EOF

chmod +x scripts/pre-commit-gith.sh

# Install hooks
pre-commit install
```

### Advanced Pre-commit Hook

More sophisticated verification:

```bash
cat > scripts/advanced-gith-hook.sh << 'EOF'
#!/bin/bash

# Check if commit message indicates human generation
if git log -1 --pretty=%B | grep -q "\--human"; then
    echo "⚠️  Detected --human flag in commit message"
    echo "Make sure to use: gith commit --human -m 'message'"
    echo "Instead of: git commit -m 'message --human'"
    exit 1
fi

# Remind about human certification for significant changes
lines_changed=$(git diff --cached --numstat | awk '{sum += $1 + $2} END {print sum}')
if [ "$lines_changed" -gt 50 ]; then
    echo "📊 Large changeset detected ($lines_changed lines)"
    echo "💡 Consider using 'gith commit --human' if this is human-generated"
fi

# Check for AI-related keywords in staged files
if git diff --cached --name-only | xargs grep -l -i -E "(chatgpt|copilot|ai.generated|gpt-)" 2>/dev/null; then
    echo "🤖 Detected AI-related keywords in staged files"
    echo "💡 Consider regular commit unless content is entirely human-generated"
fi
EOF

chmod +x scripts/advanced-gith-hook.sh
```

## IDE Integration

### VS Code Settings

Create `.vscode/settings.json` for gith integration:

```json
{
  "git.defaultCloneDirectory": "./projects",
  "git.confirmSync": false,
  "git.enableSmartCommit": true,
  "terminal.integrated.env.linux": {
    "GITH_DEFAULT_HUMAN": "false"
  },
  "terminal.integrated.env.osx": {
    "GITH_DEFAULT_HUMAN": "false"
  },
  "terminal.integrated.env.windows": {
    "GITH_DEFAULT_HUMAN": "false"
  }
}
```

### VS Code Tasks

Create `.vscode/tasks.json`:

```json
{
    "version": "2.0.0",
    "tasks": [
        {
            "label": "List Human Content",
            "type": "shell",
            "command": "gith list-human",
            "group": "build",
            "presentation": {
                "echo": true,
                "reveal": "always",
                "focus": false,
                "panel": "shared"
            }
        },
        {
            "label": "Human Commit",
            "type": "shell",
            "command": "gith commit --human -m '${input:commitMessage}'",
            "group": "build",
            "presentation": {
                "echo": true,
                "reveal": "always",
                "focus": false,
                "panel": "shared"
            }
        }
    ],
    "inputs": [
        {
            "id": "commitMessage",
            "description": "Commit message",
            "default": "Update human-generated content",
            "type": "promptString"
        }
    ]
}
```

## Docker Integration

### Development Container

Create `.devcontainer/devcontainer.json`:

```json
{
    "name": "Gith Development",
    "image": "rust:latest",
    "postCreateCommand": "cargo install gith",
    "customizations": {
        "vscode": {
            "extensions": [
                "rust-lang.rust-analyzer"
            ]
        }
    },
    "forwardPorts": [3000],
    "features": {
        "ghcr.io/devcontainers/features/git:1": {}
    }
}
```

### CI Docker Image

Create a custom image with gith pre-installed:

```dockerfile
# Dockerfile.gith-ci
FROM rust:slim

RUN apt-get update && apt-get install -y \
    git \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

RUN cargo install gith

COPY scripts/ci-verify.sh /usr/local/bin/
RUN chmod +x /usr/local/bin/ci-verify.sh

ENTRYPOINT ["/usr/local/bin/ci-verify.sh"]
```

```bash
# scripts/ci-verify.sh
#!/bin/bash
set -e

echo "🔍 Verifying gith repository..."

if [ ! -d ".gith" ]; then
    echo "ℹ️  No gith tracking detected"
    exit 0
fi

echo "📊 Human content statistics:"
total_commits=$(git rev-list --count HEAD)
human_commits=$(gith list-human --commits-only | grep -v "Human-Generated Commits" | grep -v "=" | wc -l)

echo "- Total commits: $total_commits"
echo "- Human-certified: $human_commits"
echo "- Percentage: $(( human_commits * 100 / total_commits ))%"

echo ""
echo "📝 Recent human-certified commits:"
gith list-human --commits-only | head -5

echo "✅ Verification complete"
```

## Monitoring and Analytics

### Prometheus Metrics

Create a metrics exporter:

```bash
# scripts/gith-metrics.sh
#!/bin/bash

# Export metrics in Prometheus format
cat << EOF
# HELP gith_total_commits Total number of commits in repository
# TYPE gith_total_commits counter
gith_total_commits $(git rev-list --count HEAD)

# HELP gith_human_commits Number of human-certified commits
# TYPE gith_human_commits counter
gith_human_commits $(gith list-human --commits-only 2>/dev/null | grep -v "Human-Generated Commits" | grep -v "=" | wc -l)

# HELP gith_human_files Number of files in human-certified commits
# TYPE gith_human_files counter
gith_human_files $(gith list-human --files-only 2>/dev/null | grep -v "Human-Generated Files" | grep -v "=" | wc -l)

# HELP gith_tracking_enabled Whether gith tracking is enabled (1 or 0)
# TYPE gith_tracking_enabled gauge
gith_tracking_enabled $([ -d ".gith" ] && echo 1 || echo 0)
EOF
```

### Weekly Reports

Automated reporting script:

```bash
# scripts/weekly-report.sh
#!/bin/bash

REPORT_FILE="reports/weekly-$(date +%Y-%U).md"
mkdir -p reports

cat > "$REPORT_FILE" << EOF
# Weekly Human Content Report
## Week $(date +%U), $(date +%Y)

### Summary
EOF

if [ -d ".gith" ]; then
    total=$(git rev-list --count HEAD)
    human=$(gith list-human --commits-only | grep -v "Human-Generated Commits" | grep -v "=" | wc -l)

    cat >> "$REPORT_FILE" << EOF
- Total commits: $total
- Human-certified: $human
- Human percentage: $(( human * 100 / total ))%

### Recent Human Commits
\`\`\`
$(gith list-human --commits-only | head -10)
\`\`\`

### Human-Generated Files
\`\`\`
$(gith list-human --files-only | head -20)
\`\`\`
EOF
else
    echo "- No gith tracking enabled" >> "$REPORT_FILE"
fi

echo "📊 Report generated: $REPORT_FILE"
```

These integration examples help you incorporate `gith` into your existing development and deployment workflows, providing automated verification and reporting of human-generated content.
