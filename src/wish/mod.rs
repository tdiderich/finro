//! `kazam wish <name>` — ask a few questions, emit a populated kazam YAML.
//!
//! Each wish is a small Rust module (see `deck.rs`) that declares:
//!   * the interview questions
//!   * a pure Rust template that fills a YAML file from the answers
//!   * an agent prompt template for `--agent` mode
//!   * a markdown spec for `--stdout` mode (pipe into any LLM)
//!
//! The wish *is the skill*: same markdown works in kazam's TUI, shelled out
//! to `claude -p` / `gemini -p` / `codex exec` / `opencode run`, or pasted
//! into any other agent's context.
//!
//! Flow:
//!   kazam wish deck                     → kazam asks the questions, writes deck.yaml
//!   kazam wish deck --agent claude      → kazam asks the questions, shells out for rich generation
//!   kazam wish deck --stdout            → prints the wish markdown for piping anywhere
//!   kazam wish list                     → lists available wishes

use anyhow::{anyhow, bail, Context, Result};
use clap::ValueEnum;
use std::fs;
use std::io::{self, BufRead, Read, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};

mod deck;

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

    /// Build the `Command` that feeds `prompt` to the agent in non-interactive
    /// mode and prints the generated text to stdout.
    fn command(&self, prompt: &str) -> Command {
        match self {
            Agent::Claude => {
                let mut c = Command::new("claude");
                c.arg("-p").arg(prompt);
                c
            }
            Agent::Gemini => {
                let mut c = Command::new("gemini");
                c.arg("-p").arg(prompt);
                c
            }
            Agent::Codex => {
                let mut c = Command::new("codex");
                c.arg("exec").arg(prompt);
                c
            }
            Agent::Opencode => {
                let mut c = Command::new("opencode");
                c.arg("run").arg(prompt);
                c
            }
        }
    }
}

/// One interview question asked by kazam's TUI.
pub struct Question {
    pub key: &'static str,
    pub prompt: &'static str,
    pub hint: Option<&'static str>,
    /// Multi-line: keep reading stdin lines until a blank line.
    pub multiline: bool,
}

/// Answers keyed by `Question::key`. Stored as plain strings — wishes decide
/// how to splice them into YAML or an LLM prompt.
pub struct Answers {
    values: Vec<(String, String)>,
}

impl Answers {
    pub fn new() -> Self {
        Answers { values: Vec::new() }
    }

    pub fn insert(&mut self, key: &str, value: String) {
        self.values.push((key.to_string(), value));
    }

    pub fn get(&self, key: &str) -> &str {
        self.values
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v.as_str())
            .unwrap_or("")
    }
}

impl Default for Answers {
    fn default() -> Self {
        Self::new()
    }
}

/// A wish: interview questions + rendering + agent prompt + stdout markdown.
pub struct Wish {
    pub name: &'static str,
    pub description: &'static str,
    pub default_out: &'static str,
    pub questions: &'static [Question],
    /// Pure Rust template — fills a YAML file from answers. Used when no
    /// `--agent` is given.
    pub render: fn(&Answers) -> String,
    /// Build the single prompt shelled out to the agent in `--agent` mode.
    pub agent_prompt: fn(&Answers) -> String,
    /// Markdown printed in `--stdout` mode — the portable "paste into any
    /// agent" form of the wish.
    pub stdout_markdown: &'static str,
}

fn all() -> &'static [&'static Wish] {
    const ALL: &[&Wish] = &[&deck::DECK];
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

    let out_path = out.unwrap_or_else(|| PathBuf::from(wish.default_out));
    if out_path.exists() {
        bail!(
            "'{}' already exists — pass `--out <path>` to write elsewhere",
            out_path.display()
        );
    }

    println!();
    println!("  ✨ kazam wish {} — {}", wish.name, wish.description);
    println!();
    match agent {
        Some(a) => println!(
            "  A few quick questions, then I'll hand off to {} to write the YAML.",
            a.label()
        ),
        None => println!("  A few quick questions, then I'll write the starter YAML."),
    }
    println!();

    let answers = interview(wish.questions)?;

    let yaml = match agent {
        Some(a) => generate_with_agent(a, (wish.agent_prompt)(&answers))?,
        None => (wish.render)(&answers),
    };

    fs::write(&out_path, &yaml).with_context(|| format!("writing {}", out_path.display()))?;

    println!();
    println!("  ✓ Wrote {}", out_path.display());
    println!();
    println!("  Next:");
    println!("    kazam dev .            # watch + serve at localhost:3000");
    println!();

    Ok(())
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
    println!("    --agent <claude|gemini|codex|opencode>   Let an agent write the YAML");
    println!(
        "    --stdout                                 Print the wish markdown (pipe anywhere)"
    );
    println!("    --out <path>                             Where to write the YAML");
    println!();
    Ok(())
}

fn interview(questions: &[Question]) -> Result<Answers> {
    let stdin = io::stdin();
    let mut answers = Answers::new();
    let mut input = stdin.lock();

    for (i, q) in questions.iter().enumerate() {
        println!("  {}. {}", i + 1, q.prompt);
        if let Some(h) = q.hint {
            println!("     ({})", h);
        }
        if q.multiline {
            println!("     (multiple lines OK — finish with a blank line)");
            let mut lines: Vec<String> = Vec::new();
            loop {
                print!("     > ");
                io::stdout().flush().ok();
                let mut buf = String::new();
                let n = input.read_line(&mut buf)?;
                if n == 0 {
                    break;
                }
                let trimmed = buf.trim_end_matches(&['\r', '\n'][..]).to_string();
                if trimmed.is_empty() {
                    break;
                }
                lines.push(trimmed);
            }
            answers.insert(q.key, lines.join("\n"));
        } else {
            print!("     > ");
            io::stdout().flush().ok();
            let mut buf = String::new();
            input.read_line(&mut buf)?;
            let answer = buf.trim_end_matches(&['\r', '\n'][..]).to_string();
            answers.insert(q.key, answer);
        }
        println!();
    }

    Ok(answers)
}

fn generate_with_agent(agent: Agent, prompt: String) -> Result<String> {
    println!();
    println!("  → Handing off to {}…", agent.label());
    println!();

    let mut child = agent
        .command(&prompt)
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .with_context(|| {
            format!(
                "could not start `{}` — is it installed and on your $PATH?",
                match agent {
                    Agent::Claude => "claude",
                    Agent::Gemini => "gemini",
                    Agent::Codex => "codex",
                    Agent::Opencode => "opencode",
                }
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
