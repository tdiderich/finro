use super::{collect_scripts, components, esc, resolve_href, Rendered};
use crate::theme;
use crate::types::{Page, Shell, SiteConfig, Slide};

fn head(page: &Page, config: &SiteConfig, base: &str) -> String {
    let theme = config.resolved_theme();
    let favicon = match config.favicon.as_ref() {
        Some(f) => f.render(base),
        None => default_favicon(&theme),
    };
    // Page-level texture/glow overrides beat the site-wide defaults. An
    // explicit `none` at the page level turns the effect off on that page.
    let texture = page.texture.unwrap_or(config.texture);
    let glow = page.glow.unwrap_or(config.glow);
    format!(
        r#"<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>{title} — {site}</title>
{favicon}
<style>{css}</style>
</head>"#,
        title = esc(&page.title),
        site = esc(&config.name),
        favicon = favicon,
        css = theme::render_css(&theme, texture, glow),
    )
}

/// When a site doesn't declare a `favicon:`, synthesize one from theme colors.
/// Produces the kazam genie-bottle mark as an inline data-URI SVG — accent on
/// bg. Stopper + narrow neck + bulbous body, sized for 32px and 16px alike.
fn default_favicon(theme: &theme::Theme) -> String {
    let svg = format!(
        r##"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 32 32'><rect width='32' height='32' rx='6' fill='{bg}'/><rect x='13' y='5' width='6' height='3' rx='1' fill='{accent}'/><path d='M 14 8 L 18 8 L 18 12 Q 23 13 23 19 Q 23 27 16 27 Q 9 27 9 19 Q 9 13 14 12 Z' fill='{accent}'/></svg>"##,
        bg = theme.bg,
        accent = theme.accent
    );
    let encoded = svg
        .replace('#', "%23")
        .replace('<', "%3C")
        .replace('>', "%3E")
        .replace(' ', "%20");
    format!(r#"<link rel="icon" type="image/svg+xml" href="data:image/svg+xml;utf8,{encoded}">"#)
}

/// Top-bar nav (horizontal). Parent entries with `children:` render as a
/// hover/focus-within dropdown; leaf entries render as a plain link. Returns
/// `(html, has_any_nav)` so the caller can decide whether to bundle the
/// nav-related JS.
fn nav_html(config: &SiteConfig, base: &str) -> (String, bool) {
    let Some(links) = &config.nav else {
        return (String::new(), false);
    };
    if links.is_empty() {
        return (String::new(), false);
    }
    let mut out = String::from("<nav>");
    for link in links {
        out.push_str(&render_nav_entry(link, base));
    }
    out.push_str("</nav>");
    (out, true)
}

fn render_nav_entry(link: &crate::types::NavLink, base: &str) -> String {
    match &link.children {
        Some(children) if !children.is_empty() => {
            let mut dd = String::from(r#"<div class="nav-dropdown">"#);
            for child in children {
                // Children render as plain links even if they themselves
                // have `children:` — we don't nest dropdowns beyond one
                // level, to keep the UX predictable.
                let href = child
                    .href
                    .as_deref()
                    .map(|h| resolve_href(h, base))
                    .unwrap_or_default();
                dd.push_str(&format!(
                    r#"<a href="{}" class="nav-link">{}</a>"#,
                    esc(&href),
                    esc(&child.label)
                ));
            }
            dd.push_str("</div>");
            // The outer `<button>` is focusable so keyboard users can open
            // the dropdown via Tab + Enter. `focus-within` on the parent
            // keeps the panel open while focus is inside.
            format!(
                r#"<div class="nav-link-group"><button type="button" class="nav-link nav-link-parent" aria-haspopup="true">{label}<span class="nav-chevron">▾</span></button>{dd}</div>"#,
                label = esc(&link.label),
                dd = dd,
            )
        }
        _ => {
            let href = link
                .href
                .as_deref()
                .map(|h| resolve_href(h, base))
                .unwrap_or_default();
            format!(
                r#"<a href="{}" class="nav-link">{}</a>"#,
                esc(&href),
                esc(&link.label)
            )
        }
    }
}

/// Sidebar nav (vertical, fixed to the left). Renders every `NavLink`. Parent
/// entries with `children:` become labeled sections; leaf entries at the top
/// level become standalone links. Only emitted when `nav_layout: sidebar`.
fn sidebar_html(config: &SiteConfig, base: &str) -> String {
    let Some(links) = &config.nav else {
        return String::new();
    };
    if links.is_empty() {
        return String::new();
    }
    let mut out = String::from(r#"<aside class="site-sidebar"><nav>"#);
    for link in links {
        match &link.children {
            Some(children) if !children.is_empty() => {
                out.push_str(&format!(
                    r#"<div class="sidebar-section"><div class="sidebar-section-label">{}</div>"#,
                    esc(&link.label)
                ));
                for child in children {
                    let href = child
                        .href
                        .as_deref()
                        .map(|h| resolve_href(h, base))
                        .unwrap_or_default();
                    out.push_str(&format!(
                        r#"<a href="{}" class="sidebar-link">{}</a>"#,
                        esc(&href),
                        esc(&child.label)
                    ));
                }
                out.push_str("</div>");
            }
            _ => {
                let href = link
                    .href
                    .as_deref()
                    .map(|h| resolve_href(h, base))
                    .unwrap_or_default();
                out.push_str(&format!(
                    r#"<a href="{}" class="sidebar-link sidebar-link-top">{}</a>"#,
                    esc(&href),
                    esc(&link.label)
                ));
            }
        }
    }
    out.push_str("</nav></aside>");
    out
}

fn site_bar(page: &Page, config: &SiteConfig, base: &str, right_html: &str) -> String {
    let home_href = resolve_href("index.html", base);
    let eyebrow_html = page.eyebrow.as_deref()
        .filter(|s| !s.is_empty())
        .map(|e| format!(
            r#" <span class="site-bar-divider">/</span> <span class="site-bar-eyebrow">{}</span>"#,
            esc(e)
        ))
        .unwrap_or_default();
    format!(
        r#"<div class="site-bar">
  <a class="site-bar-name" href="{home}">{site}</a>{eyebrow}
  <div class="site-bar-right">{right}</div>
</div>
"#,
        home = esc(&home_href),
        site = esc(&config.name),
        eyebrow = eyebrow_html,
        right = right_html,
    )
}

fn subtitle_span(page: &Page) -> String {
    page.subtitle
        .as_deref()
        .filter(|s| !s.is_empty())
        .map(|s| format!(r#"<span class="site-bar-subtitle">{}</span>"#, esc(s)))
        .unwrap_or_default()
}

// ── Standard shell ────────────────────────────────

pub mod standard {
    use super::*;

    pub fn wrap(
        page: &Page,
        config: &SiteConfig,
        body: Rendered,
        base: &str,
        source_href: &str,
    ) -> String {
        let is_sidebar = matches!(config.nav_layout, crate::types::NavLayout::Sidebar);
        // Sidebar layout moves the full nav (including nested children) into
        // a left-side <aside>; the top bar then only shows site name +
        // subtitle. Top layout keeps the existing inline nav in the bar.
        let (nav_in_bar, has_nav) = if is_sidebar {
            (String::new(), config.nav.as_ref().is_some_and(|n| !n.is_empty()))
        } else {
            nav_html(config, base)
        };
        let mut right = subtitle_span(page);
        right.push_str(&nav_in_bar);
        let bar = site_bar(page, config, base, &right);

        let sidebar = if is_sidebar {
            sidebar_html(config, base)
        } else {
            String::new()
        };

        let mut scripts = body.scripts.clone();
        if has_nav {
            scripts.push("nav");
        }
        scripts.push("reload");
        let view_src = view_source_html(source_href);

        let body_class = if is_sidebar {
            format!("{} nav-layout-sidebar", Shell::Standard.class())
        } else {
            Shell::Standard.class().to_string()
        };

        format!(
            r#"<!DOCTYPE html>
<html lang="en">
{head}
<body class="{cls}">
{bar}{sidebar}<main class="container main-content">
{body}
</main>
{view_src}
{scripts}
</body>
</html>"#,
            head = head(page, config, base),
            cls = body_class,
            bar = bar,
            sidebar = sidebar,
            body = body.html,
            view_src = view_src,
            scripts = collect_scripts(&scripts),
        )
    }
}

// ── Document shell ────────────────────────────────

pub mod document {
    use super::*;

    pub fn wrap(
        page: &Page,
        config: &SiteConfig,
        body: Rendered,
        base: &str,
        source_href: &str,
    ) -> String {
        let bar = site_bar(page, config, base, &subtitle_span(page));

        let mut scripts = body.scripts.clone();
        scripts.push("reload");
        let view_src = view_source_html(source_href);

        format!(
            r#"<!DOCTYPE html>
<html lang="en">
{head}
<body class="{cls}">
{bar}<div class="doc-root">
<article class="doc-card">
<div class="doc-body">
{body}
</div>
<footer class="doc-footer"></footer>
</article>
</div>
{view_src}
{scripts}
</body>
</html>"#,
            head = head(page, config, base),
            cls = Shell::Document.class(),
            bar = bar,
            body = body.html,
            view_src = view_src,
            scripts = collect_scripts(&scripts),
        )
    }
}

// ── Deck shell ────────────────────────────────────

pub mod deck {
    use super::*;

    pub fn render(_page: &Page, _config: &SiteConfig, slides: &[Slide], out: &mut Rendered) {
        out.html
            .push_str(r#"<div class="deck-viewport"><div class="deck-track">"#);
        for slide in slides {
            let (label_html, slide_cls) = if slide.hide_label {
                (String::new(), " deck-slide-cover")
            } else {
                (
                    format!(r#"<div class="deck-label">{}</div>"#, esc(&slide.label)),
                    "",
                )
            };
            out.html.push_str(&format!(
                r#"<div class="deck-slide{cls}" data-label="{label}"><div class="deck-inner">{label_html}"#,
                cls = slide_cls,
                label = esc(&slide.label),
                label_html = label_html,
            ));
            for c in &slide.components {
                out.extend(components::render(c));
            }
            out.html.push_str("</div></div>");
        }
        out.html.push_str("</div></div>");
        out.scripts.push("deck");
    }

    pub fn wrap(
        page: &Page,
        config: &SiteConfig,
        body: Rendered,
        base: &str,
        _source_href: &str,
    ) -> String {
        let mut right = subtitle_span(page);
        right.push_str(
            r#"<button class="site-bar-print-btn" onclick="window.print()">Download PDF</button>"#,
        );
        let bar = site_bar(page, config, base, &right);

        let mut scripts = body.scripts.clone();
        scripts.push("reload");

        format!(
            r#"<!DOCTYPE html>
<html lang="en">
{head}
<body class="{cls}">
<div class="deck-root">

{bar}
{body}

<div class="deck-nav">
  <button class="deck-arrow deck-prev" id="deck-prev"></button>
  <span class="deck-nav-label" id="deck-label"></span>
  <button class="deck-arrow deck-next" id="deck-next"></button>
</div>

</div>
{scripts}
</body>
</html>"#,
            head = head(page, config, base),
            cls = Shell::Deck.class(),
            bar = bar,
            body = body.html,
            scripts = collect_scripts(&scripts),
        )
    }
}

fn view_source_html(source_href: &str) -> String {
    if source_href.is_empty() {
        return String::new();
    }
    format!(
        r##"<a class="view-source" href="{src}" title="View raw YAML source">
  <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><polyline points="16 18 22 12 16 6"/><polyline points="8 6 2 12 8 18"/></svg>
  <span>View source</span>
</a>"##,
        src = esc(source_href)
    )
}
