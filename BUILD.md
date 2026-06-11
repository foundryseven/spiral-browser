# Spiral Browser — Build Guide

## Prerequisites

### All Platforms
- Rust 1.75+ (stable)
- Cargo (included with Rust)
- Git

### Linux
```bash
# Ubuntu/Debian
sudo apt-get install -y \
    libwayland-dev \
    libxkbcommon-dev \
    libfontconfig1-dev \
    libfreetype-dev \
    libssl-dev \
    pkg-config

# Fedora
sudo dnf install -y \
    wayland-devel \
    libxkbcommon-devel \
    fontconfig-devel \
    freetype-devel \
    openssl-devel
```

### macOS
```bash
# Install Xcode Command Line Tools
xcode-select --install

# Install dependencies via Homebrew (optional)
brew install pkg-config
```

### Windows
- Visual Studio 2022+ with "Desktop development with C++" workload
- Or: Visual Studio Build Tools

## Building

```bash
# Debug build (default)
cargo build

# Release build (optimized)
cargo build --release

# Build specific crate
cargo build --package spiral-core

# Build with specific target
cargo build --target x86_64-unknown-linux-gnu
cargo build --target x86_64-apple-darwin
cargo build --target x86_64-pc-windows-msvc
```

## Output Locations

```
target/
├── debug/
│   └── spiral-browser          # Debug binary
├── release/
│   └── spiral-browser          # Release binary
└── ...
```

## Running

```bash
# Run debug build
cargo run

# Run release build
cargo run --release

# Run with logging
RUST_LOG=debug cargo run
RUST_LOG=trace cargo run  # very verbose

# Run with config
cargo run -- --config /path/to/config.toml
```

## Cross-Compilation

### Linux → Windows
```bash
# Install target
rustup target add x86_64-pc-windows-msvc

# Install linker (via cross or zig)
cargo install cross

# Build
cross build --target x86_64-pc-windows-msvc --release
```

### Windows → Linux
```bash
# Install target
rustup target add x86_64-unknown-linux-gnu

# Build with cross
cross build --target x86_64-unknown-linux-gnu --release
```

### macOS → Linux
```bash
rustup target add x86_64-unknown-linux-gnu
cross build --target x86_64-unknown-linux-gnu --release
```

## Packaging

### Linux (AppImage)
```bash
# Install cargo-appimage
cargo install cargo-appimage

# Build AppImage
cargo appimage
```

### macOS (App Bundle)
```bash
# Install cargo-bundle
cargo install cargo-bundle

# Bundle
cargo bundle --release
```

### Windows (Installer)
```bash
# Install cargo-wix
cargo install cargo-wix

# Build installer
cargo wix
```

## Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `RUST_LOG` | Log level (error/warn/info/debug/trace) | `info` |
| `SPIRAL_CONFIG` | Path to config file | platform default |
| `SPIRAL_DATA_DIR` | Data directory path | platform default |
| `SPIRAL_CACHE_DIR` | Cache directory path | platform default |

## Directories

| Platform | Config | Data | Cache |
|----------|--------|------|-------|
| Linux | `~/.config/spiral/` | `~/.local/share/spiral/` | `~/.cache/spiral/` |
| macOS | `~/Library/Application Support/Spiral/` | `~/Library/Application Support/Spiral/` | `~/Library/Caches/Spiral/` |
| Windows | `%APPDATA%\Spiral\` | `%LOCALAPPDATA%\Spiral\` | `%LOCALAPPDATA%\Spiral\cache\` |

## Troubleshooting

### Build Fails with Missing Dependencies (Linux)
```bash
# Install build essentials
sudo apt-get install build-essential pkg-config
```

### Build Fails on Windows
- Ensure Visual Studio Build Tools are installed
- Ensure "Desktop development with C++" workload is selected
- Restart terminal after installation

### Linker Errors
```bash
# Check Rust toolchain
rustup show

# Update toolchain
rustup update
```
