use std::io::Result;

use confy::load;
use log::info;
use owo_colors::AnsiColors;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct Config {
    content_color: String,
    match_color: String,
}

impl Config {
    pub fn new(content_color: String, match_color: String) -> Self {
        Self {
            content_color,
            match_color,
        }
    }

    pub fn get_content_color(&self) -> Result<Option<AnsiColors>> {
        let mut color: Option<AnsiColors> = None;
        if !self.content_color.eq("") {
            color = Some(AnsiColors::from(self.content_color.as_str()));
        }
        Ok(color)
    }

    pub fn get_match_color(&self) -> Result<Option<AnsiColors>> {
        let mut color: Option<AnsiColors> = None;
        if !self.match_color.eq("") {
            color = Some(AnsiColors::from(self.match_color.as_str()));
        }
        Ok(color)
    }
}

/// Get configuration
pub fn get_config(config_name: &str) -> Result<Config> {
    // Default config
    let default_config = Config::new("".to_string(), "green".to_string());

    // Store default configuration if file not found
    // Example on macOS: /Users/USERNAME/Library/Application\ Support/rs.minigrep/local.toml
    let cfg: Config = confy::get_configuration_file_path("minigrep", config_name)
        .and_then(|file_path| {
            if file_path.exists() {
                let cfg: Config = load("minigrep", config_name)?;
                Ok(cfg)
            } else {
                info!("Config file not found");
                confy::store("minigrep", config_name, &default_config)?;
                info!("Stored default config at {}", file_path.display());
                Ok(default_config)
            }
        })
        .unwrap();

    Ok(cfg)
}
