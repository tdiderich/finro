# Plan — Chakra-inspired component additions

## Current State

Finro has 17 components: `header`, `meta`, `card_grid`, `selectable_grid`, `timeline`, `stat_grid`, `before_after`, `steps`, `markdown`, `table`, `callout`, `code`, `tabs`, `section`, `columns`, `accordion`, `image`. Three shells: `standard`, `document`, `deck`.

## Chakra → finro mapping

**Already covered (skip):** `Alert` → `callout`; `Accordion`; `Tabs`; `Card` (inside `card_grid`); `Stat` (use `stat_grid` of 1 item); `Table`; `Steps`; `Timeline`; `Heading` (inside `header`); `Image`; `Container`/`SimpleGrid` (shells + grids); `Prose` (markdown); `Blockquote` inside markdown.

**Off-scope (skip):**
- Every form input (Input, Textarea, Checkbox, Radio, Select, Switch, Slider, DatePicker, PinInput, NumberInput, Field, FileUpload, Editable, ColorPicker, RatingGroup, SegmentGroup)
- Stateful overlays (Dialog, Drawer, Modal, Menu, Popover, HoverCard, Tooltip)
- Loading indicators (Spinner, Skeleton, indeterminate Progress)
- Chart family (huge surface, data-viz is its own project)
- Layout primitives (Box, Flex, Stack, HStack, VStack, SimpleGrid, AspectRatio, AbsoluteCenter, Bleed) — finro's grids + columns + section semantically cover these
- ActionBar, Toaster, QrCode, Icon, Pagination

**Gaps worth filling:** `badge` (standalone), `tag`, `divider`, `kbd`, `progress_bar`, `button_group`, `avatar`, `avatar_group`, `breadcrumb`, `blockquote` (standalone), `status`, `empty_state`, `definition_list`.

---

## Approach 1 — Minimal (8 components, Tier 1 only)

Add just `badge`, `tag`, `divider`, `kbd`, `progress_bar`, `button_group`, `avatar`, `breadcrumb`.

- **Complexity:** M (half day)
- **Pros:** tight scope, fast to ship
- **Cons:** leaves obvious gaps (avatar_group for teams, blockquote for testimonials, empty_state)

## Approach 2 — Complete static coverage (13 components) ⭐ **RECOMMENDED**

Tier 1 (8) + Tier 2 (5 more): `avatar_group`, `blockquote`, `status`, `empty_state`, `definition_list`.

- **Complexity:** M (full day with docs)
- **Pros:** covers ~90% of static component needs for LLM-authored info sites; every addition is genuinely useful; grouping docs into category pages gives the reference real structure
- **Cons:** ~400 LOC of new CSS; 2 new doc pages
- **Risks:** `breadcrumb` partially overlaps with existing `nav_back` — keep both; `status` vs `stat` (single from stat_grid) could confuse — naming is clear enough

## Approach 3 — Superset with page-builder components (16)

Approach 2 + `hero`, `pricing_card`, `feature_grid`.

- **Complexity:** L
- **Cons:** skews finro toward marketing-site generation; `hero` is arguably just `header` + `image` + `button_group` composed; `pricing_card` is extremely specific; `feature_grid` overlaps `card_grid`
- **Risks:** scope creep — invites cta_banner, logo_cloud, testimonial_wall…

---

## Component property sketches (Approach 2)

```yaml
- type: badge
  label: New
  color: green                 # green | yellow | red | teal | default

- type: tag
  label: design-system
  color: teal                  # green | yellow | red | teal | default

- type: divider
  label: optional center text  # optional

- type: kbd
  keys: [Cmd, K]               # → <kbd>Cmd</kbd>+<kbd>K</kbd>

- type: progress_bar
  value: 72                    # 0-100
  label: Scan coverage         # optional
  color: green                 # default | green | yellow | red | teal
  detail: 94 of 130 accounts   # optional trailing text

- type: button_group
  buttons:
    - label: Get started
      href: /guide
      variant: primary         # primary | secondary | ghost
    - label: See on GitHub
      href: https://github.com/...
      variant: secondary
      external: true           # adds ↗ affordance

- type: avatar
  name: Sarah M.               # initials fallback when src missing
  src: /avatars/sarah.png      # optional
  size: md                     # sm | md | lg | xl
  subtitle: VP Engineering     # optional inline text

- type: avatar_group
  max: 4                       # show first N, then "+N"
  size: sm
  avatars:
    - name: Sarah M.
    - name: Marcus T.
    - name: Jordan K.
    - name: Priya S.

- type: breadcrumb
  items:
    - label: Home
      href: /
    - label: Customers
      href: /customers
    - label: Acme Corp         # last item: no href (current page)

- type: blockquote
  body: The best product we've shipped in years.
  attribution: Jane Doe, CTO at Acme

- type: status
  label: Operational
  color: green                 # small dot + label

- type: empty_state
  title: No customers yet
  body: Add your first customer to see them here.
  action:
    label: Add customer
    href: /customers/new

- type: definition_list
  items:
    - term: ACV
      definition: Annual contract value — total yearly revenue from a customer.
    - term: SLA
      definition: Service level agreement — uptime and performance guarantees.
```

---

## Implementation order

1. **Simple atoms** (no JS): `badge`, `tag`, `divider`, `kbd`, `status` — ~100 LOC total
2. **Structural primitives**: `breadcrumb`, `button_group`, `definition_list`, `blockquote`
3. **Composed components**: `avatar`, `avatar_group`, `progress_bar`, `empty_state`

Compile after each phase. After landing: 17 → 30 components.

---

## Product Decisions

> Tyler — edit the answers below. Defaults are pre-filled with my recommendations; the plan will execute with these unless you change them.

### 1. `nav_back` vs `breadcrumb` — keep both?
- [] **Keep both (recommended)** — `nav_back` is page-level field for simple "← back"; `breadcrumb` is a placeable component for multi-hop paths
- [x] Deprecate `nav_back`, use `breadcrumb` everywhere

### 2. Ship with icon library?
- [] **No icons (recommended)** — use unicode characters (→ ↗ ✓ ●) for affordances; use `image` + SVG for fancier illustrations; revisit after these land
- [x] Bundle a small icon set (lucide/feather)

### 3. `progress_bar` animation?
- [x] **No animation (recommended)** — pure static fill, no JS
- [ ] Subtle indeterminate shimmer like Chakra

### 4. `avatar` — src vs initials priority
- [x] **If `src` provided, use it; else derive initials from `name`** (recommended)
- [ ] Other

### 5. `button_group` — how many variants?
- [x] **Three: `primary` (solid teal), `secondary` (outlined), `ghost` (no border)** (recommended)
- [ ] More (match Chakra's 6: solid, outline, ghost, subtle, surface, plain)
- [ ] Fewer (just primary + secondary)

### 6. `tag` — include `removable` field?
- [x] **Omit the field entirely — pure display** (recommended)
- [ ] Accept field, render decorative × with "decorative" tooltip

### 7. Standalone `badge` vs card's inline badge
- [x] **Leave card's inline badge alone; new standalone `badge` uses same color palette** (recommended)
- [ ] Migrate card's badge to reference the new component type

### 8. Docs structure
- [x] **Two new category pages: `indicators.yaml` (badge/tag/status/progress/kbd/divider), `navigation.yaml` (breadcrumb/button_group); add avatar/avatar_group/blockquote/empty_state/definition_list to existing pages** (recommended)
- [ ] Other

---

## What I'll do (Approach 2, pre-filled defaults)

1. Add 13 new `Component` enum variants to `src/types.rs` with supporting prop structs
2. Add 13 render functions to `src/render/components.rs`
3. Add ~400 LOC of new CSS to `src/theme.rs`, all scoped under `.c-*`
4. Create `docs/components/indicators.yaml` and `docs/components/navigation.yaml`
5. Extend `docs/components/content.yaml` with blockquote, kbd, definition_list
6. Extend `docs/components/grids.yaml` with avatar, avatar_group
7. Extend `docs/components/layout.yaml` with divider, empty_state
8. Update `docs/components/index.yaml` to add category links for the two new pages
9. Optionally exercise new components in `docs/examples/*.yaml` where they fit
10. Build, smoke-test, commit, push to origin/main

