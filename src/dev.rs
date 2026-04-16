use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

use anyhow::{Context, Result};
use notify::{Event, RecursiveMode, Watcher};
use tiny_http::{Header, Method, Response, Server};

use crate::build;

pub fn run(dir: &Path, out: &Path, port: u16) -> Result<()> {
    build::run(dir, out)?;

    let version = Arc::new(AtomicU64::new(1));

    // ── Watcher thread ────────────────────────────
    let dir_clone = dir.to_path_buf();
    let out_clone = out.to_path_buf();
    let ver_clone = version.clone();
    thread::spawn(move || watch_loop(dir_clone, out_clone, ver_clone));

    // ── HTTP server ───────────────────────────────
    let addr = format!("0.0.0.0:{port}");
    let server = Server::http(&addr)
        .map_err(|e| anyhow::anyhow!("failed to bind {addr}: {e}"))?;

    println!("\n  ➜ http://localhost:{port}");
    println!("  watching {}\n", dir.display());

    for req in server.incoming_requests() {
        let out_path = out.to_path_buf();
        let ver = version.clone();
        thread::spawn(move || {
            if let Err(e) = handle(req, &out_path, &ver) {
                eprintln!("  request error: {e}");
            }
        });
    }
    Ok(())
}

// ── File watcher ─────────────────────────────────

fn watch_loop(dir: PathBuf, out: PathBuf, version: Arc<AtomicU64>) {
    let (tx, rx) = std::sync::mpsc::channel::<notify::Result<Event>>();
    let mut watcher = match notify::recommended_watcher(tx) {
        Ok(w) => w,
        Err(e) => { eprintln!("watcher init failed: {e}"); return; }
    };
    if let Err(e) = watcher.watch(&dir, RecursiveMode::Recursive) {
        eprintln!("watch failed: {e}");
        return;
    }

    let mut last_build = Instant::now() - Duration::from_secs(10);

    for event in rx {
        let Ok(event) = event else { continue };
        let relevant = event.paths.iter().any(|p| {
            // Ignore output dir and its contents
            if p.starts_with(&out) { return false }
            p.extension().map(|e| e == "yaml").unwrap_or(false)
                || p.file_name().map(|n| n == "pseudo.yaml").unwrap_or(false)
        });
        if !relevant { continue }
        if last_build.elapsed() < Duration::from_millis(150) { continue }
        last_build = Instant::now();

        print!("  rebuild…");
        match build::run(&dir, &out) {
            Ok(_) => {
                version.fetch_add(1, Ordering::SeqCst);
                println!(" ✓");
            }
            Err(e) => {
                println!(" ✗\n    {e:#}");
            }
        }
    }
}

// ── Request handler ──────────────────────────────

fn handle(req: tiny_http::Request, root: &Path, version: &AtomicU64) -> Result<()> {
    if req.method() != &Method::Get {
        return req.respond(Response::from_string("method not allowed").with_status_code(405))
            .context("respond");
    }

    let url = req.url().split('?').next().unwrap_or("/");

    // Live-reload version endpoint
    if url == "/__pseudo_version__" {
        let v = version.load(Ordering::SeqCst).to_string();
        let resp = Response::from_string(v)
            .with_header(hdr("Content-Type", "text/plain"))
            .with_header(hdr("Cache-Control", "no-store"));
        return req.respond(resp).context("respond");
    }

    let rel = url.trim_start_matches('/');
    let rel = if rel.is_empty() { "index.html" } else { rel };

    // Prevent escape
    if rel.contains("..") {
        return req.respond(Response::from_string("bad path").with_status_code(400))
            .context("respond");
    }

    let mut path = root.join(rel);
    if path.is_dir() {
        path = path.join("index.html");
    }

    match std::fs::read(&path) {
        Ok(data) => {
            let ct = content_type(&path);
            let resp = Response::from_data(data)
                .with_header(hdr("Content-Type", ct))
                .with_header(hdr("Cache-Control", "no-store"));
            req.respond(resp).context("respond")
        }
        Err(_) => {
            let body = format!("404 — not found: {rel}");
            let resp = Response::from_string(body).with_status_code(404);
            req.respond(resp).context("respond")
        }
    }
}

fn hdr(name: &str, value: &str) -> Header {
    Header::from_bytes(name.as_bytes(), value.as_bytes()).unwrap()
}

fn content_type(path: &Path) -> &'static str {
    match path.extension().and_then(|e| e.to_str()) {
        Some("html") | Some("htm") => "text/html; charset=utf-8",
        Some("css") => "text/css; charset=utf-8",
        Some("js") => "application/javascript; charset=utf-8",
        Some("json") => "application/json; charset=utf-8",
        Some("svg") => "image/svg+xml",
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("gif") => "image/gif",
        Some("webp") => "image/webp",
        Some("woff2") => "font/woff2",
        Some("woff") => "font/woff",
        _ => "application/octet-stream",
    }
}
