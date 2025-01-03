use glob::Pattern;
use std::{fs, io, path::PathBuf};

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

            if current_depth == 0 && i == 0 {
                println!(".");
            }
            println!("{}{}{}{}", prefix, symbol, name, suffix);

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
