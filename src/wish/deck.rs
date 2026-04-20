//! `kazam wish deck` — a 7-slide QBR / strategy-review deck.
//!
//! The user drops real context (docs, notes, transcripts, prior decks) into
//! `wish-deck/` and fills in whatever parts of `questions.md` they want. The
//! agent reads all of it (plus the schema + example in `reference/`) and
//! writes a populated `deck.yaml`.

use super::Wish;

pub static DECK: Wish = Wish {
    name: "deck",
    description: "QBR / strategy-review deck (7 slides)",
    default_out: "deck.yaml",
    questions_md: QUESTIONS_MD,
    readme_md: README_MD,
    references: REFERENCES,
    agent_prompt: AGENT_PROMPT,
    stdout_markdown: STDOUT_MARKDOWN,
};

const EXAMPLE_DECK_YAML: &str = include_str!("../../docs/examples/deck.yaml");

const REFERENCES: &[(&str, &str)] = &[
    ("kazam-schema.md", crate::agents::AGENTS_MD),
    ("example-deck.yaml", EXAMPLE_DECK_YAML),
];

const QUESTIONS_MD: &str = r#"# Deck — structured prompts

Fill in whatever you want to answer up front. Leave any section blank and
the agent will infer it from the other files you've dropped in this folder.

---

## Topic
<!-- One sentence. Becomes the cover title. -->

## Audience
<!-- Who's presenting to whom? e.g., "Leadership team", "Engineering all-hands". -->

## Timeframe
<!-- What cycle? e.g., "Q1 2026", "Last 90 days". -->

## Commitment
<!-- What did you commit to deliver this cycle? 1-2 sentences. -->

## Wins
<!-- 2-3 biggest wins. One per line. Optional format: "Short label: the win". -->

## Challenges
<!-- 1-2 honest misses. One per line. Optional format: "Short label: the honest detail". -->

## Biggest lesson
<!-- One sentence. The through-line between the wins and the misses. -->

## Priorities for next cycle
<!-- Top 2-3 bets. One per line. Optional format: "Priority: why it matters". -->

## The ask
<!-- One sentence. What you need from the audience. -->
"#;

const README_MD: &str = r#"# kazam wish deck — workspace

This folder is where you gather the real context for your deck. When you run
`kazam wish deck` from the parent directory, kazam shells out to an agent
(Claude, Gemini, Codex, or OpenCode) with this folder as the working
directory. The agent reads everything here and writes `../deck.yaml`.

## What to put here

- **`questions.md`** — structured prompts. Fill in what you know. Blanks
  are fine; the agent infers from the other files.
- **Anything with real context** — meeting notes, Slack transcripts,
  Granola recaps, last quarter's deck, metrics dumps, planning docs,
  board-deck PDFs. Drop it in. kazam does not parse files itself; the
  agent handles reading with its own tools. If the agent can read the
  format, it'll use it.
- **`reference/`** — schema + a worked example that kazam wrote here for
  you. The agent consults these for shape + exact field names. Version-
  matched to the kazam binary you have installed. You can edit them if
  you're experimenting, but usually leave them alone.

## When you're ready

```
kazam wish deck
```

from the parent directory (not from inside this folder).

## Flags

- `--agent claude|gemini|codex|opencode` — force a specific agent
- `--dry-run` — print the prompt that would be sent (don't run the agent)
- `--out path/to/out.yaml` — write somewhere other than `../deck.yaml`
"#;

const AGENT_PROMPT: &str = r#"You are generating a 7-slide QBR / strategy-review deck as a single kazam YAML file.

The current working directory is a wish workspace with this layout:

  questions.md           — structured prompts. The user may have filled some in.
  <other files>          — real context dropped in by the user (notes, transcripts,
                           prior decks, PDFs, metrics dumps, anything).
  reference/kazam-schema.md   — authoritative field/enum reference for every
                                kazam component. Consult for EXACT field names.
  reference/example-deck.yaml — a worked deck in valid kazam YAML. Use it as a
                                shape reference (headers, slides, components).

## What to do

1. Read `questions.md`. For each `##` section the user filled in, use their answer.
2. For any section left blank, read the other top-level files in this directory
   and infer a confident answer from them.
3. Consult `reference/kazam-schema.md` for field names and enum values.
   Consult `reference/example-deck.yaml` for shape/style.
4. Write a populated `shell: deck` YAML to stdout.

## Output rules

1. Output ONLY valid YAML — no prose before or after, no ``` code fences, no commentary.
2. The YAML MUST start with `title:` and describe a `shell: deck` page.
3. Every component and field name MUST match `reference/kazam-schema.md` exactly.
   Fields like `detail`, `description`, and `body` are distinct — don't guess.
4. No placeholder copy ("replace this", "your win here", etc.). Every slide
   must be fully populated. If the user's answers are thin, infer confidently
   from the other files.
5. If unsure about a field, prefer *omitting* it — most fields are optional.

## Slide plan (7 slides, in this order)

1. **Cover** — `hide_label: true`. One `header` with `align: center`,
   `title:` (topic), `subtitle:` (`"<timeframe> · <audience>"`).
2. **Context** — `header` (`title: "Going into <timeframe>"`, `subtitle: "What we set out to do"`)
   + `blockquote` with `body:` set to the commitment (expand to 2-3 sentences if terse).
3. **Wins** — `header` (`title: "What went well"`) + `stat_grid`. Each stat:
   `label:` (short punchy phrase), `value:` (the win itself). Cycle `color:`
   through green, default, teal, yellow.
4. **Challenges** — `header` (`title: "Where we fell short"`) + `card_grid`.
   Each card: `title:` (3-5 word label), `description:` (honest 1-2 sentence
   analysis + lesson). `color: yellow`.
5. **Learnings** — `header` (`title: "What we're taking forward"`) + `callout`
   (`variant: info`, `title: "The through-line"`, `body:` set to the biggest
   lesson, expanded to 2-3 sentences).
6. **Next** — `header` (`title: "After <timeframe>"`) + `steps` with
   `numbered: true` and `items:`. Each item: `title:` (priority), `detail:`
   (why it matters).
7. **The ask** — `header` (`title: "The ask"`) + `callout` (`variant: info`,
   `title: "What we need"`, `body:` set to the ask verbatim).

Now read the workspace and write the YAML.
"#;

const STDOUT_MARKDOWN: &str = r#"# kazam wish: deck

Grants a fully populated 7-slide QBR / strategy-review deck as a `kazam` YAML
file. kazam scaffolds a workspace; the user drops their real context in; the
agent reads everything and writes the deck.

## Flow

1. `kazam wish deck` (first run) — scaffolds `./wish-deck/` with:
   - `questions.md` (structured prompts)
   - `README.md` (usage hint)
   - `reference/kazam-schema.md` (full schema, version-matched to the binary)
   - `reference/example-deck.yaml` (worked example deck)
2. The user fills in as much of `questions.md` as they want and drops real
   context (docs, transcripts, prior decks, PDFs) into `./wish-deck/`.
3. `kazam wish deck` (second run) — granting. kazam shells out to the first
   agent on `$PATH` (Claude, Gemini, Codex, OpenCode) with `./wish-deck/` as
   the CWD. The agent reads everything and writes `deck.yaml`.

kazam does NOT parse files itself. File handling is the agent's job.

## Agent prompt shape (what kazam sends)

The agent receives a prompt that:
1. Describes the workspace layout (`questions.md` + user context + `reference/`).
2. Tells it to read the user's answers, fall back to other files for blanks,
   and consult `reference/` for exact field names and shape.
3. Spells out the 7-slide plan.
4. Requires YAML-only output (no fences, no prose) and no placeholder copy.

## Slide plan

1. Cover — header centered, topic as title, "<timeframe> · <audience>" as subtitle.
2. Context — header + blockquote of the commitment.
3. Wins — header + stat_grid (one stat per win, cycle green/default/teal/yellow).
4. Challenges — header + card_grid (one card per challenge, color yellow).
5. Learnings — header + callout with the biggest lesson.
6. Next — header + numbered steps (one per priority with title + detail).
7. The ask — header + callout with the ask verbatim.

## Kazam YAML conventions

- 2-space indent. No trailing whitespace.
- `shell: deck` enables slide navigation + PDF export.
- Multi-line text uses the `|` block literal.
- Full component reference: `kazam agents` or https://tdiderich.github.io/kazam/

Output ONLY the YAML. No prose, no code fences. No placeholder copy.
"#;
