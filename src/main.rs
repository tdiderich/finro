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
mod wish;

#[derive(Parser)]
#[command(name = "kazam", about = "Beautiful sites from simple YAML", version)]
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
    /// Scaffold a new kazam site in <NAME>/
    Init { name: String },
    /// Print the LLM authoring guide (full AGENTS.md to stdout)
    Agents,
    /// Grant a wish — populated YAML from a short interview
    ///
    /// `kazam wish list` shows available wishes. `kazam wish deck` runs the
    /// QBR-deck interview. Pass `--agent claude|gemini|codex|opencode` to let
    /// the agent generate the YAML from your answers; pass `--stdout` to
    /// print the portable wish markdown for piping into any agent.
    Wish {
        /// Name of the wish (e.g., "deck", or "list" to see all)
        name: String,
        /// Where to write the populated YAML
        #[arg(short, long)]
        out: Option<PathBuf>,
        /// Shell out to this agent for rich generation
        #[arg(long, value_enum)]
        agent: Option<wish::Agent>,
        /// Print the wish markdown to stdout (pipe into any agent yourself)
        #[arg(long)]
        stdout: bool,
    },
}

fn main() -> Result<()> {
    match Cli::parse().command {
        Command::Build { dir, out, release } => build::run(&dir, &out, release),
        Command::Dev { dir, out, port } => dev::run(&dir, &out, port),
        Command::Init { name } => init::run(&name),
        Command::Agents => agents::run(),
        Command::Wish {
            name,
            out,
            agent,
            stdout,
        } => wish::run(&name, out, agent, stdout),
    }
}
