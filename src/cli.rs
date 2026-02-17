use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "mimic", version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Validate fragment files for correctness
    Lint {
        /// Show warnings in addition to errors
        #[arg(short, long)]
        warnings: bool,
    },
}
