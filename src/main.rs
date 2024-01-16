#![doc = include_str!("../README.md")]

use log::info;
use std::fs::File;
use std::io::{stdin, stdout, BufRead, BufReader};

use anyhow::{Context, Result};
use clap::Parser;
use confy::load;
use env_logger::Env;
use owo_colors::AnsiColors;

use crate::args::Args;
use crate::config::Config;

mod args;
mod config;

use minigreplib::{count, find, write_count_results, write_find_results};

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
    // Parse arguments
    let args = Args::parse();

    // Setup logging
    setup_logging(args.log_level).unwrap();

    info!("Minigrep");

    // Get configuration
    let cfg: Config = get_config()?;

    // Find color from configuration
    let color = AnsiColors::from(cfg.color.as_str());

    // Read content
    let mut reader: Box<dyn BufRead> = match &args.path {
        None => Box::new(BufReader::new(stdin())),
        Some(file_path) => Box::new(BufReader::new(
            File::open(file_path)
                .with_context(|| format!("Error reading file {}", file_path.display()))
                .unwrap(),
        )),
    };

    if args.count {
        let results = count(&mut reader, &args.pattern, &args.ignore_case)?;
        write_count_results(results.to_string(), color, stdout());
    } else {
        let results = find(&mut reader, &args.pattern, &args.ignore_case)?;
        write_find_results(results, color, stdout());
    }

    info!("Exiting...");

    Ok(())
}

fn setup_logging(log_level: Option<String>) -> Result<()> {
    let env = Env::default().filter_or("MINIGREP_LOG_LEVEL", log_level.unwrap());

    env_logger::init_from_env(env);

    Ok(())
}

/// Get configuration
fn get_config() -> Result<Config> {
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

    Ok(cfg)
}
