use crate::types::{BadgeColor, Page};
use super::esc;

pub fn render(page: &Page) -> String {
    let mut out = String::new();

    out.push_str(&format!(
        "<div class=\"page-header\"><h1 class=\"page-title\">{}</h1>",
        esc(&page.title)
    ));
    if let Some(sub) = &page.subtitle {
        out.push_str(&format!("<p class=\"page-subtitle\">{}</p>", esc(sub)));
    }
    out.push_str("</div>\n");

    let Some(cards) = &page.cards else { return out };

    out.push_str("<div class=\"card-grid\">\n");
    for card in cards {
        out.push_str("<div class=\"card\">\n<div class=\"card-top\">");
        out.push_str(&format!("<h2 class=\"card-name\">{}</h2>", esc(&card.name)));

        if let Some(badge) = &card.badge {
            let cls = match card.badge_color {
                BadgeColor::Green => "badge badge-green",
                BadgeColor::Yellow => "badge badge-yellow",
                BadgeColor::Red => "badge badge-red",
                BadgeColor::Default => "badge",
            };
            out.push_str(&format!("<span class=\"{}\">{}</span>", cls, esc(badge)));
        }
        out.push_str("</div>\n");

        if let Some(desc) = &card.description {
            out.push_str(&format!("<p class=\"card-desc\">{}</p>\n", esc(desc)));
        }

        if let Some(links) = &card.links {
            out.push_str("<div class=\"card-links\">\n");
            for link in links {
                out.push_str(&format!(
                    "<a href=\"{}\" class=\"card-link\">{}</a>\n",
                    esc(&link.href),
                    esc(&link.label)
                ));
            }
            out.push_str("</div>\n");
        }

        out.push_str("</div>\n");
    }
    out.push_str("</div>\n");
    out
}
