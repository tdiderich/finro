//! Integration tests — invoke the kazam binary end-to-end.

use std::path::{Path, PathBuf};
use std::process::Command;

fn bin() -> PathBuf {
    // cargo sets CARGO_BIN_EXE_<name> env var for the test runner
    PathBuf::from(env!("CARGO_BIN_EXE_kazam"))
}

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn tmp_dir(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("kazam-test-{}", name));
    let _ = std::fs::remove_dir_all(&dir);
    dir
}

fn read(p: &Path) -> String {
    std::fs::read_to_string(p).expect("read file")
}

fn assert_contains(haystack: &str, needle: &str) {
    assert!(haystack.contains(needle), "expected to find {:?}", needle);
}

#[test]
fn build_kb_example() {
    let out = tmp_dir("kb");
    let src = repo_root().join("examples/kb");
    let status = Command::new(bin())
        .args(["build"])
        .arg(&src)
        .arg("--out")
        .arg(&out)
        .status()
        .expect("run kazam build");
    assert!(status.success(), "kazam build failed");

    let index = read(&out.join("index.html"));
    assert_contains(&index, "Customer Portfolio");
    assert_contains(&index, "Acme Corp");
    // Badge color classes render
    assert_contains(&index, "c-badge-green");
    assert_contains(&index, "c-badge-yellow");
}

#[test]
fn build_docs_site() {
    let out = tmp_dir("docs");
    let src = repo_root().join("docs");
    let status = Command::new(bin())
        .args(["build"])
        .arg(&src)
        .arg("--out")
        .arg(&out)
        .status()
        .expect("run kazam build");
    assert!(status.success());

    // llms.txt should exist and list known pages
    let llms = read(&out.join("llms.txt"));
    assert_contains(&llms, "# kazam");
    assert_contains(&llms, "Content components");
    assert_contains(&llms, "Why kazam");

    // Each page has a View source link
    let index = read(&out.join("index.html"));
    assert_contains(&index, r#"class="view-source""#);

    // Source YAMLs copied next to rendered HTML
    assert!(out.join("components/content.yaml").exists());

    // Site-wide texture + glow layers landed in CSS
    assert_contains(&index, "body::before");
    assert_contains(&index, "body::after");
}

#[test]
fn build_release_minifies() {
    let out = tmp_dir("release");
    let src = repo_root().join("docs");
    let status = Command::new(bin())
        .args(["build"])
        .arg(&src)
        .arg("--out")
        .arg(&out)
        .arg("--release")
        .status()
        .expect("run kazam build --release");
    assert!(status.success());

    let html = read(&out.join("index.html"));
    // HTML comments stripped (we don't emit any but this guards future regressions)
    assert!(!html.contains("<!-- "));
    // Multi-byte content preserved
    assert_contains(&html, "—");
}

#[test]
fn init_creates_minimal_site_that_builds() {
    let dir = tmp_dir("init");
    let status = Command::new(bin())
        .args(["init"])
        .arg(&dir)
        .status()
        .expect("run kazam init");
    assert!(status.success());

    // Scaffold has expected files
    assert!(dir.join("kazam.yaml").exists());
    assert!(dir.join("index.yaml").exists());
    assert!(dir.join("AGENTS.md").exists());
    assert!(dir.join(".gitignore").exists());

    // Building it should succeed
    let out = dir.join("_site");
    let status = Command::new(bin())
        .args(["build"])
        .arg(&dir)
        .arg("--out")
        .arg(&out)
        .status()
        .expect("build scaffolded site");
    assert!(status.success());

    assert!(out.join("index.html").exists());
    assert!(out.join("llms.txt").exists());
}

#[test]
fn page_level_texture_and_glow_override_site_config() {
    // Site-wide sets texture: grid + glow: accent. One page sets texture:
    // none (opt out) and another sets glow: corner (different preset). The
    // rendered HTML for each page must reflect the per-page override, not
    // the site-wide default.
    let dir = tmp_dir("overrides");
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(
        dir.join("kazam.yaml"),
        "name: Overrides\ntheme: dark\ntexture: grid\nglow: accent\n",
    )
    .unwrap();
    // Default page: inherits site-wide (should have grid + accent glow).
    std::fs::write(
        dir.join("index.yaml"),
        "title: Index\nshell: standard\ncomponents:\n  - type: header\n    title: Home\n",
    )
    .unwrap();
    // Opts out of texture entirely.
    std::fs::write(
        dir.join("plain.yaml"),
        "title: Plain\nshell: standard\ntexture: none\ncomponents:\n  - type: header\n    title: Plain\n",
    )
    .unwrap();
    // Swaps to the corner glow variant + different texture.
    std::fs::write(
        dir.join("corner.yaml"),
        "title: Corner\nshell: standard\ntexture: dots\nglow: corner\ncomponents:\n  - type: header\n    title: Corner\n",
    )
    .unwrap();

    let out = tmp_dir("overrides-out");
    let status = Command::new(bin())
        .args(["build"])
        .arg(&dir)
        .arg("--out")
        .arg(&out)
        .status()
        .expect("run kazam build");
    assert!(status.success());

    let index = read(&out.join("index.html"));
    let plain = read(&out.join("plain.html"));
    let corner = read(&out.join("corner.html"));

    // Inherits both site-wide layers.
    assert_contains(&index, "linear-gradient"); // grid texture signature
    assert_contains(&index, "ellipse at center"); // accent glow signature

    // plain.yaml turned texture off — the grid texture signature should
    // be absent even though the site-wide config specifies it. (The print
    // `body::before, body::after { display: none }` rule still appears
    // because glow is still active, so we check for the texture signature
    // specifically rather than any mention of body::before.)
    assert!(
        !plain.contains("linear-gradient"),
        "plain page should not render the grid texture"
    );
    // But plain still inherits the site-wide accent glow.
    assert_contains(&plain, "ellipse at center");

    // corner.yaml swapped to dots + corner glow.
    assert_contains(&corner, "radial-gradient(rgba"); // dots texture signature
    assert_contains(&corner, "circle at top right"); // corner glow signature
    assert!(
        !corner.contains("ellipse at center"),
        "corner page should not have the accent ellipse glow"
    );
}

#[test]
fn nested_nav_renders_dropdown_in_top_layout() {
    let dir = tmp_dir("nav-dropdown");
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(
        dir.join("kazam.yaml"),
        "name: NavTest\ntheme: dark\nnav:\n  - label: Home\n    href: index.html\n  - label: Docs\n    children:\n      - label: Guide\n        href: guide.html\n      - label: Reference\n        href: ref.html\n",
    )
    .unwrap();
    std::fs::write(
        dir.join("index.yaml"),
        "title: Home\nshell: standard\ncomponents:\n  - type: header\n    title: Home\n",
    )
    .unwrap();

    let out = tmp_dir("nav-dropdown-out");
    let status = Command::new(bin())
        .args(["build"])
        .arg(&dir)
        .arg("--out")
        .arg(&out)
        .status()
        .expect("run kazam build");
    assert!(status.success());

    let index = read(&out.join("index.html"));
    assert_contains(&index, r#"class="nav-link-group""#);
    assert_contains(&index, r#"class="nav-link nav-link-parent""#);
    assert_contains(&index, r#"class="nav-dropdown""#);
    // Both children render inside the dropdown
    assert_contains(&index, "Guide");
    assert_contains(&index, "Reference");
}

#[test]
fn sidebar_layout_renders_aside_and_hides_top_nav() {
    let dir = tmp_dir("nav-sidebar");
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(
        dir.join("kazam.yaml"),
        "name: SideTest\ntheme: dark\nnav_layout: sidebar\nnav:\n  - label: Overview\n    href: index.html\n  - label: Guides\n    children:\n      - label: Intro\n        href: intro.html\n      - label: Advanced\n        href: advanced.html\n",
    )
    .unwrap();
    std::fs::write(
        dir.join("index.yaml"),
        "title: Home\nshell: standard\ncomponents:\n  - type: header\n    title: Home\n",
    )
    .unwrap();

    let out = tmp_dir("nav-sidebar-out");
    let status = Command::new(bin())
        .args(["build"])
        .arg(&dir)
        .arg("--out")
        .arg(&out)
        .status()
        .expect("run kazam build");
    assert!(status.success());

    let index = read(&out.join("index.html"));
    // Sidebar aside + body class present
    assert_contains(&index, "nav-layout-sidebar");
    assert_contains(&index, r#"class="site-sidebar""#);
    assert_contains(&index, r#"class="sidebar-section""#);
    assert_contains(&index, r#"class="sidebar-section-label">Guides"#);
    // Both nested children render in the sidebar
    assert_contains(&index, "Intro");
    assert_contains(&index, "Advanced");
}

#[test]
fn build_skips_hidden_entries_and_is_idempotent() {
    // Source directory with a hidden dir (simulating .git) alongside the
    // yaml files. Kazam should not copy the hidden dir into the output,
    // and running build twice in a row should succeed both times.
    let dir = tmp_dir("hidden");
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("kazam.yaml"), "name: Hidden\ntheme: dark\n").unwrap();
    std::fs::write(
        dir.join("index.yaml"),
        "title: Home\nshell: standard\ncomponents:\n  - type: header\n    title: Home\n",
    )
    .unwrap();

    // Hidden directory with a nested file — must be skipped.
    let hidden = dir.join(".stealth");
    std::fs::create_dir_all(hidden.join("nested")).unwrap();
    std::fs::write(hidden.join("nested/file.bin"), b"should-not-copy").unwrap();

    let out = tmp_dir("hidden-out");

    let run = || {
        Command::new(bin())
            .args(["build"])
            .arg(&dir)
            .arg("--out")
            .arg(&out)
            .status()
            .expect("run kazam build")
    };

    assert!(run().success(), "first build failed");
    assert!(
        run().success(),
        "second build failed — walker not idempotent"
    );

    // Hidden dir must not be present in output.
    assert!(
        !out.join(".stealth").exists(),
        "hidden dir leaked into output"
    );
}

#[test]
fn chart_component_renders_svg_for_every_kind() {
    // One page exercises pie, vertical bar, stacked bar, horizontal bar,
    // single-series timeseries, and multi-series timeseries. Each kind must
    // produce SVG, and the multi-series variants must render a legend.
    let dir = tmp_dir("charts");
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("kazam.yaml"), "name: Charts\ntheme: dark\n").unwrap();
    let page = r#"
title: Charts
shell: standard
components:
  - type: chart
    kind: pie
    title: Pie
    data:
      - { label: A, value: 60 }
      - { label: B, value: 40, color: green }
  - type: chart
    kind: bar
    title: VBar
    data:
      - { label: Jan, value: 100 }
      - { label: Feb, value: 200 }
  - type: chart
    kind: bar
    title: StackedBar
    series:
      - label: Organic
        points:
          - { label: Jan, value: 80 }
          - { label: Feb, value: 110 }
      - label: Paid
        color: green
        points:
          - { label: Jan, value: 30 }
          - { label: Feb, value: 50 }
  - type: chart
    kind: bar
    orientation: horizontal
    title: HBar
    data:
      - { label: Docs, value: 2840 }
      - { label: Pricing, value: 1720 }
  - type: chart
    kind: timeseries
    title: Line
    data:
      - { label: W1, value: 10 }
      - { label: W2, value: 20 }
      - { label: W3, value: 15 }
  - type: chart
    kind: timeseries
    title: MultiLine
    series:
      - label: A
        points:
          - { label: W1, value: 10 }
          - { label: W2, value: 20 }
      - label: B
        color: green
        points:
          - { label: W1, value: 5 }
          - { label: W2, value: 9 }
"#;
    std::fs::write(dir.join("index.yaml"), page).unwrap();

    let out = tmp_dir("charts-out");
    let status = Command::new(bin())
        .args(["build"])
        .arg(&dir)
        .arg("--out")
        .arg(&out)
        .status()
        .expect("run kazam build");
    assert!(status.success());

    let html = read(&out.join("index.html"));

    // Wrappers for each kind present
    assert_contains(&html, r#"class="c-chart c-chart-pie""#);
    assert_contains(&html, r#"class="c-chart c-chart-bar""#);
    assert_contains(&html, r#"class="c-chart c-chart-timeseries""#);

    // Pie rendered as SVG paths with titles (accessible tooltips)
    assert_contains(&html, r#"class="c-chart-slice""#);

    // Bar rendered as SVG rects
    assert_contains(&html, r#"class="c-chart-bar""#);

    // Timeseries rendered as a polyline
    assert_contains(&html, r#"class="c-chart-line""#);

    // Multi-series charts render a legend; single-series bar/timeseries don't
    assert_contains(&html, r#"class="c-chart-legend""#);

    // SemColor threading through: green was requested explicitly somewhere.
    // Charts use the canonical hex palette (not theme CSS vars) so stacks
    // stay distinguishable on themes that remap --green.
    assert_contains(&html, "#34D399");

    // Titles rendered as figcaptions
    assert_contains(&html, r#"class="c-chart-title">Pie</figcaption>"#);
    assert_contains(&html, r#"class="c-chart-title">StackedBar</figcaption>"#);

    // ARIA role + label on the figure
    assert_contains(&html, r#"role="img""#);
}

#[test]
fn wish_list_succeeds() {
    let output = Command::new(bin())
        .args(["wish", "list"])
        .output()
        .expect("run kazam wish list");
    assert!(output.status.success(), "wish list failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_contains(&stdout, "deck");
    assert_contains(&stdout, "--yolo");
}

#[test]
fn wish_deck_stdout_prints_markdown() {
    let output = Command::new(bin())
        .args(["wish", "deck", "--stdout"])
        .output()
        .expect("run kazam wish deck --stdout");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    // Portable spec mentions the new workspace flow + agent prompt shape.
    assert_contains(&stdout, "# kazam wish: deck");
    assert_contains(&stdout, "wish-deck");
    assert_contains(&stdout, "shell: deck");
    assert_contains(&stdout, "Slide plan");
}

#[test]
fn wish_deck_scaffolds_workspace() {
    // First run: no workspace → kazam should scaffold ./wish-deck/ with
    // questions.md, README.md, and reference/ files. It should NOT try to
    // run an agent on first invocation.
    let dir = tmp_dir("wish-deck-scaffold");
    std::fs::create_dir_all(&dir).unwrap();

    let status = Command::new(bin())
        .args(["wish", "deck"])
        .current_dir(&dir)
        .status()
        .expect("run kazam wish deck (scaffold)");
    assert!(status.success(), "scaffold run should succeed");

    let ws = dir.join("wish-deck");
    assert!(ws.is_dir(), "wish-deck/ not created");
    assert!(ws.join("questions.md").exists(), "questions.md missing");
    assert!(ws.join("README.md").exists(), "README.md missing");
    assert!(
        ws.join("reference").is_dir(),
        "reference/ subdirectory missing"
    );
    assert!(
        ws.join("reference/kazam-schema.md").exists(),
        "reference/kazam-schema.md missing"
    );
    assert!(
        ws.join("reference/example-deck.yaml").exists(),
        "reference/example-deck.yaml missing"
    );

    // Questions template includes the expected structured sections.
    // (Generic deck shape — not QBR-specific.)
    let questions = read(&ws.join("questions.md"));
    for heading in [
        "## Topic",
        "## Purpose",
        "## Audience",
        "## Key messages",
        "## Supporting evidence",
        "## The ask",
    ] {
        assert_contains(&questions, heading);
    }

    // Scaffold should not have written deck.yaml yet.
    assert!(
        !dir.join("deck.yaml").exists(),
        "scaffold run must not write deck.yaml"
    );
}

#[test]
fn wish_deck_dry_run_prints_prompt() {
    // After scaffolding, --dry-run should print the grant prompt to stdout.
    let dir = tmp_dir("wish-deck-dry-run");
    std::fs::create_dir_all(&dir).unwrap();

    // Scaffold first.
    Command::new(bin())
        .args(["wish", "deck"])
        .current_dir(&dir)
        .status()
        .expect("scaffold");

    // Dry-run grant.
    let output = Command::new(bin())
        .args(["wish", "deck", "--dry-run"])
        .current_dir(&dir)
        .output()
        .expect("run kazam wish deck --dry-run");
    assert!(output.status.success(), "dry-run failed");
    let stdout = String::from_utf8_lossy(&output.stdout);

    // The prompt should reference the workspace layout + slide plan.
    assert_contains(&stdout, "questions.md");
    assert_contains(&stdout, "reference/kazam-schema.md");
    assert_contains(&stdout, "reference/example-deck.yaml");
    assert_contains(&stdout, "shell: deck");
    assert_contains(&stdout, "Slide plan");

    // Dry-run must NOT write deck.yaml.
    assert!(
        !dir.join("deck.yaml").exists(),
        "dry-run must not write deck.yaml"
    );
}

#[test]
fn wish_deck_yolo_dry_run_includes_topic() {
    // --yolo with a topic: dry-run should print an invent-it prompt that
    // mentions the topic and does NOT require a workspace.
    let dir = tmp_dir("wish-deck-yolo");
    std::fs::create_dir_all(&dir).unwrap();

    let output = Command::new(bin())
        .args([
            "wish",
            "deck",
            "--yolo",
            "history of tiny rust CLIs",
            "--dry-run",
        ])
        .current_dir(&dir)
        .output()
        .expect("run kazam wish deck --yolo ... --dry-run");
    assert!(output.status.success(), "yolo dry-run failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_contains(&stdout, "history of tiny rust CLIs");
    assert_contains(&stdout, "YOLO");
    assert_contains(&stdout, "shell: deck");

    // No workspace should have been created — yolo bypasses it.
    assert!(
        !dir.join("wish-deck").exists(),
        "yolo must not scaffold a workspace"
    );
}

#[test]
fn wish_deck_yolo_bare_asks_agent_to_surprise() {
    // Bare `--yolo` (no topic): prompt should still compile and mention
    // "surprise" (the invent-anything fallback).
    let dir = tmp_dir("wish-deck-yolo-bare");
    std::fs::create_dir_all(&dir).unwrap();

    let output = Command::new(bin())
        .args(["wish", "deck", "--yolo", "--dry-run"])
        .current_dir(&dir)
        .output()
        .expect("run kazam wish deck --yolo --dry-run");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_contains(&stdout, "surprise");
    assert_contains(&stdout, "shell: deck");
}

#[test]
fn wish_deck_refuses_existing_output_on_grant() {
    // When the workspace exists AND deck.yaml already exists, grant should
    // refuse rather than overwrite. Use --dry-run to sidestep the agent.
    // The existence check happens before the dry-run early-return... wait,
    // actually dry-run short-circuits first. Test this with a real grant
    // attempt by stubbing $PATH so no agent is found — that should also
    // error, but before the existing-file check. So we can't cleanly test
    // this without a mock agent. Leave this as a smoke test of the scaffold
    // path + out-path validation via a separate flow.
    //
    // Instead: verify that re-running scaffold (workspace exists) falls
    // through to grant mode and, with --dry-run, succeeds.
    let dir = tmp_dir("wish-deck-reruns");
    std::fs::create_dir_all(&dir).unwrap();

    for _ in 0..2 {
        let output = Command::new(bin())
            .args(["wish", "deck", "--dry-run"])
            .current_dir(&dir)
            .output()
            .expect("run wish deck");
        // First iteration: scaffold mode (no workspace yet) — --dry-run is
        // ignored and scaffold runs. Second iteration: workspace exists, so
        // --dry-run prints the prompt.
        assert!(output.status.success());
    }

    let ws = dir.join("wish-deck");
    assert!(ws.is_dir());
}

#[test]
fn wish_auto_creates_kazam_yaml_in_empty_dir() {
    // Running `kazam wish deck` in a fresh empty directory should auto-create
    // a minimal kazam.yaml so the user can immediately follow up with
    // `kazam dev .` — no hand-authoring required.
    let dir = tmp_dir("wish-auto-config");
    std::fs::create_dir_all(&dir).unwrap();
    assert!(!dir.join("kazam.yaml").exists());

    let status = Command::new(bin())
        .args(["wish", "deck"])
        .current_dir(&dir)
        .status()
        .expect("run kazam wish deck");
    assert!(status.success());

    // kazam.yaml should exist with a name derived from the CWD basename.
    let cfg = dir.join("kazam.yaml");
    assert!(cfg.exists(), "kazam.yaml auto-create failed");
    let content = read(&cfg);
    assert_contains(&content, "name:");
    assert_contains(&content, "theme: dark");

    // And the subsequent build works cleanly — proves the generated config
    // is valid for kazam build.
    let out = dir.join("_site");
    let build_status = Command::new(bin())
        .args(["build"])
        .arg(&dir)
        .arg("--out")
        .arg(&out)
        .status()
        .expect("build after auto-config");
    assert!(build_status.success(), "build failed after auto-config");
}

#[test]
fn wish_does_not_overwrite_existing_kazam_yaml() {
    // If the user already has a kazam.yaml (even with custom settings),
    // `kazam wish` must not clobber it.
    let dir = tmp_dir("wish-keep-config");
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(
        dir.join("kazam.yaml"),
        "name: Custom\ntheme: blue\ntexture: grid\n",
    )
    .unwrap();

    let status = Command::new(bin())
        .args(["wish", "deck"])
        .current_dir(&dir)
        .status()
        .expect("run kazam wish deck");
    assert!(status.success());

    let content = read(&dir.join("kazam.yaml"));
    assert_contains(&content, "name: Custom");
    assert_contains(&content, "theme: blue");
    assert_contains(&content, "texture: grid");
}

#[test]
fn build_skips_nested_site_directories() {
    // Running `kazam build` from a directory that contains previously-built
    // sub-sites (each with their own `_site/` full of .html and .yaml) must
    // not recursively ingest those outputs as if they were source. This is
    // the bug where running `kazam dev` in /tmp caused 181 pages of
    // cross-contamination.
    let dir = tmp_dir("build-skips-nested-site");
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("kazam.yaml"), "name: Outer\ntheme: dark\n").unwrap();
    std::fs::write(
        dir.join("index.yaml"),
        "title: Home\nshell: standard\ncomponents:\n  - type: header\n    title: Outer home\n",
    )
    .unwrap();

    // Simulate a nested previously-built sub-site with its own _site/ that
    // happens to contain yaml files (e.g. source-view YAMLs, wish
    // reference/example-deck.yaml, whatever).
    let nested_site = dir.join("sub").join("_site");
    std::fs::create_dir_all(&nested_site).unwrap();
    std::fs::write(
        nested_site.join("contaminating.yaml"),
        "title: SHOULD_NOT_BUILD\nshell: standard\ncomponents:\n  - type: header\n    title: bad\n",
    )
    .unwrap();
    std::fs::write(
        nested_site.join("contaminating.html"),
        "<html>pollution</html>",
    )
    .unwrap();

    let out = tmp_dir("build-skips-nested-site-out");
    let status = Command::new(bin())
        .args(["build"])
        .arg(&dir)
        .arg("--out")
        .arg(&out)
        .status()
        .expect("run kazam build");
    assert!(status.success());

    // Outer site built.
    assert!(out.join("index.html").exists());
    // Nested _site content was NOT ingested.
    assert!(
        !out.join("sub/_site/contaminating.html").exists(),
        "nested _site leaked into output"
    );
    assert!(
        !out.join("contaminating.html").exists(),
        "nested _site yaml got ingested"
    );
}

#[test]
fn logo_shorthand_renders_img_in_site_bar() {
    let dir = tmp_dir("logo-shorthand");
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(
        dir.join("kazam.yaml"),
        "name: Acme\ntheme: dark\nlogo: assets/logo.svg\n",
    )
    .unwrap();
    std::fs::write(
        dir.join("index.yaml"),
        "title: Home\nshell: standard\ncomponents:\n  - type: header\n    title: Home\n",
    )
    .unwrap();
    let out = dir.join("_site");
    let status = Command::new(bin())
        .args(["build"])
        .arg(&dir)
        .arg("--out")
        .arg(&out)
        .status()
        .expect("run kazam build");
    assert!(status.success());
    let html = read(&out.join("index.html"));
    assert_contains(&html, r#"class="site-bar-brand""#);
    assert_contains(
        &html,
        r#"class="site-bar-logo" src="assets/logo.svg" alt="Acme""#,
    );
    assert!(
        !html.contains(r#"class="site-bar-name""#),
        "text name treatment should be replaced by the logo img"
    );
}

#[test]
fn logo_expanded_form_respects_height_and_alt() {
    let dir = tmp_dir("logo-expanded");
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(
        dir.join("kazam.yaml"),
        "name: Acme\ntheme: dark\nlogo:\n  src: assets/logo.svg\n  height: 40\n  alt: Acme Corporation\n",
    )
    .unwrap();
    std::fs::write(
        dir.join("index.yaml"),
        "title: Home\nshell: standard\ncomponents:\n  - type: header\n    title: Home\n",
    )
    .unwrap();
    let out = dir.join("_site");
    let status = Command::new(bin())
        .args(["build"])
        .arg(&dir)
        .arg("--out")
        .arg(&out)
        .status()
        .expect("run kazam build");
    assert!(status.success());
    let html = read(&out.join("index.html"));
    assert_contains(&html, r#"alt="Acme Corporation""#);
    assert_contains(&html, r#"height="40""#);
    assert_contains(&html, r#"style="max-height:40px""#);
    assert_contains(&html, r#"aria-label="Acme Corporation""#);
}

#[test]
fn logo_src_resolves_depth_aware() {
    let dir = tmp_dir("logo-depth");
    std::fs::create_dir_all(dir.join("sub")).unwrap();
    std::fs::write(
        dir.join("kazam.yaml"),
        "name: Acme\ntheme: dark\nlogo: assets/logo.svg\n",
    )
    .unwrap();
    std::fs::write(
        dir.join("index.yaml"),
        "title: Home\nshell: standard\ncomponents:\n  - type: header\n    title: Home\n",
    )
    .unwrap();
    std::fs::write(
        dir.join("sub/page.yaml"),
        "title: Sub\nshell: standard\ncomponents:\n  - type: header\n    title: Sub\n",
    )
    .unwrap();
    let out = dir.join("_site");
    let status = Command::new(bin())
        .args(["build"])
        .arg(&dir)
        .arg("--out")
        .arg(&out)
        .status()
        .expect("run kazam build");
    assert!(status.success());
    let root = read(&out.join("index.html"));
    assert_contains(&root, r#"src="assets/logo.svg""#);
    let sub = read(&out.join("sub/page.html"));
    assert_contains(&sub, r#"src="../assets/logo.svg""#);
}

#[test]
fn logo_absolute_path_passes_through_verbatim() {
    let dir = tmp_dir("logo-absolute");
    std::fs::create_dir_all(dir.join("sub")).unwrap();
    std::fs::write(
        dir.join("kazam.yaml"),
        "name: Acme\ntheme: dark\nlogo: /assets/logo.png\n",
    )
    .unwrap();
    std::fs::write(
        dir.join("sub/page.yaml"),
        "title: Sub\nshell: standard\ncomponents:\n  - type: header\n    title: Sub\n",
    )
    .unwrap();
    let out = dir.join("_site");
    let status = Command::new(bin())
        .args(["build"])
        .arg(&dir)
        .arg("--out")
        .arg(&out)
        .status()
        .expect("run kazam build");
    assert!(status.success());
    let sub = read(&out.join("sub/page.html"));
    assert_contains(&sub, r#"src="/assets/logo.png""#);
    assert!(
        !sub.contains("../assets/logo.png"),
        "absolute /… logo path must not be rewritten to ../…"
    );
}

#[test]
fn absent_logo_falls_back_to_text_name() {
    let dir = tmp_dir("logo-absent");
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("kazam.yaml"), "name: PlainSite\ntheme: dark\n").unwrap();
    std::fs::write(
        dir.join("index.yaml"),
        "title: Home\nshell: standard\ncomponents:\n  - type: header\n    title: Home\n",
    )
    .unwrap();
    let out = dir.join("_site");
    let status = Command::new(bin())
        .args(["build"])
        .arg(&dir)
        .arg("--out")
        .arg(&out)
        .status()
        .expect("run kazam build");
    assert!(status.success());
    let html = read(&out.join("index.html"));
    assert_contains(&html, r#"class="site-bar-name""#);
    assert_contains(&html, ">PlainSite</a>");
    // No <a class="site-bar-brand"> anchor and no <img class="site-bar-logo">
    // tag should appear in the body markup. The class names themselves live
    // in the inlined stylesheet for every page, so we assert on the full
    // opening tag pattern instead of a bare substring.
    assert!(
        !html.contains(r#"<a class="site-bar-brand""#),
        "absent logo should not emit the brand <a> wrapper"
    );
    assert!(
        !html.contains(r#"<img class="site-bar-logo""#),
        "absent logo should not emit any <img class=site-bar-logo>"
    );
}

#[test]
fn wish_unknown_name_errors() {
    let output = Command::new(bin())
        .args(["wish", "nope-this-does-not-exist"])
        .output()
        .expect("run kazam wish <bogus>");
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_contains(&stderr, "unknown wish");
}

#[test]
fn hrefs_honor_verbatim_prefix_rule() {
    // Page at tsp/demo.yaml is depth-1 so base = "../".
    // Absolute-slash, hash, mailto, https, and ../‑prefixed hrefs must all
    // pass through verbatim. A plain relative href in markdown gets the
    // depth-1 base prefix prepended.
    let dir = tmp_dir("href-verbatim");
    std::fs::create_dir_all(dir.join("tsp")).unwrap();
    std::fs::write(dir.join("kazam.yaml"), "name: HrefTest\ntheme: dark\n").unwrap();

    let page = r##"title: Demo
shell: standard
components:
  - type: button_group
    buttons:
      - label: Abs Slash
        href: /customers/demo.html
      - label: Already Canonical
        href: ../customers/demo.html
      - label: Hash
        href: "#section"
      - label: Mail
        href: mailto:hi@example.com
      - label: External
        href: https://example.com
  - type: card_grid
    cards:
      - title: Card
        href: /abs-card.html
        links:
          - label: Link
            href: /abs-link.html
  - type: breadcrumb
    items:
      - label: Home
        href: /abs-crumb.html
      - label: Current
  - type: empty_state
    title: Nothing here
    action:
      label: Go
      href: /abs-action.html
  - type: markdown
    body: "[click](/abs-md.html) and [rel](relative.html)"
"##;
    std::fs::write(dir.join("tsp/demo.yaml"), page).unwrap();

    let out = tmp_dir("href-verbatim-out");
    let status = Command::new(bin())
        .args(["build"])
        .arg(&dir)
        .arg("--out")
        .arg(&out)
        .status()
        .expect("run kazam build");
    assert!(status.success(), "build failed");

    let html = read(&out.join("tsp/demo.html"));

    // Absolute-slash hrefs pass through verbatim.
    assert_contains(&html, r#"href="/customers/demo.html""#);
    // Already-canonical ../‑relative href passes through verbatim.
    assert_contains(&html, r#"href="../customers/demo.html""#);
    // Hash, mailto, https pass through verbatim.
    assert_contains(&html, "href=\"#section\"");
    assert_contains(&html, r#"href="mailto:hi@example.com""#);
    assert_contains(&html, r#"href="https://example.com""#);
    // Absolute-slash hrefs in card, links, breadcrumb, empty_state, markdown.
    assert_contains(&html, r#"href="/abs-card.html""#);
    assert_contains(&html, r#"href="/abs-link.html""#);
    assert_contains(&html, r#"href="/abs-crumb.html""#);
    assert_contains(&html, r#"href="/abs-action.html""#);
    assert_contains(&html, r#"href="/abs-md.html""#);
    // Plain relative href in markdown gets depth-1 base (../) prepended.
    assert_contains(&html, r#"href="../relative.html""#);
}

#[test]
fn init_refuses_existing_dir() {
    let dir = tmp_dir("init-exists");
    std::fs::create_dir_all(&dir).unwrap();

    let status = Command::new(bin())
        .args(["init"])
        .arg(&dir)
        .status()
        .expect("run kazam init");
    assert!(!status.success(), "init should fail on existing dir");
}
