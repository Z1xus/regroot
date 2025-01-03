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

                !ignore_patterns.iter().any(|pattern| pattern.matches(name))
            })
            .filter(|entry| !dirs_only || entry.path().is_dir())
            .collect();

        entries.sort_by_key(|e| e.path());

        for (i, entry) in entries.iter().enumerate() {
            let path = entry.path();
            let dir = path.display().to_string();
            let pure_name = dir.trim_start_matches(dir_path);
            let pure_name = pure_name
                .split('/')
                .nth(current_depth + 1)
                .unwrap_or(dir.as_str());

            let symbol = if i == entries.len() - 1 {
                "└── "
            } else {
                "├── "
            };
            let indent = if current_depth == 0 {
                String::new()
            } else {
                format!(
                    "{}{}",
                    "│   ".repeat(current_depth),
                    "    ".repeat(current_depth.saturating_sub(1))
                )
            };

            println!("{}{}{}", indent, symbol, pure_name);

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
