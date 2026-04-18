use anyhow::{Context, Result};
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

use crate::llms::{self, PageEntry};
use crate::minify;
use crate::render;
use crate::types::{Page, SiteConfig};

pub fn run(dir: &Path, out: &Path, release: bool) -> Result<()> {
    let config = load_config(dir)?;
    fs::create_dir_all(out)?;

    // Canonicalize the output dir so we can reliably skip walking into it
    // when it lives inside the source dir (e.g. docs/_site under docs/).
    let out_canonical = out.canonicalize().unwrap_or_else(|_| out.to_path_buf());

    let mut pages = 0;
    let mut assets = 0;
    let mut entries: Vec<PageEntry> = Vec::new();

    for entry in WalkDir::new(dir)
        .follow_links(true)
        .into_iter()
        .filter_entry(|e| {
            // Skip the output directory (e.g. _site/ nested in source dir)
            if e.path()
                .canonicalize()
                .map(|p| p.starts_with(&out_canonical))
                .unwrap_or(false)
            {
                return false;
            }
            // Skip hidden entries (.git, .DS_Store, .vscode, etc.) at any depth
            // except the source dir itself, which is often passed as "." and
            // would be filtered by a naive starts-with check.
            if e.depth() > 0 {
                if let Some(name) = e.file_name().to_str() {
                    if name.starts_with('.') {
                        return false;
                    }
                }
            }
            true
        })
    {
        let entry = entry?;
        let path = entry.path();
        if !entry.file_type().is_file() {
            continue;
        }

        let fname = path.file_name().unwrap_or_default();
        if fname == "kazam.yaml" {
            continue;
        }

        let rel = path.strip_prefix(dir)?;
        let is_yaml = path.extension().map(|e| e == "yaml").unwrap_or(false);

        if is_yaml {
            let content =
                fs::read_to_string(path).with_context(|| format!("reading {:?}", path))?;
            let page: Page =
                serde_yaml::from_str(&content).with_context(|| format!("parsing {:?}", path))?;

            let base = base_path_for(rel);
            let source_filename = rel
                .file_name()
                .map(|f| f.to_string_lossy().into_owned())
                .unwrap_or_default();
            let source_stem = rel
                .file_stem()
                .map(|f| f.to_string_lossy().into_owned())
                .unwrap_or_default();

            // The "View source" pill + rendered source-view page are opt-in
            // via `view_source: true` in kazam.yaml. Most sites don't need it;
            // docs/examples sites do.
            let source_view_href = if config.view_source {
                format!("{}.source.html", source_stem)
            } else {
                String::new()
            };

            let mut html = render::render_page(&page, &config, &base, &source_view_href);
            if release {
                html = minify::minify_html(&html);
            }

            let out_path = out.join(rel).with_extension("html");
            if let Some(parent) = out_path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(&out_path, html)?;

            if config.view_source {
                let mut source_view =
                    render::render_source_view(&page, &config, &content, &base, &source_filename);
                if release {
                    source_view = minify::minify_html(&source_view);
                }
                let source_view_path =
                    out_path.with_file_name(format!("{}.source.html", source_stem));
                fs::write(&source_view_path, source_view)?;
            }

            // Always copy the raw YAML — llms.txt points at it and it's
            // useful for `curl` / programmatic access even without view_source.
            let yaml_out = out.join(rel);
            fs::copy(path, &yaml_out)?;

            // Collect metadata for llms.txt (unless marked unlisted)
            if !page.unlisted {
                let html_path = rel.with_extension("html").to_string_lossy().to_string();
                let yaml_path = rel.to_string_lossy().to_string();
                entries.push(PageEntry {
                    title: page.title.clone(),
                    subtitle: page.subtitle.clone(),
                    html_path,
                    yaml_path,
                });
            }

            println!("  {}", out_path.display());
            pages += 1;
        } else {
            // Static asset — copy verbatim
            let out_path = out.join(rel);
            if let Some(parent) = out_path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(path, &out_path)?;
            assets += 1;
        }
    }

    // Emit llms.txt
    if !entries.is_empty() {
        llms::write(out, &config, &entries)?;
    }

    if assets > 0 {
        println!(
            "\n✓ {} page(s), {} asset(s) → {}{}",
            pages,
            assets,
            out.display(),
            if release { " (minified)" } else { "" }
        );
    } else {
        println!(
            "\n✓ {} page(s) → {}{}",
            pages,
            out.display(),
            if release { " (minified)" } else { "" }
        );
    }
    Ok(())
}

fn base_path_for(rel: &Path) -> String {
    let depth = rel.parent().map(|p| p.components().count()).unwrap_or(0);
    "../".repeat(depth)
}

fn load_config(dir: &Path) -> Result<SiteConfig> {
    let config_path = dir.join("kazam.yaml");
    if config_path.exists() {
        let content = fs::read_to_string(&config_path)?;
        serde_yaml::from_str(&content).context("parsing kazam.yaml")
    } else {
        Ok(SiteConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_path_at_root_is_empty() {
        assert_eq!(base_path_for(Path::new("index.yaml")), "");
    }

    #[test]
    fn base_path_one_level_deep() {
        assert_eq!(base_path_for(Path::new("customers/acme.yaml")), "../");
    }

    #[test]
    fn base_path_two_levels_deep() {
        assert_eq!(base_path_for(Path::new("a/b/c.yaml")), "../../");
    }
}
