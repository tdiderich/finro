use crate::theme;
use crate::types::{Page, Shell, SiteConfig, Slide};
use super::{components, collect_scripts, esc, resolve_href, Rendered};

fn head(page: &Page, config: &SiteConfig, base: &str) -> String {
    let favicon = config.favicon.as_ref().map(|f| f.render(base)).unwrap_or_default();
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
        css = theme::render_css(&config.resolved_theme()),
    )
}

fn nav_html(config: &SiteConfig, base: &str) -> (String, bool) {
    let Some(links) = &config.nav else { return (String::new(), false) };
    if links.is_empty() { return (String::new(), false) }
    let mut out = String::from("<nav>");
    for link in links {
        out.push_str(&format!(
            r#"<a href="{}" class="nav-link">{}</a>"#,
            esc(&resolve_href(&link.href, base)), esc(&link.label)
        ));
    }
    out.push_str("</nav>");
    (out, true)
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
    page.subtitle.as_deref()
        .filter(|s| !s.is_empty())
        .map(|s| format!(r#"<span class="site-bar-subtitle">{}</span>"#, esc(s)))
        .unwrap_or_default()
}

// ── Standard shell ────────────────────────────────

pub mod standard {
    use super::*;

    pub fn wrap(page: &Page, config: &SiteConfig, body: Rendered, base: &str, source_href: &str) -> String {
        let (nav, has_nav) = nav_html(config, base);
        let mut right = subtitle_span(page);
        right.push_str(&nav);
        let bar = site_bar(page, config, base, &right);

        let mut scripts = body.scripts.clone();
        if has_nav { scripts.push("nav"); }
        scripts.push("reload");
        let view_src = view_source_html(source_href);

        format!(
            r#"<!DOCTYPE html>
<html lang="en">
{head}
<body class="{cls}">
{bar}<main class="container main-content">
{body}
</main>
{view_src}
{scripts}
</body>
</html>"#,
            head = head(page, config, base),
            cls = Shell::Standard.class(),
            bar = bar,
            body = body.html,
            view_src = view_src,
            scripts = collect_scripts(&scripts),
        )
    }
}

// ── Document shell ────────────────────────────────

pub mod document {
    use super::*;

    pub fn wrap(page: &Page, config: &SiteConfig, body: Rendered, base: &str, source_href: &str) -> String {
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
        out.html.push_str(r#"<div class="deck-viewport"><div class="deck-track">"#);
        for slide in slides {
            out.html.push_str(&format!(
                r#"<div class="deck-slide" data-label="{}"><div class="deck-inner"><div class="deck-label">{}</div>"#,
                esc(&slide.label), esc(&slide.label)
            ));
            for c in &slide.components {
                out.extend(components::render(c));
            }
            out.html.push_str("</div></div>");
        }
        out.html.push_str("</div></div>");
        out.scripts.push("deck");
    }

    pub fn wrap(page: &Page, config: &SiteConfig, body: Rendered, base: &str, _source_href: &str) -> String {
        let mut right = subtitle_span(page);
        right.push_str(r#"<button class="site-bar-print-btn" onclick="window.print()">Download PDF</button>"#);
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
    if source_href.is_empty() { return String::new(); }
    format!(
        r##"<a class="view-source" href="{src}" title="View raw YAML source">
  <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><polyline points="16 18 22 12 16 6"/><polyline points="8 6 2 12 8 18"/></svg>
  <span>View source</span>
</a>"##,
        src = esc(source_href)
    )
}
