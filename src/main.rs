use clap::Parser;
use std::{fs::read_to_string, path::PathBuf};

/// Search for a pattern in a file
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The pattern to search for
    #[arg(value_name = "PATTERN")]
    pattern: String,

    /// The path to the file to search
    #[arg(value_name = "PATH")]
    path: PathBuf,
}

fn main() {
    let args = Cli::parse();

    // println!("pattern: {:?}, path: {:?}", args.pattern, args.path)

    // TODO: ensure the path points to a file and not a directory
    // TODO: use a more efficient mechanism. Reading the whole file into memory first is very
    // inefficient for large files for our case
    let content = read_to_string(&args.path).expect("could not read file");

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}
