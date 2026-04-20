//! Integration tests — invoke the kazam binary end-to-end.

use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

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
fn wish_list_succeeds() {
    let output = Command::new(bin())
        .args(["wish", "list"])
        .output()
        .expect("run kazam wish list");
    assert!(output.status.success(), "wish list failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_contains(&stdout, "deck");
    assert_contains(&stdout, "QBR");
}

#[test]
fn wish_deck_stdout_prints_markdown() {
    let output = Command::new(bin())
        .args(["wish", "deck", "--stdout"])
        .output()
        .expect("run kazam wish deck --stdout");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    // Markdown spec has these structural headers
    assert_contains(&stdout, "# kazam wish: deck");
    assert_contains(&stdout, "Ask the user these questions");
    assert_contains(&stdout, "shell: deck");
}

#[test]
fn wish_deck_populates_and_builds() {
    let dir = tmp_dir("wish-deck");
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("kazam.yaml"), "name: WishDeck\ntheme: dark\n").unwrap();

    // Pipe answers through stdin. Multi-line fields terminate on a blank line.
    let answers = "Q1 recap\nLeadership team\nQ1 2026\nShipped v0.4.0\nRebuilt personal site\n\nBehind on arc\n\nGreen-light the launch\n";

    let mut child = Command::new(bin())
        .args(["wish", "deck"])
        .current_dir(&dir)
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .expect("run kazam wish deck");
    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(answers.as_bytes())
        .unwrap();
    let status = child.wait().expect("wait wish deck");
    assert!(status.success(), "wish deck failed");

    let deck_yaml = dir.join("deck.yaml");
    assert!(deck_yaml.exists(), "deck.yaml not written");
    let yaml = read(&deck_yaml);
    assert_contains(&yaml, "shell: deck");
    assert_contains(&yaml, "Q1 recap");
    assert_contains(&yaml, "Shipped v0.4.0");
    assert_contains(&yaml, "Green-light the launch");

    // And kazam build should accept it cleanly.
    let out = dir.join("_site");
    let status = Command::new(bin())
        .args(["build"])
        .arg(&dir)
        .arg("--out")
        .arg(&out)
        .status()
        .expect("build the populated deck");
    assert!(status.success(), "kazam build failed on generated deck");
    assert!(out.join("deck.html").exists());
    let html = read(&out.join("deck.html"));
    assert_contains(&html, "Q1 recap");
    assert_contains(&html, "Shipped v0.4.0");
}

#[test]
fn wish_deck_refuses_existing_output() {
    let dir = tmp_dir("wish-deck-exists");
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("kazam.yaml"), "name: X\ntheme: dark\n").unwrap();
    std::fs::write(dir.join("deck.yaml"), "placeholder").unwrap();

    let output = Command::new(bin())
        .args(["wish", "deck"])
        .current_dir(&dir)
        .output()
        .expect("run kazam wish deck");
    assert!(
        !output.status.success(),
        "wish deck should refuse to overwrite existing deck.yaml"
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
