//! `kazam wish deck` — a 7-slide QBR / strategy-review deck.

use super::{Answers, Question, Wish};

pub static DECK: Wish = Wish {
    name: "deck",
    description: "QBR / strategy-review deck (7 slides)",
    default_out: "deck.yaml",
    questions: &[
        Question {
            key: "topic",
            prompt: "What's this deck about?",
            hint: Some("one sentence — becomes the cover title"),
            multiline: false,
        },
        Question {
            key: "audience",
            prompt: "Who's the audience?",
            hint: Some("e.g., \"Leadership team\", \"Engineering all-hands\", \"The board\""),
            multiline: false,
        },
        Question {
            key: "timeframe",
            prompt: "What timeframe does this cover?",
            hint: Some("e.g., \"Q1 2026\", \"Last 90 days\""),
            multiline: false,
        },
        Question {
            key: "wins",
            prompt: "What are the 2-3 biggest wins to highlight?",
            hint: Some("one per line"),
            multiline: true,
        },
        Question {
            key: "challenges",
            prompt: "What are the 1-2 challenges or misses?",
            hint: Some("one per line — be honest"),
            multiline: true,
        },
        Question {
            key: "ask",
            prompt: "What's the single ask or call-to-action at the close?",
            hint: Some("one sentence"),
            multiline: false,
        },
    ],
    render,
    agent_prompt,
    stdout_markdown: STDOUT_MARKDOWN,
};

fn render(a: &Answers) -> String {
    let topic = a.get("topic");
    let audience = a.get("audience");
    let timeframe = a.get("timeframe");
    let wins: Vec<&str> = a
        .get("wins")
        .lines()
        .filter(|l| !l.trim().is_empty())
        .collect();
    let challenges: Vec<&str> = a
        .get("challenges")
        .lines()
        .filter(|l| !l.trim().is_empty())
        .collect();
    let ask = a.get("ask");

    let mut s = String::new();
    s.push_str(&format!("title: {}\n", yaml_string(topic)));
    s.push_str("shell: deck\n");
    s.push_str(&format!("eyebrow: {}\n", yaml_string(timeframe)));
    s.push_str(&format!(
        "subtitle: {}\n",
        yaml_string(&format!("For {}", audience))
    ));
    s.push('\n');
    s.push_str("slides:\n");

    // 1. Cover
    s.push_str("  - label: Cover\n");
    s.push_str("    hide_label: true\n");
    s.push_str("    components:\n");
    s.push_str("      - type: header\n");
    s.push_str(&format!("        title: {}\n", yaml_string(topic)));
    s.push_str(&format!(
        "        subtitle: {}\n",
        yaml_string(&format!("{} · {}", timeframe, audience))
    ));
    s.push_str("        align: center\n\n");

    // 2. Context
    s.push_str("  - label: Context\n");
    s.push_str("    components:\n");
    s.push_str("      - type: header\n");
    s.push_str(&format!(
        "        title: {}\n",
        yaml_string(&format!("{} at a glance", timeframe))
    ));
    s.push_str("      - type: markdown\n");
    s.push_str("        body: |\n");
    s.push_str("          A quick recap before we dig in. Replace this paragraph with the shape of the quarter: where we started, the theme of the cycle, and the context the audience needs before seeing the numbers.\n");
    s.push('\n');

    // 3. Wins
    s.push_str("  - label: Wins\n");
    s.push_str("    components:\n");
    s.push_str("      - type: header\n");
    s.push_str("        title: What went well\n");
    if !wins.is_empty() {
        let columns = wins.len().clamp(1, 4);
        s.push_str("      - type: stat_grid\n");
        s.push_str(&format!("        columns: {}\n", columns));
        s.push_str("        stats:\n");
        for (i, w) in wins.iter().enumerate() {
            let color = ["green", "default", "teal", "yellow"][i % 4];
            s.push_str(&format!(
                "          - label: {}\n",
                yaml_string(&format!("Win {}", i + 1))
            ));
            s.push_str(&format!("            value: {}\n", yaml_string(w)));
            s.push_str(&format!("            color: {}\n", color));
        }
    } else {
        s.push_str("      - type: markdown\n");
        s.push_str("        body: |\n");
        s.push_str(
            "          Add your wins here — swap this block for a `stat_grid` or `card_grid`.\n",
        );
    }
    s.push('\n');

    // 4. Challenges
    s.push_str("  - label: Challenges\n");
    s.push_str("    components:\n");
    s.push_str("      - type: header\n");
    s.push_str("        title: Where we fell short\n");
    if !challenges.is_empty() {
        s.push_str("      - type: card_grid\n");
        s.push_str(&format!(
            "        columns: {}\n",
            challenges.len().clamp(1, 3)
        ));
        s.push_str("        cards:\n");
        for (i, c) in challenges.iter().enumerate() {
            s.push_str(&format!(
                "          - title: {}\n",
                yaml_string(&format!("Challenge {}", i + 1))
            ));
            s.push_str(&format!("            description: {}\n", yaml_string(c)));
            s.push_str("            color: yellow\n");
        }
    } else {
        s.push_str("      - type: callout\n");
        s.push_str("        variant: warning\n");
        s.push_str("        title: Be honest\n");
        s.push_str("        body: Swap this for a specific challenge and the lesson learned. Decks that only show wins get less trust than decks that admit misses.\n");
    }
    s.push('\n');

    // 5. What we learned
    s.push_str("  - label: Learnings\n");
    s.push_str("    components:\n");
    s.push_str("      - type: header\n");
    s.push_str("        title: What we're taking forward\n");
    s.push_str("      - type: markdown\n");
    s.push_str("        body: |\n");
    s.push_str("          The through-line between the wins and the misses. Replace this with the 2-3 patterns you want the audience to remember.\n\n");

    // 6. Next
    s.push_str("  - label: Next\n");
    s.push_str("    components:\n");
    s.push_str("      - type: header\n");
    s.push_str(&format!(
        "        title: {}\n",
        yaml_string(&format!("After {}", timeframe))
    ));
    s.push_str("      - type: steps\n");
    s.push_str("        numbered: true\n");
    s.push_str("        items:\n");
    s.push_str("          - title: Priority 1\n");
    s.push_str("            detail: Replace with your first next-quarter bet and why.\n");
    s.push_str("          - title: Priority 2\n");
    s.push_str("            detail: Replace with the second bet. Keep this list short.\n");
    s.push_str("          - title: Priority 3\n");
    s.push_str("            detail: Replace or delete — three is a healthy ceiling for a QBR.\n\n");

    // 7. The ask
    s.push_str("  - label: The ask\n");
    s.push_str("    components:\n");
    s.push_str("      - type: header\n");
    s.push_str("        title: The ask\n");
    s.push_str("      - type: callout\n");
    s.push_str("        variant: info\n");
    s.push_str("        title: What we need\n");
    s.push_str(&format!("        body: {}\n", yaml_string(ask)));

    s
}

fn agent_prompt(a: &Answers) -> String {
    format!(
        r#"You are generating a 7-slide QBR / strategy-review deck as a single kazam YAML file.

Output ONLY valid YAML — no prose before or after, no code fences. The YAML must start with `title:` and describe a `shell: deck` page.

## Inputs from the user

- Topic:       {topic}
- Audience:    {audience}
- Timeframe:   {timeframe}
- Wins:
{wins}
- Challenges:
{challenges}
- Ask:         {ask}

## Slide plan (7 slides, in this order)

1. Cover — center-aligned header with the topic as title and "{timeframe} · {audience}" as subtitle. Use `hide_label: true`.
2. Context — header + a markdown paragraph framing where the quarter started.
3. Wins — header + `stat_grid` with one stat per win. Use colors: green, default, teal, yellow (cycle).
4. Challenges — header + `card_grid` with one card per challenge, `color: yellow`. Cards: `title` (3-5 word label), `description` (1-2 sentences of honest analysis + what you learned).
5. Learnings — header + markdown with the through-line between wins and misses.
6. Next — header ("After {timeframe}") + `steps` (numbered) with 2-4 next-quarter priorities.
7. The ask — header + `callout` with `variant: info`, the ask verbatim in the body.

## Kazam YAML conventions

- Every page starts with top-level `title`, `shell`, optional `eyebrow` and `subtitle`.
- `slides:` is a list of slides. Each slide has a `label` and `components`.
- Components are typed maps with `type:` — e.g. `type: header`, `type: markdown`, `type: stat_grid`, `type: card_grid`, `type: callout`, `type: steps`.
- Markdown `body:` uses the `|` block literal style.
- Keep the YAML tidy: 2-space indent, no trailing whitespace.

Write the YAML now.
"#,
        topic = a.get("topic"),
        audience = a.get("audience"),
        timeframe = a.get("timeframe"),
        wins = bullet_list(a.get("wins")),
        challenges = bullet_list(a.get("challenges")),
        ask = a.get("ask"),
    )
}

fn bullet_list(s: &str) -> String {
    let lines: Vec<String> = s
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| format!("    - {}", l))
        .collect();
    if lines.is_empty() {
        "    - (none provided)".to_string()
    } else {
        lines.join("\n")
    }
}

/// YAML-safe double-quoted string. Handles backslashes and double quotes.
/// Keep this conservative — we'd rather over-quote than emit invalid YAML.
fn yaml_string(s: &str) -> String {
    let escaped = s.replace('\\', "\\\\").replace('"', "\\\"");
    format!("\"{}\"", escaped)
}

const STDOUT_MARKDOWN: &str = r#"# kazam wish: deck

Grants a 7-slide QBR / strategy-review deck as a populated `kazam` YAML file.

## Ask the user these questions

1. **Topic** — one sentence. Becomes the cover title.
2. **Audience** — e.g., "Leadership team", "Engineering all-hands".
3. **Timeframe** — e.g., "Q1 2026", "Last 90 days".
4. **Wins** — 2-3 biggest wins to highlight, one per line.
5. **Challenges** — 1-2 honest misses, one per line.
6. **Ask** — one-sentence call-to-action for the close.

## Emit this exact YAML shape

```yaml
title: "<topic>"
shell: deck
eyebrow: "<timeframe>"
subtitle: "For <audience>"

slides:
  - label: Cover
    hide_label: true
    components:
      - type: header
        title: "<topic>"
        subtitle: "<timeframe> · <audience>"
        align: center

  - label: Context
    components:
      - type: header
        title: "<timeframe> at a glance"
      - type: markdown
        body: |
          <1-2 paragraphs framing where the quarter started>

  - label: Wins
    components:
      - type: header
        title: What went well
      - type: stat_grid
        columns: <len(wins) clamped 1..=4>
        stats:
          - label: "Win 1"
            value: "<win 1>"
            color: green
          # ... cycle colors: green, default, teal, yellow

  - label: Challenges
    components:
      - type: header
        title: Where we fell short
      - type: card_grid
        columns: <len(challenges) clamped 1..=3>
        cards:
          - title: "<3-5 word label>"
            description: "<honest 1-2 sentence analysis + lesson learned>"
            color: yellow

  - label: Learnings
    components:
      - type: header
        title: What we're taking forward
      - type: markdown
        body: |
          <through-line between wins and misses>

  - label: Next
    components:
      - type: header
        title: "After <timeframe>"
      - type: steps
        numbered: true
        items:
          - title: "Priority 1"
            detail: "<first next-quarter bet + why>"
          # ... 2-4 priorities, keep it short

  - label: The ask
    components:
      - type: header
        title: The ask
      - type: callout
        variant: info
        title: What we need
        body: "<ask verbatim>"
```

## Kazam YAML conventions

- 2-space indent. No trailing whitespace.
- `shell: deck` enables slide navigation + PDF export.
- Markdown body uses `|` block literal.
- Full component reference: `kazam agents` or https://tdiderich.github.io/kazam/

Output ONLY the YAML — no prose before or after, no code fences.
"#;
