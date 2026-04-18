use pulldown_cmark::{html as md_html, Options, Parser as MdParser};

use super::{esc, Rendered};
use crate::icons;
use crate::types::*;

pub fn render(c: &Component) -> Rendered {
    match c {
        Component::Header {
            title,
            subtitle,
            eyebrow,
            align,
        } => header(title, subtitle, eyebrow, *align),
        Component::Meta { fields } => meta(fields),
        Component::CardGrid {
            cards,
            min_width,
            connector,
        } => card_grid(cards, *min_width, *connector),
        Component::SelectableGrid {
            cards,
            interaction,
            connector,
        } => selectable_grid(cards, *interaction, *connector),
        Component::Timeline { items } => timeline(items),
        Component::StatGrid { stats, columns } => stat_grid(stats, *columns),
        Component::BeforeAfter { items } => before_after(items),
        Component::Steps { items, numbered } => steps(items, *numbered),
        Component::Markdown { body } => markdown(body),
        Component::Table {
            columns,
            rows,
            filterable,
        } => table(columns, rows, *filterable),
        Component::Callout {
            variant,
            title,
            body,
            links,
        } => callout(*variant, title, body, links.as_deref()),
        Component::Code { language, code } => code_block(language, code),
        Component::Tabs { tabs } => tabs_component(tabs),
        Component::Section {
            heading,
            eyebrow,
            components,
            align,
        } => section(heading, eyebrow, components, *align),
        Component::Columns {
            columns,
            equal_heights,
        } => columns_component(columns, *equal_heights),
        Component::Accordion { items } => accordion(items),
        Component::Image {
            src,
            alt,
            caption,
            max_width,
            align,
        } => image(src, alt, caption, *max_width, *align),
        // Phase 1 additions
        Component::Badge { label, color } => badge(label, *color),
        Component::Tag { label, color } => tag(label, *color),
        Component::Divider { label } => divider(label),
        Component::Kbd { keys } => kbd(keys),
        Component::Status { label, color } => status(label, *color),
        Component::Breadcrumb { items } => breadcrumb(items),
        Component::ButtonGroup { buttons } => button_group(buttons),
        Component::DefinitionList { items } => definition_list(items),
        Component::Blockquote { body, attribution } => blockquote(body, attribution),
        Component::Avatar {
            name,
            src,
            size,
            subtitle,
        } => avatar(name, src, *size, subtitle),
        Component::AvatarGroup { avatars, size, max } => avatar_group(avatars, *size, *max),
        Component::ProgressBar {
            value,
            label,
            color,
            detail,
        } => progress_bar(*value, label, *color, detail),
        Component::EmptyState {
            title,
            body,
            action,
            icon,
        } => empty_state(title, body, action, icon),
        Component::Icon { name, size, color } => icon_component(name, *size, *color),
    }
}

fn sem_color_class(c: SemColor) -> &'static str {
    c.class_suffix()
}

// ── Header ────────────────────────────────────────

fn header(
    title: &str,
    subtitle: &Option<String>,
    eyebrow: &Option<String>,
    align: Align,
) -> Rendered {
    let mut h = format!(r#"<div class="c-header {}">"#, align.class());
    if let Some(e) = eyebrow {
        h.push_str(&format!(
            r#"<div class="c-header-eyebrow">{}</div>"#,
            esc(e)
        ));
    }
    h.push_str(&format!(
        r#"<h1 class="c-header-title">{}</h1>"#,
        esc(title)
    ));
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

fn card_grid(cards: &[Card], min_width: Option<u32>, connector: Connector) -> Rendered {
    let mw = min_width.unwrap_or(320);
    let is_arrow = matches!(connector, Connector::Arrow);
    let mut h = if is_arrow {
        String::from(r#"<div class="c-card-grid c-card-grid-arrow">"#)
    } else {
        format!(
            r#"<div class="c-card-grid" style="grid-template-columns: repeat(auto-fill, minmax({mw}px, 1fr))">"#
        )
    };
    for (i, card) in cards.iter().enumerate() {
        if is_arrow && i > 0 {
            h.push_str(r#"<div class="c-card-arrow" aria-hidden="true">→</div>"#);
        }
        let tag = if card.href.is_some() { "a" } else { "div" };
        let href_attr = card
            .href
            .as_ref()
            .map(|h| format!(r#" href="{}""#, esc(h)))
            .unwrap_or_default();
        h.push_str(&format!(
            r#"<{tag} class="c-card c-card-{color}"{href_attr}>"#,
            color = sem_color_class(card.color),
        ));
        h.push_str(r#"<div class="c-card-top">"#);
        h.push_str(&format!(
            r#"<h2 class="c-card-title">{}</h2>"#,
            esc(&card.title)
        ));
        if let Some(b) = &card.badge {
            h.push_str(&format!(
                r#"<span class="c-badge c-badge-{color}">{label}</span>"#,
                color = sem_color_class(b.color),
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
                    esc(&l.href),
                    esc(&l.label)
                ));
            }
            h.push_str("</div>");
        }
        h.push_str(&format!("</{tag}>"));
    }
    h.push_str("</div>");
    Rendered::new(h)
}

// ── Selectable Grid ───────────────────────────────

fn selectable_grid(
    cards: &[SelectableCard],
    interaction: Interaction,
    connector: Connector,
) -> Rendered {
    let interaction_attr = match interaction {
        Interaction::SingleSelect => "single_select",
        Interaction::MultiSelect => "multi_select",
        Interaction::None => "none",
    };
    let is_arrow = matches!(connector, Connector::Arrow);

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

    if is_arrow {
        h.push_str(r#"<div class="c-sel-cards c-sel-cards-arrow">"#);
    } else {
        h.push_str(&format!(
            r#"<div class="c-sel-cards" style="grid-template-columns: repeat({}, 1fr)">"#,
            cards.len().max(1)
        ));
    }
    for (i, card) in cards.iter().enumerate() {
        if is_arrow && i > 0 {
            h.push_str(r#"<div class="c-card-arrow" aria-hidden="true">→</div>"#);
        }
        let n = i + 1;
        h.push_str(&format!(
            r#"<button class="sel-card sel-card-{color}" data-n="{n}">"#,
            color = sem_color_class(card.color),
        ));
        let eyebrow = card
            .eyebrow
            .as_deref()
            .map(|s| s.to_string())
            .unwrap_or_else(|| format!("Item {n}"));
        h.push_str(&format!(
            r#"<div class="c-sel-eyebrow">{}</div>"#,
            esc(&eyebrow)
        ));
        h.push_str(&format!(
            r#"<div class="c-sel-title">{}</div>"#,
            esc(&card.title)
        ));
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
            h.push_str(&format!(
                r#"<div class="c-sel-body">{}</div>"#,
                parse_markdown(body)
            ));
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
        h.push_str(&format!(
            r#"<div class="c-stat" style="--stat-color: {color}"><div class="c-stat-label">{label}</div><div class="c-stat-value">{value}</div>"#,
            color = s.color.hex(),
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
        let ctx_span = if ctx.is_empty() {
            String::new()
        } else {
            format!(" {}", esc(ctx))
        };
        h.push_str(&format!(
            r#"<div class="c-ba-card">
  <div class="c-ba-title">{title}</div>
  <div class="c-ba-before">Before: {before}</div>
  <div class="c-ba-after">Now: <span class="c-ba-highlight">{after}</span>{ctx}</div>
</div>"#,
            title = esc(&item.title),
            before = parse_markdown_inline(&item.before),
            after = parse_markdown_inline(&item.after),
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
        h.push_str(&format!(
            r#"<div><div class="c-step-title">{}</div>"#,
            esc(&s.title)
        ));
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
    Rendered::new(format!(
        r#"<div class="c-markdown">{}</div>"#,
        parse_markdown(body)
    ))
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

/// Parse a short string as markdown and strip the outer `<p>...</p>` wrapping
/// so the result can be embedded inline inside another element. Falls back to
/// the full HTML if the input spans multiple blocks.
pub(super) fn parse_markdown_inline(md: &str) -> String {
    let html = parse_markdown(md);
    let trimmed = html.trim_end_matches('\n');
    if let Some(inner) = trimmed.strip_prefix("<p>").and_then(|s| s.strip_suffix("</p>")) {
        inner.to_string()
    } else {
        html
    }
}

// ── Table ─────────────────────────────────────────

fn table(
    columns: &[TableColumn],
    rows: &[std::collections::HashMap<String, serde_yaml::Value>],
    filterable: bool,
) -> Rendered {
    let mut h = String::from(r#"<div class="c-table-wrap">"#);
    if filterable {
        h.push_str(
            r#"<input type="text" class="c-table-filter" data-table-filter placeholder="Filter…">"#,
        );
    }
    h.push_str(r#"<table class="c-table" data-kazam-table><thead><tr>"#);
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

fn callout(
    variant: CalloutVariant,
    title: &Option<String>,
    body: &str,
    links: Option<&[ButtonConfig]>,
) -> Rendered {
    let mut h = format!(r#"<div class="c-callout {}">"#, variant.class());
    if let Some(t) = title {
        h.push_str(&format!(r#"<div class="c-callout-title">{}</div>"#, esc(t)));
    }
    h.push_str(&format!(
        r#"<div class="c-callout-body">{}</div>"#,
        parse_markdown(body)
    ));
    if let Some(ls) = links {
        if !ls.is_empty() {
            h.push_str(r#"<div class="c-callout-links">"#);
            h.push_str(&button_group(ls).html);
            h.push_str("</div>");
        }
    }
    h.push_str("</div>");
    Rendered::new(h)
}

// ── Code ──────────────────────────────────────────

fn code_block(language: &Option<String>, code: &str) -> Rendered {
    let lang = language.as_deref().unwrap_or("");
    let lang_attr = if lang.is_empty() {
        String::new()
    } else {
        format!(r#" data-lang="{}""#, esc(lang))
    };
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

fn section(
    heading: &Option<String>,
    eyebrow: &Option<String>,
    comps: &[Component],
    align: Align,
) -> Rendered {
    let mut r = Rendered::default();
    r.html
        .push_str(&format!(r#"<section class="c-section {}">"#, align.class()));
    if eyebrow.is_some() || heading.is_some() {
        r.html.push_str(r#"<div class="c-section-header">"#);
        if let Some(e) = eyebrow {
            r.html.push_str(&format!(
                r#"<div class="c-section-eyebrow">{}</div>"#,
                esc(e)
            ));
        }
        if let Some(h) = heading {
            r.html
                .push_str(&format!(r#"<h2 class="c-section-heading">{}</h2>"#, esc(h)));
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

fn columns_component(cols: &[Vec<Component>], equal_heights: bool) -> Rendered {
    let mut r = Rendered::default();
    let class = if equal_heights {
        "c-columns c-columns-stretch"
    } else {
        "c-columns"
    };
    r.html.push_str(&format!(
        r#"<div class="{class}" style="grid-template-columns: repeat({}, 1fr)">"#,
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
        r.html
            .push_str(r#"<div class="c-accordion-item" data-accordion-item>"#);
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

fn image(
    src: &str,
    alt: &Option<String>,
    caption: &Option<String>,
    max_width: Option<u32>,
    align: Align,
) -> Rendered {
    let alt_txt = alt.as_deref().unwrap_or("");
    let style = max_width
        .map(|w| format!(r#" style="max-width: {w}px""#))
        .unwrap_or_default();
    let mut h = format!(
        r#"<figure class="c-image {align}"{style}><img src="{src}" alt="{alt}">"#,
        align = align.class(),
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

// ═════════════════════════════════════════════════════════════════════════
// Phase 1 additions
// ═════════════════════════════════════════════════════════════════════════

// ── Badge ────────────────────────────────────────

fn badge(label: &str, color: SemColor) -> Rendered {
    Rendered::new(format!(
        r#"<span class="c-badge c-badge-{color}">{label}</span>"#,
        color = sem_color_class(color),
        label = esc(label)
    ))
}

// ── Tag ──────────────────────────────────────────

fn tag(label: &str, color: SemColor) -> Rendered {
    Rendered::new(format!(
        r#"<span class="c-tag c-tag-{color}">{label}</span>"#,
        color = sem_color_class(color),
        label = esc(label)
    ))
}

// ── Divider ──────────────────────────────────────

fn divider(label: &Option<String>) -> Rendered {
    match label {
        Some(l) => Rendered::new(format!(
            r#"<div class="c-divider c-divider-labeled"><span class="c-divider-line"></span><span class="c-divider-label">{}</span><span class="c-divider-line"></span></div>"#,
            esc(l)
        )),
        None => Rendered::new(r#"<hr class="c-divider">"#.to_string()),
    }
}

// ── Kbd ──────────────────────────────────────────

fn kbd(keys: &[String]) -> Rendered {
    let mut h = String::from(r#"<span class="c-kbd-group">"#);
    for (i, k) in keys.iter().enumerate() {
        if i > 0 {
            h.push_str(r#"<span class="c-kbd-sep">+</span>"#);
        }
        h.push_str(&format!(r#"<kbd class="c-kbd">{}</kbd>"#, esc(k)));
    }
    h.push_str("</span>");
    Rendered::new(h)
}

// ── Status ───────────────────────────────────────

fn status(label: &str, color: SemColor) -> Rendered {
    Rendered::new(format!(
        r#"<span class="c-status c-status-{color}"><span class="c-status-dot"></span><span>{label}</span></span>"#,
        color = sem_color_class(color),
        label = esc(label)
    ))
}

// ── Breadcrumb ───────────────────────────────────

fn breadcrumb(items: &[BreadcrumbItem]) -> Rendered {
    let mut h = String::from(r#"<nav class="c-breadcrumb" aria-label="Breadcrumb"><ol>"#);
    for (i, item) in items.iter().enumerate() {
        if i > 0 {
            h.push_str(r#"<li class="c-breadcrumb-sep" aria-hidden="true">/</li>"#);
        }
        h.push_str(r#"<li class="c-breadcrumb-item">"#);
        match &item.href {
            Some(href) => {
                h.push_str(&format!(
                    r#"<a href="{}">{}</a>"#,
                    esc(href),
                    esc(&item.label)
                ));
            }
            None => {
                h.push_str(&format!(
                    r#"<span aria-current="page">{}</span>"#,
                    esc(&item.label)
                ));
            }
        }
        h.push_str("</li>");
    }
    h.push_str("</ol></nav>");
    Rendered::new(h)
}

// ── Button Group ─────────────────────────────────

fn button_group(buttons: &[ButtonConfig]) -> Rendered {
    let mut h = String::from(r#"<div class="c-button-group">"#);
    for b in buttons {
        let variant_class = match b.variant {
            ButtonVariant::Primary => "c-button-primary",
            ButtonVariant::Secondary => "c-button-secondary",
            ButtonVariant::Ghost => "c-button-ghost",
        };
        let target = if b.external {
            r#" target="_blank" rel="noopener""#
        } else {
            ""
        };
        h.push_str(&format!(
            r#"<a href="{href}" class="c-button {variant}"{target}>"#,
            href = esc(&b.href),
            variant = variant_class,
            target = target
        ));
        if let Some(icon_name) = &b.icon {
            h.push_str(&format!(
                r#"<span class="c-button-icon">{}</span>"#,
                icons::render(icon_name, 14, "currentColor")
            ));
        }
        h.push_str(&format!(r#"<span>{}</span>"#, esc(&b.label)));
        if b.external {
            h.push_str(&format!(
                r#"<span class="c-button-icon">{}</span>"#,
                icons::render("arrow-up-right", 14, "currentColor")
            ));
        }
        h.push_str("</a>");
    }
    h.push_str("</div>");
    Rendered::new(h)
}

// ── Definition List ──────────────────────────────

fn definition_list(items: &[DefinitionItem]) -> Rendered {
    let mut h = String::from(r#"<dl class="c-definition-list">"#);
    for i in items {
        h.push_str(&format!(
            r#"<div class="c-dl-row"><dt class="c-dl-term">{term}</dt><dd class="c-dl-def">{def}</dd></div>"#,
            term = esc(&i.term),
            def = esc(&i.definition)
        ));
    }
    h.push_str("</dl>");
    Rendered::new(h)
}

// ── Blockquote ───────────────────────────────────

fn blockquote(body: &str, attribution: &Option<String>) -> Rendered {
    let mut h = format!(
        r#"<figure class="c-blockquote"><blockquote><p>{}</p></blockquote>"#,
        esc(body)
    );
    if let Some(a) = attribution {
        h.push_str(&format!(
            r#"<figcaption class="c-blockquote-attribution">— {}</figcaption>"#,
            esc(a)
        ));
    }
    h.push_str("</figure>");
    Rendered::new(h)
}

// ── Avatar ───────────────────────────────────────

fn initials(name: &str) -> String {
    name.split_whitespace()
        .take(2)
        .filter_map(|w| w.chars().next())
        .map(|c| c.to_ascii_uppercase())
        .collect()
}

fn avatar(
    name: &str,
    src: &Option<String>,
    size: AvatarSize,
    subtitle: &Option<String>,
) -> Rendered {
    let size_class = size.class_suffix();
    let wrapper_open = if subtitle.is_some() {
        format!(r#"<div class="c-avatar-row"><div class="c-avatar c-avatar-{size_class}">"#)
    } else {
        format!(r#"<div class="c-avatar c-avatar-{size_class}">"#)
    };
    let mut h = wrapper_open;
    match src {
        Some(s) => {
            h.push_str(&format!(r#"<img src="{}" alt="{}">"#, esc(s), esc(name)));
        }
        None => {
            h.push_str(&format!(
                r#"<span class="c-avatar-initials">{}</span>"#,
                esc(&initials(name))
            ));
        }
    }
    h.push_str("</div>");
    if let Some(sub) = subtitle {
        h.push_str(&format!(
            r#"<div class="c-avatar-meta"><div class="c-avatar-name">{}</div><div class="c-avatar-sub">{}</div></div></div>"#,
            esc(name), esc(sub)
        ));
    }
    Rendered::new(h)
}

// ── Avatar Group ─────────────────────────────────

fn avatar_group(avatars: &[AvatarConfig], size: AvatarSize, max: usize) -> Rendered {
    let size_class = size.class_suffix();
    let mut h = format!(r#"<div class="c-avatar-group c-avatar-group-{size_class}">"#);
    let visible = avatars.len().min(max);
    for a in avatars.iter().take(visible) {
        h.push_str(&format!(
            r#"<div class="c-avatar c-avatar-{size_class}" title="{}">"#,
            esc(&a.name)
        ));
        match &a.src {
            Some(s) => h.push_str(&format!(r#"<img src="{}" alt="{}">"#, esc(s), esc(&a.name))),
            None => h.push_str(&format!(
                r#"<span class="c-avatar-initials">{}</span>"#,
                esc(&initials(&a.name))
            )),
        }
        h.push_str("</div>");
    }
    if avatars.len() > max {
        let remaining = avatars.len() - max;
        h.push_str(&format!(
            r#"<div class="c-avatar c-avatar-{size_class} c-avatar-more"><span class="c-avatar-initials">+{}</span></div>"#,
            remaining
        ));
    }
    h.push_str("</div>");
    Rendered::new(h)
}

// ── Progress Bar ─────────────────────────────────

fn progress_bar(
    value: u8,
    label: &Option<String>,
    color: SemColor,
    detail: &Option<String>,
) -> Rendered {
    let clamped = value.min(100);
    let color_class = sem_color_class(color);

    let mut h = String::from(r#"<div class="c-progress">"#);
    if label.is_some() || detail.is_some() {
        h.push_str(r#"<div class="c-progress-labels">"#);
        if let Some(l) = label {
            h.push_str(&format!(
                r#"<span class="c-progress-label">{}</span>"#,
                esc(l)
            ));
        } else {
            h.push_str(r#"<span></span>"#);
        }
        h.push_str(&format!(
            r#"<span class="c-progress-value">{}%</span>"#,
            clamped
        ));
        h.push_str("</div>");
    }
    h.push_str(&format!(
        r#"<div class="c-progress-track" role="progressbar" aria-valuenow="{v}" aria-valuemin="0" aria-valuemax="100"><div class="c-progress-fill c-progress-fill-{color}" style="width: {v}%"></div></div>"#,
        v = clamped, color = color_class
    ));
    if let Some(d) = detail {
        h.push_str(&format!(
            r#"<div class="c-progress-detail">{}</div>"#,
            esc(d)
        ));
    }
    h.push_str("</div>");
    Rendered::new(h)
}

// ── Empty State ──────────────────────────────────

fn empty_state(
    title: &str,
    body: &Option<String>,
    action: &Option<EmptyStateAction>,
    icon: &Option<String>,
) -> Rendered {
    let mut h = String::from(r#"<div class="c-empty-state">"#);
    let icon_name = icon.as_deref().unwrap_or("inbox");
    h.push_str(&format!(
        r#"<div class="c-empty-state-icon">{}</div>"#,
        icons::render(icon_name, 32, "currentColor")
    ));
    h.push_str(&format!(
        r#"<h3 class="c-empty-state-title">{}</h3>"#,
        esc(title)
    ));
    if let Some(b) = body {
        h.push_str(&format!(r#"<p class="c-empty-state-body">{}</p>"#, esc(b)));
    }
    if let Some(a) = action {
        h.push_str(&format!(
            r#"<a href="{href}" class="c-button c-button-primary">{label}</a>"#,
            href = esc(&a.href),
            label = esc(&a.label)
        ));
    }
    h.push_str("</div>");
    Rendered::new(h)
}

// ── Icon ─────────────────────────────────────────

fn icon_component(name: &str, size: IconSize, color: SemColor) -> Rendered {
    let px = size.pixels();
    let color_value = match color {
        SemColor::Default => "currentColor".to_string(),
        _ => color.hex().to_string(),
    };
    Rendered::new(icons::render(name, px, &color_value))
}
