use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod build;
mod render;
mod theme;
mod types;


#[derive(Parser)]
#[command(name = "pseudo", about = "Beautiful sites from simple YAML", version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Build a site from a directory of .yaml files
    Build {
        /// Source directory (default: current directory)
        #[arg(default_value = ".")]
        dir: PathBuf,
        /// Output directory
        #[arg(short, long, default_value = "_site")]
        out: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Build { dir, out } => build::run(&dir, &out),
    }
}
