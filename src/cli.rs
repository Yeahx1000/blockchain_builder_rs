use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "Rust Blockchain")]
#[command(about = "A simple blockchain implementation in Rust", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Init,
    Add { data: String },
    View,
    Validate,
    SetDifficulty { level: u32 },
}
