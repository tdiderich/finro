use crate::theme;
use crate::types::{HealthColor, Page, PhaseStatus, SiteConfig};
use super::esc;

pub fn render(page: &Page, config: &SiteConfig) -> String {
    let customer = page.customer.as_deref().unwrap_or(&page.title);
    let date = page.date.as_deref().unwrap_or("");

    let slide1 = render_outcomes(page);
    let slide2 = render_progress(page);
    let slide3 = render_next_steps(page);

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
<div class="qbr-root">

  <div class="qbr-bar">
    <span class="qbr-site-name">{site}</span>
    <span class="qbr-bar-divider">/</span>
    <span class="qbr-bar-customer">{customer}</span>
    <div class="qbr-bar-right">
      <span class="qbr-bar-date">{date}</span>
      <button class="qbr-print-btn" onclick="window.print()">Download PDF</button>
    </div>
  </div>

  <div class="qbr-viewport">
    <div class="qbr-track">
      <div class="qbr-slide">
        <div class="slide-inner">{slide1}</div>
      </div>
      <div class="qbr-slide">
        <div class="slide-inner">{slide2}</div>
      </div>
      <div class="qbr-slide">
        <div class="slide-inner">{slide3}</div>
      </div>
    </div>
  </div>

  <div class="qbr-nav">
    <button class="qbr-arrow qbr-prev" id="qbr-prev"></button>
    <span class="qbr-nav-label" id="qbr-label"></span>
    <button class="qbr-arrow qbr-next" id="qbr-next"></button>
  </div>

</div>
{js}
</body>
</html>"#,
        title = esc(&page.title),
        site = esc(&config.name),
        css = theme::QBR_CSS,
        customer = esc(customer),
        date = esc(date),
        slide1 = slide1,
        slide2 = slide2,
        slide3 = slide3,
        js = QBR_JS,
    )
}

fn render_outcomes(page: &Page) -> String {
    let mut out = String::from("<div class=\"slide-label\">Success Outcomes</div>\n<div class=\"outcomes-stack\">\n");
    if let Some(outcomes) = &page.success_outcomes {
        for o in outcomes {
            let ctx = o.now_context.as_deref().unwrap_or("");
            out.push_str(&format!(
                r#"<div class="outcome-card">
  <div class="outcome-title">{title}</div>
  <div class="outcome-before">Before: {before}</div>
  <div class="outcome-now">Now: <span class="outcome-highlight">{highlight}</span>{ctx}</div>
</div>
"#,
                title = esc(&o.title),
                before = esc(&o.before),
                highlight = esc(&o.now_highlight),
                ctx = if ctx.is_empty() { String::new() } else { format!(" {}", esc(ctx)) },
            ));
        }
    }
    out.push_str("</div>");
    out
}

fn render_progress(page: &Page) -> String {
    let mut out = String::from("<div class=\"slide-label\">Deployment Progress</div>\n");

    // Phase timeline
    if let Some(timeline) = &page.phase_timeline {
        out.push_str("<div class=\"timeline\">\n");
        for phase in timeline {
            let cls = match phase.status {
                PhaseStatus::Completed => "completed",
                PhaseStatus::Active => "active",
                PhaseStatus::Upcoming => "upcoming",
            };
            out.push_str(&format!(
                "<div class=\"timeline-phase {cls}\"><div class=\"timeline-dot\"></div><div class=\"timeline-label-text\">{name}</div><div class=\"timeline-bar {cls}\"></div></div>\n",
                cls = cls,
                name = esc(&phase.name)
            ));
        }
        out.push_str("</div>\n");
    }

    // Health cards
    if let Some(cards) = &page.health_cards {
        out.push_str("<div class=\"health-grid\">\n");
        for card in cards {
            let color = match card.color {
                HealthColor::Green => "#34D399",
                HealthColor::Yellow => "#FBBF24",
                HealthColor::Red => "#F87171",
            };
            out.push_str(&format!(
                r#"<div class="health-card" style="--card-color: {color}">
  <div class="health-label">{label}</div>
  <div class="health-value">{value}</div>
  <div class="health-detail">{detail}</div>
</div>
"#,
                color = color,
                label = esc(&card.label),
                value = esc(&card.value),
                detail = esc(&card.detail),
            ));
        }
        out.push_str("</div>\n");
    }

    out
}

fn render_next_steps(page: &Page) -> String {
    let mut out = String::from("<div class=\"slide-label\">Next Steps</div>\n<ol class=\"next-steps\">\n");
    if let Some(steps) = &page.next_steps {
        for (i, step) in steps.iter().enumerate() {
            out.push_str(&format!(
                r#"<li class="next-step">
  <div class="step-num">{n}</div>
  <div><div class="step-title">{title}</div><div class="step-detail">{detail}</div></div>
</li>
"#,
                n = i + 1,
                title = esc(&step.title),
                detail = esc(&step.detail),
            ));
        }
    }
    out.push_str("</ol>");
    out
}

const QBR_JS: &str = r#"<script>
(function() {
  var slides = ['Success Outcomes', 'Deployment Progress', 'Next Steps'];
  var current = 0;
  var track = document.querySelector('.qbr-track');
  var label = document.getElementById('qbr-label');
  var prevBtn = document.getElementById('qbr-prev');
  var nextBtn = document.getElementById('qbr-next');

  function go(n) {
    current = Math.max(0, Math.min(slides.length - 1, n));
    track.style.transform = 'translateX(-' + (current * 100) + '%)';
    label.textContent = slides[current];
    if (current === 0) {
      prevBtn.textContent = '';
      prevBtn.style.visibility = 'hidden';
    } else {
      prevBtn.textContent = '← ' + slides[current - 1];
      prevBtn.style.visibility = 'visible';
    }
    if (current === slides.length - 1) {
      nextBtn.textContent = '';
      nextBtn.style.visibility = 'hidden';
    } else {
      nextBtn.textContent = slides[current + 1] + ' →';
      nextBtn.style.visibility = 'visible';
    }
  }

  prevBtn.addEventListener('click', function() { go(current - 1); });
  nextBtn.addEventListener('click', function() { go(current + 1); });
  document.addEventListener('keydown', function(e) {
    if (e.key === 'ArrowRight' || e.key === 'ArrowDown') go(current + 1);
    if (e.key === 'ArrowLeft' || e.key === 'ArrowUp') go(current - 1);
  });

  go(0);
})();
</script>"#;
