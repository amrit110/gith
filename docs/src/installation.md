# Installation

This page covers the different methods to install `gith` on your system.

## Method 1: Install from Crates.io (Recommended)

The easiest way to install `gith` is through Rust's package manager:

```bash
cargo install gith
```

This method ensures you get the latest stable release and automatically handles dependencies.

## Method 2: Install from Source

If you want to build from the latest source code or contribute to development:

```bash
git clone https://github.com/amrit110/gith.git
cd gith
cargo build --release
sudo cp target/release/gith /usr/local/bin/
```

## Method 3: Download Pre-built Binaries

For systems without Rust installed, you can download pre-built binaries:

1. Visit the [GitHub releases page](https://github.com/amrit110/gith/releases)
2. Download the appropriate binary for your platform
3. Extract and place in your PATH

## Verification

After installation, verify that `gith` is working correctly:

```bash
gith --version
```

You should see version information displayed.

## Prerequisites

- **Git**: `gith` requires Git to be installed and available in your PATH
- **Rust** (for source installation): Version 1.70 or later if building from source

## Troubleshooting Installation

### Command Not Found

If you get "command not found" after installation:

```bash
# Check if gith is in your PATH
which gith
echo $PATH

# For cargo install, ensure ~/.cargo/bin is in your PATH
export PATH="$HOME/.cargo/bin:$PATH"
```

### Permission Issues

If you encounter permission issues during installation:

```bash
# For source installation, use a local directory instead
mkdir -p ~/bin
cp target/release/gith ~/bin/
export PATH="$HOME/bin:$PATH"
```

### Rust Installation

If you don't have Rust installed and want to use Method 1 or 2:

```bash
# Install Rust via rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```
