mod components;
mod scripts;
mod shells;

use crate::types::{Component, Page, Shell, SiteConfig};

pub fn render_source_view(
    original: &Page,
    config: &SiteConfig,
    yaml_content: &str,
    base: &str,
    source_filename: &str,
) -> String {
    let html_href = source_filename
        .strip_suffix(".yaml")
        .map(|s| format!("{}.html", s))
        .unwrap_or_else(|| source_filename.to_string());

    let synthetic = Page {
        title: format!("{} — Source", original.title),
        shell: Shell::Standard,
        eyebrow: original.eyebrow.clone(),
        subtitle: Some(source_filename.to_string()),
        components: Some(vec![
            Component::Markdown {
                body: format!("[← Back to rendered page]({})", html_href),
            },
            Component::Code {
                language: Some("yaml".to_string()),
                code: yaml_content.to_string(),
            },
        ]),
        slides: None,
        unlisted: true,
    };

    render_page(&synthetic, config, base, "")
}

pub fn render_page(page: &Page, config: &SiteConfig, base: &str, source_href: &str) -> String {
    let mut rendered = Rendered::default();

    match page.shell {
        Shell::Deck => {
            if let Some(slides) = &page.slides {
                shells::deck::render(page, config, slides, &mut rendered);
            }
        }
        _ => {
            if let Some(comps) = &page.components {
                for c in comps {
                    rendered.extend(components::render(c));
                }
            }
        }
    }

    match page.shell {
        Shell::Standard => shells::standard::wrap(page, config, rendered, base, source_href),
        Shell::Document => shells::document::wrap(page, config, rendered, base, source_href),
        Shell::Deck => shells::deck::wrap(page, config, rendered, base, source_href),
    }
}

pub(super) fn resolve_href(href: &str, base: &str) -> String {
    if href.starts_with("http://")
        || href.starts_with("https://")
        || href.starts_with('/')
        || href.starts_with('#')
        || href.starts_with("mailto:")
        || href.starts_with("tel:")
    {
        return href.to_string();
    }
    format!("{}{}", base, href)
}

// ── Rendered: HTML + required JS fragment names ──

#[derive(Default)]
pub(super) struct Rendered {
    pub html: String,
    pub scripts: Vec<&'static str>,
}

impl Rendered {
    pub fn new(html: String) -> Self {
        Self {
            html,
            scripts: Vec::new(),
        }
    }
    pub fn with_script(mut self, name: &'static str) -> Self {
        self.scripts.push(name);
        self
    }
    pub fn extend(&mut self, other: Rendered) {
        self.html.push_str(&other.html);
        self.scripts.extend(other.scripts);
    }
}

pub(super) fn esc(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

pub(super) fn collect_scripts(names: &[&'static str]) -> String {
    let mut seen = std::collections::HashSet::new();
    let mut out = String::new();
    for name in names {
        if seen.insert(*name) {
            if let Some(src) = scripts::get(name) {
                out.push_str("<script>");
                out.push_str(src);
                out.push_str("</script>\n");
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn esc_escapes_html_special_chars() {
        assert_eq!(esc("hello"), "hello");
        assert_eq!(esc("<script>"), "&lt;script&gt;");
        assert_eq!(esc("\"q\""), "&quot;q&quot;");
        assert_eq!(esc("a & b"), "a &amp; b");
    }

    #[test]
    fn resolve_href_passes_through_absolute_urls() {
        assert_eq!(
            resolve_href("https://example.com", "../"),
            "https://example.com"
        );
        assert_eq!(
            resolve_href("http://example.com", "../"),
            "http://example.com"
        );
    }

    #[test]
    fn resolve_href_passes_through_root_relative_and_fragments() {
        assert_eq!(resolve_href("/foo", "../"), "/foo");
        assert_eq!(resolve_href("#anchor", "../"), "#anchor");
        assert_eq!(
            resolve_href("mailto:hi@example.com", "../"),
            "mailto:hi@example.com"
        );
        assert_eq!(resolve_href("tel:+15551234", "../"), "tel:+15551234");
    }

    #[test]
    fn resolve_href_prepends_base_for_relative() {
        assert_eq!(resolve_href("index.html", ""), "index.html");
        assert_eq!(resolve_href("index.html", "../"), "../index.html");
        assert_eq!(
            resolve_href("sub/page.html", "../../"),
            "../../sub/page.html"
        );
    }
}
