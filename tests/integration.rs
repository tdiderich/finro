//! Integration tests — invoke the finro binary end-to-end.

use std::process::Command;
use std::path::{Path, PathBuf};

fn bin() -> PathBuf {
    // cargo sets CARGO_BIN_EXE_<name> env var for the test runner
    PathBuf::from(env!("CARGO_BIN_EXE_finro"))
}

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn tmp_dir(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("finro-test-{}", name));
    let _ = std::fs::remove_dir_all(&dir);
    dir
}

fn read(p: &Path) -> String {
    std::fs::read_to_string(p).expect("read file")
}

fn assert_contains(haystack: &str, needle: &str) {
    assert!(
        haystack.contains(needle),
        "expected to find {:?}", needle
    );
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
        .expect("run finro build");
    assert!(status.success(), "finro build failed");

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
        .expect("run finro build");
    assert!(status.success());

    // llms.txt should exist and list known pages
    let llms = read(&out.join("llms.txt"));
    assert_contains(&llms, "# finro");
    assert_contains(&llms, "Content components");
    assert_contains(&llms, "Q1 Product Strategy Review");

    // Each page has a View source link
    let index = read(&out.join("index.html"));
    assert_contains(&index, r#"class="view-source""#);

    // Source YAMLs copied next to rendered HTML
    assert!(out.join("components/content.yaml").exists());
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
        .expect("run finro build --release");
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
        .expect("run finro init");
    assert!(status.success());

    // Scaffold has expected files
    assert!(dir.join("finro.yaml").exists());
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
fn init_refuses_existing_dir() {
    let dir = tmp_dir("init-exists");
    std::fs::create_dir_all(&dir).unwrap();

    let status = Command::new(bin())
        .args(["init"])
        .arg(&dir)
        .status()
        .expect("run finro init");
    assert!(!status.success(), "init should fail on existing dir");
}
