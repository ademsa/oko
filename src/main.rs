#![doc = include_str!("../README.md")]

use anyhow::{Context, Result};
use clap::ArgAction::SetTrue;
use clap::Parser;
use confy::load;
use env_logger::Env;

use log::info;
use minigreplib::{count, count_regex, find, find_regex, write_results};
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
    #[arg(short, long, help = "Is pattern regex?", action = SetTrue)]
    regex: bool,
    #[arg(short, long, help = "Count matches", action = SetTrue)]
    count: bool,
    #[arg(short, long, help = "Ignore case", action = SetTrue)]
    ignore_case: bool,
    #[arg(help = "File path")]
    path: Option<PathBuf>,
    #[arg(help = "Log level", default_value = "warn")]
    log_level: Option<String>,
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
    let results;
    if !args.path.is_none() {
        // Read file contents with BufReader
        let file_path = &args.path.unwrap();
        let file = File::open(file_path)
            .with_context(|| format!("Error reading file {}", file_path.display()))
            .unwrap();
        let mut reader = BufReader::new(file);

        results = get_results(
            &mut reader,
            &args.pattern,
            &args.count,
            &args.regex,
            &args.ignore_case,
            color,
        )?;
    } else {
        // Read contents from stdin
        let mut input = std::io::stdin().lock();
        let mut reader = input.fill_buf()?;

        results = get_results(
            &mut reader,
            &args.pattern,
            &args.count,
            &args.regex,
            &args.ignore_case,
            color,
        )?;
    }

    // Write results
    write_results(results, stdout());

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

/// Find or Count pattern in content
fn get_results<R: BufRead>(
    reader: &mut R,
    pattern: &str,
    count_mode: &bool,
    regex: &bool,
    ignore_case: &bool,
    color: AnsiColors,
) -> Result<Vec<String>> {
    let mut results: Vec<String> = vec![];

    if *count_mode && *regex {
        results.push(count_regex(reader, &pattern)?.to_string());
    } else if *count_mode && !*regex {
        results.push(count(reader, &pattern, &ignore_case)?.to_string());
    } else if !*count_mode && *regex {
        results = find_regex(reader, &pattern, color)?;
    } else {
        results = find(reader, &pattern, &ignore_case, color)?;
    }

    Ok(results)
}
