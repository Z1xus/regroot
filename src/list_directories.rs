use std::{fs, io, path::PathBuf};

pub fn list_directories(
    path: PathBuf,
    max_depth: usize,
    current_depth: usize,
    dir_path: &String,
) -> io::Result<()> {
    if current_depth <= max_depth {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                let dir = path.display().to_string();
                let pure_dir = dir.trim_start_matches(dir_path);
                let pure_dir = pure_dir
                    .split("/")
                    .nth(current_depth + 1)
                    .unwrap_or(dir.as_str());

                let symbol = "└─ ";

                println!("{}{}{}", "   ".repeat(current_depth + 1), symbol, pure_dir);
                list_directories(path, max_depth, current_depth + 1, dir_path).unwrap();
            }
        }
    }

    Ok(())
}
