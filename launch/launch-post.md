# I deleted 1,500 lines of React and my personal site got better

Draft launch post for tylerdiderich.com. Tone: the voice from the kazam
docs landing — direct, slightly opinionated, grounded in real numbers.

---

A personal site has one job: render a few pages that describe who you
are and what you've built. For five years mine did that with Create
React App, Tailwind, React Router, `axios`, a `react-gravatar` dependency
I'd long forgotten why I added, and two Firebase cloud functions
patching over SPA quirks.

It was **1,722 lines of source** I didn't really maintain.

I rebuilt it in one conversation with Claude. The new version is
**210 lines of YAML**, zero npm dependencies, and ships as plain HTML
and CSS behind Firebase Hosting. No functions. No runtime JavaScript.
No framework. One Rust binary that turns the YAML into the site.

The binary is called [kazam](https://github.com/tdiderich/kazam). I
wrote it because the docs/static-site space has calcified around two
bad options:

- **Markdown** renders prose. That's fine for a blog post but wastes
  what the web can actually do — cards, stats, interactive tables,
  side-by-side layouts, deck shells, print-friendly one-pagers.
- **Next.js, Docusaurus, Mintlify, and friends** give you the rich
  output but drag in a thousand-package npm tree, a webpack build, a
  client-side runtime, and a supply chain you'll never audit. For a
  site that doesn't need any of it.

kazam is the middle path. You write YAML:

```yaml
title: About me
shell: standard

components:
  - type: header
    title: Tyler Diderich
    subtitle: Building Maze, formerly at Signifyd.

  - type: stat_grid
    columns: 3
    stats:
      - { label: Years in infra, value: "10", color: green }
      - { label: Companies, value: "4" }
      - { label: Side projects, value: "∞", color: yellow }
```

You get a themed static site. That's the whole tool.

## Why AI agents

The format is deliberately boring — typed components, narrow schemas,
no prose rules to interpret. That's exactly the shape large language
models produce correctly on the first try. kazam ships an `AGENTS.md`
authoring guide *inside the binary* (run `kazam agents`), and every
page of the [docs site](https://tdiderich.github.io/kazam/) was
written by an agent following that guide.

I expect agents to be the primary contributors going forward — both
to sites built with kazam and to kazam itself. The project is
structured around that assumption.

## Why no JavaScript

Static sites shouldn't carry a framework-sized attack surface. The
kazam output is HTML and CSS. No hydration, no client router, no
bundled framework. Your attack surface is whatever bytes your CDN
serves.

The binary itself has ~10 direct Rust crates. `Cargo.lock` is
committed, `cargo-audit` runs in CI, and every PR requires CODEOWNER
review on a branch-protected `main`. The bar for a new dependency
is "can we justify it in the PR?" — not "does it save us ten lines?"

## Receipts

The [before/after](https://tdiderich.github.io/kazam/about.html) of
my own CRA → kazam migration:

| | Before | After |
|---|---|---|
| Hand-authored LOC | 1,722 | 210 |
| npm dependencies | react, router, axios, tailwind, +1000s transitive | 0 |
| Build tool | webpack | one Rust binary |
| Deploy | Firebase Hosting + 2 cloud functions | Firebase Hosting (static only) |
| Migration effort | — | one conversation |

## Try it

```bash
cargo install --git https://github.com/tdiderich/kazam
kazam init my-site && cd my-site
kazam dev . --port 3000
```

Edit `index.yaml`. Save. The browser reloads. That's the loop.

Full docs and examples: https://tdiderich.github.io/kazam/
