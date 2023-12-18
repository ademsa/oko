use anyhow::{Context, Result};
use clap::Parser;
use confy::load;
use env_logger;

use log::info;
use minigrep::find_matches;
use owo_colors::AnsiColors;
use serde::{Deserialize, Serialize};
use std::fs::read_to_string;
use std::io::stdout;
use std::path::PathBuf;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: PathBuf,
}

#[derive(Default, Debug, Serialize, Deserialize)]
struct Config {
    color: String,
}

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

    let args = Cli::parse();

    let content = read_to_string(&args.path)
        .with_context(|| format!("Error reading file {}", args.path.display()))?;

    find_matches(color, &content, &args.pattern, &mut stdout());

    info!("Bye!");
    Ok(())
}
