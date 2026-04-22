mod charts;
mod components;
mod scripts;
mod shells;
mod slug;

use crate::types::{Component, Page, Shell, SiteConfig};

pub fn render_source_view(
    original: &Page,
    config: &SiteConfig,
    yaml_content: &str,
    base: &str,
    source_filename: &str,
    rel_path: &str,
    release: bool,
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
        texture: None,
        glow: None,
        print_flow: None,
        freshness: None,
    };

    render_page(&synthetic, config, base, "", rel_path, release)
}

pub fn render_page(
    page: &Page,
    config: &SiteConfig,
    base: &str,
    source_href: &str,
    rel_path: &str,
    release: bool,
) -> String {
    // Clear the per-page anchor-id dedup map so slug collisions don't leak
    // between pages in a single build.
    slug::reset();

    let mut rendered = Rendered::default();

    // Inject the stale-review banner at the top of the page body when the
    // freshness metadata is expired. Zero runtime JS — this is evaluated
    // at build time against `KAZAM_TODAY` or the system clock.
    if let Some(banner) = freshness_banner(page, base) {
        rendered.html.push_str(&banner);
    }

    match page.shell {
        Shell::Deck => {
            if let Some(slides) = &page.slides {
                shells::deck::render(page, config, slides, base, &mut rendered);
            }
        }
        _ => {
            if let Some(comps) = &page.components {
                for c in comps {
                    rendered.extend(components::render(c, base));
                }
            }
        }
    }

    match page.shell {
        Shell::Standard => {
            shells::standard::wrap(page, config, rendered, base, source_href, rel_path, release)
        }
        Shell::Document => {
            shells::document::wrap(page, config, rendered, base, source_href, rel_path, release)
        }
        Shell::Deck => {
            shells::deck::wrap(page, config, rendered, base, source_href, rel_path, release)
        }
    }
}

/// Build the freshness banner HTML for a page, or return `None` when the
/// page is fresh (or has no freshness metadata). The banner reuses the
/// existing callout variants so color treatment stays consistent with the
/// rest of the theme: yellow (`c-callout-warn`) for "due soon" — within
/// 7 days of the review deadline — and red (`c-callout-danger`) for
/// overdue pages. A `c-freshness-banner` class is added for future
/// per-element styling.
fn freshness_banner(page: &Page, base: &str) -> Option<String> {
    use crate::freshness::FreshnessStatus;

    let freshness = page.freshness.as_ref()?;
    let today = crate::freshness::today_iso();
    let info = crate::freshness::info_for(Some(freshness), &today)?;

    let (variant_class, title, headline) = match info.status() {
        FreshnessStatus::Fresh => return None,
        FreshnessStatus::DueSoon { days_until_due } => (
            "c-callout-warn",
            "Review due soon",
            if days_until_due == 0 {
                "Review is due today.".to_string()
            } else {
                format!(
                    "Review is due in <strong>{} {}</strong>.",
                    days_until_due,
                    if days_until_due == 1 { "day" } else { "days" }
                )
            },
        ),
        FreshnessStatus::Overdue { days_overdue } => (
            "c-callout-danger",
            "Review overdue",
            format!(
                "Review is <strong>{} {} overdue</strong>.",
                days_overdue,
                if days_overdue == 1 { "day" } else { "days" }
            ),
        ),
    };

    let updated_iso = freshness.updated.as_deref().unwrap_or("");
    let elapsed = info.days_since_update().unwrap_or(0);
    let cadence = freshness
        .review_every
        .as_deref()
        .unwrap_or("(no cadence set)");

    let mut body = format!(
        r#"{headline} Last updated <strong>{updated}</strong> ({elapsed} {day_word} ago). Review cadence: <strong>every {cadence}</strong>. Site last built: {today}."#,
        headline = headline,
        updated = esc(&human_date(updated_iso)),
        elapsed = elapsed,
        day_word = if elapsed == 1 { "day" } else { "days" },
        cadence = esc(cadence),
        today = esc(&human_date(&today)),
    );
    if let Some(owner) = freshness.owner.as_deref() {
        body.push_str(&format!(r#" Owner: <strong>{}</strong>."#, esc(owner)));
    }

    let mut h = format!(
        r#"<div class="c-callout {variant_class} c-freshness-banner"><div class="c-callout-title">{title}</div>"#,
        variant_class = variant_class,
        title = esc(title),
    );
    h.push_str(&format!(r#"<div class="c-callout-body">{body}</div>"#));

    if let Some(sources) = freshness.sources_of_truth.as_ref() {
        if !sources.is_empty() {
            h.push_str(
                r#"<div class="c-freshness-sources"><span class="c-freshness-sources-label">Sources of truth:</span><ul>"#,
            );
            for src in sources {
                let href = resolve_href(src.href(), base);
                h.push_str(&format!(
                    r#"<li><a href="{}">{}</a></li>"#,
                    esc(&href),
                    esc(src.label()),
                ));
            }
            h.push_str("</ul></div>");
        }
    }
    h.push_str("</div>");
    Some(h)
}

/// Format an ISO `YYYY-MM-DD` date into a short human-readable form like
/// `Jan 15, 2026`. Falls back to the raw input when parsing fails.
fn human_date(iso: &str) -> String {
    const MONTHS: [&str; 12] = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];
    let mut parts = iso.split('-');
    let y = parts.next().and_then(|p| p.parse::<i32>().ok());
    let m = parts.next().and_then(|p| p.parse::<u32>().ok());
    let d = parts.next().and_then(|p| p.parse::<u32>().ok());
    match (y, m, d) {
        (Some(y), Some(m), Some(d)) if (1..=12).contains(&m) && (1..=31).contains(&d) => {
            format!("{} {}, {}", MONTHS[(m - 1) as usize], d, y)
        }
        _ => iso.to_string(),
    }
}

pub(super) fn resolve_href(href: &str, base: &str) -> String {
    if href.starts_with("http://")
        || href.starts_with("https://")
        || href.starts_with('/')
        || href.starts_with('#')
        || href.starts_with("mailto:")
        || href.starts_with("tel:")
        || href.starts_with("../")
        || href.starts_with("./")
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
    fn resolve_href_passes_through_dot_relative_hrefs() {
        assert_eq!(
            resolve_href("../customers/demo.html", "../"),
            "../customers/demo.html"
        );
        assert_eq!(resolve_href("./sibling.html", "../"), "./sibling.html");
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
