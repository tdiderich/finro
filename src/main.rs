use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod build;
mod dev;
mod render;
mod theme;
mod types;

#[derive(Parser)]
#[command(name = "pseudo", about = "Beautiful sites from simple YAML", version = "0.2.0")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Build a site from a directory of .yaml files
    Build {
        #[arg(default_value = ".")]
        dir: PathBuf,
        #[arg(short, long, default_value = "_site")]
        out: PathBuf,
    },
    /// Watch source, rebuild on change, serve at localhost:PORT
    Dev {
        #[arg(default_value = ".")]
        dir: PathBuf,
        #[arg(short, long, default_value = "_site")]
        out: PathBuf,
        #[arg(short, long, default_value_t = 3000)]
        port: u16,
    },
}

fn main() -> Result<()> {
    match Cli::parse().command {
        Command::Build { dir, out } => build::run(&dir, &out),
        Command::Dev { dir, out, port } => dev::run(&dir, &out, port),
    }
}
