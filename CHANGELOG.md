# Changelog

All notable changes to kazam are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and versioning
follows [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- `kazam wish <name>` — runs a short interview, then either populates a
  template directly or hands the answers off to an agent CLI for rich
  generation. First wish: `kazam wish deck` (7-slide QBR / strategy-review).
  Supports `--agent claude|gemini|codex|opencode` (shells out to each
  agent's non-interactive mode: `-p`, `exec`, `run`) and `--stdout` (prints
  the portable wish markdown for piping into any other agent).
- `/wishes` docs page with the agent-applications panel and 8-week roadmap.
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

## [0.3.0] — 2026-04-18

Renamed from `finro` to `kazam`. No functional changes. Existing
`cargo install --git` users pick up the rename via GitHub's repository
redirect; binary name is now `kazam` (was `finro`).
