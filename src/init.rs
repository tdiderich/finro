//! `finro init <name>` scaffolds a new site directory with a minimal,
//! well-commented starter — finro.yaml, index.yaml, AGENTS.md, .gitignore.

use anyhow::{bail, Context, Result};
use std::fs;
use std::path::Path;

pub fn run(path: &str) -> Result<()> {
    let dir = Path::new(path);
    if dir.exists() {
        bail!(
            "'{}' already exists — choose another name or remove it first",
            path
        );
    }

    let site_name = dir
        .file_name()
        .and_then(|f| f.to_str())
        .unwrap_or("my-site")
        .to_string();

    fs::create_dir_all(dir).with_context(|| format!("creating {:?}", dir))?;

    fs::write(
        dir.join("finro.yaml"),
        FINRO_YAML.replace("{{SITE_NAME}}", &site_name),
    )?;
    fs::write(
        dir.join("index.yaml"),
        INDEX_YAML.replace("{{SITE_NAME}}", &site_name),
    )?;
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

const FINRO_YAML: &str = r##"# Site configuration. Shared across every page.
# Every available field is listed below — uncomment the ones you want.

name: {{SITE_NAME}}

# Theme: "dark" (default) or "light".
theme: dark

# Override individual theme tokens. Keys not listed here fall back to the
# base theme's default. Full list of tokens: bg, surface, surface_strong,
# border, border_strong, accent, accent_soft, text, text_muted, text_subtle,
# overlay_hover, green, yellow, red, header_border.
# colors:
#   accent: "#3CCECE"          # primary brand color (links, eyebrows, borders)
#   bg: "#090D18"              # page background
#   text: "#F0F0F7"             # primary text
#   text_muted: "#ABABC1"       # secondary text
#   text_subtle: "#4C556A"      # labels, captions
#   green: "#34D399"
#   yellow: "#FBBF24"
#   red: "#F87171"

# Point to a favicon. Any non-.yaml file in this directory is copied
# verbatim into the build output, so a relative path just works.
# Simple form: one file for every slot.
# favicon: favicon.svg
# Full form: named slots for each icon variant.
# favicon:
#   svg: favicon.svg
#   png: favicon.png
#   ico: favicon.ico
#   apple_touch_icon: apple-touch-icon.png

# Opt-in: render a companion `<page>.source.html` for every page and show
# a floating "View source" pill that links to it. Handy for docs/demo
# sites. Off by default — most sites don't need it.
# view_source: true

# Subtle background pattern painted behind every page. Tinted via the
# theme's text color so it adapts to dark/light. Off by default.
# Options: none | dots | grid | grain | topography | diagonal
# texture: dots

# Soft accent-colored radial glow painted behind the page header area.
# Off by default. Options: none | accent | corner
# glow: accent

# Nav appears in the sticky header of every `shell: standard` page.
# Hrefs are auto-resolved per-page based on directory depth, so
# `index.html` works from any subdirectory.
nav:
  - label: Home
    href: index.html
  # - label: Docs
  #   href: docs.html
  # - label: GitHub
  #   href: https://github.com/your-org/your-repo
"##;

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
