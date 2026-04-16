use anyhow::{Context, Result};
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

use crate::render;
use crate::types::{Page, SiteConfig};

pub fn run(dir: &Path, out: &Path) -> Result<()> {
    let config = load_config(dir)?;
    fs::create_dir_all(out)?;

    let mut count = 0;

    for entry in WalkDir::new(dir).follow_links(true) {
        let entry = entry?;
        let path = entry.path();

        if path.extension().map(|e| e == "yaml").unwrap_or(false) {
            let fname = path.file_name().unwrap_or_default();
            if fname == "pseudo.yaml" {
                continue;
            }

            let content = fs::read_to_string(path)
                .with_context(|| format!("reading {:?}", path))?;
            let page: Page = serde_yaml::from_str(&content)
                .with_context(|| format!("parsing {:?}", path))?;

            let rel = path.strip_prefix(dir)?;
            let base = base_path_for(rel);
            let html = render::render_page(&page, &config, &base);

            let out_path = out.join(rel).with_extension("html");
            if let Some(parent) = out_path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(&out_path, html)?;

            println!("  {}", out_path.display());
            count += 1;
        }
    }

    println!("\n✓ {} page(s) → {}", count, out.display());
    Ok(())
}

fn base_path_for(rel: &Path) -> String {
    let depth = rel.parent()
        .map(|p| p.components().count())
        .unwrap_or(0);
    "../".repeat(depth)
}

fn load_config(dir: &Path) -> Result<SiteConfig> {
    let config_path = dir.join("pseudo.yaml");
    if config_path.exists() {
        let content = fs::read_to_string(&config_path)?;
        serde_yaml::from_str(&content).context("parsing pseudo.yaml")
    } else {
        Ok(SiteConfig::default())
    }
}
