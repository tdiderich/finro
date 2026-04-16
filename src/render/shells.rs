use crate::theme;
use crate::types::{Page, Shell, SiteConfig, Slide};
use super::{components, collect_scripts, esc, resolve_href, Rendered};

fn head(page: &Page, config: &SiteConfig) -> String {
    format!(
        r#"<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>{title} — {site}</title>
<style>{css}</style>
</head>"#,
        title = esc(&page.title),
        site = esc(&config.name),
        css = theme::CSS,
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

fn nav_back_html(page: &Page, base: &str) -> String {
    let Some(nb) = &page.nav_back else { return String::new() };
    format!(
        r#"<a href="{}" class="nav-back">{}</a>"#,
        esc(&resolve_href(&nb.href, base)), esc(&nb.label)
    )
}

// ── Standard shell ────────────────────────────────

pub mod standard {
    use super::*;

    pub fn wrap(page: &Page, config: &SiteConfig, body: Rendered, base: &str) -> String {
        let (nav, has_nav) = nav_html(config, base);
        let mut scripts = body.scripts.clone();
        if has_nav { scripts.push("nav"); }
        scripts.push("reload");

        format!(
            r#"<!DOCTYPE html>
<html lang="en">
{head}
<body class="{cls}">
<header class="site-header">
  <div class="container header-inner">
    <span class="site-name">{site}</span>
    {nav}
  </div>
</header>
<main class="container main-content">
{nav_back}
{body}
</main>
{scripts}
</body>
</html>"#,
            head = head(page, config),
            cls = Shell::Standard.class(),
            site = esc(&config.name),
            nav = nav,
            nav_back = nav_back_html(page, base),
            body = body.html,
            scripts = collect_scripts(&scripts),
        )
    }
}

// ── Document shell ────────────────────────────────

pub mod document {
    use super::*;

    pub fn wrap(page: &Page, config: &SiteConfig, body: Rendered, base: &str) -> String {
        let eyebrow = page.eyebrow.as_deref().unwrap_or("");
        let subtitle = page.subtitle.as_deref().unwrap_or("");

        let doc_header = if !eyebrow.is_empty() || !subtitle.is_empty() {
            format!(
                r#"<header class="doc-header">
  <span class="doc-eyebrow">{}</span>
  <span class="doc-subtitle">{}</span>
</header>
"#,
                esc(eyebrow), esc(subtitle)
            )
        } else { String::new() };

        let mut scripts = body.scripts.clone();
        scripts.push("reload");

        format!(
            r#"<!DOCTYPE html>
<html lang="en">
{head}
<body class="{cls}">
<div class="doc-root">
{nav_back}
<article class="doc-card">
{doc_header}<div class="doc-body">
{body}
</div>
<footer class="doc-footer"></footer>
</article>
</div>
{scripts}
</body>
</html>"#,
            head = head(page, config),
            cls = Shell::Document.class(),
            nav_back = nav_back_html(page, base),
            doc_header = doc_header,
            body = body.html,
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

    pub fn wrap(page: &Page, config: &SiteConfig, body: Rendered, _base: &str) -> String {
        let eyebrow = page.eyebrow.as_deref().unwrap_or("");
        let subtitle = page.subtitle.as_deref().unwrap_or("");

        let mut scripts = body.scripts.clone();
        scripts.push("reload");

        format!(
            r#"<!DOCTYPE html>
<html lang="en">
{head}
<body class="{cls}">
<div class="deck-root">

<div class="deck-bar">
  <span class="deck-site-name">{site}</span>
  <span class="deck-bar-divider">/</span>
  <span class="deck-eyebrow">{eyebrow}</span>
  <div class="deck-bar-right">
    <span class="deck-subtitle">{subtitle}</span>
    <button class="deck-print-btn" onclick="window.print()">Download PDF</button>
  </div>
</div>

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
            head = head(page, config),
            cls = Shell::Deck.class(),
            site = esc(&config.name),
            eyebrow = esc(eyebrow),
            subtitle = esc(subtitle),
            body = body.html,
            scripts = collect_scripts(&scripts),
        )
    }
}

