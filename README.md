# finro

YAML in, themed static HTML out. One Rust binary, no frontend build system.

**Full docs + live examples:** https://tdiderich.github.io/finro/

## Quickstart

Install Rust via [rustup](https://rustup.rs), then:

```bash
cargo install --git https://github.com/tdiderich/finro
finro --version         # finro 0.3.0
```

Scaffold a new site:

```bash
finro init my-site
cd my-site
```

This drops a `finro.yaml` (site config) + `index.yaml` (landing page) into `my-site/`.

## Dev loop

```bash
finro dev . --port 3000
```

Watches the directory, rebuilds on every `.yaml` save, serves at `localhost:3000` with live reload. Edit a file, switch tabs, see it.

## Build

```bash
finro build . --out dist                 # one-shot build
finro build . --out dist --release       # minified production build
```

Output is plain static HTML/CSS — drop it on S3, Pages, Vercel, or any static host. Recipes: https://tdiderich.github.io/finro/deploy.html

## Samples

```yaml
# index.yaml — a landing page
title: Acme
shell: standard

components:
  - type: header
    title: Acme
    subtitle: One binary, no build system
    align: center

  - type: card_grid
    connector: arrow
    cards:
      - title: Write YAML
        description: One file per page.
      - title: finro build
        description: Rust binary renders each page.
      - title: Ship static HTML
        description: Upload anywhere.
```

```yaml
# deck.yaml — a slide deck
title: Q1 Review
shell: deck

slides:
  - label: Cover
    hide_label: true
    components:
      - type: header
        title: Q1 Product Review
        subtitle: April 2026
        align: center

  - label: Wins
    components:
      - type: stat_grid
        stats:
          - { label: MAU, value: 142K, color: green }
          - { label: Revenue, value: $4.8M, color: green }
          - { label: Open P1s, value: "3", color: yellow }
```

```yaml
# finro.yaml — site config
name: Acme
theme: dark
favicon: assets/logo.svg
colors:
  accent: "#3CCECE"     # optional — pin any theme token
nav:
  - { label: Home, href: index.html }
  - { label: Docs, href: docs.html }
```

Live versions of each of these (and ~30 more components) are at https://tdiderich.github.io/finro/ — the docs site is itself built with finro.

## For LLMs

`AGENTS.md` is the authoring guide — point Claude/GPT/Codex at it when generating finro YAML. `finro init` also scaffolds an `AGENTS.md` + `llms.txt` into new sites so agents find them automatically.

## License

MIT — see `LICENSE`.
