use clap::Parser;
use std::path::PathBuf;

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

    println!("pattern: {:?}, path: {:?}", args.pattern, args.path)
}
