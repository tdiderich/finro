use crate::theme;
use crate::types::SiteConfig;
use super::esc;

pub fn wrap(title: &str, config: &SiteConfig, content: &str) -> String {
    let nav_html = render_nav(config);

    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>{title} — {site}</title>
  <style>{css}</style>
</head>
<body>
  <header class="site-header">
    <div class="container header-inner">
      <span class="site-name">{site}</span>
      {nav}
    </div>
  </header>
  <main class="container main-content">
    {content}
  </main>
  <script>
    document.addEventListener('DOMContentLoaded', function() {{
      var href = window.location.pathname.split('/').pop() || 'index.html';
      document.querySelectorAll('.nav-link').forEach(function(a) {{
        if (a.getAttribute('href') === href) a.classList.add('nav-link-active');
      }});
    }});
  </script>
</body>
</html>"#,
        title = esc(title),
        site = esc(&config.name),
        css = theme::SITE_CSS,
        nav = nav_html,
        content = content,
    )
}

fn render_nav(config: &SiteConfig) -> String {
    let links = match &config.nav {
        Some(links) if !links.is_empty() => links,
        _ => return String::new(),
    };
    let mut out = String::from("<nav>");
    for link in links {
        out.push_str(&format!(
            r#"<a href="{}" class="nav-link">{}</a>"#,
            esc(&link.href),
            esc(&link.label)
        ));
    }
    out.push_str("</nav>");
    out
}
