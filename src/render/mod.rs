mod agenda;
mod article;
mod dashboard;
mod phases;
mod qbr;
mod shell;

use crate::types::{Layout, Page, SiteConfig};

pub fn render_page(page: &Page, config: &SiteConfig) -> String {
    match &page.layout {
        Layout::Qbr => qbr::render(page, config),
        _ => {
            let body = match &page.layout {
                Layout::Dashboard => dashboard::render(page),
                Layout::Article => article::render(page),
                Layout::Agenda => agenda::render(page),
                Layout::Phases => phases::render(page),
                Layout::Qbr => unreachable!(),
            };
            shell::wrap(&page.title, config, &body)
        }
    }
}

pub(super) fn esc(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
