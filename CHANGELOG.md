# Changelog

All notable changes to kazam are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and versioning
follows [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Build-time link report — every `kazam build` now walks the page graph
  and surfaces **orphan pages** (built but unreachable from `index.html`
  or the `nav:`) and **broken internal links** (`.html` hrefs that don't
  match any built page). Silent on clean builds. When anything surfaces,
  the build prints a grouped summary and writes `_site/links.md` so an
  agent can consume the list directly. `kazam dev` and
  `kazam build --allow-orphans` silence the orphan check (useful for
  draft pages); broken links always surface. `unlisted: true` on a page
  excludes it from the orphan check.
- `freshness:` page metadata — declare last-updated date, review cadence,
  owner, and sources-of-truth pointers per page. kazam computes status
  at build time (zero runtime JS) and injects a banner at the top of
  stale pages: **yellow** when the review comes due within 7 days,
  **red** when it's already overdue. Every build also prints a grouped
  summary of every stale page (silent when everything is fresh), sorted
  most-urgent-first. Use `KAZAM_TODAY=YYYY-MM-DD` for deterministic
  builds. Full docs at `/freshness`. Example:
  ```yaml
  freshness:
    updated: 2026-01-15
    review_every: 90d
    owner: owner@example.com
    sources_of_truth:
      - https://notion.so/abc123
      - label: "#ts-hub"
        href: https://company.slack.com/archives/C012345
  ```
- `logo:` field on `kazam.yaml` site config — replaces the text `name:`
  treatment in the site bar with an `<img>`. Accepts both shorthand
  (`logo: assets/logo.svg`) and expanded object form
  (`logo: { src, height, alt }`). Rendered height is capped at the
  site-bar content height so a tall logo never pushes the bar taller;
  width flows from aspect ratio and caps at 240px so a wide wordmark
  doesn't crush the nav. `src` routes through the depth-aware path
  rewriter, so absolute `/…` paths pass through verbatim and relative
  paths resolve from any subfolder page. Absent → falls back cleanly
  to the text-name treatment (no layout regression).
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
