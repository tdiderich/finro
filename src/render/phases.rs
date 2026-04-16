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
        "<div class=\"page-header\"><div class=\"page-eyebrow\">Reference</div><h1 class=\"page-title\">{}</h1></div>\n",
        esc(&page.title)
    ));

    let Some(phases) = &page.phases else { return out };

    // Dot row
    out.push_str("<div class=\"phase-dots-row\">\n");
    out.push_str("<div class=\"phase-track-line\"></div>\n");
    for phase in phases {
        out.push_str(&format!(
            "<button class=\"phase-dot\" data-n=\"{}\" aria-label=\"Phase {}: {}\">{}</button>\n",
            phase.number, phase.number, esc(&phase.name), phase.number
        ));
    }
    out.push_str("</div>\n");

    // Phase cards
    out.push_str(&format!(
        "<div class=\"phase-grid\" style=\"grid-template-columns: repeat({}, 1fr)\">\n",
        phases.len()
    ));
    for phase in phases {
        out.push_str(&format!(
            "<button class=\"phase-card\" data-n=\"{}\">\n",
            phase.number
        ));
        out.push_str(&format!(
            "<div class=\"phase-eyebrow\">PHASE {}</div>\n<div class=\"phase-name\">{}</div>\n",
            phase.number, esc(&phase.name)
        ));
        out.push_str("<ul class=\"phase-bullets\">\n");
        for bullet in &phase.bullets {
            out.push_str(&format!(
                "<li><span class=\"phase-dot-bullet\"></span><span>{}</span></li>\n",
                esc(bullet)
            ));
        }
        out.push_str("</ul>\n</button>\n");
    }
    out.push_str("</div>\n");

    out.push_str(PHASES_JS);
    out
}

const PHASES_JS: &str = r#"<script>
(function() {
  var active = null;
  function update(n) {
    document.querySelectorAll('.phase-dot').forEach(function(el) {
      el.classList.toggle('phase-dot-active', el.dataset.n === String(n));
    });
    document.querySelectorAll('.phase-card').forEach(function(el) {
      el.classList.remove('phase-card-active', 'phase-card-dimmed');
      if (n !== null) {
        if (el.dataset.n === String(n)) el.classList.add('phase-card-active');
        else el.classList.add('phase-card-dimmed');
      }
    });
  }
  document.querySelectorAll('.phase-dot, .phase-card').forEach(function(el) {
    el.addEventListener('click', function() {
      var n = parseInt(el.dataset.n);
      active = (active === n) ? null : n;
      update(active);
    });
  });
})();
</script>"#;
