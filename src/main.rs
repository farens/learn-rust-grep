use clap::Parser;
use std::{error::Error, fs::read_to_string, path::PathBuf};

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

// TODO: for future cli apps and libs: have a look at thiserror and anyhow
fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    // TODO: ensure the path points to a file and not a directory
    // TODO: use a more efficient mechanism. Reading the whole file into memory first is very
    let content = read_to_string(&args.path).map_err(|err| {
        std::io::Error::new(
            err.kind(),
            format!("Error reading `{}`: {}", args.path.display(), err),
        )
    })?;

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    Ok(())
}
