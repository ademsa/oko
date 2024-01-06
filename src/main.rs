#![doc = include_str!("../README.md")]

use anyhow::{Context, Result};
use clap::ArgAction::SetTrue;
use clap::Parser;
use confy::load;
use env_logger;

use log::info;
use minigreplib::{count, find, write_results};
use owo_colors::AnsiColors;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{stdout, BufRead, BufReader};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about)]
struct Args {
    #[arg(help = "Pattern")]
    pattern: String,
    #[arg(short, long, help = "Count matches", action = SetTrue)]
    count: bool,
    #[arg(short, long, help = "Ignore case", action = SetTrue)]
    ignore_case: bool,
    #[arg(help = "File path")]
    path: Option<PathBuf>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
struct Config {
    color: String,
}

/// CLI entrypoint
///
/// Example (Search "my" in content.txt file):
/// ```bash
/// minigrep my content.txt
/// ```
/// or
/// ```bash
/// cat content.txt | minigrep my
/// ```
fn main() -> Result<()> {
    // Initialise logger
    env_logger::init();

    info!("Welcome!");

    // Default config
    let default_config = Config {
        color: "green".to_string(),
    };

    // Store default configuration file if none found
    // Config file path: /Users/USERNAME/Library/Application\ Support/rs.minigrep/local.toml
    let cfg: Config =
        confy::get_configuration_file_path("minigrep", "local").and_then(|file_path| {
            if file_path.exists() {
                let cfg: Config = load("minigrep", "local")?;
                Ok(cfg)
            } else {
                info!("Config file not found");
                confy::store("minigrep", "local", &default_config)?;
                info!("Stored default config at {}", file_path.display());
                Ok(default_config)
            }
        })?;

    // Display cfg
    //dbg!(&cfg);

    // Find color from configuration
    let color = AnsiColors::from(cfg.color.as_str());

    // Parse arguments
    let args = Args::parse();
    let mut results = vec![];
    if !args.path.is_none() {
        // Read file contents with BufReader
        let file_path = &args.path.unwrap();
        let file = File::open(file_path)
            .with_context(|| format!("Error reading file {}", file_path.display()))
            .unwrap();
        let mut reader = BufReader::new(file);

        // Find or Count pattern in content
        if args.count {
            results.push(count(&mut reader, &args.pattern, &args.ignore_case)?.to_string());
        } else {
            results = find(&mut reader, &args.pattern, &args.ignore_case, color)?;
        }
    } else {
        let mut input = std::io::stdin().lock();
        let mut reader = input.fill_buf()?;

        // Find or Count pattern in content
        if args.count {
            results.push(count(&mut reader, &args.pattern, &args.ignore_case)?.to_string());
        } else {
            results = find(&mut reader, &args.pattern, &args.ignore_case, color)?;
        }
    }

    // Write results
    write_results(results, stdout());

    info!("Bye!");
    Ok(())
}
