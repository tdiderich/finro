//! Emits `_site/llms.txt` following the llmstxt.org convention.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use crate::types::SiteConfig;

pub struct PageEntry {
    pub title: String,
    pub subtitle: Option<String>,
    /// Relative HTML path, e.g. "components/content.html"
    pub html_path: String,
    /// Relative YAML source path, e.g. "components/content.yaml"
    pub yaml_path: String,
}

pub fn write(out: &Path, config: &SiteConfig, pages: &[PageEntry]) -> std::io::Result<()> {
    let mut body = String::new();
    body.push_str(&format!("# {}\n\n", config.name));

    // Group by top-level directory
    let mut groups: BTreeMap<String, Vec<&PageEntry>> = BTreeMap::new();
    for p in pages {
        let group = top_dir(&p.html_path);
        groups.entry(group).or_default().push(p);
    }

    // "Root" group first (alphabetically "" comes first anyway with BTreeMap)
    for (group, entries) in &groups {
        let heading = if group.is_empty() {
            "Pages".to_string()
        } else {
            title_case(group)
        };
        body.push_str(&format!("## {}\n\n", heading));
        let mut sorted = entries.clone();
        sorted.sort_by(|a, b| a.html_path.cmp(&b.html_path));
        for e in sorted {
            body.push_str(&format!(
                "- [{title}](./{html}) ([source](./{yaml}))",
                title = e.title,
                html = e.html_path,
                yaml = e.yaml_path
            ));
            if let Some(sub) = &e.subtitle {
                body.push_str(&format!(": {}", sub));
            }
            body.push('\n');
        }
        body.push('\n');
    }

    let path: PathBuf = out.join("llms.txt");
    std::fs::write(path, body)
}

fn top_dir(path: &str) -> String {
    match path.find('/') {
        Some(i) => path[..i].to_string(),
        None => String::new(),
    }
}

fn title_case(s: &str) -> String {
    s.split('-')
        .map(|w| {
            let mut chars = w.chars();
            match chars.next() {
                Some(c) => c.to_ascii_uppercase().to_string() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}
