use crate::types::Page;
use super::esc;
use super::article::parse_markdown;

pub fn render(page: &Page) -> String {
    let mut out = String::new();

    if let Some(nav) = &page.nav_back {
        out.push_str(&format!(
            "<div class=\"agenda-nav no-print\"><a href=\"{}\" class=\"nav-back\">{}</a></div>\n",
            esc(&nav.href), esc(&nav.label)
        ));
    }

    let customer = page.customer.as_deref().unwrap_or(&page.title);
    let date = page.date.as_deref().unwrap_or("");

    out.push_str("<div class=\"agenda-card\">\n");

    // Header bar: type label + customer + date
    out.push_str("<header class=\"agenda-header\">\n");
    out.push_str("<span class=\"agenda-type\">Meeting Agenda</span>\n");
    out.push_str(&format!("<span class=\"agenda-customer\">{}</span>\n", esc(customer)));
    if !date.is_empty() {
        out.push_str(&format!("<span class=\"agenda-date\">{}</span>\n", esc(date)));
    }
    out.push_str("</header>\n");

    // Body
    if let Some(body) = &page.body {
        out.push_str(&format!(
            "<div class=\"agenda-body\">{}</div>\n",
            parse_markdown(body)
        ));
    }

    // Footer
    out.push_str("<footer class=\"agenda-footer\"></footer>\n");
    out.push_str("</div>\n");

    out
}
