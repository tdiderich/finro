//! `finro agents` — dumps the bundled LLM authoring guide to stdout.
//!
//! Point an agent at the output of this command when you want it to generate
//! finro YAML. The guide is the same `AGENTS.md.template` that `finro init`
//! writes into new sites, with a short preamble that links out to the hosted
//! component catalog for the very latest reference.

use anyhow::Result;

const AGENTS_MD: &str = include_str!("../AGENTS.md.template");

pub fn run() -> Result<()> {
    let preamble = "\
<!-- finro agents — piped from the `finro agents` CLI command.

Full docs + live component catalog: https://tdiderich.github.io/finro/
Source: https://github.com/tdiderich/finro

The guide below is bundled with the `finro` binary you invoked, so it always
matches the version you have installed. If the hosted docs reference
components or props that look missing here, `cargo install --git
https://github.com/tdiderich/finro` to upgrade.
-->

";
    print!("{}{}", preamble, AGENTS_MD);
    Ok(())
}
