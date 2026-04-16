mod components;
mod scripts;
mod shells;

use crate::types::{Page, Shell, SiteConfig};

pub fn render_page(page: &Page, config: &SiteConfig) -> String {
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
        Shell::Standard => shells::standard::wrap(page, config, rendered),
        Shell::Document => shells::document::wrap(page, config, rendered),
        Shell::Deck => shells::deck::wrap(page, config, rendered),
    }
}

// ── Rendered: HTML + required JS fragment names ──

#[derive(Default)]
pub(super) struct Rendered {
    pub html: String,
    pub scripts: Vec<&'static str>,
}

impl Rendered {
    pub fn new(html: String) -> Self { Self { html, scripts: Vec::new() } }
    pub fn with_script(mut self, name: &'static str) -> Self {
        self.scripts.push(name); self
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
