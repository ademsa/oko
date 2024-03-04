use std::path::PathBuf;

use clap::ArgAction::SetTrue;
use clap::{command, Parser, Subcommand};

use minigreplib::output_format::OutputFormat;

#[derive(Parser)]
#[clap(
    author,
    name = "minigrep",
    about = "Search, Count, Transform and Output",
    after_help = "For help with specific command, run `minigrep help <command>`."
)]
#[clap(version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,

    // If command is not used, use these top level arguments
    #[arg(help = "Pattern", default_value = "")]
    pub pattern: String,
    #[arg(short = 'c', long, help = "Ignore case", action = SetTrue)]
    pub ignore_case: bool,
    #[arg(short = 'i', long, help = "Input file path")]
    pub input_path: Option<PathBuf>,
    #[arg(short = 'o', long, help = "Output file path [default: console]")]
    pub output_path: Option<PathBuf>,
    #[arg(value_enum, short = 'f', long, help = "Output format", default_value_t = OutputFormat::Plain)]
    pub output_format: OutputFormat,
    #[arg(short = 'n', long, help = "Output line number?", action = SetTrue)]
    pub output_line_number: bool,

    // Control logging
    #[arg(short = 'l', long, help = "Log level", default_value = "warn")]
    pub log_level: Option<String>,
}

#[derive(Subcommand)]
pub enum Command {
    #[clap(alias = "--search")]
    Search {
        #[arg(help = "Pattern")]
        pattern: String,
        #[arg(short = 'c', long, help = "Ignore case", action = SetTrue)]
        ignore_case: bool,
        #[arg(short = 'i', long, help = "Input file path")]
        input_path: Option<PathBuf>,
        #[arg(short = 'o', long, help = "Output file path [default: console]")]
        output_path: Option<PathBuf>,
        #[arg(value_enum, short = 'f', long, help = "Output format", default_value_t = OutputFormat::Plain)]
        output_format: OutputFormat,
        #[arg(short = 'n', long, help = "Output line number?", action = SetTrue)]
        output_line_number: bool,
    },

    #[clap(alias = "--count")]
    Count {
        #[arg(help = "Pattern")]
        pattern: String,
        #[arg(short = 'c', long, help = "Ignore case", action = SetTrue)]
        ignore_case: bool,
        #[arg(short = 'i', long, help = "Input file path")]
        input_path: Option<PathBuf>,
        #[arg(short = 'o', long, help = "Output file path [default: console]")]
        output_path: Option<PathBuf>,
        #[arg(value_enum, short = 'f', long, help = "Output format", default_value_t = OutputFormat::Plain)]
        output_format: OutputFormat,
        #[arg(short = 'n', long, help = "Output line number?", action = SetTrue)]
        output_line_number: bool,
    },
}
