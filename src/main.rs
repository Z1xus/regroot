use clap::Parser;
use std::{io, path::PathBuf};

pub mod list_directories;
use list_directories::list_directories;

#[derive(Parser)]
struct Args {
    /// Path of where do you want to mount your tree
    path: PathBuf,

    /// Depth of the tree
    #[arg(short, long, default_value_t = 0)]
    depth: usize,
}

fn main() -> io::Result<()> {
    let args: Args = Args::parse();

    let path = args.path;
    let max_depth = args.depth;
    let path_str = path.to_string_lossy().to_string();

    list_directories(path, max_depth, 0, &path_str).unwrap();

    Ok(())
}
