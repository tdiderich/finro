use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod agents;
mod build;
mod dev;
mod freshness;
mod icons;
mod init;
mod links;
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
        /// Silence the orphan-page check (broken links still reported).
        /// Useful for draft pages you haven't wired into nav yet.
        #[arg(long)]
        allow_orphans: bool,
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
    /// Grant a wish — populated YAML from a workspace full of your context.
    /// First run scaffolds `./wish-<name>/` with `questions.md`, `README.md`,
    /// and a `reference/` folder containing the schema and a worked example.
    /// Fill in what you can, drop real context (docs, notes, transcripts)
    /// into the workspace, then run again to grant: kazam shells out to the
    /// first agent on $PATH with the workspace as CWD and writes the YAML.
    /// `kazam wish list` shows available wishes.
    Wish {
        /// Name of the wish (e.g., "deck", or "list" to see all)
        name: String,
        /// Where to write the populated YAML
        #[arg(short, long)]
        out: Option<PathBuf>,
        /// Force a specific agent (otherwise auto-detect first one on $PATH)
        #[arg(long, value_enum)]
        agent: Option<wish::Agent>,
        /// Print the portable wish markdown spec (no scaffold, no grant)
        #[arg(long)]
        stdout: bool,
        /// Print the grant prompt instead of running the agent
        #[arg(long)]
        dry_run: bool,
        /// YOLO mode: skip the workspace, let the agent invent everything.
        /// Pass a topic (`--yolo "about me"`) or use bare `--yolo` for a
        /// surprise.
        #[arg(long, value_name = "TOPIC", num_args = 0..=1, default_missing_value = "")]
        yolo: Option<String>,
    },
}

fn main() -> Result<()> {
    match Cli::parse().command {
        Command::Build {
            dir,
            out,
            release,
            allow_orphans,
        } => build::run(&dir, &out, release, allow_orphans),
        Command::Dev { dir, out, port } => dev::run(&dir, &out, port),
        Command::Init { name } => init::run(&name),
        Command::Agents => agents::run(),
        Command::Wish {
            name,
            out,
            agent,
            stdout,
            dry_run,
            yolo,
        } => wish::run(&name, out, agent, stdout, dry_run, yolo),
    }
}
