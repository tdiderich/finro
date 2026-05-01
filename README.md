# kazam

**The tool your coding agent didn't know it needed.**

Codebase indexing, task tracking, and a visual board — one Rust binary, no dependencies.

---

## Why

Your agent re-reads files it already saw. It scans entire directories to find one function. It loses track of what it's done across sessions. And you watch all of this happen in a terminal you can barely follow.

kazam fixes the three things that make agent-assisted coding slower and more expensive than it should be:

| | Without kazam | With kazam |
|---|---|---|
| **Navigation** | Agent explores with `find`, `grep`, `ls` — burning tokens on every turn | Two-tier anatomy index. Agent reads a 68-line summary, drills into the directory it needs |
| **Tracking** | Work disappears into terminal scroll. No record of what was done or what's left | Structured task tracking — add, claim, close, block. Persists across sessions |
| **Visibility** | You read a terminal, or you don't | Visual board with live-updating task status, anatomy, and activity log |

### Benchmarks

Tested with Sonnet 4.6, identical prompts, git worktrees — kazam-equipped vs vanilla Claude Code:

| Repo | Files | Task | Cost | Speed |
|---|---|---|---|---|
| Internal tools repo | 8,000+ | Add CLI flag + thread to SQL | **45% cheaper** | **41% faster** |
| Plugin repo | 126 | Add config field to skill | **44% cheaper** | **59% faster** |
| React/TS app | 89 | Add loading skeleton | **46% cheaper** | **47% faster** |
| Python service | 233 | Cross-cutting model change | **45% cheaper** | **44% faster** |

Input tokens per turn dropped 81–94% across the board. The anatomy index eliminates exploratory file reads — the agent navigates instead of scanning.

## Install

```bash
# Homebrew (macOS / Linux)
brew install tdiderich/tap/kazam

# Cargo (any platform with Rust)
cargo install kazam

# Bleeding edge
cargo install --git https://github.com/tdiderich/kazam
```

## Quickstart — agent workspace

```bash
cd your-repo
kazam workspace init --agent claude
```

That's it. kazam scans your codebase, writes a two-tier anatomy index to `.kazam/`, installs Claude Code hooks, and writes workspace rules. Your agent now:

1. **Reads the anatomy index first** instead of exploring with `find`/`grep`
2. **Tracks work** with `kazam track` — tasks persist across sessions
3. **Logs activity** so you can see what changed

Open the visual board:

```bash
kazam board
```

Live-updating task status, file anatomy, and activity log — served locally with auto-refresh on any `.kazam/` change.

## Quickstart — static sites

```bash
kazam init my-site && cd my-site
kazam dev .    # live reload at localhost:3000
```

Edit `index.yaml`, save, browser reloads. 30+ themed components, three shell types (standard, document, deck), zero runtime JS. Let your agent write the content:

```bash
kazam wish deck --yolo "Q3 pipeline review"
```

**[Docs + live examples](https://tdiderich.github.io/kazam/)** · **[Components](https://tdiderich.github.io/kazam/components/index.html)** · **[Themes](https://tdiderich.github.io/kazam/themes.html)** · **[Deploy recipes](https://tdiderich.github.io/kazam/deploy.html)**

## How the workspace works

### Anatomy — persistent codebase context

`kazam ctx scan` walks your repo and builds a two-tier index:

- **Summary** (`.kazam/ctx/anatomy.tsv`) — root files + top-level directory rollups with file counts, token estimates, and descriptions. TSV format — ~60% fewer tokens than the previous YAML format.
- **Detail** (`.kazam/ctx/anatomy/<dir>.tsv`) — individual files in each directory, with per-file descriptions and token counts.

Agents read the summary first, then drill into the directory they need. No `find`. No `grep`. No wasted turns.

### Task tracking — structured, persistent, session-spanning

```bash
kazam track add "Fix the auth middleware" --priority 1
kazam track claim kz-a1b2 --name claude
kazam track close kz-a1b2 --reason "patched token validation"
kazam track ready --json    # what's unblocked, sorted by priority
```

Tasks live in `.kazam/track/tasks.yaml`. They survive session restarts, context compaction, and agent handoffs. The workspace rules tell agents to close tasks as they go — not batch at the end.

### Board — visual workspace

```bash
kazam board
```

A themed, auto-refreshing local dashboard showing task status, codebase anatomy, and activity. Built with kazam's own rendering engine. More natural than watching a terminal scroll.

### Corrections — agents that learn from mistakes

```bash
kazam ctx correction "assumed Express middleware" "it's custom Koa" --file src/auth.rs
kazam ctx corrections --json
```

When an agent gets something wrong, record it. Corrections are surfaced in workspace rules so future sessions don't repeat the same mistakes.

### Consolidation — keep context lean

```bash
kazam ctx consolidate          # remove resolved bugs >30 days old, deduplicate learnings
kazam ctx consolidate --days 14
```

### Rules override — project-specific conventions

Create `.kazam/ctx/rules-override.md` and its contents are appended to the generated agent rules on every workspace init. Version-controlled, team-wide.

### Hooks — invisible wiring

`kazam workspace init --agent claude` installs three Claude Code hooks:

- **Session start** — surfaces anatomy drift and ready tasks
- **Post-write** — logs file modifications to the activity feed
- **Session stop** — rescans anatomy and suggests enrichment

No workflow changes. The hooks fire silently and only surface output when something is actionable.

## Security

~10 direct Rust crates, `Cargo.lock` committed, `cargo-audit` in CI, protected main, signed release tags. Full scope: [`SECURITY.md`](SECURITY.md). Report vulnerabilities privately via the [GitHub advisory form](https://github.com/tdiderich/kazam/security/advisories/new).

## Contributing

PRs welcome — agent-assisted contributions explicitly encouraged. See [`CONTRIBUTING.md`](CONTRIBUTING.md).

## License

MIT — see [`LICENSE`](LICENSE).
