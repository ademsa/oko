use anyhow::{Context, Result};
use clap::Parser;
use env_logger;
use log::info;
use minigrep::find_matches;
use std::fs::read_to_string;
use std::io::stdout;
use std::path::PathBuf;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: PathBuf,
}

fn main() -> Result<()> {
    env_logger::init();
    info!("Welcome!");

    let args = Cli::parse();

    let content = read_to_string(&args.path)
        .with_context(|| format!("Error reading file {}", args.path.display()))?;

    find_matches(&content, &args.pattern, &mut stdout());

    info!("Bye!");
    Ok(())
}
