//! `kazam wish <name>` — point the agent at a workspace full of your real
//! context and get back a populated kazam YAML.
//!
//! kazam's job: scaffold the workspace and shell out to an agent. File
//! parsing, PDF handling, large-file chunking — all of that is the agent's
//! problem. kazam just sets the CWD to the workspace and tells the agent to
//! read whatever's there.
//!
//! Each wish (see `deck.rs`) declares:
//!   * a template `questions.md` written to the workspace on init
//!   * a template `README.md` explaining what to drop in
//!   * a static agent prompt that tells the agent how to turn the workspace
//!     into a valid kazam YAML for this wish type
//!   * a markdown spec for `--stdout` mode (portable, pipe into any agent)
//!
//! Flow:
//!   kazam wish deck              → scaffold ./wish-deck/ if missing, else
//!                                  shell out to the first agent on $PATH
//!                                  from inside the workspace to write deck.yaml
//!   kazam wish deck --agent X    → force a specific agent
//!   kazam wish deck --dry-run    → print the prompt that would be sent
//!   kazam wish deck --stdout     → print the portable wish markdown spec
//!   kazam wish list              → enumerate available wishes

use anyhow::{anyhow, bail, Context, Result};
use clap::ValueEnum;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

mod brief;
mod deck;

/// Shared instruction block spliced into every wish's `--yolo` prompt. Tells
/// the agent it can use whatever MCP tools it has — calendar, slack, linear,
/// granola, gmail — but only when the topic looks like the user's own
/// work/life, not for a public/external subject. Placed in `mod.rs` so all
/// wishes opt into the same wording; if the policy changes (or a wish wants
/// to opt out), it changes once here.
pub const MCP_GUIDANCE: &str = r#"## You may have MCP tools available — use them when the topic warrants it

If the agent runtime exposes MCP servers (Google Calendar, Gmail, Slack,
Linear, Granola, HubSpot, Attention, etc.), they are fair game when the
topic is clearly about the *user's* world: a meeting on their calendar, a
deal in their CRM, a ticket they own, an incident in their channels, a
person they work with. Use them to gather real, current context before
writing — a brief about "my 1:1 with Sam tomorrow" should pull the calendar
event, recent shared meetings, and any open tickets between them.

Do NOT use MCP for public/external topics ("the history of TLS", "what is
RAG", "a deck about coffee"). Those are general-knowledge subjects — pulling
the user's private data into them is a privacy leak, not helpful research.
When in doubt, lean toward NOT calling MCP tools.

When you do call MCP tools, keep it tasteful: a few targeted queries, not a
fishing expedition. The output is a finished artifact, not a research log —
don't cite tool calls or surface raw IDs."#;

#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum Agent {
    Claude,
    Gemini,
    Codex,
    Opencode,
}

impl Agent {
    fn label(&self) -> &'static str {
        match self {
            Agent::Claude => "Claude Code",
            Agent::Gemini => "Gemini CLI",
            Agent::Codex => "Codex",
            Agent::Opencode => "OpenCode",
        }
    }

    fn bin(&self) -> &'static str {
        match self {
            Agent::Claude => "claude",
            Agent::Gemini => "gemini",
            Agent::Codex => "codex",
            Agent::Opencode => "opencode",
        }
    }

    /// Build the `Command` that feeds `prompt` to the agent in non-interactive
    /// mode and prints the generated text to stdout.
    fn command(&self, prompt: &str) -> Command {
        let mut c = Command::new(self.bin());
        match self {
            Agent::Claude | Agent::Gemini => {
                c.arg("-p").arg(prompt);
            }
            Agent::Codex => {
                c.arg("exec").arg(prompt);
            }
            Agent::Opencode => {
                c.arg("run").arg(prompt);
            }
        }
        c
    }

    /// Order matters — first match on $PATH wins when `--agent` isn't given.
    const AUTO_DETECT_ORDER: &'static [Agent] =
        &[Agent::Claude, Agent::Gemini, Agent::Codex, Agent::Opencode];
}

/// A wish: workspace scaffolding + agent prompt + portable markdown.
pub struct Wish {
    pub name: &'static str,
    pub description: &'static str,
    /// Where the populated YAML is written (e.g. "deck.yaml"). Next to the
    /// workspace directory, not inside it.
    pub default_out: &'static str,
    /// Template written to `<workspace>/questions.md` on init.
    pub questions_md: &'static str,
    /// Template written to `<workspace>/README.md` on init.
    pub readme_md: &'static str,
    /// Reference material dropped into `<workspace>/reference/` on init —
    /// schema guide, example decks, anything the agent should consult for
    /// shape/validation. Pairs of (filename, contents).
    pub references: &'static [(&'static str, &'static str)],
    /// The single prompt shelled out to the agent. The agent is run with its
    /// CWD set to the workspace, so the prompt refers to files in "this
    /// directory" and lets the agent read them with its own tools.
    pub agent_prompt: &'static str,
    /// Builds the prompt for `--yolo` mode: no workspace, no questions —
    /// the agent invents the whole thing. Topic is optional; `None` means
    /// "surprise me." The prompt embeds the schema inline since there's no
    /// workspace `reference/` folder to read from.
    pub yolo_prompt: fn(Option<&str>) -> String,
    /// Portable spec printed by `--stdout`.
    pub stdout_markdown: &'static str,
}

fn all() -> &'static [&'static Wish] {
    const ALL: &[&Wish] = &[&deck::DECK, &brief::BRIEF];
    ALL
}

fn find(name: &str) -> Option<&'static Wish> {
    all().iter().copied().find(|w| w.name == name)
}

pub fn run(
    name: &str,
    out: Option<PathBuf>,
    agent: Option<Agent>,
    stdout_mode: bool,
    dry_run: bool,
    yolo: Option<String>,
) -> Result<()> {
    if name == "list" {
        return list();
    }

    let wish =
        find(name).ok_or_else(|| anyhow!("unknown wish '{}'. Try `kazam wish list`", name))?;

    if stdout_mode {
        print!("{}", wish.stdout_markdown);
        return Ok(());
    }

    // Every path from here ends in a `kazam dev .` suggestion, which needs
    // kazam.yaml. Auto-create a minimal one if missing so users can run
    // `kazam wish` in any empty directory and have the whole flow just work.
    ensure_site_config()?;

    if let Some(topic) = yolo {
        let topic = topic.trim();
        let topic = if topic.is_empty() { None } else { Some(topic) };
        return grant_yolo(wish, topic, out, agent, dry_run);
    }

    let workspace = PathBuf::from(format!("wish-{}", wish.name));
    if !workspace.exists() {
        scaffold(wish, &workspace)?;
        return Ok(());
    }

    // Workspace exists → grant.
    grant(wish, &workspace, out, agent, dry_run)
}

/// Write a minimal `kazam.yaml` in the current directory if one isn't
/// there already. Name derives from the CWD's basename. Lets
/// `kazam wish` work in any empty directory without forcing the user to
/// hand-author site config first.
fn ensure_site_config() -> Result<()> {
    let cfg = Path::new("kazam.yaml");
    if cfg.exists() {
        return Ok(());
    }
    let name = std::env::current_dir()
        .ok()
        .as_deref()
        .and_then(|p| p.file_name())
        .and_then(|n| n.to_str())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .unwrap_or_else(|| "Site".to_string());
    fs::write(cfg, format!("name: {}\ntheme: dark\n", name)).context("writing kazam.yaml")?;
    println!("  (wrote a minimal kazam.yaml — site config for `kazam dev` / `kazam build`)");
    Ok(())
}

fn scaffold(wish: &Wish, workspace: &Path) -> Result<()> {
    fs::create_dir_all(workspace)
        .with_context(|| format!("creating workspace {}", workspace.display()))?;
    fs::write(workspace.join("questions.md"), wish.questions_md)?;
    fs::write(workspace.join("README.md"), wish.readme_md)?;

    // Reference material — version-matched to this binary. The agent reads
    // these during grant; the user can edit or inspect them too.
    if !wish.references.is_empty() {
        let ref_dir = workspace.join("reference");
        fs::create_dir_all(&ref_dir).with_context(|| format!("creating {}", ref_dir.display()))?;
        for (name, contents) in wish.references {
            fs::write(ref_dir.join(name), contents)
                .with_context(|| format!("writing reference/{}", name))?;
        }
    }

    println!();
    println!("  ✨ Making your {} wish — {}", wish.name, wish.description);
    println!();
    println!("  Created workspace: {}/", workspace.display());
    println!("    questions.md       structured prompts — fill in what you know");
    println!("    README.md          what to drop in this folder");
    if !wish.references.is_empty() {
        println!("    reference/         kazam schema + worked example (read-only to the agent)");
    }
    println!();
    println!("  Next:");
    println!(
        "    1. Fill in {}/questions.md with whatever you want to",
        workspace.display()
    );
    println!("       answer up front (blanks are fine — the agent will infer).");
    println!("    2. Drop any real context (docs, notes, transcripts, last");
    println!("       quarter's deck) into {}/.", workspace.display());
    println!("    3. Run `kazam wish {}` again to grant it.", wish.name);
    println!();

    Ok(())
}

fn grant(
    wish: &Wish,
    workspace: &Path,
    out: Option<PathBuf>,
    agent: Option<Agent>,
    dry_run: bool,
) -> Result<()> {
    if dry_run {
        print!("{}", wish.agent_prompt);
        return Ok(());
    }

    let out_path = out.unwrap_or_else(|| PathBuf::from(wish.default_out));
    if out_path.exists() {
        bail!(
            "'{}' already exists — pass `--out <path>` to write elsewhere",
            out_path.display()
        );
    }

    let agent = agent.or_else(detect_agent).ok_or_else(no_agent_error)?;

    println!();
    println!(
        "  ✨ Granting your {} wish via {}",
        wish.name,
        agent.label()
    );
    println!();
    println!(
        "  Running {} in {} (can take 30-90s)…",
        agent.bin(),
        workspace.display()
    );
    println!();

    let yaml = run_agent(agent, wish.agent_prompt, workspace)?;
    fs::write(&out_path, &yaml).with_context(|| format!("writing {}", out_path.display()))?;

    let page_url = out_path
        .file_stem()
        .and_then(|s| s.to_str())
        .map(|stem| format!("http://localhost:3000/{}.html", stem))
        .unwrap_or_else(|| "http://localhost:3000/".to_string());

    println!();
    println!("  ✓ Wrote {}", out_path.display());
    println!();
    println!("  Next:");
    println!("    kazam dev .");
    println!("    → open {}", page_url);
    println!();

    Ok(())
}

fn grant_yolo(
    wish: &Wish,
    topic: Option<&str>,
    out: Option<PathBuf>,
    agent: Option<Agent>,
    dry_run: bool,
) -> Result<()> {
    let prompt = (wish.yolo_prompt)(topic);

    if dry_run {
        print!("{}", prompt);
        return Ok(());
    }

    let out_path = out.unwrap_or_else(|| PathBuf::from(wish.default_out));
    if out_path.exists() {
        bail!(
            "'{}' already exists — pass `--out <path>` to write elsewhere",
            out_path.display()
        );
    }

    let agent = agent.or_else(detect_agent).ok_or_else(no_agent_error)?;

    let cwd = std::env::current_dir().context("reading current directory")?;
    println!();
    match topic {
        Some(t) => println!(
            "  🎲 Granting a YOLO {} wish via {} — topic: {}",
            wish.name,
            agent.label(),
            t
        ),
        None => println!(
            "  🎲 Granting a YOLO {} wish via {} — topic: surprise me",
            wish.name,
            agent.label()
        ),
    }
    println!();
    println!("  Running {} (can take 30-90s)…", agent.bin());
    println!();

    let yaml = run_agent(agent, &prompt, &cwd)?;
    fs::write(&out_path, &yaml).with_context(|| format!("writing {}", out_path.display()))?;

    let page_url = out_path
        .file_stem()
        .and_then(|s| s.to_str())
        .map(|stem| format!("http://localhost:3000/{}.html", stem))
        .unwrap_or_else(|| "http://localhost:3000/".to_string());

    println!();
    println!("  ✓ Wrote {}", out_path.display());
    println!();
    println!("  Next:");
    println!("    kazam dev .");
    println!("    → open {}", page_url);
    println!();

    Ok(())
}

fn detect_agent() -> Option<Agent> {
    Agent::AUTO_DETECT_ORDER
        .iter()
        .copied()
        .find(|a| cmd_on_path(a.bin()))
}

fn no_agent_error() -> anyhow::Error {
    anyhow!(
        "no agent CLI found on $PATH.

kazam wish shells out to your agent to write the YAML. Install any one of:

  Claude Code    https://docs.claude.com/en/docs/claude-code/install
  Gemini CLI     https://github.com/google-gemini/gemini-cli
  Codex          https://developers.openai.com/codex/cli
  OpenCode       https://opencode.ai

Then re-run this command. Or for a no-CLI-agent path, pass --dry-run
to print the prompt and paste it into ChatGPT, Claude.ai, Gemini web,
or any other LLM context window — save what comes back as deck.yaml."
    )
}

fn cmd_on_path(name: &str) -> bool {
    let path = match std::env::var_os("PATH") {
        Some(p) => p,
        None => return false,
    };
    for dir in std::env::split_paths(&path) {
        if dir.join(name).is_file() {
            return true;
        }
    }
    false
}

fn run_agent(agent: Agent, prompt: &str, cwd: &Path) -> Result<String> {
    let mut child = agent
        .command(prompt)
        .current_dir(cwd)
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .with_context(|| {
            format!(
                "could not start `{}` — is it installed and on your $PATH?",
                agent.bin()
            )
        })?;

    let mut stdout = child.stdout.take().expect("captured stdout");
    let mut raw = String::new();
    stdout.read_to_string(&mut raw)?;
    let status = child.wait()?;
    if !status.success() {
        bail!("{} exited with {}", agent.label(), status);
    }
    Ok(strip_code_fence(&raw))
}

/// Strip a fenced ```yaml ... ``` block if the agent wrapped its output. Falls
/// back to the raw string.
fn strip_code_fence(s: &str) -> String {
    let trimmed = s.trim();
    if let Some(rest) = trimmed.strip_prefix("```") {
        let after_lang = rest.find('\n').map(|i| &rest[i + 1..]).unwrap_or(rest);
        if let Some(body) = after_lang.rsplit_once("```") {
            return body.0.trim_end().to_string() + "\n";
        }
    }
    if !trimmed.ends_with('\n') {
        format!("{}\n", trimmed)
    } else {
        trimmed.to_string()
    }
}

fn list() -> Result<()> {
    println!();
    println!("  Available wishes:");
    println!();
    for w in all() {
        println!("    kazam wish {:<10}  {}", w.name, w.description);
    }
    println!();
    println!("  Flags:");
    println!("    --agent <claude|gemini|codex|opencode>   Force a specific agent");
    println!("    --yolo [topic]                           Skip workspace, let the agent invent everything");
    println!("    --dry-run                                Print the prompt instead of running the agent");
    println!("    --stdout                                 Print the portable wish markdown");
    println!("    --out <path>                             Where to write the populated YAML");
    println!();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strip_code_fence_removes_yaml_wrapper() {
        let raw = "```yaml\ntitle: hi\nshell: deck\n```\n";
        let s = strip_code_fence(raw);
        assert!(s.starts_with("title: hi"));
        assert!(!s.contains("```"));
    }

    #[test]
    fn strip_code_fence_passes_through_plain_yaml() {
        let raw = "title: hi\nshell: deck\n";
        let s = strip_code_fence(raw);
        assert!(s.starts_with("title: hi"));
    }

    #[test]
    fn all_wishes_have_unique_names() {
        let mut names: Vec<&str> = all().iter().map(|w| w.name).collect();
        names.sort();
        let before = names.len();
        names.dedup();
        assert_eq!(before, names.len(), "duplicate wish names");
    }
}
