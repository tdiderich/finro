# AGENTS.md — kazam authoring guide for LLMs

This file tells LLMs (Claude, GPT, etc.) how to author pages in a kazam site.
Read this before generating or editing any `.yaml` file.

## What kazam is

kazam is a Rust CLI that turns YAML files into themed static HTML. Every `.yaml`
file in the source directory becomes one page of output. The format is designed
for you — the machine — to generate easily: consistent `type:` tags, no nested
conditionals, no templating logic.

## File structure

```
my-site/
  kazam.yaml         # site-wide config (name, theme, nav, favicon)
  index.yaml         # → _site/index.html
  guide.yaml         # → _site/guide.html
  reference/
    api.yaml         # → _site/reference/api.html
```

Any non-`.yaml` file (images, SVGs, fonts) is copied verbatim into the output.

## kazam.yaml — site config

```yaml
name: Site Name
theme: dark              # dark | light | red | orange | yellow | green | blue | indigo | violet
mode: dark               # optional: dark (default) | light — flips rainbow themes onto the light base
colors:                  # optional per-token overrides
  accent: '#14b8b8'
favicon: favicon.svg
texture: dots            # optional: none | dots | grid | grain | topography | diagonal
glow: accent             # optional: none | accent | corner
nav_layout: top          # optional: top (default) | sidebar
nav:
  - label: Home
    href: index.html
  - label: Guide
    href: guide.html
  - label: Components     # parent with a dropdown / sidebar section
    href: components/index.html   # optional — acts as default link when clicked
    children:
      - label: Content
        href: components/content.html
      - label: Grids
        href: components/grids.html
```

`nav_layout: top` (default) renders the nav in the sticky top bar. Parent
entries with `children:` render as a hover/focus dropdown. `nav_layout:
sidebar` moves the full nav into a fixed 240px left sidebar; parent entries
with children become labeled sections, their leaves become indented links.
Sidebar layout is only applied to `shell: standard` pages.

`theme` picks a built-in palette. The seven rainbow themes share the
neutral dark base by default and only swap the accent color — safe to use
with any `texture`/`glow` combination. Set `mode: light` to flip any
rainbow theme onto the light base (#F7F7F2 paper + near-black text).
`theme: dark` and `theme: light` are self-contained and ignore `mode`.
Use `colors:` to override individual tokens on top.

`texture` paints a subtle pattern behind every page (tinted via the active
theme's text color, so dark/light just work). `glow` paints a soft
accent-colored radial behind the header area. Both are off by default;
both are stripped under `@media print`.

## Page structure

Every page has a `title`, a `shell`, and EITHER `components:` (for most shells)
OR `slides:` (for deck). Nothing else is required.

```yaml
title: Page Title           # required
shell: standard             # standard | document | deck
eyebrow: Reference          # optional — small label in document/deck headers
subtitle: Q4 2026           # optional — date / context in document/deck
texture: none               # optional — override site-wide texture on this page
glow: corner                # optional — override site-wide glow on this page
components:                 # for standard + document shells
  - type: header
    title: Hello
slides:                     # for deck shell only
  - label: Slide One
    components: [...]
```

`texture:` and `glow:` at the page level override the site-wide values in
`kazam.yaml`. Setting either to `none` turns that layer off on this page;
setting it to any other preset swaps it in. Omit to inherit the site-wide
value.

## Shells

- **standard** — sticky site header + nav + 1200px container. Default. Use for
  dashboards, reference pages, wikis, blog-style content.
- **document** — centered 720px card, teal-bordered header (driven by `eyebrow` +
  `subtitle`), print-optimized. Use for meeting agendas, one-pagers, briefs.
- **deck** — full-viewport slides. Arrow keys to navigate. Print/PDF exports all
  slides paginated. Use for QBRs, pitch decks, strategy reviews.

## Component catalog

Every component has `type:` (snake_case name) + props. Keep YAML indentation
consistent (2 spaces). Quote any string that looks like a number (e.g. `"47"`).

### Content

- **header** — page title block
  ```yaml
  - type: header
    title: Required
    subtitle: Optional subtitle
    eyebrow: Optional label
  ```

- **meta** — key-value strip (author, date, status, version)
  ```yaml
  - type: meta
    fields:
      - key: Author
        value: Someone
      - key: Updated
        value: '2026-04-17'
  ```

- **markdown** — rich prose (CommonMark + tables + strikethrough)
  ```yaml
  - type: markdown
    body: |
      ## Heading
      Paragraph with **bold** and [link](/path).

      | a | b |
      |---|---|
      | 1 | 2 |
  ```

- **code** — syntax-styled code block
  ```yaml
  - type: code
    language: rust
    code: |
      fn main() { println!("hi"); }
  ```

- **callout** — info/warn/success/danger box
  ```yaml
  - type: callout
    variant: warn            # info | warn | success | danger
    title: Heads up
    body: Body can include **markdown**.
  ```

- **blockquote** — testimonial with optional attribution
  ```yaml
  - type: blockquote
    body: The best product we've shipped.
    attribution: Jane Doe, CTO
  ```

- **image** — figure with caption + max_width
  ```yaml
  - type: image
    src: /dashboard.png
    alt: Dashboard screenshot
    caption: Optional caption.
    max_width: 600
  ```

- **kbd** — keyboard keys joined by `+`
  ```yaml
  - type: kbd
    keys: [Cmd, K]
  ```

### Indicators

- **badge** — small colored label. Colors: default, green, yellow, red, teal.
- **tag** — monospace pill label. Same color palette as badge.
- **status** — dot + label (operational / degraded / down). Same colors.
- **progress_bar** — horizontal fill
  ```yaml
  - type: progress_bar
    value: 72                # 0-100
    label: Scan coverage
    color: green
    detail: 72 of 100 accounts
  ```
- **divider** — horizontal rule, optionally with a center label
  ```yaml
  - type: divider
    label: Operational data  # optional
  ```

### Grids

- **card_grid** — responsive cards with optional badge, description, links
  ```yaml
  - type: card_grid
    min_width: 320           # optional; default 320
    cards:
      - title: Acme Corp
        badge:
          label: Healthy
          color: green
        description: Enterprise — AWS
        links:
          - label: Open
            href: /acme
  ```

- **stat_grid** — big-number metric tiles
  ```yaml
  - type: stat_grid
    columns: 3
    stats:
      - label: Users
        value: '1,284'
        detail: Up 12% MoM
        color: green
  ```

- **selectable_grid** — interactive phase tracker / click-to-focus cards
  ```yaml
  - type: selectable_grid
    interaction: single_select   # single_select | multi_select | none
    connector: dots_line         # none | dots_line
    cards:
      - eyebrow: Phase 1
        title: Planning
        bullets:
          - Use cases defined
  ```

- **before_after** — transformation storytelling (QBR-style)
  ```yaml
  - type: before_after
    items:
      - title: Deployment time
        before: Manual, 2 weeks
        after: 4 hours
        after_context: fully automated
  ```

- **avatar** — profile circle with initials fallback
  ```yaml
  - type: avatar
    name: Sarah M.
    src: /sarah.png            # optional
    size: md                   # sm | md | lg | xl
    subtitle: VP Engineering   # optional inline text
  ```

- **avatar_group** — overlapping avatar stack
  ```yaml
  - type: avatar_group
    size: md
    max: 4
    avatars:
      - name: Sarah M.
      - name: Marcus T.
  ```

- **definition_list** — term/definition pairs
  ```yaml
  - type: definition_list
    items:
      - term: ACV
        definition: Annual contract value.
  ```

### Interactive

- **table** — sortable + filterable
  ```yaml
  - type: table
    filterable: true
    columns:
      - key: name
        label: Name
        sortable: true
        align: left              # left | right | center
    rows:
      - name: Acme Corp
  ```

- **tabs** — tabbed panels with arbitrary content
  ```yaml
  - type: tabs
    tabs:
      - label: Overview
        components: [...]
      - label: Details
        components: [...]
  ```

- **accordion** — collapsible sections
  ```yaml
  - type: accordion
    items:
      - title: Question 1
        components: [...]
  ```

### Navigation

- **breadcrumb** — multi-hop trail (last item = current, no href)
  ```yaml
  - type: breadcrumb
    items:
      - label: Home
        href: /
      - label: Reference
        href: /reference
      - label: Components       # current page — no href
  ```

- **button_group** — CTA button row
  ```yaml
  - type: button_group
    buttons:
      - label: Get started
        href: /guide
        variant: primary         # primary | secondary | ghost
      - label: GitHub
        href: https://github.com/...
        variant: secondary
        external: true           # adds ↗ and target=_blank
        icon: github             # any bundled lucide icon
  ```

### Layout

- **section** — grouping with eyebrow + heading + nested components
  ```yaml
  - type: section
    eyebrow: Category
    heading: Section Title
    components: [...]
  ```

- **columns** — multi-column row
  ```yaml
  - type: columns
    equal_heights: true          # optional; makes children fill column height
    columns:
      - - type: callout
          ...
      - - type: callout
          ...
  ```
  Note the `- -` pattern: each column is itself a list of components.

- **timeline** — horizontal phase tracker
  ```yaml
  - type: timeline
    items:
      - name: Planning
        status: completed        # completed | active | upcoming
  ```

- **steps** — numbered or bulleted ordered list
  ```yaml
  - type: steps
    numbered: true               # default; false for bullets
    items:
      - title: Install
        detail: Optional detail.
  ```

- **empty_state** — zero-data placeholder with optional action
  ```yaml
  - type: empty_state
    icon: inbox                  # any bundled lucide icon
    title: No items yet
    body: Add your first item.
    action:
      label: Add item
      href: /new
  ```

- **icon** — standalone inline SVG
  ```yaml
  - type: icon
    name: github                 # any bundled lucide icon
    size: md                     # xs | sm | md | lg | xl
    color: teal                  # same palette as badge/tag/status
  ```

## Bundled icons

~30 lucide icons: `arrow-left`, `arrow-right`, `arrow-up-right`,
`chevron-left`, `chevron-right`, `chevron-down`, `check`, `x`, `plus`,
`search`, `info`, `alert-triangle`, `alert-circle`, `check-circle`,
`x-circle`, `file`, `folder`, `link`, `mail`, `inbox`, `lock`, `bell`,
`calendar`, `clock`, `user`, `users`, `home`, `menu`, `settings`,
`github`.

## Colors (semantic palette)

Every colored component accepts the same 5-value palette:

- `default` — teal accent (brand)
- `green` — success, healthy
- `yellow` — warning, at-risk
- `red` — danger, critical
- `teal` — explicit teal (same as default)

## Common gotchas

- **YAML number-parsing**: `"47"`, `"1.2"` — quote anything that should stay a
  string. Unquoted `47` is an integer, which will fail `label: expected string`.
- **Columns pattern**: each column is a list of components, so you get `- -` at
  the start of each inner list. This is correct YAML.
- **nav_back was removed**. Use `breadcrumb` instead (first component on the page).
- **Hrefs** starting with `http://`, `https://`, `/`, `#`, `mailto:`, `tel:` are
  emitted verbatim. Any other href is rewritten per-page based on directory
  depth so `index.html` works from any subfolder.
- **deck shell** expects `slides:` not `components:` at the page root. Each
  slide has its own `label:` + `components:` list.
- **Markdown body scalars**: use the `|` literal block style in YAML to preserve
  newlines — `body: |` followed by indented content.

## Running

```bash
kazam dev .              # watch + serve at localhost:3000 (live reload)
kazam build .            # one-shot build to _site/
kazam build . --release  # minified production build
```
