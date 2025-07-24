# Troubleshooting

This page covers common issues you might encounter when using `gith` and their solutions.

## Installation Issues

### Command Not Found

**Problem**: `gith: command not found` after installation

**Solutions**:

```bash
# Check if gith is installed and in PATH
which gith
echo $PATH

# For cargo install, ensure ~/.cargo/bin is in PATH
export PATH="$HOME/.cargo/bin:$PATH"
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc

# Verify installation
gith --version
```

### Permission Denied

**Problem**: Permission issues during source installation

**Solution**:

```bash
# Use local installation instead of system-wide
mkdir -p ~/bin
cp target/release/gith ~/bin/
export PATH="$HOME/bin:$PATH"

# Or use cargo install (recommended)
cargo install gith
```

### Build Failures

**Problem**: Compilation errors when building from source

**Solutions**:

```bash
# Update Rust toolchain
rustup update stable
rustup default stable

# Clean and rebuild
cargo clean
cargo build --release

# Check system dependencies (Linux)
sudo apt update
sudo apt install build-essential pkg-config libssl-dev

# Check system dependencies (macOS)
xcode-select --install
```

## Repository Setup Issues

### Not a Git Repository

**Problem**: `gith init-tracking` fails with "not a git repository"

**Solution**:

```bash
# Ensure you're in a Git repository
git status

# If not initialized, initialize first
git init
git add .
git commit -m "Initial commit"

# Then enable gith tracking
gith init-tracking
```

### Tracking Already Enabled

**Problem**: `gith init-tracking` reports tracking already enabled

**Solution**:

```bash
# Check if .gith directory exists
ls -la .gith/

# If corrupted, reinitialize (safe - preserves Git history)
rm -rf .gith/
gith init-tracking

# Verify tracking is working
gith list-human
```

### Permission Issues with .gith Directory

**Problem**: Cannot write to `.gith` directory

**Solution**:

```bash
# Check directory permissions
ls -la .gith/

# Fix permissions
chmod 755 .gith/
chmod 644 .gith/human_manifest.json

# If still issues, recreate
sudo rm -rf .gith/
gith init-tracking
```

## Command Issues

### Human Flag Not Working

**Problem**: `--human` flag not adding trailers to commits

**Diagnostic Steps**:

```bash
# Check if gith tracking is enabled
ls -la .gith/

# Verify commit was made with gith (not git)
git log -1 --pretty=format:"%B"

# Check for human-flag trailer
git log -1 --grep="Human-Flag: true"
```

**Solutions**:

```bash
# Ensure using gith command
gith commit --human -m "message"  # ✅ Correct
git commit --human -m "message"   # ❌ Wrong - no human flag

# Check if tracking is initialized
gith init-tracking

# Verify with test commit
echo "test" > test.txt
gith add test.txt
gith commit --human -m "test human commit"
git log -1 --pretty=format:"%B"
```

### Commands Not Forwarding to Git

**Problem**: Standard Git commands fail when using `gith`

**Diagnostic Steps**:

```bash
# Check if git is in PATH
which git

# Test git directly
git status

# Test gith forwarding
gith status
```

**Solutions**:

```bash
# Ensure git is installed and accessible
git --version

# Check PATH includes git location
echo $PATH

# If git not found, install or fix PATH
# Ubuntu/Debian:
sudo apt install git

# macOS:
xcode-select --install
# or
brew install git
```

### Manifest Corruption

**Problem**: Human manifest appears corrupted or empty

**Symptoms**:

```bash
# Empty or malformed output
gith list-human
# Shows no content despite human commits
```

**Solutions**:

```bash
# Check manifest file
cat .gith/human_manifest.json

# Verify JSON is valid
python -m json.tool .gith/human_manifest.json

# If corrupted, recreate (loses tracking history)
rm .gith/human_manifest.json
gith init-tracking

# Alternative: Manual repair
echo '{"commits": {}, "files": {}}' > .gith/human_manifest.json
```

## Performance Issues

### Slow Operations

**Problem**: `gith` commands are significantly slower than `git`

**Diagnostic Steps**:

```bash
# Time comparison
time git status
time gith status

# Check repository size
git count-objects -vH

# Check manifest size
ls -lh .gith/human_manifest.json
```

**Solutions**:

```bash
# For very large repositories, consider selective tracking
# Clean up manifest if it's grown too large
cp .gith/human_manifest.json .gith/human_manifest.json.backup
echo '{"commits": {}, "files": {}}' > .gith/human_manifest.json

# For large files, check .gitignore includes .gith/
echo ".gith/" >> .gitignore
```

## Integration Issues

### CI/CD Failures

**Problem**: Build pipelines fail when using `gith`

**Common Issues and Solutions**:

```yaml
# Issue: gith not available in CI
# Solution: Install in CI pipeline
- name: Install gith
  run: cargo install gith

# Issue: Git history not available
# Solution: Fetch full history
- uses: actions/checkout@v4
  with:
    fetch-depth: 0

# Issue: Permission problems
# Solution: Use appropriate runner
jobs:
  test:
    runs-on: ubuntu-latest  # Ensure compatible OS
```

### Pre-commit Hook Failures

**Problem**: Pre-commit hooks fail with gith

**Solution**:

```bash
# Check hook configuration
cat .git/hooks/pre-commit

# Ensure gith is available in hook environment
#!/bin/bash
export PATH="$HOME/.cargo/bin:$PATH"
which gith || { echo "gith not found"; exit 1; }

# Test hook manually
.git/hooks/pre-commit
```

## Data Recovery

### Lost Human Certifications

**Problem**: Human-flagged commits not showing in `gith list-human`

**Recovery Steps**:

```bash
# Search for human-flagged commits in history
git log --grep="Human-Flag: true" --oneline

# Rebuild manifest from commit history
cat > rebuild_manifest.sh << 'EOF'
#!/bin/bash
echo '{"commits": {}, "files": {}}' > .gith/human_manifest.json

git log --grep="Human-Flag: true" --pretty=format:"%H %ct %s" | while read hash timestamp message; do
    # This would need to be implemented properly
    echo "Found human commit: $hash"
done
EOF
```

### Corrupted Repository

**Problem**: `.gith` directory or tracking completely broken

**Solution**:

```bash
# Complete reset (preserves Git history, loses gith tracking history)
rm -rf .gith/
gith init-tracking

# Verify basic functionality
echo "test" > test.txt
gith add test.txt
gith commit --human -m "test commit after reset"
gith list-human
```

## Getting Help

### Debug Information

When reporting issues, include this information:

```bash
# System information
uname -a
echo "Git version: $(git --version)"
echo "Gith version: $(gith --version)"
echo "Rust version: $(rustc --version)"

# Repository state
echo "Git status:"
git status --porcelain
echo "Gith directory:"
ls -la .gith/ 2>/dev/null || echo "No .gith directory"

# Recent commits
echo "Recent commits:"
git log --oneline -5

# Human commits
echo "Human commits:"
gith list-human --commits-only 2>&1
```

### Common Error Messages

**Error**: `error: failed to execute git`

**Solution**:
```bash
# Ensure git is installed and in PATH
which git
git --version
```

**Error**: `IO error: Permission denied`

**Solution**:
```bash
# Check file permissions
ls -la .gith/
# Fix permissions
chmod -R 755 .gith/
```

**Error**: `JSON parse error in manifest`

**Solution**:
```bash
# Reset manifest
echo '{"commits": {}, "files": {}}' > .gith/human_manifest.json
```

### Support Resources

- **Documentation**: This guide and inline help (`gith --help`)
- **Issues**: [GitHub Issues](https://github.com/amrit110/gith/issues)
- **Discussions**: [GitHub Discussions](https://github.com/amrit110/gith/discussions)

When reporting issues, please include:
1. Full error message
2. Steps to reproduce
3. System information (from debug section above)
4. Expected vs actual behavior
