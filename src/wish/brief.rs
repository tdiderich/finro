//! `kazam wish brief` — a tight, print-optimized brief for a meeting,
//! incident, or any short-form artifact you'd hand someone to read in two
//! minutes before a conversation.
//!
//! Document shell, not deck. The brief adapts to the subject — a 1:1 prep,
//! a vendor-evaluation kickoff, an incident postcard, a meeting agenda —
//! but the shape is always: who, why, where things stand, what we're
//! deciding, what could go wrong, action items.

use super::Wish;

pub static BRIEF: Wish = Wish {
    name: "brief",
    description: "A short, print-optimized brief for a meeting or incident",
    default_out: "brief.yaml",
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
            "The topic the user asked for:\n\n  {}\n\nThis is a real artifact that the user may walk into a real meeting (or send to a real audience) carrying. Treat it as the user's own meeting, incident, or event — and treat fabricated specifics as a serious failure mode, not a creative liberty.",
            t
        ),
        None => "No topic was specified — pick a real, near-future meeting or incident from the user's world. You MUST ground the choice in a verifiable signal (a calendar event in the next 7 days, an open Linear incident, a deal in HubSpot's pipeline, a prominent Slack thread). Do not invent a fictional scenario. If you genuinely cannot find any grounding signal after a few targeted MCP queries, abort and write a single-line YAML comment explaining what you searched and what you didn't find.".to_string(),
    };

    format!(
        r#"You are generating a short brief as a single kazam YAML file in YOLO mode. The brief is an artifact the user will actually use — fabricated specifics (made-up attendee names, invented numbers, plausible-but-fake history) are a HARD FAILURE, not a creative liberty.

{topic_block}

## MCP-first rule (non-negotiable for personal-world briefs)

If the topic names a person, company, meeting, deal, ticket, channel, or
incident — i.e. anything from the user's own world — your FIRST actions
before writing any YAML are MCP lookups. Do not skip this step. Do not
write a single line of YAML until you have searched.

Default sequence (skip a tool only if it's clearly irrelevant to the topic):

  1. HubSpot — `search_crm_objects` for the company / deal name. Capture
     domain, owner, deal stage, amount, last activity, key contacts.
  2. Google Calendar — `list_events` over the next 7 days (or the date
     mentioned), filter to the named meeting. Capture exact start time,
     duration, attendees.
  3. Granola — `query_granola_meetings` for prior calls with the same
     attendees / company. Capture how many calls, when, what was decided.
  4. Linear — `list_issues` filtered by any project or customer name in
     the topic, in case there are open tickets the user owns.
  5. Slack — `slack_search_public_and_private` for the company / topic
     name to surface recent threads.
  6. Attention — `search_calls` if the meeting is sales-shaped.

Use what you find. For every concrete claim in the brief — attendee
names, dates, deal amounts, prior-call counts, ticket counts, account
overlap, history — you must be able to point to the MCP result it came
from. If a tool returns nothing useful, the corresponding fact does not
go in the brief — write `TBD — confirm before sending` or omit the
section entirely.

## What you may improvise

- The brief's *structure* (which sections, what order).
- *Voice and framing* (how you phrase the one-line goal, the talking points).
- The user's *intent / goal* for the meeting, when it's reasonably
  inferable from deal stage, ticket state, prior-call notes.
- *Suggested* talking points and risks based on the grounded context.

## What you must NEVER fabricate

- Names of attendees you didn't see in Calendar / HubSpot / Granola.
- Numbers (deal size, account counts, durations, percentages, dates) you
  didn't read from a tool result.
- Prior-meeting history (call counts, what was discussed) you didn't pull
  from Granola or Attention.
- Dates and times beyond what Calendar / Linear / HubSpot returned.

When in doubt, write `TBD — confirm before sending` for that field. A
brief that's 60% real + 40% TBD is far more useful than one that's 100%
plausible-but-fake.

## Use whatever context you have

If you're running with auto-memory or a `CLAUDE.md`, use it for the
user's role, team, voice, and product context. That's character, not
specifics — still subject to the MCP-first rule for any concrete claim.

{mcp_guidance}

## Output rules

1. Output ONLY valid YAML — no prose before or after, no ``` code fences, no commentary.
2. The YAML MUST start with `title:` and describe a `shell: document` page.
3. Every component and field name MUST match the Kazam Authoring Guide below. Fields like `detail`, `description`, and `body` are distinct — don't guess.
4. No placeholder copy beyond explicit `TBD — confirm before sending` markers where MCP turned up nothing.
5. If unsure about a field, prefer *omitting* it — most fields are optional.

## Top-level keys

Use `title`, `shell: document`, `eyebrow`, and `subtitle`. Do NOT add `print_flow` — that field only applies to `shell: deck` pages.

## Brief shape (guidance, not a rigid script)

A good brief reads cold in two minutes. Default sections — adapt to the subject:

1. **`meta` block** — the at-a-glance facts: meeting time + attendees, or incident severity + status. 4-6 fields. Each field is `key:` + `value:`.
2. **One-line goal** — a `callout` with `variant: info` stating the single thing this brief is for. One sentence.
3. **Context section** — where things stand. Mix `stat_grid` (numbers — adoption, revenue, severity, duration) with a tight `markdown` paragraph for narrative. Skip `stat_grid` if there are no real numbers to show.
4. **Agenda or timeline section** — `steps` (with `items:`, each having `title:` + optional `detail:`) for a meeting agenda (3-5 steps with time boxes), or `timeline` for an incident timeline.
5. **Talking points / what to land** — `card_grid` of 3-4 cards. Each card is one thing the reader should walk in knowing.
6. **Risks / what could go sideways** — `callout` blocks with `variant: warn`. Skip if genuinely none.
7. **Action items** — `table` with owner / action / due columns, OR a tight `markdown` checklist if the briefer prefers that shape.

Stay short. A brief is not a deck and not a doc — a printed page or two, max.

## Kazam Authoring Guide (authoritative schema — validate your output against this)

{guide}

Now write the YAML.
"#,
        topic_block = topic_block,
        mcp_guidance = super::MCP_GUIDANCE,
        guide = crate::agents::AGENTS_MD,
    )
}

const EXAMPLE_BRIEF_YAML: &str = include_str!("../../docs/examples/brief.yaml");

const REFERENCES: &[(&str, &str)] = &[
    ("kazam-schema.md", crate::agents::AGENTS_MD),
    ("example-brief.yaml", EXAMPLE_BRIEF_YAML),
];

const QUESTIONS_MD: &str = r#"# Brief — structured prompts

Fill in whatever you want to answer up front. Leave any section blank and
the agent will infer it from the other files you've dropped in this folder.
Briefs flex to the subject — meeting prep, incident postcard, vendor
kickoff, 1:1 agenda, exec readout.

---

## Subject
<!-- What's this brief about? One sentence. e.g., "Renewal sync with Northwind"
or "P2 latency incident — Apr 23, 02:14 UTC". Becomes the title. -->

## Type
<!-- Optional — meeting | incident | proposal | other. Shapes the agenda vs. timeline section. -->

## When and who
<!-- Time + attendees for a meeting. Detection time + responders for an incident.
Skip whichever doesn't apply. -->

## One-line goal
<!-- The single thing this brief is for. The reader should know what success
looks like after reading the first paragraph. -->

## Where things stand
<!-- Current state — numbers if you have them (revenue, adoption, severity,
duration, ticket count), narrative if you don't. -->

## Agenda or timeline
<!-- For a meeting: the 3-5 things you want to walk through, ideally with time
boxes. For an incident: the timeline of detection, escalation, mitigation,
resolution. -->

## What to land / talking points
<!-- 3-4 things the reader should walk in knowing or saying. -->

## Risks
<!-- Optional — what could go sideways. Procurement timing, new stakeholders,
unresolved technical risks, political headwinds. -->

## Action items
<!-- Who's doing what by when. Owner / action / due, one per line. -->
"#;

const README_MD: &str = r#"# kazam wish brief — workspace

This folder is where you gather the real context for your brief. When you run
`kazam wish brief` from the parent directory, kazam shells out to an agent
(Claude, Gemini, Codex, or OpenCode) with this folder as the working
directory. The agent reads everything here and writes `../brief.yaml`.

## What to put here

- **`questions.md`** — structured prompts. Fill in what you know. Blanks
  are fine; the agent infers from the other files.
- **Anything with real context** — calendar invites, prior meeting notes,
  Granola transcripts, Slack threads, ticket exports, incident channels,
  PDFs. Drop it in. kazam does not parse files itself; the agent handles
  reading with its own tools.
- **`reference/`** — schema + a worked example that kazam wrote here for
  you. The agent consults these for shape + exact field names. Version-
  matched to the kazam binary you have installed.

## When you're ready

```
kazam wish brief
```

from the parent directory (not from inside this folder).

## Flags

- `--agent claude|gemini|codex|opencode` — force a specific agent
- `--dry-run` — print the prompt that would be sent (don't run the agent)
- `--out path/to/out.yaml` — write somewhere other than `../brief.yaml`
"#;

const AGENT_PROMPT: &str = r#"You are generating a short brief as a single kazam YAML file. The subject — a meeting, incident, vendor sync, 1:1, exec readout — comes from the user. This wish is NOT meeting-specific or incident-specific; adapt to whatever they're actually briefing.

The current working directory is a wish workspace with this layout:

  questions.md           — structured prompts. The user may have filled some in.
  <other files>          — real context dropped in by the user (calendar invites,
                           transcripts, Slack threads, ticket exports, PDFs).
  reference/kazam-schema.md   — authoritative field/enum reference for every
                                kazam component. Consult for EXACT field names.
  reference/example-brief.yaml — a worked brief in valid kazam YAML. Use it as a
                                shape reference (sections, components, density).

## What to do

1. Read `questions.md`. For each `##` section the user filled in, use their answer.
2. For any section left blank, read the other top-level files in this directory
   and infer a confident answer from them.
3. Consult `reference/kazam-schema.md` for field names and enum values.
   Consult `reference/example-brief.yaml` for shape/style.
4. Choose a section structure that fits the subject + type (see "Brief shape" below).
5. Write a populated `shell: document` YAML to stdout.

## Output rules

1. Output ONLY valid YAML — no prose before or after, no ``` code fences, no commentary.
2. The YAML MUST start with `title:` and describe a `shell: document` page.
3. Every component and field name MUST match `reference/kazam-schema.md` exactly.
   Fields like `detail`, `description`, and `body` are distinct — don't guess.
4. No placeholder copy ("replace this", "your point here", etc.). Every field
   must be fully populated. If the user's answers are thin, infer confidently
   from the other files.
5. If unsure about a field, prefer *omitting* it — most fields are optional.

## Top-level keys

Use `title`, `shell: document`, `eyebrow`, and `subtitle`. Do NOT add `print_flow` — that field only applies to `shell: deck` pages.

## Brief shape (guidance, not a rigid script)

A good brief reads cold in two minutes. Default sections — adapt to the subject:

1. **`meta` block** — the at-a-glance facts: meeting time + attendees, or
   incident severity + status. 4-6 fields, each `key:` + `value:`. Always
   lead with this.
2. **One-line goal** — a `callout` with `variant: info` stating the single
   thing this brief is for. One sentence.
3. **Context section** (eyebrow "Context", heading like "Where things stand")
   — `stat_grid` for real numbers (adoption %, ARR, severity, duration,
   tickets) plus a tight `markdown` paragraph. Skip the `stat_grid` if there
   are no real numbers.
4. **Agenda OR timeline section** — `steps` (with `items:` — each `title:`
   plus optional `detail:`) and time boxes for a meeting agenda, or
   `timeline` (with `items:`) for an incident.
5. **Talking points / what to land** — `card_grid` with 3-4 cards. Each card
   is one thing the reader should walk in knowing.
6. **Risks** (optional — only if genuinely present) — `callout` blocks with
   `variant: warn`.
7. **Action items** — `table` with owner / action / due columns. Always
   include this section even if short — briefs without next steps are notes,
   not briefs.

Briefs are SHORT. One printed page is the target; two is the ceiling. Don't
pad. If a section would be empty or trivial, drop it.

Section labels should match the subject's vocabulary (a sales brief uses
"Renewal", "Pipeline"; an incident brief uses "Detection", "Mitigation").

Now read the workspace and write the YAML.
"#;

const STDOUT_MARKDOWN: &str = r#"# kazam wish: brief

Grants a short, print-optimized brief as a `kazam` YAML file. The brief's
shape adapts to the subject — meeting prep, incident postcard, vendor
kickoff, 1:1 agenda, exec readout. kazam scaffolds a workspace; the user
drops their real context in; the agent reads everything and writes the brief.

## Flow

1. `kazam wish brief` (first run) — scaffolds `./wish-brief/` with:
   - `questions.md` (structured prompts — subject, type, attendees, goal,
     context, agenda/timeline, talking points, risks, action items)
   - `README.md` (usage hint)
   - `reference/kazam-schema.md` (full schema, version-matched to the binary)
   - `reference/example-brief.yaml` (worked example brief)
2. The user fills in as much of `questions.md` as they want and drops real
   context (calendar invites, transcripts, Slack threads, tickets) into
   `./wish-brief/`.
3. `kazam wish brief` (second run) — granting. kazam shells out to the first
   agent on `$PATH` (Claude, Gemini, Codex, OpenCode) with `./wish-brief/`
   as the CWD. The agent reads everything and writes `brief.yaml`.

kazam does NOT parse files itself. File handling is the agent's job.

## YOLO mode

`kazam wish brief --yolo "1:1 with Sam tomorrow"` skips the workspace and
asks the agent to invent everything from the topic and any context it has
(CLAUDE.md, auto-memory, MCP tools). When the topic looks like the user's
own world (a real meeting, a recent incident), agents with MCP access may
pull from Calendar / Slack / Linear / Granola / Gmail. Public/external
topics ("the history of TLS") never trigger MCP.

## Brief shape (default — agent adapts to subject)

- `meta` block — time, attendees, owner, goal
- One-line goal — `callout` (info)
- Context — `stat_grid` + tight narrative
- Agenda (`steps`) OR timeline (`timeline`)
- Talking points — `card_grid` of 3-4 cards
- Risks (optional) — `callout` (warning) blocks
- Action items — `table` (owner / action / due)

## Kazam YAML conventions

- 2-space indent. No trailing whitespace.
- `shell: document` enables print-optimized layout.
- Multi-line text uses the `|` block literal.
- Full component reference: `kazam agents` or https://tdiderich.github.io/kazam/

Output ONLY the YAML. No prose, no code fences. No placeholder copy.
"#;
