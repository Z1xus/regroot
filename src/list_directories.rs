use glob::Pattern;
use std::{fs, io, path::PathBuf};

const RESET: &str = "\x1b[0m";
const HIDDEN: &str = "\x1b[2m"; // Dim
const DIR: &str = "\x1b[1;34m"; // Blue
const SYMLINK: &str = "\x1b[36m"; // Cyan
const EXECUTABLE: &str = "\x1b[32m"; // Green

fn is_hidden(path: &PathBuf) -> bool {
    let hidden_by_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .map(|name| name.starts_with('.'))
        .unwrap_or(false);

    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt;
        hidden_by_name
            || path
                .metadata()
                .map(|m| (m.file_attributes() & 0x2) != 0)
                .unwrap_or(false)
    }

    #[cfg(not(windows))]
    {
        hidden_by_name
    }
}

fn get_file_style(entry: &fs::DirEntry) -> &'static str {
    let path = entry.path();

    if is_hidden(&path) {
        HIDDEN
    } else if path.is_symlink() {
        SYMLINK
    } else if path.is_dir() {
        DIR
    } else if is_executable(&path) {
        EXECUTABLE
    } else {
        RESET
    }
}

fn is_executable(path: &PathBuf) -> bool {
    let is_executable_ext = path
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| {
            matches!(
                ext.to_ascii_lowercase().as_str(),
                "exe"
                    | "bat"
                    | "cmd"
                    | "ps1"
                    | "psd1"
                    | "psm1"
                    | "scr"
                    | "msi"
                    | "sh"
                    | "bash"
                    | "py"
                    | "pl"
            )
        })
        .unwrap_or(false);

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        is_executable_ext
            || path
                .metadata()
                .map(|m| m.permissions().mode() & 0o111 != 0)
                .unwrap_or(false)
    }

    #[cfg(not(unix))]
    {
        is_executable_ext
    }
}

pub fn list_directories(
    path: &PathBuf,
    max_depth: usize,
    current_depth: usize,
    dir_path: &String,
    ignore_patterns: &[Pattern],
    dirs_only: bool,
) -> io::Result<()> {
    if current_depth <= max_depth {
        let mut entries: Vec<_> = fs::read_dir(path)?
            .filter_map(|e| e.ok())
            .filter(|entry| {
                let path = entry.path();
                let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

                if current_depth == 0 {
                    !ignore_patterns.iter().any(|pattern| pattern.matches(name))
                } else {
                    !ignore_patterns.iter().any(|pattern| {
                        let pattern_str = pattern.as_str();
                        pattern_str.contains('/') && pattern.matches(&path.to_string_lossy())
                    })
                }
            })
            .filter(|entry| !dirs_only || entry.path().is_dir())
            .collect();

        entries.sort_by_key(|e| {
            let p = e.path();
            (!p.is_dir(), p)
        });

        for (i, entry) in entries.iter().enumerate() {
            let path = entry.path();
            let is_last = i == entries.len() - 1;
            let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

            let prefix = if current_depth == 0 {
                String::new()
            } else {
                let mut prefix = String::new();
                for d in 0..current_depth {
                    if d == current_depth - 1 {
                        prefix.push_str(if is_last { "    " } else { "│   " });
                    } else {
                        prefix.push_str("│   ");
                    }
                }
                prefix
            };

            let symbol = if is_last { "└── " } else { "├── " };
            let suffix = if path.is_dir() { "/" } else { "" };
            let style = get_file_style(entry);

            if current_depth == 0 && i == 0 {
                println!(".");
            }
            println!("{}{}{}{}{}{}", prefix, symbol, style, name, suffix, RESET);

            if path.is_dir() {
                list_directories(
                    &path,
                    max_depth,
                    current_depth + 1,
                    dir_path,
                    ignore_patterns,
                    dirs_only,
                )?;
            }
        }
    }

    Ok(())
}
