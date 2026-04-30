//! `kazam wish dashboard` — a data-dense monitoring or status dashboard.
//!
//! Standard shell, not deck or document. The dashboard adapts to the subject
//! — customer health, deployment status, team metrics, portfolio overview,
//! incident tracker — but the shape is always: headline metrics, entity
//! cards with health badges, detailed table, activity feed, coverage bars.

use super::Wish;

pub static DASHBOARD: Wish = Wish {
    name: "dashboard",
    description: "A data-dense monitoring or status dashboard",
    default_out: "dashboard.yaml",
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
            "The topic the user asked for:\n\n  {}\n\nThis is a real dashboard the user will use to monitor something they care about. Treat it as the user's actual portfolio, team, or system — fabricated entity names are acceptable for illustrative cards, but headline metrics and activity events should be grounded in real data when MCP tools are available.",
            t
        ),
        None => "No topic was specified — pick something from the user's world that would benefit from a dashboard view. Look for: a customer portfolio in HubSpot, open issues in Linear grouped by team, upcoming calendar load, recent deal activity, or deployment/incident status. You MUST ground the choice in a verifiable signal from at least one MCP query. If you genuinely cannot find any grounding signal, build a customer health dashboard with plausible but clearly marked sample data.".to_string(),
    };

    format!(
        r#"You are generating a monitoring/status dashboard as a single kazam YAML file in YOLO mode. The dashboard should feel like a real operational tool — dense with data, scannable at a glance, actionable.

{topic_block}

## MCP-first rule

If the topic names customers, deals, a team, a project, services, or metrics
— i.e. anything from the user's own world — your FIRST actions before writing
any YAML are MCP lookups. Do not skip this step.

Default sequence (skip a tool only if it's clearly irrelevant):

  1. HubSpot — `search_crm_objects` for companies/deals. Capture names, stages,
     amounts, owners, health signals.
  2. Linear — `list_issues` / `list_projects` for open work, blockers, team load.
  3. Google Calendar — `list_events` for upcoming meetings related to the topic.
  4. Granola — `query_granola_meetings` for recent call context.
  5. Slack — `slack_search_public_and_private` for recent threads.
  6. Attention — `search_calls` if the dashboard is sales/CS shaped.

Use what you find. Headline metrics in the `stat_grid` should come from real
counts when possible. Entity cards should use real names and real status.

## What you may improvise

- The dashboard's *structure* (which sections, what order).
- *Thresholds* (what counts as healthy/at-risk/critical) when the user didn't specify.
- *Sample entities* in the card_grid and table when MCP returns fewer than 3-4 items.
- *Formatting* and labels.

## What you must NEVER fabricate (when MCP data is available)

- Headline metric numbers that don't match tool results.
- Entity names you didn't see in any tool output (mark sample data clearly).
- Dates and activity events not grounded in tool results.

When in doubt, use `TBD` or mark with a comment. A dashboard that's 70% real
data + 30% sample is far more useful than one that's 100% plausible-but-fake.

{mcp_guidance}

## Output rules

1. Output ONLY valid YAML — no prose before or after, no ``` code fences, no commentary.
2. The YAML MUST start with `title:` and describe a `shell: standard` page.
3. Every component and field name MUST match the Kazam Authoring Guide below.
4. No placeholder copy. Every section fully populated.
5. If unsure about a field, prefer *omitting* it — most fields are optional.

## Dashboard shape (guidance, not a rigid script)

A good dashboard is scannable in 10 seconds. Default sections:

1. **`stat_grid`** — 3-6 headline metrics. The at-a-glance numbers. Use `color:` to signal health (green/yellow/red). Always lead with this.
2. **Alert callout** (optional) — `callout` with `variant: danger` or `warn` if anything needs immediate attention. Skip if everything is healthy.
3. **Entity cards** — `card_grid` showing the top entities (customers, services, projects) with health `badge:` on each card. 4-6 cards.
4. **Detail table** — `table` with `filterable: true` for the full data set. Columns should include name, status, key metric, owner/assignee.
5. **Activity feed** — `event_timeline` with `show_filter_toggle: true` for recent events. 5-10 events.
6. **Coverage/capacity** (optional) — `progress_bar` components showing utilization, SLA compliance, or workload. Skip if not relevant.

## Kazam Authoring Guide (authoritative schema — validate your output against this)

{guide}

Now write the YAML.
"#,
        topic_block = topic_block,
        mcp_guidance = super::MCP_GUIDANCE,
        guide = crate::agents::AGENTS_MD,
    )
}

const EXAMPLE_DASHBOARD_YAML: &str = include_str!("../../docs/examples/dashboard.yaml");

const REFERENCES: &[(&str, &str)] = &[
    ("kazam-schema.md", crate::agents::AGENTS_MD),
    ("example-dashboard.yaml", EXAMPLE_DASHBOARD_YAML),
];

const QUESTIONS_MD: &str = r#"# Dashboard — structured prompts

Fill in whatever you want to answer up front. Leave any section blank and
the agent will infer it from the other files you've dropped in this folder.
Dashboards adapt to the subject — customer health, deployment status, team
metrics, incident tracking, portfolio overview.

---

## Subject
<!-- What is this dashboard monitoring? One sentence.
e.g., "Customer portfolio health", "Q2 deployment status", "Engineering team load". -->

## Audience
<!-- Who reads this? e.g., ops team, leadership, customer success, SRE. -->

## Headline metrics
<!-- 3-6 numbers that should be visible at a glance. One per line.
e.g., "Total customers: 142", "Healthy: 118", "Critical: 5". -->

## Entities
<!-- What things are tracked? Customers, services, deployments, team members,
projects? List the ones that should appear as cards. -->

## Status model
<!-- How is health determined? Green/yellow/red? Score-based? SLA compliance?
Describe the thresholds. -->

## Activity feed
<!-- What events should show in the timeline? Deploys, incidents, customer
activity, ticket updates? What counts as major vs. minor? -->

## Alerts
<!-- What conditions warrant a danger/warning callout at the top?
e.g., "Any account with no login in 14+ days", "P1 tickets open >24h". -->

## Data sources
<!-- Optional — where does the data come from? HubSpot, Linear, Datadog,
internal API? Helps the agent know which MCP tools to query. -->
"#;

const README_MD: &str = r#"# kazam wish dashboard — workspace

This folder is where you gather the real data for your dashboard. When you run
`kazam wish dashboard` from the parent directory, kazam shells out to an agent
(Claude, Gemini, Codex, or OpenCode) with this folder as the working
directory. The agent reads everything here and writes `../dashboard.yaml`.

## What to put here

- **`questions.md`** — structured prompts. Fill in what you know. Blanks
  are fine; the agent infers from the other files.
- **Anything with real data** — CSV exports, metric dumps, customer lists,
  deployment logs, incident reports, monitoring tool output. Drop it in.
  kazam does not parse files itself; the agent handles reading with its
  own tools.
- **`reference/`** — schema + a worked example that kazam wrote here for
  you. The agent consults these for shape + exact field names. Version-
  matched to the kazam binary you have installed.

## When you're ready

```
kazam wish dashboard
```

from the parent directory (not from inside this folder).

## Flags

- `--agent claude|gemini|codex|opencode` — force a specific agent
- `--dry-run` — print the prompt that would be sent (don't run the agent)
- `--out path/to/out.yaml` — write somewhere other than `../dashboard.yaml`
"#;

const AGENT_PROMPT: &str = r#"You are generating a monitoring/status dashboard as a single kazam YAML file. The subject — customer health, deployment status, team metrics, portfolio overview, incident tracker — comes from the user. Adapt to what they actually need to monitor.

The current working directory is a wish workspace with this layout:

  questions.md           — structured prompts. The user may have filled some in.
  <other files>          — real data dropped in by the user (CSV exports, metric
                           dumps, customer lists, monitoring output, anything).
  reference/kazam-schema.md   — authoritative field/enum reference for every
                                kazam component. Consult for EXACT field names.
  reference/example-dashboard.yaml — a worked dashboard in valid kazam YAML.
                                Use it as a shape reference (sections, density).

## What to do

1. Read `questions.md`. For each `##` section the user filled in, use their answer.
2. For any section left blank, read the other top-level files in this directory
   and infer a confident answer from them.
3. Consult `reference/kazam-schema.md` for field names and enum values.
   Consult `reference/example-dashboard.yaml` for shape/style.
4. Choose a section structure that fits the subject (see "Dashboard shape" below).
5. Write a populated `shell: standard` YAML to stdout.

## Output rules

1. Output ONLY valid YAML — no prose before or after, no ``` code fences, no commentary.
2. The YAML MUST start with `title:` and describe a `shell: standard` page.
3. Every component and field name MUST match `reference/kazam-schema.md` exactly.
   Fields like `detail`, `description`, and `body` are distinct — don't guess.
4. No placeholder copy ("replace this", "your metric here", etc.). Every field
   must be fully populated. If the user's data is thin, infer confidently
   from the other files.
5. If unsure about a field, prefer *omitting* it — most fields are optional.

## Dashboard shape (guidance, not a rigid script)

A good dashboard is scannable in 10 seconds. Default sections — adapt to the subject:

1. **`stat_grid`** — 3-6 headline metrics at the top. Use `color:` to signal
   health (green = good, yellow = attention, red = critical). Always lead
   with this.
2. **Alert callout** (optional) — `callout` with `variant: danger` if
   anything needs immediate attention. Skip if everything is healthy.
3. **Entity cards** — `card_grid` with the top entities. Each card gets a
   health `badge:` (green/yellow/red) and a short `description:`. 4-6 cards
   showing a mix of healthy and unhealthy for contrast.
4. **Detail table** — `table` with `filterable: true`. Include name, status,
   key metric, and owner columns at minimum. This is the drill-down view.
5. **Activity feed** — `event_timeline` with `show_filter_toggle: true` and
   5-10 recent events. Use `severity:` consistently (major for escalations,
   minor for notable changes, info for routine).
6. **Coverage/capacity** (optional) — `progress_bar` components showing team
   workload, SLA compliance, or system utilization. Skip if not relevant.

Dashboards are DENSE. Fill every component with real or realistic data.
Section labels should match the subject's vocabulary (a customer dashboard
uses "Portfolio", "Accounts"; a deployment dashboard uses "Services",
"Rollouts").

Now read the workspace and write the YAML.
"#;

const STDOUT_MARKDOWN: &str = r#"# kazam wish: dashboard

Grants a data-dense monitoring or status dashboard as a `kazam` YAML file.
The dashboard adapts to the subject — customer health, deployment status,
team metrics, incident tracker, portfolio overview. kazam scaffolds a
workspace; the user drops their real data in; the agent reads everything
and writes the dashboard.

## Flow

1. `kazam wish dashboard` (first run) — scaffolds `./wish-dashboard/` with:
   - `questions.md` (structured prompts — subject, audience, metrics,
     entities, status model, activity feed, alerts)
   - `README.md` (usage hint)
   - `reference/kazam-schema.md` (full schema, version-matched to the binary)
   - `reference/example-dashboard.yaml` (worked example dashboard)
2. The user fills in as much of `questions.md` as they want and drops real
   data (CSV exports, metric dumps, customer lists) into `./wish-dashboard/`.
3. `kazam wish dashboard` (second run) — granting. kazam shells out to the
   first agent on `$PATH` (Claude, Gemini, Codex, OpenCode) with
   `./wish-dashboard/` as the CWD. The agent reads everything and writes
   `dashboard.yaml`.

kazam does NOT parse files itself. File handling is the agent's job.

## YOLO mode

`kazam wish dashboard --yolo "customer health"` skips the workspace and asks
the agent to invent everything from the topic and any context it has
(CLAUDE.md, auto-memory, MCP tools). When the topic looks like the user's own
world (real customers, real services), agents with MCP access pull from
HubSpot / Linear / Calendar / Slack to populate real data.

## Dashboard shape (default — agent adapts to subject)

- `stat_grid` — 3-6 headline metrics with health colors
- Alert callout (if anything critical)
- `card_grid` — top entities with health badges
- `table` — full data set, filterable + sortable
- `event_timeline` — recent activity with severity filter
- `progress_bar` — capacity/coverage (optional)

## Kazam YAML conventions

- 2-space indent. No trailing whitespace.
- `shell: standard` enables full-width layout with sticky nav.
- Multi-line text uses the `|` block literal.
- Full component reference: `kazam agents` or https://tdiderich.github.io/kazam/

Output ONLY the YAML. No prose, no code fences. No placeholder copy.
"#;
