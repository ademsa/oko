use std::path::PathBuf;

use clap::ArgAction::SetTrue;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about)]
pub struct Args {
    #[arg(help = "Pattern")]
    pub pattern: String,
    #[arg(short, long, help = "Count matches", action = SetTrue)]
    pub count: bool,
    #[arg(short, long, help = "Ignore case", action = SetTrue)]
    pub ignore_case: bool,
    #[arg(help = "File path")]
    pub path: Option<PathBuf>,
    #[arg(help = "Log level", default_value = "warn")]
    pub log_level: Option<String>,
}
