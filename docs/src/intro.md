# gith: Track Human-Generated Content in Git

`gith` is a friendly Git wrapper that helps developers track and certify human-generated content in their repositories. It provides a seamless way to mark commits as entirely human-created while maintaining full Git compatibility.

## Why Use gith?

In an era of AI-assisted development, `gith` provides essential tools to:

- **🎯 Certify Human Work**: Mark commits as entirely human-generated with cryptographic verification
- **🛡️ Protect Your Content**: Apply licensing that excludes AI training without explicit permission
- **📋 Maintain Transparency**: Keep a clear manifest of human vs AI-assisted contributions
- **🔄 Stay Compatible**: Works as a drop-in replacement for Git with full command forwarding

## Getting Started

### Quick Installation

The fastest way to install `gith`:

```bash
cargo install gith
```

### Basic Usage

```bash
# Enable tracking in a repository
gith init-tracking

# Make a human-certified commit
gith commit --human -m "Implement custom algorithm"

# View human-generated content
gith list-human
```

## Key Features

- **Drop-in Git replacement**: All Git commands work exactly as expected
- **Human certification**: Flag commits as entirely human-generated
- **Automatic licensing**: Apply "NO AI TRAINING" license to certified content
- **Manifest tracking**: Maintain a record of all human-generated contributions
- **Team collaboration**: Share and verify human content across teams

## What's Next?

- **[Installation](./installation.md)**: Detailed installation instructions for all platforms
- **[Quick Start](./quick-start.md)**: Get up and running in minutes
- **[Commands](./commands.md)**: Complete reference for all gith commands
- **[Examples](./examples.md)**: Real-world usage scenarios and workflows
- **[Best Practices](./best-practices.md)**: Guidelines for effective usage
- **[Licensing](./licensing.md)**: Understanding human-generated content licenses

---

**Ready to start tracking your human-generated content?** Check out the [Quick Start guide](./quick-start.md) to begin certifying your authentic contributions today!
