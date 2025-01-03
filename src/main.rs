use clap::Parser;
use glob::Pattern;
use std::{io, path::PathBuf};

pub mod list_directories;
use list_directories::list_directories;

#[derive(Parser)]
struct Args {
    /// Path of where do you want to mount your tree. Now with glob support! "src/{bin,lib}/*"
    path: String,

    /// Depth of the tree
    #[arg(short, long, default_value_t = 0)]
    depth: usize,

    /// Patterns to ignore (e.g., "node_modules", "*.pyc")
    #[arg(short, long, value_delimiter = ',')]
    ignore: Vec<String>,

    /// Show only directories
    #[arg(long)]
    dirs_only: bool,
}

fn main() -> io::Result<()> {
    let args: Args = Args::parse();

    // Convert ignore patterns to glob::Pattern
    let ignore_patterns: Vec<Pattern> = args
        .ignore
        .iter()
        .filter_map(|p| Pattern::new(p).ok())
        .collect();

    // Handle glob patterns in path
    let paths: Vec<PathBuf> = glob::glob(&args.path)
        .expect("Failed to read glob pattern")
        .filter_map(Result::ok)
        .collect();

    for path in paths {
        let path_str = path.to_string_lossy().to_string();
        list_directories(
            &path,
            args.depth,
            0,
            &path_str,
            &ignore_patterns,
            args.dirs_only,
        )?;
    }

    Ok(())
}
