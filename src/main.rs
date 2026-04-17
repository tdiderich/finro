use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod agents;
mod build;
mod dev;
mod icons;
mod init;
mod llms;
mod minify;
mod render;
mod theme;
mod types;

#[derive(Parser)]
#[command(name = "finro", about = "Beautiful sites from simple YAML", version)]
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
        /// Minify HTML, CSS, and JS in the output
        #[arg(short, long)]
        release: bool,
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
    /// Scaffold a new finro site in <NAME>/
    Init { name: String },
    /// Print the LLM authoring guide (full AGENTS.md to stdout)
    Agents,
}

fn main() -> Result<()> {
    match Cli::parse().command {
        Command::Build { dir, out, release } => build::run(&dir, &out, release),
        Command::Dev { dir, out, port } => dev::run(&dir, &out, port),
        Command::Init { name } => init::run(&name),
        Command::Agents => agents::run(),
    }
}
