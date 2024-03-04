#![doc = include_str!("../README.md")]

use std::io::Result;

use clap::Parser;
use log::info;

use crate::args::{Cli, Command};
use crate::config::{get_config, Config};
use crate::logging::setup_logging;
use crate::reader::get_reader;
use crate::writer::get_writer;

use minigreplib::output::{write_count_results, write_search_results};
use minigreplib::search::{count, search};

mod args;
mod config;
mod logging;
mod reader;
mod writer;

/// CLI
///
/// Example (Search "here" in content.txt file):
/// ```bash
/// minigrep here -i examples/content.txt
/// ```
/// or
/// ```bash
/// minigrep search here -i examples/content.txt
/// ```
/// or
/// ```bash
/// cat examples/content.txt | minigrep here
/// ```
fn main() -> Result<()> {
    // Parse arguments
    let args = Cli::parse();

    // Setup logging
    setup_logging(args.log_level).unwrap();

    info!("MINIGREP");

    // Get configuration
    let cfg: Config = get_config("local").unwrap();

    // Execute command or defaults
    match args.command {
        Some(Command::Search {
            pattern,
            ignore_case,
            input_path,
            output_path,
            output_format,
            output_line_number,
        }) => {
            // Get reader, ie content
            let mut reader = get_reader(input_path).unwrap();

            // Search pattern
            let results = search(&mut reader, &pattern, &ignore_case).unwrap();

            // Get writer
            let mut writer = get_writer(
                output_path,
                output_format,
                output_line_number,
                cfg.get_content_color().unwrap(),
                cfg.get_match_color().unwrap(),
            )
            .unwrap();

            // Output results
            write_search_results(results, &mut writer);
        }
        Some(Command::Count {
            pattern,
            ignore_case,
            input_path,
            output_path,
            output_format,
            output_line_number,
        }) => {
            // Get reader, ie content
            let mut reader = get_reader(input_path).unwrap();

            // Count pattern
            let results = count(&mut reader, &pattern, &ignore_case).unwrap();

            // Get writer
            let mut writer = get_writer(
                output_path,
                output_format,
                output_line_number,
                cfg.get_content_color().unwrap(),
                cfg.get_match_color().unwrap(),
            )
            .unwrap();

            // Output results
            write_count_results(results.to_string(), &mut writer);
        }
        None => {
            // Get reader, ie content
            let mut reader = get_reader(args.input_path).unwrap();

            // Search pattern
            let results = search(&mut reader, &args.pattern, &args.ignore_case).unwrap();

            // Get writer
            let mut writer = get_writer(
                args.output_path,
                args.output_format,
                args.output_line_number,
                cfg.get_content_color().unwrap(),
                cfg.get_match_color().unwrap(),
            )
            .unwrap();

            // Output results
            write_search_results(results, &mut writer);
        }
    }

    info!("Exiting...");

    Ok(())
}
