# kazam

**Write YAML. Ship a themed static site. Skip the 1,500-line Node.js tax.**

One Rust binary. No framework, no npm, no runtime JS. Built so AI agents can author your site end-to-end.

**Full docs + live examples:** https://tdiderich.github.io/kazam/

---

## Why it matters

- **88% less source than React.** A real personal site migrated from Create-React-App: **1,722 → 210** hand-authored lines, **0 npm deps**. Full numbers in [`STATS.md`](STATS.md).
- **Agents are first-class authors.** `AGENTS.md` ships bundled in the binary. Point Claude / GPT / Codex at it and the YAML writes itself — no prose rules to interpret, just a typed schema that validates or doesn't.
- **No JavaScript supply chain.** The output is HTML and CSS. No webpack, no thousand-package transitive tree, no hydration, no `_next/`. Your attack surface is whatever your CDN serves.
- **Boring is the feature.** One binary, one format, one build step. `kazam dev` reloads on save; `kazam build --release` emits minified static HTML. That's the toolchain.

## Before → After

A real migration — not a pitch deck.

| | Before: Create-React-App | After: kazam |
|---|---|---|
| Hand-authored LOC | 1,722 | 210 |
| npm dependencies | react, react-router, axios, tailwind, react-gravatar, react-toggle + ~1000s transitive | **0** |
| Build tool | webpack | single Rust binary |
| Deploy | Firebase Hosting + 2 cloud functions | Firebase Hosting (static only) |
| Migration effort | — | one conversation |

Full breakdown: [`STATS.md`](STATS.md).

## Install

```bash
# Homebrew (macOS / Linux)
brew install tdiderich/tap/kazam

# Cargo (any platform with Rust — install via rustup.rs first)
cargo install kazam

# Bleeding edge — straight from main
cargo install --git https://github.com/tdiderich/kazam
```

## 60-second quickstart

```bash
kazam init my-site && cd my-site
kazam dev . --port 3000
```

Edit `index.yaml`. Save. The browser reloads. That's the loop.

## Dev & build

```bash
kazam dev   . --port 3000            # watch + live reload
kazam build . --out dist             # one-shot build
kazam build . --out dist --release   # minified production build
```

Output is plain static HTML/CSS — drop it on S3, Pages, Cloudflare, Firebase, Vercel, or any static host. Recipes: https://tdiderich.github.io/kazam/deploy.html

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
      - title: kazam build
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
# kazam.yaml — site config
name: Acme
theme: dark
favicon: assets/logo.svg
colors:
  accent: "#3CCECE"     # optional — pin any theme token
nav:
  - { label: Home, href: index.html }
  - { label: Docs, href: docs.html }
```

Live versions of each of these (and ~30 more components) are at https://tdiderich.github.io/kazam/ — the docs site is itself built with kazam.

## Built for AI agents

kazam's primary audience isn't humans typing YAML — it's Claude, GPT, and Codex generating it. Every page on the docs site was written that way.

- `AGENTS.md` is the authoring guide. It ships inside the binary: run `kazam agents` and it prints the exact syntax for the installed version. No drift between the docs an agent reads and the parser it's feeding.
- `kazam init` scaffolds `AGENTS.md` and `llms.txt` into new sites so any agent opening the repo finds them without being told.
- Components are deliberately typed, narrow, and composable — the shape LLMs produce correctly on the first try. Validation is structural: the YAML parses or it doesn't.

Going forward, agents are expected to be the #1 contributors — both to sites built with kazam and to kazam itself. The project is structured around that assumption: short schemas, strict validation, machine-readable guides, one binary per version.

## Security

Static sites shouldn't carry a Next.js-sized supply chain. kazam's doesn't.

- **Zero runtime JS on the output.** No hydration, no client router, no bundled framework. The attacker's surface is whatever bytes your CDN serves.
- **~10 direct Rust crates.** `Cargo.lock` committed, `cargo-audit` runs in CI, new dependencies require justification in the PR.
- **No network at build time.** Build scripts that reach the network are rejected. No post-install scripts, no npm-style drive-by compromise.
- **Protected main.** Branch protection, required CODEOWNER review, required CI (`cargo test` / `fmt` / `clippy -D warnings` / `cargo-audit`), no force-pushes. Release tags are signed.
- **Pin a specific version** for reproducibility: `cargo install kazam --version 0.4.0` (or `--git … --rev <sha>` for an unreleased commit).

Report vulnerabilities privately via the [GitHub advisory form](https://github.com/tdiderich/kazam/security/advisories/new) — **do not** open a public issue. Full scope and supply-chain protections: [`SECURITY.md`](SECURITY.md).

## Contributing

PRs welcome — agent-assisted contributions explicitly encouraged. See [`CONTRIBUTING.md`](CONTRIBUTING.md) for the fork/PR flow, local dev checks, and guidance on authoring changes with an agent.

## License

MIT — see [`LICENSE`](LICENSE).
