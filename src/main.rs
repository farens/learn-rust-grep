use anyhow::{Context, Result};
use clap::Parser;
use std::{fs::read_to_string, io::stdout, path::PathBuf};

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

fn main() -> Result<()> {
    let args = Cli::parse();

    // TODO: ensure the path points to a file and not a directory
    // TODO: use a more efficient mechanism. Reading the whole file into memory first is very
    let content = read_to_string(&args.path)
        .with_context(|| format!("Failed to read file `{}`", args.path.display()))?;

    grrs::find_matches(&args.pattern, &content, &mut stdout())?;

    Ok(())
}
