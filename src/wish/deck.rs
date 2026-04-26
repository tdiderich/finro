//! `kazam wish deck` — a ~7-slide deck for any topic.
//!
//! The user drops real context (docs, notes, transcripts, prior decks) into
//! `wish-deck/` and fills in whatever parts of `questions.md` they want.
//! The agent reads all of it (plus the schema + example in `reference/`)
//! and writes a populated `deck.yaml`. The deck's shape adapts to the
//! topic — QBR, launch review, product pitch, strategy update, or whatever
//! the user is actually presenting.

use super::Wish;

pub static DECK: Wish = Wish {
    name: "deck",
    description: "A ~7-slide deck for any topic",
    default_out: "deck.yaml",
    questions_md: QUESTIONS_MD,
    readme_md: README_MD,
    references: REFERENCES,
    agent_prompt: AGENT_PROMPT,
    yolo_prompt,
    stdout_markdown: STDOUT_MARKDOWN,
};

fn yolo_prompt(topic: Option<&str>) -> String {
    let topic_block = match topic {
        Some(t) => format!(
            "The topic the user asked for:\n\n  {}\n\nMake the deck about this, using whatever knowledge, context, or creative license you need.",
            t
        ),
        None => "No topic was specified — surprise the user. Pick something interesting, unexpected, or charmingly specific. It could be a real subject (a technical deep-dive, a historical moment, an idea you care about) or a playful fiction (a pitch from 2045, a QBR from an imaginary team). Commit to the bit — invent every slide's content.".to_string(),
    };

    format!(
        r#"You are generating a ~7-slide deck as a single kazam YAML file in YOLO mode — no user questions, no workspace, no context files. You invent everything.

{topic_block}

## Use whatever context you have

If you're running with context about the user (a `CLAUDE.md`, auto-memory, or conversation history), use it. The user might say "about me" and expect you to draw on what you already know about their work, voice, and projects. If you have no context, improvise confidently — the goal is a deck that feels deliberate and finished, not generic.

{mcp_guidance}

## Output rules

1. Output ONLY valid YAML — no prose before or after, no ``` code fences, no commentary.
2. The YAML MUST start with `title:` and describe a `shell: deck` page.
3. Every component and field name MUST match the Kazam Authoring Guide below. Fields like `detail`, `description`, and `body` are distinct — don't guess.
4. No placeholder copy. Every slide fully written.
5. If unsure about a field, prefer *omitting* it — most fields are optional.

## Top-level keys

Always include `print_flow: continuous` alongside `title`, `shell: deck`, `eyebrow`, and `subtitle`. PDF export flows as one readable document.

## Slide plan (guidance, not a rigid script)

Aim for 5-8 slides. Default shape:

1. **Cover** — `hide_label: true`. Centered `header` with the topic as title and a punchy subtitle.
2. **Opener** — set the stakes or frame the "why." One sentence as a `blockquote` or tight `markdown`.
3-6. **One slide per key point.** Vary the kazam primitives — `stat_grid` for numbers, `card_grid` for themed points, `steps` for sequences, `blockquote` for punchlines, `callout` for a single emphasised idea, `before_after` for transformations, `timeline` for phases, `table` for structured data. Decks that repeat the same component feel flat.
7. **The ask / close** — `callout` with `variant: info`. Land the takeaway.

Slide labels should match each slide's purpose in plain language.

## Kazam Authoring Guide (authoritative schema — validate your output against this)

{guide}

Now write the YAML.
"#,
        topic_block = topic_block,
        mcp_guidance = super::MCP_GUIDANCE,
        guide = crate::agents::AGENTS_MD,
    )
}

const EXAMPLE_DECK_YAML: &str = include_str!("../../docs/examples/deck.yaml");

const REFERENCES: &[(&str, &str)] = &[
    ("kazam-schema.md", crate::agents::AGENTS_MD),
    ("example-deck.yaml", EXAMPLE_DECK_YAML),
];

const QUESTIONS_MD: &str = r#"# Deck — structured prompts

Fill in whatever you want to answer up front. Leave any section blank and
the agent will infer it from the other files you've dropped in this folder.
Everything here flexes to the topic — QBR, launch review, product pitch,
strategy update, retrospective, whatever.

---

## Topic
<!-- What's this deck about? One sentence. Becomes the cover title. -->

## Purpose
<!-- What do you want the audience to do, feel, or decide after seeing this? -->

## Audience
<!-- Who's presenting to whom? e.g., "Leadership team", "Board", "Prospects", "All-hands". -->

## Timeframe or context
<!-- Optional — the cycle, campaign, quarter, project, or event this covers. -->

## Key messages
<!-- 3-5 things you want the audience to walk away knowing. One per line. -->

## Supporting evidence
<!-- Wins, metrics, anecdotes, quotes, data — anything that backs up the messages. -->

## Concerns or counter-narratives
<!-- Optional — things the audience might push back on, honest misses, open risks. -->

## The ask
<!-- What single thing do you want the audience to commit to or take away? -->
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
  PDFs. Drop it in. kazam does not parse files itself; the agent handles
  reading with its own tools. If the agent can read the format, it'll
  use it.
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

const AGENT_PROMPT: &str = r#"You are generating a ~7-slide deck as a single kazam YAML file. The topic, purpose, and shape of the deck come from the user — this wish is NOT QBR-specific. Adapt to what the user actually wants to present: a launch review, a product pitch, a strategy update, a retrospective, a proposal, etc.

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
4. Choose a slide structure that fits the topic + purpose (see "Slide plan" below).
5. Write a populated `shell: deck` YAML to stdout.

## Output rules

1. Output ONLY valid YAML — no prose before or after, no ``` code fences, no commentary.
2. The YAML MUST start with `title:` and describe a `shell: deck` page.
3. Every component and field name MUST match `reference/kazam-schema.md` exactly.
   Fields like `detail`, `description`, and `body` are distinct — don't guess.
4. No placeholder copy ("replace this", "your win here", etc.). Every slide
   must be fully populated. If the user's answers are thin, infer confidently
   from the other files.
5. If unsure about a field, prefer *omitting* it — most fields are optional.

## Top-level keys

Always include `print_flow: continuous` alongside `title`, `shell: deck`, `eyebrow`, and `subtitle`. This makes the PDF export flow as one readable document with thin separators between slides instead of one slide per landscape page.

## Slide plan (guidance, not a rigid script)

Aim for 5-8 slides total. The default shape:

1. **Cover** — `hide_label: true`. Centered `header` with the topic as title,
   audience + timeframe (if present) as subtitle.
2. **Opener** — frame *why this matters*. The purpose distilled into one
   sentence or a short `blockquote`. If a prior-cycle commitment or inciting
   context is relevant, put it here.
3-6. **One slide per key message.** Pick the kazam component that best fits
   the shape of the content — `stat_grid` for numeric wins or headline
   metrics, `card_grid` for themed points or comparisons, `steps` for a
   sequenced plan or roadmap, `blockquote` for a quote or punchline,
   `callout` for a single emphasised idea, `before_after` for
   transformations, `timeline` for phases, `table` for structured data.
   Vary the primitives — decks that use the same component on every slide
   feel flat.
7. **Concerns** (optional — include only if the user supplied concerns or
   counter-narratives, or the context implies ones worth naming). `callout`
   with `variant: warning`, or a small `card_grid`.
8. **The ask** — close with a `callout` (`variant: info`, `title` like
   "The ask" or "What we need") and `body:` set to the ask verbatim. This
   slide is never optional.

Slide labels should match the slide's purpose in plain language. Don't name
slides with QBR-specific titles ("Where we fell short") unless the user's
topic actually is a QBR.

Now read the workspace and write the YAML.
"#;

const STDOUT_MARKDOWN: &str = r#"# kazam wish: deck

Grants a fully populated ~7-slide deck as a `kazam` YAML file. The deck's
shape adapts to the topic — QBR, launch review, product pitch, strategy
update, retrospective, whatever the user is presenting. kazam scaffolds a
workspace; the user drops their real context in; the agent reads everything
and writes the deck.

## Flow

1. `kazam wish deck` (first run) — scaffolds `./wish-deck/` with:
   - `questions.md` (structured prompts — topic, purpose, audience,
     messages, evidence, ask)
   - `README.md` (usage hint)
   - `reference/kazam-schema.md` (full schema, version-matched to the binary)
   - `reference/example-deck.yaml` (worked example deck)
2. The user fills in as much of `questions.md` as they want and drops real
   context (docs, transcripts, prior decks, PDFs) into `./wish-deck/`.
3. `kazam wish deck` (second run) — granting. kazam shells out to the first
   agent on `$PATH` (Claude, Gemini, Codex, OpenCode) with `./wish-deck/`
   as the CWD. The agent reads everything and writes `deck.yaml`.

kazam does NOT parse files itself. File handling is the agent's job.

## Agent prompt shape (what kazam sends)

The agent receives a prompt that:
1. Describes the workspace layout (`questions.md` + user context + `reference/`).
2. Tells it to read the user's answers, fall back to other files for blanks,
   and consult `reference/` for exact field names and shape.
3. Gives slide-plan *guidance* (opener, one slide per key message, close
   with the ask) without prescribing QBR-specific titles.
4. Requires YAML-only output (no fences, no prose) and no placeholder copy.

## Slide plan (default shape — agent adapts to topic)

- Cover — topic, audience, timeframe.
- Opener — why this matters / the purpose framed.
- One slide per key message — agent picks the best kazam component per
  message (`stat_grid`, `card_grid`, `steps`, `blockquote`, `callout`,
  `before_after`, `timeline`, `table`).
- Optional concerns slide — if the user supplied counter-narratives.
- The ask — close with a `callout`.

## Kazam YAML conventions

- 2-space indent. No trailing whitespace.
- `shell: deck` enables slide navigation + PDF export.
- Multi-line text uses the `|` block literal.
- Full component reference: `kazam agents` or https://tdiderich.github.io/kazam/

Output ONLY the YAML. No prose, no code fences. No placeholder copy.
"#;
