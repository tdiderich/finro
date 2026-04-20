# Changelog

All notable changes to kazam are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and versioning
follows [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- `AGENTS.md` bug-filing + feature-request protocols. When an agent
  reproduces a bug or has a kazam-shaped feature idea, the guide now
  tells it to check `gh auth`, dedup against existing issues/PRs
  (including closed ones — a closed bug may mean the fix shipped in a
  newer version), then file with a consistent template. Feature
  requests also include a scope-check step ("does this fit kazam?")
  before filing, so wontfix noise stays down.

### Fixed
- Every component that emits an `href` now routes through the canonical
  `resolve_href` helper, honoring the verbatim-prefix rule documented
  in `AGENTS.md` (`/`, `http://`, `https://`, `#`, `mailto:`, `tel:`
  pass through untouched). Previously only the site nav followed this;
  `button_group`, `card_grid` (card href + links), `breadcrumb`,
  `empty_state`, `callout` links, and markdown link destinations all
  stripped leading `/` and emitted relative paths that 404'd from
  pages at depth ≥ 1.
- `kazam dev` now walks forward to the next free port when the
  requested one is in use (matches Vite / Next.js / Parcel UX) instead
  of failing to bind. Prints a one-line warning when it falls back:
  `⚠ port 3000 is in use — serving on 3001 instead`.
- `kazam dev` no longer rebuilds itself in an infinite loop when `out`
  is relative. The watcher canonicalizes `out` up front and also
  ignores any nested `_site` in the watched tree.
- `kazam build` skips nested `_site` directories. Running from a
  parent dir that contains previously-built sub-sites no longer
  recursively ingests those outputs as source.
- `kazam wish` auto-creates a minimal `kazam.yaml` in the current
  directory if one is missing, so the flow works in any fresh empty
  directory without forcing the user to hand-write site config first.

## [0.4.0] — 2026-04-20

### Added
- `kazam wish <name>` — scaffolds a `wish-<name>/` workspace with structured
  prompts (`questions.md`), usage hints (`README.md`), and a version-matched
  schema + worked example (`reference/`). Fill in what you know, drop real
  context (docs, notes, transcripts, PDFs) into the workspace, then run the
  same command again to grant: kazam shells out to the first agent it finds
  on `$PATH` (Claude, Gemini, Codex, OpenCode) with the workspace as CWD.
  The agent reads everything with its own file tools and writes a populated
  YAML. kazam itself does no file parsing. First wish: `kazam wish deck` —
  a ~7-slide deck for any topic (QBR, launch review, pitch, retrospective,
  etc.). Flags: `--agent` (force a specific CLI), `--yolo [topic]` (skip
  the workspace, let the agent invent everything), `--dry-run` (print the
  grant prompt), `--stdout` (portable wish markdown spec), `--out`
  (override output path).
- `/wishes` docs page with the scaffold→grant flow, agent-applications
  panel, and 8-week roadmap.
- Deck shell typography + layout pass — non-cover slides vertically
  center their content, inner width widened 900 → 1100px, every content
  primitive steps one type tier up on `shell: deck` so slides read as
  slides, not doc pages. Mobile scales down proportionally.
- Mobile responsiveness pass across the whole theme: stat grids, callout
  columns, before/after, tab buttons, tables, code blocks, and the deck
  shell all adapt to phone (≤640px) and tablet (≤768px) viewports.
- Social/SEO meta: `<meta name="description">`, full Open Graph and
  Twitter-card tags, and `<link rel="canonical">` on every page.
- Automatic `sitemap.xml` and `robots.txt` generation when a site's
  `url:` is configured.
- New `description:`, `url:`, and `og_image:` fields on site config.
- Site-wide Open Graph image (`docs/assets/og.svg`).
- `API reference` example page (`docs/examples/api.html`), demonstrating
  a Scalar-style endpoint doc composed entirely from existing components.
- Dedicated `About` and `How it works` pages; landing slimmed to
  manifesto + 30-second demo + three link cards.

### Fixed
- `before_after` component now renders inline markdown (`**bold**`,
  `` `code` ``) in its `before`/`after` fields instead of escaping them
  as literal characters.
- Build walker skips hidden entries (`.git`, `.DS_Store`) at any depth.
- Deck PDF export: cover slides now vertically center on landscape pages
  instead of hugging the top. New `print_flow: continuous` page option
  flows slides as one portrait document with thin separators between them,
  for sharing as a readable artifact rather than a presentation.
- Chart component renders inline SVG for pie, bar (vertical / horizontal /
  stacked), and timeseries (single + multi-series) — themed, zero runtime
  JS, stackable inside decks/grids/callouts.

## [0.3.0] — 2026-04-18

Renamed from `finro` to `kazam`. No functional changes. Existing
`cargo install --git` users pick up the rename via GitHub's repository
redirect; binary name is now `kazam` (was `finro`).
