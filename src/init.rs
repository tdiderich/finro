//! `finro init <name>` scaffolds a new site directory with a minimal,
//! well-commented starter — finro.yaml, index.yaml, AGENTS.md, .gitignore.

use anyhow::{bail, Context, Result};
use std::fs;
use std::path::Path;

pub fn run(path: &str) -> Result<()> {
    let dir = Path::new(path);
    if dir.exists() {
        bail!("'{}' already exists — choose another name or remove it first", path);
    }

    let site_name = dir.file_name()
        .and_then(|f| f.to_str())
        .unwrap_or("my-site")
        .to_string();

    fs::create_dir_all(dir).with_context(|| format!("creating {:?}", dir))?;

    fs::write(dir.join("finro.yaml"), FINRO_YAML.replace("{{SITE_NAME}}", &site_name))?;
    fs::write(dir.join("index.yaml"), INDEX_YAML.replace("{{SITE_NAME}}", &site_name))?;
    fs::write(dir.join("AGENTS.md"), AGENTS_MD)?;
    fs::write(dir.join(".gitignore"), GITIGNORE)?;

    println!("\n  ✓ Created {} with:", path);
    println!("    finro.yaml       site config (name, theme, nav)");
    println!("    index.yaml       home page");
    println!("    AGENTS.md        LLM authoring guide");
    println!("    .gitignore");
    println!();
    println!("  Next:");
    println!("    cd {}", path);
    println!("    finro dev .          # watch + serve at localhost:3000");
    println!();

    Ok(())
}

const FINRO_YAML: &str = r#"# Site configuration. Shared across every page.
name: {{SITE_NAME}}

# Theme: "dark" (default) or "light". Or add a `colors:` map below to
# override individual tokens of the chosen base theme.
theme: dark

# Uncomment and point to a favicon file (any non-.yaml file in this
# directory gets copied into the build output verbatim).
# favicon: favicon.svg

# Nav appears in the sticky header of every `shell: standard` page.
# Hrefs are auto-resolved per-page based on directory depth, so
# `index.html` works from any subdirectory.
nav:
  - label: Home
    href: index.html
"#;

const INDEX_YAML: &str = r#"# The home page. See AGENTS.md for the full component catalog.
title: {{SITE_NAME}}
shell: standard

components:
  - type: header
    title: {{SITE_NAME}}
    eyebrow: Welcome
    subtitle: A starter site scaffolded by `finro init`.

  - type: section
    eyebrow: Next steps
    heading: Make it yours
    components:
      - type: markdown
        body: |
          1. Edit this file (`index.yaml`) — every page is a list of components.
          2. Add new pages — any `.yaml` file in this directory becomes a page.
          3. Read `AGENTS.md` for the full component reference.

      - type: button_group
        buttons:
          - label: Read AGENTS.md
            href: AGENTS.md
            variant: primary
          - label: finro source
            href: https://github.com/tdiderich/finro
            variant: secondary
            external: true

  - type: section
    eyebrow: Components in action
    heading: A few primitives
    components:
      - type: stat_grid
        columns: 3
        stats:
          - label: Pages
            value: "1"
            detail: Only this one, for now.
            color: default
          - label: Components
            value: "30+"
            detail: All documented in AGENTS.md.
            color: green
          - label: Runtime
            value: "0"
            detail: Static HTML. Serve from anywhere.
            color: teal

      - type: callout
        variant: info
        title: Tip
        body: "Run `finro dev .` to watch your files and live-reload the browser on every save."
"#;

const GITIGNORE: &str = r#"/_site
"#;

const AGENTS_MD: &str = include_str!("../AGENTS.md.template");
