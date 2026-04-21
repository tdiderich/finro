# kazam

**Beautiful static sites from simple YAML. One Rust binary, no framework, no npm, no runtime JS. Designed so AI agents can author your site end-to-end.**

**[▶ 3-minute demo](https://www.loom.com/share/528d98fef421443497753af86cd7d737)** · **[Docs + live examples](https://tdiderich.github.io/kazam/)** · **[60-second quickstart](https://tdiderich.github.io/kazam/guide.html)** · **[Full tour](https://tdiderich.github.io/kazam/about.html)**

---

## Install

```bash
# Homebrew (macOS / Linux)
brew install tdiderich/tap/kazam

# Cargo (any platform with Rust)
cargo install kazam

# Bleeding edge
cargo install --git https://github.com/tdiderich/kazam
```

## Quickstart

```bash
kazam init my-site && cd my-site
kazam dev . --port 3000    # → http://localhost:3000, live reload
```

Edit `index.yaml`. Save. The browser reloads. That's the loop.

To build a static bundle: `kazam build . --out dist --release`. Drop `dist/` on any static host — [deploy recipes](https://tdiderich.github.io/kazam/deploy.html) cover Vercel, Netlify, Cloudflare Pages, GitHub Pages, S3 + CloudFront, Firebase, and self-hosted nginx.

## Let your agent write it

```bash
kazam wish deck --yolo "about me, based on our interaction history"
```

One command, one populated deck. Or drop real context into `wish-deck/` and run `kazam wish deck` without `--yolo` — the agent reads your files (notes, transcripts, prior decks, PDFs) and writes the deck from them. Works with Claude Code, Gemini CLI, Codex, and OpenCode. Full walkthrough: [wishes](https://tdiderich.github.io/kazam/wishes.html).

## Docs

Everything lives at **[tdiderich.github.io/kazam](https://tdiderich.github.io/kazam/)**:

- **[Quickstart](https://tdiderich.github.io/kazam/guide.html)** — install, scaffold, run
- **[Full tour](https://tdiderich.github.io/kazam/about.html)** — how pages are shaped, shells, kazam.yaml, starter pages
- **[Components](https://tdiderich.github.io/kazam/components/index.html)** — every primitive with live examples
- **[Themes](https://tdiderich.github.io/kazam/themes.html)** — stock palettes and overrides
- **[Recipes](https://tdiderich.github.io/kazam/wishes.html)** — `kazam wish` for agent-authored artifacts, `freshness:` for KB review tracking
- **[Deploy](https://tdiderich.github.io/kazam/deploy.html)** — copy-paste host configs

## Built for AI agents

kazam's #1 audience isn't humans typing YAML — it's Claude, GPT, and Codex generating it. Run `kazam agents` to print the exact authoring guide for the installed version. `kazam init` scaffolds `AGENTS.md` and `llms.txt` into new sites so any agent opening the repo finds them without being told.

## Security

Zero runtime JS on the output, ~10 direct Rust crates, `Cargo.lock` committed, `cargo-audit` in CI, protected main, signed release tags. Full scope: [`SECURITY.md`](SECURITY.md). Report vulnerabilities privately via the [GitHub advisory form](https://github.com/tdiderich/kazam/security/advisories/new) — not a public issue.

## Contributing

PRs welcome — agent-assisted contributions explicitly encouraged. See [`CONTRIBUTING.md`](CONTRIBUTING.md).

## License

MIT — see [`LICENSE`](LICENSE).
