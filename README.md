# regroot

An updated version of the original [groot-tree](https://crates.io/crates/groot-tree) crate by [RodrigoRVSN](https://github.com/RodrigoRVSN).

<p align="center">
  <img src="https://github.com/user-attachments/assets/e29f561d-e49d-46f9-b302-01d3178d16d1" alt="regroot preview" />
</p>

<p align="center">
    <a href="https://crates.io/crates/regroot">
        <img src="https://img.shields.io/crates/v/regroot.svg" alt="Version" /></a>
    <a href="https://crates.io/crates/regroot">
        <img src="https://img.shields.io/crates/d/regroot.svg" alt="Downloads" /></a>
    <a href="https://github.com/Z1xus/regroot/issues?q=is%3Aissue+is%3Aopen+" alt="GitHub issues">
        <img src="https://img.shields.io/github/issues/z1xus/regroot"></a>
    <a href="https://github.com/Z1xus/regroot/pulls?q=is%3Apr+is%3Aopen+" alt="GitHub pull requests">
        <img src="https://img.shields.io/github/issues-pr/z1xus/regroot"></a>
</p>

## Features

- 🎨 Color-coding for files and directories
- 🔍 Glob pattern support
- 📊 Customizable depth levels
- 📁 Directory-only view option
- 🌳 Clean and minimal tree structure

## Installation

Via cargo:
```bash
cargo install regroot
```

Or build from source:
```bash
git clone https://github.com/z1xus/regroot
cd regroot
cargo build --release
```

## Usage

Basic usage:
```bash
regroot .
tree .     # alias
```

Common options:
```bash
# Show only 2 levels deep
regroot -d 2 .

# Use glob patterns for path
regroot "src/{bin,lib}/*"

# Ignore paths using glob patterns
regroot -i "*.pyc,tests/*" .

# Show only directories
regroot --dirs-only .
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
