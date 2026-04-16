use pulldown_cmark::{html as md_html, Options, Parser as MdParser};

use crate::types::*;
use super::{esc, Rendered};

pub fn render(c: &Component) -> Rendered {
    match c {
        Component::Header { title, subtitle, eyebrow } => header(title, subtitle, eyebrow),
        Component::Meta { fields } => meta(fields),
        Component::CardGrid { cards, min_width } => card_grid(cards, *min_width),
        Component::SelectableGrid { cards, interaction, connector } => selectable_grid(cards, *interaction, *connector),
        Component::Timeline { items } => timeline(items),
        Component::StatGrid { stats, columns } => stat_grid(stats, *columns),
        Component::BeforeAfter { items } => before_after(items),
        Component::Steps { items, numbered } => steps(items, *numbered),
        Component::Markdown { body } => markdown(body),
        Component::Table { columns, rows, filterable } => table(columns, rows, *filterable),
        Component::Callout { variant, title, body } => callout(*variant, title, body),
        Component::Code { language, code } => code_block(language, code),
        Component::Tabs { tabs } => tabs_component(tabs),
        Component::Section { heading, eyebrow, components } => section(heading, eyebrow, components),
        Component::Columns { columns } => columns_component(columns),
        Component::Accordion { items } => accordion(items),
        Component::Image { src, alt, caption, max_width } => image(src, alt, caption, *max_width),
    }
}

// ── Header ────────────────────────────────────────

fn header(title: &str, subtitle: &Option<String>, eyebrow: &Option<String>) -> Rendered {
    let mut h = String::from(r#"<div class="c-header">"#);
    if let Some(e) = eyebrow {
        h.push_str(&format!(r#"<div class="c-header-eyebrow">{}</div>"#, esc(e)));
    }
    h.push_str(&format!(r#"<h1 class="c-header-title">{}</h1>"#, esc(title)));
    if let Some(s) = subtitle {
        h.push_str(&format!(r#"<p class="c-header-subtitle">{}</p>"#, esc(s)));
    }
    h.push_str("</div>");
    Rendered::new(h)
}

// ── Meta ──────────────────────────────────────────

fn meta(fields: &[MetaField]) -> Rendered {
    let mut h = String::from(r#"<div class="c-meta">"#);
    for f in fields {
        h.push_str(&format!(
            r#"<div class="c-meta-item"><span class="c-meta-key">{}</span><span class="c-meta-value">{}</span></div>"#,
            esc(&f.key), esc(&f.value)
        ));
    }
    h.push_str("</div>");
    Rendered::new(h)
}

// ── Card Grid ─────────────────────────────────────

fn card_grid(cards: &[Card], min_width: Option<u32>) -> Rendered {
    let mw = min_width.unwrap_or(320);
    let mut h = format!(
        r#"<div class="c-card-grid" style="grid-template-columns: repeat(auto-fill, minmax({mw}px, 1fr))">"#
    );
    for card in cards {
        let tag = if card.href.is_some() { "a" } else { "div" };
        let href_attr = card.href.as_ref().map(|h| format!(r#" href="{}""#, esc(h))).unwrap_or_default();
        h.push_str(&format!(r#"<{tag} class="c-card"{href_attr}>"#));
        h.push_str(r#"<div class="c-card-top">"#);
        h.push_str(&format!(r#"<h2 class="c-card-title">{}</h2>"#, esc(&card.title)));
        if let Some(b) = &card.badge {
            h.push_str(&format!(
                r#"<span class="c-badge c-badge-{color}">{label}</span>"#,
                color = badge_color_class(b.color),
                label = esc(&b.label)
            ));
        }
        h.push_str("</div>");
        if let Some(d) = &card.description {
            h.push_str(&format!(r#"<p class="c-card-desc">{}</p>"#, esc(d)));
        }
        if let Some(links) = &card.links {
            h.push_str(r#"<div class="c-card-links">"#);
            for l in links {
                h.push_str(&format!(
                    r#"<a href="{}" class="c-card-link">{}</a>"#,
                    esc(&l.href), esc(&l.label)
                ));
            }
            h.push_str("</div>");
        }
        h.push_str(&format!("</{tag}>"));
    }
    h.push_str("</div>");
    Rendered::new(h)
}

fn badge_color_class(c: BadgeColor) -> &'static str {
    match c {
        BadgeColor::Green => "green",
        BadgeColor::Yellow => "yellow",
        BadgeColor::Red => "red",
        BadgeColor::Default => "default",
    }
}

// ── Selectable Grid ───────────────────────────────

fn selectable_grid(cards: &[SelectableCard], interaction: Interaction, connector: Connector) -> Rendered {
    let interaction_attr = match interaction {
        Interaction::SingleSelect => "single_select",
        Interaction::MultiSelect => "multi_select",
        Interaction::None => "none",
    };

    let mut h = format!(
        r#"<div class="c-selectable-grid" data-selectable-grid data-interaction="{interaction_attr}">"#
    );

    if matches!(connector, Connector::DotsLine) {
        h.push_str(r#"<div class="c-sel-dots-row"><div class="c-sel-dots-line"></div>"#);
        for (i, _) in cards.iter().enumerate() {
            let n = i + 1;
            h.push_str(&format!(
                r#"<button class="sel-dot" data-n="{n}">{n}</button>"#
            ));
        }
        h.push_str("</div>");
    }

    h.push_str(&format!(
        r#"<div class="c-sel-cards" style="grid-template-columns: repeat({}, 1fr)">"#,
        cards.len().max(1)
    ));
    for (i, card) in cards.iter().enumerate() {
        let n = i + 1;
        h.push_str(&format!(r#"<button class="sel-card" data-n="{n}">"#));
        let eyebrow = card.eyebrow.as_deref()
            .map(|s| s.to_string())
            .unwrap_or_else(|| format!("Item {n}"));
        h.push_str(&format!(r#"<div class="c-sel-eyebrow">{}</div>"#, esc(&eyebrow)));
        h.push_str(&format!(r#"<div class="c-sel-title">{}</div>"#, esc(&card.title)));
        if let Some(bullets) = &card.bullets {
            h.push_str(r#"<ul class="c-sel-bullets">"#);
            for b in bullets {
                h.push_str(&format!(
                    r#"<li><span class="c-sel-bullet-dot"></span><span>{}</span></li>"#,
                    esc(b)
                ));
            }
            h.push_str("</ul>");
        }
        if let Some(body) = &card.body {
            h.push_str(&format!(r#"<div class="c-sel-body">{}</div>"#, parse_markdown(body)));
        }
        h.push_str("</button>");
    }
    h.push_str("</div></div>");
    Rendered::new(h).with_script("selectable_grid")
}

// ── Timeline ──────────────────────────────────────

fn timeline(items: &[TimelineItem]) -> Rendered {
    let mut h = String::from(r#"<div class="c-timeline">"#);
    for item in items {
        let cls = match item.status {
            TimelineStatus::Completed => "completed",
            TimelineStatus::Active => "active",
            TimelineStatus::Upcoming => "upcoming",
        };
        h.push_str(&format!(
            r#"<div class="c-timeline-phase {cls}"><div class="c-timeline-dot"></div><div class="c-timeline-label">{name}</div><div class="c-timeline-bar {cls}"></div></div>"#,
            cls = cls, name = esc(&item.name)
        ));
    }
    h.push_str("</div>");
    Rendered::new(h)
}

// ── Stat Grid ─────────────────────────────────────

fn stat_grid(stats: &[Stat], columns: u32) -> Rendered {
    let mut h = format!(
        r#"<div class="c-stat-grid" style="grid-template-columns: repeat({columns}, 1fr)">"#
    );
    for s in stats {
        let color = match s.color {
            StatColor::Green => "#34D399",
            StatColor::Yellow => "#FBBF24",
            StatColor::Red => "#F87171",
            StatColor::Default => "#3CCECE",
        };
        h.push_str(&format!(
            r#"<div class="c-stat" style="--stat-color: {color}"><div class="c-stat-label">{label}</div><div class="c-stat-value">{value}</div>"#,
            color = color,
            label = esc(&s.label),
            value = esc(&s.value),
        ));
        if let Some(d) = &s.detail {
            h.push_str(&format!(r#"<div class="c-stat-detail">{}</div>"#, esc(d)));
        }
        h.push_str("</div>");
    }
    h.push_str("</div>");
    Rendered::new(h)
}

// ── Before / After ────────────────────────────────

fn before_after(items: &[BeforeAfterItem]) -> Rendered {
    let mut h = String::from(r#"<div class="c-before-after">"#);
    for item in items {
        let ctx = item.after_context.as_deref().unwrap_or("");
        let ctx_span = if ctx.is_empty() { String::new() } else { format!(" {}", esc(ctx)) };
        h.push_str(&format!(
            r#"<div class="c-ba-card">
  <div class="c-ba-title">{title}</div>
  <div class="c-ba-before">Before: {before}</div>
  <div class="c-ba-after">Now: <span class="c-ba-highlight">{after}</span>{ctx}</div>
</div>"#,
            title = esc(&item.title),
            before = esc(&item.before),
            after = esc(&item.after),
            ctx = ctx_span,
        ));
    }
    h.push_str("</div>");
    Rendered::new(h)
}

// ── Steps ─────────────────────────────────────────

fn steps(items: &[Step], numbered: bool) -> Rendered {
    let tag = if numbered { "ol" } else { "ul" };
    let mut h = format!(r#"<{tag} class="c-steps">"#);
    for (i, s) in items.iter().enumerate() {
        h.push_str(r#"<li class="c-step">"#);
        if numbered {
            h.push_str(&format!(r#"<div class="c-step-num">{}</div>"#, i + 1));
        } else {
            h.push_str(r#"<div class="c-step-bullet"></div>"#);
        }
        h.push_str(&format!(r#"<div><div class="c-step-title">{}</div>"#, esc(&s.title)));
        if let Some(d) = &s.detail {
            h.push_str(&format!(r#"<div class="c-step-detail">{}</div>"#, esc(d)));
        }
        h.push_str("</div></li>");
    }
    h.push_str(&format!("</{tag}>"));
    Rendered::new(h)
}

// ── Markdown ──────────────────────────────────────

fn markdown(body: &str) -> Rendered {
    Rendered::new(format!(r#"<div class="c-markdown">{}</div>"#, parse_markdown(body)))
}

pub(super) fn parse_markdown(md: &str) -> String {
    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_TABLES);
    opts.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = MdParser::new_ext(md, opts);
    let mut html = String::new();
    md_html::push_html(&mut html, parser);
    html
}

// ── Table ─────────────────────────────────────────

fn table(columns: &[TableColumn], rows: &[std::collections::HashMap<String, serde_yaml::Value>], filterable: bool) -> Rendered {
    let mut h = String::from(r#"<div class="c-table-wrap">"#);
    if filterable {
        h.push_str(r#"<input type="text" class="c-table-filter" data-table-filter placeholder="Filter…">"#);
    }
    h.push_str(r#"<table class="c-table" data-finro-table><thead><tr>"#);
    for col in columns {
        let sortable_attr = if col.sortable { " data-sortable" } else { "" };
        h.push_str(&format!(
            r#"<th class="{align}"{sortable_attr}>{label}</th>"#,
            align = col.align.class(),
            sortable_attr = sortable_attr,
            label = esc(&col.label),
        ));
    }
    h.push_str("</tr></thead><tbody>");
    for row in rows {
        h.push_str("<tr>");
        for col in columns {
            let v = row.get(&col.key).map(value_to_string).unwrap_or_default();
            h.push_str(&format!(
                r#"<td class="{}">{}</td>"#,
                col.align.class(),
                esc(&v)
            ));
        }
        h.push_str("</tr>");
    }
    h.push_str("</tbody></table></div>");
    Rendered::new(h).with_script("table")
}

// ── Callout ───────────────────────────────────────

fn callout(variant: CalloutVariant, title: &Option<String>, body: &str) -> Rendered {
    let mut h = format!(r#"<div class="c-callout {}">"#, variant.class());
    if let Some(t) = title {
        h.push_str(&format!(r#"<div class="c-callout-title">{}</div>"#, esc(t)));
    }
    h.push_str(&format!(r#"<div class="c-callout-body">{}</div>"#, parse_markdown(body)));
    h.push_str("</div>");
    Rendered::new(h)
}

// ── Code ──────────────────────────────────────────

fn code_block(language: &Option<String>, code: &str) -> Rendered {
    let lang = language.as_deref().unwrap_or("");
    let lang_attr = if lang.is_empty() { String::new() } else { format!(r#" data-lang="{}""#, esc(lang)) };
    Rendered::new(format!(
        r#"<pre class="c-code"{lang_attr}><code>{}</code></pre>"#,
        esc(code)
    ))
}

// ── Tabs ──────────────────────────────────────────

fn tabs_component(tabs: &[Tab]) -> Rendered {
    let mut body_html = String::from(r#"<div class="c-tabs" data-tabs>"#);
    body_html.push_str(r#"<div class="c-tab-buttons">"#);
    for tab in tabs {
        body_html.push_str(&format!(
            r#"<button class="tab-btn">{}</button>"#,
            esc(&tab.label)
        ));
    }
    body_html.push_str("</div>");

    let mut scripts: Vec<&'static str> = vec!["tabs"];
    for tab in tabs {
        body_html.push_str(r#"<div class="tab-panel">"#);
        for c in &tab.components {
            let r = render(c);
            body_html.push_str(&r.html);
            scripts.extend(r.scripts);
        }
        body_html.push_str("</div>");
    }
    body_html.push_str("</div>");
    let mut out = Rendered::new(body_html);
    out.scripts = scripts;
    out
}

// ── Section ───────────────────────────────────────

fn section(heading: &Option<String>, eyebrow: &Option<String>, comps: &[Component]) -> Rendered {
    let mut r = Rendered::default();
    r.html.push_str(r#"<section class="c-section">"#);
    if eyebrow.is_some() || heading.is_some() {
        r.html.push_str(r#"<div class="c-section-header">"#);
        if let Some(e) = eyebrow {
            r.html.push_str(&format!(r#"<div class="c-section-eyebrow">{}</div>"#, esc(e)));
        }
        if let Some(h) = heading {
            r.html.push_str(&format!(r#"<h2 class="c-section-heading">{}</h2>"#, esc(h)));
        }
        r.html.push_str("</div>");
    }
    for c in comps {
        r.extend(render(c));
    }
    r.html.push_str("</section>");
    r
}

// ── Columns ───────────────────────────────────────

fn columns_component(cols: &[Vec<Component>]) -> Rendered {
    let mut r = Rendered::default();
    r.html.push_str(&format!(
        r#"<div class="c-columns" style="grid-template-columns: repeat({}, 1fr)">"#,
        cols.len().max(1)
    ));
    for col in cols {
        r.html.push_str(r#"<div class="c-column">"#);
        for c in col {
            r.extend(render(c));
        }
        r.html.push_str("</div>");
    }
    r.html.push_str("</div>");
    r
}

// ── Accordion ─────────────────────────────────────

fn accordion(items: &[AccordionItem]) -> Rendered {
    let mut r = Rendered::default();
    r.html.push_str(r#"<div class="c-accordion">"#);
    for item in items {
        r.html.push_str(r#"<div class="c-accordion-item" data-accordion-item>"#);
        r.html.push_str(&format!(
            r#"<button class="accordion-head">{}<span class="accordion-chevron">›</span></button>"#,
            esc(&item.title)
        ));
        r.html.push_str(r#"<div class="accordion-body">"#);
        for c in &item.components {
            r.extend(render(c));
        }
        r.html.push_str("</div></div>");
    }
    r.html.push_str("</div>");
    r.scripts.push("accordion");
    r
}

// ── Image ─────────────────────────────────────────

fn image(src: &str, alt: &Option<String>, caption: &Option<String>, max_width: Option<u32>) -> Rendered {
    let alt_txt = alt.as_deref().unwrap_or("");
    let style = max_width.map(|w| format!(r#" style="max-width: {w}px""#)).unwrap_or_default();
    let mut h = format!(
        r#"<figure class="c-image"{style}><img src="{src}" alt="{alt}">"#,
        style = style,
        src = esc(src),
        alt = esc(alt_txt),
    );
    if let Some(cap) = caption {
        h.push_str(&format!(r#"<figcaption>{}</figcaption>"#, esc(cap)));
    }
    h.push_str("</figure>");
    Rendered::new(h)
}
