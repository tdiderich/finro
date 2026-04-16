use pulldown_cmark::{html as md_html, Options, Parser as MdParser};
use crate::types::Page;
use super::esc;

pub fn render(page: &Page) -> String {
    let mut out = String::new();

    if let Some(nav) = &page.nav_back {
        out.push_str(&format!(
            "<a href=\"{}\" class=\"nav-back\">{}</a>\n",
            esc(&nav.href), esc(&nav.label)
        ));
    }

    out.push_str(&format!(
        "<div class=\"article-header\"><h1 class=\"page-title\">{}</h1></div>\n",
        esc(&page.title)
    ));

    if let Some(meta) = &page.meta {
        out.push_str("<div class=\"meta-grid\">\n");
        for f in meta {
            out.push_str(&format!(
                "<div class=\"meta-item\"><span class=\"meta-key\">{}</span><span class=\"meta-value\">{}</span></div>\n",
                esc(&f.key), esc(&f.value)
            ));
        }
        out.push_str("</div>\n");
    }

    if let Some(body) = &page.body {
        out.push_str(&format!(
            "<div class=\"article-body\">{}</div>\n",
            parse_markdown(body)
        ));
    }

    out
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
