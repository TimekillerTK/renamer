use clap::{arg, command, Parser};

/// CLI struct containing various options
#[derive(Parser, Debug, Clone)]
#[command(version)]
#[command(propagate_version = true)]
pub struct Cli {
    #[arg(long, short)]
    pub path: Option<String>,
}
