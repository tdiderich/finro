use anyhow::{Context, Result};
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

use crate::render;
use crate::types::{Page, SiteConfig};

pub fn run(dir: &Path, out: &Path) -> Result<()> {
    let config = load_config(dir)?;
    fs::create_dir_all(out)?;

    // Canonicalize the output dir so we can reliably skip walking into it
    // when it lives inside the source dir (e.g. docs/_site under docs/).
    let out_canonical = out.canonicalize().unwrap_or_else(|_| out.to_path_buf());

    let mut pages = 0;
    let mut assets = 0;

    for entry in WalkDir::new(dir).follow_links(true).into_iter().filter_entry(|e| {
        !e.path().canonicalize()
            .map(|p| p.starts_with(&out_canonical))
            .unwrap_or(false)
    }) {
        let entry = entry?;
        let path = entry.path();
        if !entry.file_type().is_file() { continue; }

        let fname = path.file_name().unwrap_or_default();
        if fname == "finro.yaml" { continue; }

        let rel = path.strip_prefix(dir)?;
        let is_yaml = path.extension().map(|e| e == "yaml").unwrap_or(false);

        if is_yaml {
            let content = fs::read_to_string(path)
                .with_context(|| format!("reading {:?}", path))?;
            let page: Page = serde_yaml::from_str(&content)
                .with_context(|| format!("parsing {:?}", path))?;

            let base = base_path_for(rel);
            let html = render::render_page(&page, &config, &base);

            let out_path = out.join(rel).with_extension("html");
            if let Some(parent) = out_path.parent() { fs::create_dir_all(parent)?; }
            fs::write(&out_path, html)?;
            println!("  {}", out_path.display());
            pages += 1;
        } else {
            // Static asset — copy verbatim
            let out_path = out.join(rel);
            if let Some(parent) = out_path.parent() { fs::create_dir_all(parent)?; }
            fs::copy(path, &out_path)?;
            assets += 1;
        }
    }

    if assets > 0 {
        println!("\n✓ {} page(s), {} asset(s) → {}", pages, assets, out.display());
    } else {
        println!("\n✓ {} page(s) → {}", pages, out.display());
    }
    Ok(())
}

fn base_path_for(rel: &Path) -> String {
    let depth = rel.parent()
        .map(|p| p.components().count())
        .unwrap_or(0);
    "../".repeat(depth)
}

fn load_config(dir: &Path) -> Result<SiteConfig> {
    let config_path = dir.join("finro.yaml");
    if config_path.exists() {
        let content = fs::read_to_string(&config_path)?;
        serde_yaml::from_str(&content).context("parsing finro.yaml")
    } else {
        Ok(SiteConfig::default())
    }
}
