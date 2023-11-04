use std::{env, fs, io, path::PathBuf};

fn list_directories(
    vector: &mut Vec<PathBuf>,
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
                vector.push(path.clone());
                println!(
                    "|{}-{}",
                    "-".repeat(current_depth * 3),
                    path.display()
                        .to_string()
                        .trim_start_matches(dir_path)
                        .splitn(current_depth + 1, "/")
                        .nth(1)
                        .unwrap_or(path.display().to_string().as_str())
                );
                list_directories(vector, path, max_depth, current_depth + 1, dir_path);
            }
        }
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut vector = Vec::new();

    let path = PathBuf::from(&args[1]);
    let max_depth = args[2].parse::<usize>().unwrap_or(1);

    list_directories(&mut vector, path, max_depth, 0, &args[1]);

    Ok(())
}
