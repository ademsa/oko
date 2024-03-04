use clap::ValueEnum;

#[derive(Clone, ValueEnum)]
pub enum OutputFormat {
    Plain,
    Json,
}
