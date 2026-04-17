use std::collections::HashMap;

/// A theme is a set of named color tokens. Any page rendered with this theme
/// gets its CSS `:root` block populated from these tokens; the rest of the CSS
/// references them via `var(--token)` so component styles are theme-agnostic.
#[derive(Clone)]
pub struct Theme {
    pub bg: String,
    pub surface: String,          // card backgrounds (low-contrast overlay)
    pub surface_strong: String,   // stronger surface (code, kbd)
    pub border: String,           // default border color
    pub border_strong: String,    // stronger/active border
    pub accent: String,           // primary brand color (teal by default)
    pub accent_soft: String,      // accent on a translucent background
    pub text: String,             // primary text
    pub text_muted: String,       // secondary text
    pub text_subtle: String,      // tertiary (labels, captions)
    pub overlay_hover: String,    // hover/active surface overlay
    pub green: String,
    pub yellow: String,
    pub red: String,
    pub header_border: String,
}

impl Theme {
    pub fn named(name: &str) -> Theme {
        match name {
            "light" => light(),
            _ => dark(),
        }
    }

    /// Apply a map of user overrides on top of this theme. Keys that don't
    /// match any known token are silently ignored.
    pub fn with_overrides(mut self, colors: &HashMap<String, String>) -> Theme {
        macro_rules! apply {
            ($key:literal, $field:ident) => {
                if let Some(v) = colors.get($key) { self.$field = v.clone(); }
            };
        }
        apply!("bg", bg);
        apply!("surface", surface);
        apply!("surface_strong", surface_strong);
        apply!("border", border);
        apply!("border_strong", border_strong);
        apply!("accent", accent);
        apply!("accent_soft", accent_soft);
        apply!("text", text);
        apply!("text_muted", text_muted);
        apply!("text_subtle", text_subtle);
        apply!("overlay_hover", overlay_hover);
        apply!("green", green);
        apply!("yellow", yellow);
        apply!("red", red);
        apply!("header_border", header_border);
        self
    }

    fn root_block(&self) -> String {
        let accent_rgb = hex_to_rgb_triple(&self.accent).unwrap_or_else(|| "60, 206, 206".into());
        let bg_rgb = hex_to_rgb_triple(&self.bg).unwrap_or_else(|| "9, 13, 24".into());
        let text_rgb = hex_to_rgb_triple(&self.text).unwrap_or_else(|| "255, 255, 255".into());
        format!(
            ":root {{\
             --bg: {bg};\
             --bg-rgb: {bg_rgb};\
             --card-bg: {surface};\
             --card-border: {border};\
             --card-hover-border: {border_strong};\
             --teal: {accent};\
             --accent-rgb: {accent_rgb};\
             --accent-soft: {accent_soft};\
             --snow: {text};\
             --text-rgb: {text_rgb};\
             --muted: {text_subtle};\
             --light-muted: {text_muted};\
             --header-border: {header_border};\
             --surface-strong: {surface_strong};\
             --overlay-hover: {overlay_hover};\
             --green: {green};\
             --yellow: {yellow};\
             --red: {red};\
             }}\n",
            bg = self.bg,
            bg_rgb = bg_rgb,
            surface = self.surface,
            surface_strong = self.surface_strong,
            border = self.border,
            border_strong = self.border_strong,
            accent = self.accent,
            accent_rgb = accent_rgb,
            accent_soft = self.accent_soft,
            text = self.text,
            text_rgb = text_rgb,
            text_muted = self.text_muted,
            text_subtle = self.text_subtle,
            overlay_hover = self.overlay_hover,
            green = self.green,
            yellow = self.yellow,
            red = self.red,
            header_border = self.header_border,
        )
    }
}

fn hex_to_rgb_triple(hex: &str) -> Option<String> {
    let h = hex.trim().trim_start_matches('#');
    if h.len() != 6 { return None; }
    let r = u8::from_str_radix(&h[0..2], 16).ok()?;
    let g = u8::from_str_radix(&h[2..4], 16).ok()?;
    let b = u8::from_str_radix(&h[4..6], 16).ok()?;
    Some(format!("{}, {}, {}", r, g, b))
}

pub fn dark() -> Theme {
    Theme {
        bg: "#121113".into(),
        surface: "rgba(var(--text-rgb), 0.03)".into(),
        surface_strong: "rgba(var(--text-rgb), 0.06)".into(),
        border: "rgba(var(--text-rgb), 0.07)".into(),
        border_strong: "rgba(var(--accent-rgb), 0.35)".into(),
        accent: "#899878".into(),
        accent_soft: "rgba(var(--accent-rgb), 0.08)".into(),
        text: "#F7F7F2".into(),
        text_muted: "#B0B3AD".into(),
        text_subtle: "#5C5F5A".into(),
        overlay_hover: "rgba(var(--text-rgb), 0.05)".into(),
        green: "#899878".into(),
        yellow: "#E4E6C3".into(),
        red: "#C97B8A".into(),
        header_border: "rgba(var(--accent-rgb), 0.15)".into(),
    }
}

pub fn light() -> Theme {
    Theme {
        bg: "#F7F7F2".into(),
        surface: "rgba(var(--text-rgb), 0.04)".into(),
        surface_strong: "rgba(var(--text-rgb), 0.08)".into(),
        border: "rgba(var(--text-rgb), 0.12)".into(),
        border_strong: "rgba(var(--accent-rgb), 0.45)".into(),
        accent: "#222725".into(),
        accent_soft: "rgba(var(--accent-rgb), 0.06)".into(),
        text: "#121113".into(),
        text_muted: "#3D423F".into(),
        text_subtle: "#899878".into(),
        overlay_hover: "rgba(var(--text-rgb), 0.06)".into(),
        green: "#5A7A4A".into(),
        yellow: "#9A9540".into(),
        red: "#8B4A5A".into(),
        header_border: "rgba(var(--accent-rgb), 0.25)".into(),
    }
}

pub fn render_css(theme: &Theme) -> String {
    let mut out = theme.root_block();
    out.push_str(STATIC_CSS);
    out
}

const STATIC_CSS: &str = r#"

* { margin: 0; padding: 0; box-sizing: border-box; }

body {
  background-color: var(--bg);
  color: var(--snow);
  font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
  line-height: 1.6;
}
body.shell-standard, body.shell-document { min-height: 100vh; }
body.shell-deck { height: 100vh; overflow: hidden; }

a { color: inherit; text-decoration: none; }
h1, h2, h3 { font-weight: 600; color: var(--snow); }

.container { max-width: 1200px; margin: 0 auto; padding: 0 40px; }

/* ──────────────────── Shared site bar ──────────────────── */

.site-bar {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 0 48px;
  height: 56px;
  flex-shrink: 0;
  border-bottom: 1.5px solid var(--teal);
  background: var(--bg);
}
.site-bar-name {
  font-size: 14px; font-weight: 500;
  opacity: 0.6;
  transition: opacity 0.15s, color 0.15s;
}
a.site-bar-name:hover { opacity: 1; color: var(--teal); }
.site-bar-divider { color: rgba(var(--text-rgb),0.15); font-size: 20px; font-weight: 300; }
.site-bar-eyebrow {
  font-size: 11px;
  font-weight: 700;
  color: var(--teal);
  text-transform: uppercase;
  letter-spacing: 1.5px;
}
.site-bar-right { display: flex; align-items: center; gap: 16px; margin-left: auto; }
.site-bar-subtitle { font-size: 12px; color: var(--muted); }
.site-bar-print-btn {
  font-size: 12px; font-weight: 500;
  color: var(--teal);
  border: 1px solid rgba(var(--accent-rgb), 0.3);
  border-radius: 6px;
  background: none;
  padding: 5px 12px;
  cursor: pointer;
  transition: all 0.15s;
}
.site-bar-print-btn:hover { background: rgba(var(--accent-rgb), 0.08); border-color: rgba(var(--accent-rgb), 0.6); }

.site-bar nav { display: flex; align-items: center; gap: 4px; }
.site-bar .nav-link {
  font-size: 13px; font-weight: 500;
  padding: 6px 12px;
  border-radius: 6px;
  color: rgba(var(--text-rgb), 0.55);
  transition: all 0.15s;
}
.site-bar .nav-link:hover { color: var(--snow); background: rgba(var(--text-rgb), 0.05); }
.site-bar .nav-link-active { color: var(--teal) !important; background: rgba(var(--accent-rgb), 0.08) !important; }

body.shell-standard .site-bar, body.shell-document .site-bar {
  position: sticky;
  top: 0;
  z-index: 10;
  background: rgba(var(--bg-rgb), 0.92);
  backdrop-filter: blur(12px);
}

/* ──────────────────── Standard shell ──────────────────── */

body.shell-standard .main-content { padding-top: 60px; padding-bottom: 100px; }

/* ──────────────────── Document shell ──────────────────── */

body.shell-document .doc-root {
  width: 100%;
  max-width: 720px;
  margin: 0 auto;
  padding: 40px 20px 100px;
}
body.shell-document .doc-card {
  background: rgba(var(--text-rgb), 0.02);
  border: 1px solid rgba(var(--text-rgb), 0.06);
  border-radius: 16px;
  padding: 40px 48px;
  box-shadow: 0 4px 40px rgba(0,0,0,0.3);
}
body.shell-document .doc-body { line-height: 1.7; color: rgba(var(--text-rgb),0.9); font-size: 15px; }
body.shell-document .doc-footer {
  margin-top: 40px;
  padding-top: 20px;
  border-top: 1px solid rgba(var(--text-rgb),0.05);
}

/* ──────────────────── Deck shell ──────────────────── */

body.shell-deck .deck-root {
  position: fixed;
  inset: 0;
  display: flex;
  flex-direction: column;
  background: var(--bg);
}

body.shell-deck .deck-viewport { flex: 1; overflow: hidden; position: relative; }
body.shell-deck .deck-track {
  display: flex;
  height: 100%;
  transition: transform 0.4s cubic-bezier(0.4, 0, 0.2, 1);
}
body.shell-deck .deck-slide { min-width: 100%; height: 100%; overflow-y: auto; }
body.shell-deck .deck-inner {
  max-width: 900px;
  margin: 0 auto;
  padding: 48px 48px 80px;
  display: flex;
  flex-direction: column;
  min-height: 100%;
  gap: 20px;
}
body.shell-deck .deck-label {
  font-size: 13px;
  font-weight: 600;
  color: var(--teal);
  text-transform: uppercase;
  letter-spacing: 2px;
  margin-bottom: 12px;
}
body.shell-deck .deck-nav {
  height: 52px;
  flex-shrink: 0;
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 48px;
  border-top: 1px solid rgba(var(--text-rgb),0.05);
}
body.shell-deck .deck-nav-label {
  font-size: 11px;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 1.5px;
  color: rgba(var(--text-rgb),0.25);
}
body.shell-deck .deck-arrow {
  font-size: 12px; font-weight: 500;
  color: rgba(var(--text-rgb),0.4);
  background: none; border: none;
  cursor: pointer;
  padding: 4px 0;
  transition: color 0.15s;
  min-width: 200px;
}
body.shell-deck .deck-prev { text-align: left; }
body.shell-deck .deck-next { text-align: right; }
body.shell-deck .deck-arrow:hover { color: var(--teal); }

/* ──────────────────── Components ──────────────────── */

/* — stack spacing for components in main flow — */
.main-content > *, .deck-inner > *, .doc-body > *, .c-section > *:not(.c-section-header), .tab-panel > * {
  margin-bottom: 32px;
}
.main-content > *:last-child, .deck-inner > *:last-child, .doc-body > *:last-child { margin-bottom: 0; }

/* Header component */
.c-header { }
.c-header-eyebrow {
  font-size: 11px;
  font-weight: 600;
  color: var(--teal);
  margin-bottom: 6px;
  text-transform: uppercase;
  letter-spacing: 1px;
}
.c-header-title { font-size: 24px; font-weight: 700; margin-bottom: 8px; }
.c-header-subtitle { font-size: 14px; color: var(--light-muted); }

/* Meta */
.c-meta {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
  gap: 1px;
  background: var(--card-border);
  border: 1px solid var(--card-border);
  border-radius: 10px;
  overflow: hidden;
}
.c-meta-item { background: var(--card-bg); padding: 16px 20px; display: flex; flex-direction: column; gap: 4px; }
.c-meta-key {
  font-size: 11px; font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.5px;
  color: var(--muted);
}
.c-meta-value { font-size: 15px; font-weight: 500; color: var(--snow); }

/* Card Grid */
.c-card-grid { display: grid; gap: 20px; }
.c-card {
  background: var(--card-bg);
  border: 1px solid var(--card-border);
  border-radius: 12px;
  padding: 24px;
  display: flex;
  flex-direction: column;
  gap: 16px;
  transition: border-color 0.2s;
}
a.c-card { color: inherit; }
.c-card:hover { border-color: var(--card-hover-border); }
.c-card-top { display: flex; justify-content: space-between; align-items: flex-start; gap: 12px; }
.c-card-title { font-size: 18px; font-weight: 600; }
.c-card-desc { font-size: 14px; color: var(--light-muted); line-height: 1.5; }
.c-card-links { display: flex; gap: 8px; flex-wrap: wrap; margin-top: auto; }
.c-card-link {
  display: inline-flex; align-items: center;
  padding: 6px 12px;
  border-radius: 6px;
  font-size: 12px; font-weight: 500;
  color: var(--teal);
  border: 1px solid rgba(var(--accent-rgb), 0.2);
  background: rgba(var(--accent-rgb), 0.05);
  transition: background 0.15s;
}
.c-card-link:hover { background: rgba(var(--accent-rgb), 0.1); }

/* Badges */
.c-badge {
  font-size: 11px; font-weight: 600;
  padding: 4px 10px;
  border-radius: 100px;
  white-space: nowrap;
  background: rgba(var(--text-rgb), 0.06);
  opacity: 0.8;
}
.c-badge-default { background: rgba(var(--text-rgb), 0.06); color: var(--snow); opacity: 0.8; }
.c-badge-green { background: rgba(52, 211, 153, 0.12); color: var(--green); opacity: 1; }
.c-badge-yellow { background: rgba(251, 191, 36, 0.12); color: var(--yellow); opacity: 1; }
.c-badge-red { background: rgba(248, 113, 113, 0.12); color: var(--red); opacity: 1; }
.c-badge-teal { background: rgba(var(--accent-rgb), 0.12); color: var(--teal); opacity: 1; }

/* Selectable Grid */
.c-selectable-grid { position: relative; }
.c-sel-dots-row {
  position: relative;
  display: flex;
  justify-content: space-around;
  align-items: center;
  margin-bottom: 32px;
  padding: 20px 0;
}
.c-sel-dots-line {
  position: absolute;
  top: 50%;
  left: calc(100% / 8);
  right: calc(100% / 8);
  height: 1px;
  background: rgba(var(--accent-rgb), 0.25);
  transform: translateY(-50%);
  pointer-events: none;
}
.sel-dot {
  width: 40px; height: 40px;
  border-radius: 50%;
  border: 2px solid rgba(var(--text-rgb), 0.12);
  background: var(--bg);
  color: rgba(var(--text-rgb), 0.4);
  font-size: 13px; font-weight: 700;
  cursor: pointer;
  display: flex; align-items: center; justify-content: center;
  transition: all 0.2s;
  position: relative;
  z-index: 1;
}
.sel-dot:hover { border-color: rgba(var(--accent-rgb), 0.5); color: var(--teal); }
.sel-dot.sel-active {
  border-color: var(--teal);
  background: rgba(14, 42, 42, 1);
  color: var(--teal);
}
.c-sel-cards { display: grid; gap: 16px; }
.sel-card {
  text-align: left;
  background: rgba(var(--text-rgb), 0.02);
  border: 1px solid rgba(var(--text-rgb), 0.07);
  border-radius: 12px;
  padding: 24px 20px;
  cursor: pointer;
  transition: all 0.2s;
  display: flex; flex-direction: column; gap: 16px;
  font: inherit; color: inherit;
}
.sel-card:hover { border-color: rgba(var(--text-rgb), 0.14); }
.sel-card.sel-active {
  background: rgba(var(--accent-rgb), 0.06);
  border-color: rgba(var(--accent-rgb), 0.35);
}
.sel-card.sel-dimmed { opacity: 0.35; }
.c-sel-eyebrow {
  font-size: 11px; font-weight: 600;
  color: var(--teal);
  opacity: 0.7;
  letter-spacing: 0.5px;
  text-transform: uppercase;
}
.c-sel-title { font-size: 15px; font-weight: 600; color: var(--snow); line-height: 1.3; }
.c-sel-bullets { list-style: none; display: flex; flex-direction: column; gap: 10px; }
.c-sel-bullets li { display: flex; gap: 10px; align-items: flex-start; }
.c-sel-bullet-dot {
  width: 5px; height: 5px;
  border-radius: 50%;
  background: var(--teal);
  opacity: 0.5;
  flex-shrink: 0;
  margin-top: 7px;
}
.c-sel-bullets span:last-child { font-size: 13px; color: rgba(var(--text-rgb), 0.65); line-height: 1.5; }
.c-sel-body { font-size: 13px; color: rgba(var(--text-rgb), 0.65); line-height: 1.5; }

/* Timeline */
.c-timeline { display: flex; }
.c-timeline-phase { flex: 1; text-align: center; padding: 12px 6px 0; }
.c-timeline-dot {
  width: 8px; height: 8px;
  border-radius: 50%;
  margin: 0 auto 8px;
  background: rgba(240,240,247,0.1);
}
.c-timeline-phase.completed .c-timeline-dot { background: var(--green); }
.c-timeline-phase.active .c-timeline-dot { background: var(--teal); box-shadow: 0 0 8px rgba(var(--accent-rgb),0.5); }
.c-timeline-label {
  font-size: 11px; font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.6px;
  color: rgba(240,240,247,0.25);
  margin-bottom: 10px;
}
.c-timeline-phase.completed .c-timeline-label { color: var(--green); }
.c-timeline-phase.active .c-timeline-label { color: var(--teal); }
.c-timeline-bar { height: 3px; background: rgba(240,240,247,0.06); }
.c-timeline-bar.completed { background: var(--green); }
.c-timeline-bar.active { background: var(--teal); }

/* Stat Grid */
.c-stat-grid { display: grid; gap: 14px; }
.c-stat {
  background: rgba(var(--text-rgb),0.025);
  border: 1px solid rgba(240,240,247,0.06);
  border-radius: 12px;
  padding: 20px 22px;
  position: relative;
  overflow: hidden;
  display: flex; flex-direction: column; gap: 6px;
}
.c-stat::before {
  content: '';
  position: absolute;
  top: 0; left: 0; right: 0;
  height: 3px;
  background: var(--stat-color);
}
.c-stat-label {
  font-size: 11px; font-weight: 600;
  color: rgba(240,240,247,0.35);
  text-transform: uppercase;
  letter-spacing: 1px;
}
.c-stat-value { font-size: 28px; font-weight: 700; line-height: 1.1; }
.c-stat-detail { font-size: 13px; color: rgba(240,240,247,0.4); line-height: 1.4; }

/* Before / After */
.c-before-after { display: flex; flex-direction: column; gap: 20px; }
.c-ba-card {
  padding: 32px 36px;
  background: rgba(var(--text-rgb),0.02);
  border: 1px solid rgba(var(--accent-rgb),0.08);
  border-radius: 14px;
  display: flex; flex-direction: column; gap: 12px;
}
.c-ba-title { font-size: 22px; font-weight: 700; }
.c-ba-before { font-size: 16px; color: rgba(240,240,247,0.35); line-height: 1.5; }
.c-ba-after { font-size: 16px; color: rgba(240,240,247,0.7); line-height: 1.5; }
.c-ba-highlight { color: var(--teal); font-weight: 600; }

/* Steps */
.c-steps { list-style: none; display: flex; flex-direction: column; gap: 12px; }
.c-step {
  display: flex;
  align-items: flex-start;
  gap: 16px;
  padding: 20px 24px;
  background: rgba(var(--text-rgb),0.02);
  border: 1px solid rgba(var(--accent-rgb),0.08);
  border-radius: 12px;
}
.c-step-num {
  width: 24px; height: 24px;
  border-radius: 6px;
  flex-shrink: 0;
  background: rgba(var(--accent-rgb),0.12);
  color: var(--teal);
  font-size: 11px; font-weight: 700;
  display: flex; align-items: center; justify-content: center;
  margin-top: 2px;
}
.c-step-bullet {
  width: 6px; height: 6px;
  border-radius: 50%;
  background: var(--teal);
  flex-shrink: 0;
  margin-top: 10px;
  opacity: 0.6;
}
.c-step-title { font-size: 17px; font-weight: 600; margin-bottom: 4px; }
.c-step-detail { font-size: 14px; color: rgba(240,240,247,0.5); line-height: 1.5; }

/* Markdown */
.c-markdown {
  color: var(--light-muted);
  font-size: 15px;
  line-height: 1.75;
}
.c-markdown h1, .c-markdown h2, .c-markdown h3 { color: var(--snow); margin-top: 2em; margin-bottom: 0.75em; }
.c-markdown h1 { font-size: 24px; }
.c-markdown h2 { font-size: 18px; }
.c-markdown h3 { font-size: 16px; color: var(--teal); }
.c-markdown h1:first-child, .c-markdown h2:first-child, .c-markdown h3:first-child { margin-top: 0; }
.c-markdown p { margin-bottom: 1em; }
.c-markdown ul, .c-markdown ol { padding-left: 1.5em; margin-bottom: 1em; }
.c-markdown li { margin-bottom: 0.4em; }
.c-markdown strong { color: var(--snow); font-weight: 600; }
.c-markdown code {
  font-family: 'SF Mono', 'Monaco', monospace;
  font-size: 13px;
  background: rgba(var(--text-rgb), 0.06);
  padding: 2px 6px;
  border-radius: 4px;
  color: var(--teal);
}
.c-markdown pre {
  background: rgba(var(--text-rgb), 0.04);
  border: 1px solid var(--card-border);
  border-radius: 8px;
  padding: 20px;
  overflow-x: auto;
  margin-bottom: 1.5em;
}
.c-markdown pre code { background: none; padding: 0; color: var(--snow); }
.c-markdown table { width: 100%; border-collapse: collapse; margin-bottom: 1.5em; }
.c-markdown th {
  text-align: left;
  font-size: 11px; font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.5px;
  color: var(--muted);
  padding: 10px 16px;
  border-bottom: 1px solid var(--card-border);
}
.c-markdown td {
  padding: 12px 16px;
  border-bottom: 1px solid rgba(var(--text-rgb), 0.04);
  color: var(--light-muted);
}
.c-markdown tr:last-child td { border-bottom: none; }
.c-markdown a { color: var(--teal); }
.c-markdown a:hover { text-decoration: underline; }
.c-markdown blockquote {
  border-left: 3px solid rgba(var(--accent-rgb), 0.3);
  padding-left: 20px;
  color: var(--muted);
  font-style: italic;
  margin-bottom: 1em;
}

/* Document body inherits markdown styling for h3 in teal etc. */
body.shell-document .doc-body h3 { color: var(--teal); }
body.shell-document .doc-body h1 { font-size: 24px; margin-bottom: 20px; }
body.shell-document .doc-body h2 { font-size: 18px; margin: 24px 0 12px; }
body.shell-document .doc-body h3 { font-size: 16px; margin: 20px 0 10px; }
body.shell-document .doc-body p { margin-bottom: 12px; }
body.shell-document .doc-body strong { color: #fff; }

/* Table */
.c-table-wrap { display: flex; flex-direction: column; gap: 12px; }
.c-table-filter {
  padding: 10px 14px;
  border: 1px solid var(--card-border);
  border-radius: 8px;
  background: rgba(var(--text-rgb),0.02);
  color: var(--snow);
  font-size: 14px;
  font-family: inherit;
  max-width: 320px;
}
.c-table-filter:focus { outline: none; border-color: var(--card-hover-border); }
.c-table {
  width: 100%;
  border-collapse: collapse;
  background: rgba(var(--text-rgb),0.02);
  border: 1px solid var(--card-border);
  border-radius: 10px;
  overflow: hidden;
}
.c-table th {
  text-align: left;
  font-size: 11px; font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.5px;
  color: var(--muted);
  padding: 14px 18px;
  border-bottom: 1px solid var(--card-border);
  background: rgba(var(--text-rgb),0.02);
  user-select: none;
}
.c-table th[data-sortable] { cursor: pointer; transition: color 0.15s; }
.c-table th[data-sortable]:hover { color: var(--snow); }
.c-table th[data-sortable]::after {
  content: ' ↕';
  opacity: 0.3;
  font-size: 10px;
}
.c-table th.sort-asc::after { content: ' ↑'; opacity: 1; color: var(--teal); }
.c-table th.sort-desc::after { content: ' ↓'; opacity: 1; color: var(--teal); }
.c-table td {
  padding: 14px 18px;
  border-bottom: 1px solid rgba(var(--text-rgb), 0.04);
  color: var(--light-muted);
  font-size: 14px;
}
.c-table tr:last-child td { border-bottom: none; }
.c-table .align-left { text-align: left; }
.c-table .align-right { text-align: right; }
.c-table .align-center { text-align: center; }

/* Callout */
.c-callout {
  padding: 16px 20px;
  border-radius: 0 10px 10px 0;
  border-left: 3px solid;
  background: rgba(var(--text-rgb), 0.05);
}
.c-callout-info { border-left-color: var(--teal); }
.c-callout-warn { border-left-color: var(--yellow); background: rgba(251, 191, 36, 0.04); }
.c-callout-success { border-left-color: var(--green); background: rgba(52, 211, 153, 0.04); }
.c-callout-danger { border-left-color: var(--red); background: rgba(248, 113, 113, 0.04); }
.c-callout-title { font-weight: 600; margin-bottom: 4px; color: var(--snow); }
.c-callout-info .c-callout-title { color: var(--teal); }
.c-callout-warn .c-callout-title { color: var(--yellow); }
.c-callout-success .c-callout-title { color: var(--green); }
.c-callout-danger .c-callout-title { color: var(--red); }
.c-callout-body { font-size: 14px; color: var(--light-muted); line-height: 1.6; }
.c-callout-body > *:last-child { margin-bottom: 0 !important; }
.c-callout-body p { margin-bottom: 0.5em; }
.c-callout-body a { color: var(--teal); text-decoration: underline; text-decoration-color: rgba(var(--accent-rgb), 0.3); text-underline-offset: 2px; }
.c-callout-body a:hover { text-decoration-color: var(--teal); }
.c-callout-links { margin-top: 12px; }

/* Code */
.c-code {
  background: rgba(var(--text-rgb), 0.04);
  border: 1px solid var(--card-border);
  border-radius: 8px;
  padding: 20px;
  overflow-x: auto;
  font-family: 'SF Mono', 'Monaco', monospace;
  font-size: 13px;
  color: var(--snow);
  line-height: 1.6;
}

/* Tabs */
.c-tabs { }
.c-tab-buttons {
  display: flex;
  gap: 4px;
  border-bottom: 1px solid var(--card-border);
  margin-bottom: 24px;
}
.tab-btn {
  font: inherit;
  font-size: 13px; font-weight: 500;
  padding: 10px 16px;
  color: rgba(var(--text-rgb), 0.5);
  background: none;
  border: none;
  border-bottom: 2px solid transparent;
  cursor: pointer;
  margin-bottom: -1px;
  transition: all 0.15s;
}
.tab-btn:hover { color: var(--snow); }
.tab-btn.tab-btn-active { color: var(--teal); border-bottom-color: var(--teal); }
.tab-panel { }

/* Section */
.c-section { }
.c-section-header { margin-bottom: 24px; }
.c-section-eyebrow {
  font-size: 11px;
  font-weight: 600;
  color: var(--teal);
  text-transform: uppercase;
  letter-spacing: 1px;
  margin-bottom: 6px;
}
.c-section-heading { font-size: 18px; font-weight: 600; }

/* Columns */
.c-columns { display: grid; gap: 24px; }
.c-column { display: flex; flex-direction: column; gap: 20px; }
.c-column > *:last-child { margin-bottom: 0; }
.c-columns-stretch .c-column > * { flex: 1; }

/* Accordion */
.c-accordion { display: flex; flex-direction: column; gap: 8px; }
.c-accordion-item {
  background: rgba(var(--text-rgb),0.02);
  border: 1px solid var(--card-border);
  border-radius: 10px;
  overflow: hidden;
}
.accordion-head {
  width: 100%;
  padding: 14px 20px;
  text-align: left;
  background: none;
  border: none;
  color: var(--snow);
  font: inherit;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  display: flex;
  justify-content: space-between;
  align-items: center;
  transition: color 0.15s;
}
.accordion-head:hover { color: var(--teal); }
.accordion-chevron {
  font-size: 18px;
  opacity: 0.5;
  transition: transform 0.2s;
}
.c-accordion-item.accordion-open .accordion-chevron { transform: rotate(90deg); color: var(--teal); opacity: 1; }
.accordion-body {
  padding: 0 20px 20px;
  border-top: 1px solid var(--card-border);
  padding-top: 16px;
}

/* Image */
.c-image { margin: 0; }
.c-image img { width: 100%; height: auto; border-radius: 8px; display: block; }
.c-image figcaption {
  font-size: 13px;
  color: var(--muted);
  margin-top: 8px;
  text-align: center;
}

/* ──────────────────── Phase 1 additions ──────────────────── */

/* Tag (pill-shaped decorative) */
.c-tag {
  display: inline-flex; align-items: center;
  font-size: 12px; font-weight: 500;
  padding: 3px 10px;
  border-radius: 6px;
  background: rgba(var(--text-rgb), 0.04);
  border: 1px solid rgba(var(--text-rgb), 0.08);
  color: var(--light-muted);
  font-family: 'SF Mono', 'Monaco', monospace;
}
.c-tag-green { background: rgba(52, 211, 153, 0.08); border-color: rgba(52, 211, 153, 0.25); color: var(--green); }
.c-tag-yellow { background: rgba(251, 191, 36, 0.08); border-color: rgba(251, 191, 36, 0.25); color: var(--yellow); }
.c-tag-red { background: rgba(248, 113, 113, 0.08); border-color: rgba(248, 113, 113, 0.25); color: var(--red); }
.c-tag-teal { background: rgba(var(--accent-rgb), 0.08); border-color: rgba(var(--accent-rgb), 0.25); color: var(--teal); }

/* Divider */
.c-divider {
  border: none;
  border-top: 1px solid var(--card-border);
  width: 100%;
  margin: 0;
}
.c-divider-labeled {
  display: flex; align-items: center; gap: 16px;
}
.c-divider-labeled .c-divider-line {
  flex: 1;
  height: 1px;
  background: var(--card-border);
}
.c-divider-label {
  font-size: 12px; font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 1px;
  color: var(--muted);
}

/* Kbd */
.c-kbd-group { display: inline-flex; align-items: center; gap: 4px; }
.c-kbd {
  display: inline-block;
  font-family: 'SF Mono', 'Monaco', monospace;
  font-size: 11px; font-weight: 600;
  padding: 2px 6px;
  border-radius: 4px;
  background: rgba(var(--text-rgb), 0.08);
  border: 1px solid rgba(var(--text-rgb), 0.12);
  border-bottom-width: 2px;
  color: var(--snow);
  line-height: 1;
}
.c-kbd-sep { font-size: 11px; color: var(--muted); }

/* Status */
.c-status {
  display: inline-flex; align-items: center; gap: 8px;
  font-size: 13px; font-weight: 500;
}
.c-status-dot {
  width: 8px; height: 8px;
  border-radius: 50%;
  background: rgba(var(--text-rgb), 0.3);
}
.c-status-green .c-status-dot { background: var(--green); box-shadow: 0 0 0 3px rgba(52, 211, 153, 0.15); }
.c-status-yellow .c-status-dot { background: var(--yellow); box-shadow: 0 0 0 3px rgba(251, 191, 36, 0.15); }
.c-status-red .c-status-dot { background: var(--red); box-shadow: 0 0 0 3px rgba(248, 113, 113, 0.15); }
.c-status-teal .c-status-dot { background: var(--teal); box-shadow: 0 0 0 3px rgba(var(--accent-rgb), 0.15); }

/* Breadcrumb */
.c-breadcrumb ol {
  list-style: none;
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  font-size: 13px;
}
.c-breadcrumb-item a { color: var(--light-muted); transition: color 0.15s; }
.c-breadcrumb-item a:hover { color: var(--teal); }
.c-breadcrumb-item span[aria-current="page"] { color: var(--snow); font-weight: 500; }
.c-breadcrumb-sep { color: var(--muted); opacity: 0.5; }

/* Button Group */
.c-button-group {
  display: flex; gap: 10px; flex-wrap: wrap;
}
.c-button {
  display: inline-flex; align-items: center; gap: 8px;
  font-size: 13px; font-weight: 500;
  padding: 8px 16px;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.15s;
  text-decoration: none;
  line-height: 1.4;
}
.c-button-icon { display: inline-flex; align-items: center; }
.c-button-primary {
  background: var(--teal);
  color: var(--bg);
  border: 1px solid var(--teal);
}
.c-button-primary:hover { background: #5fdada; border-color: #5fdada; }
.c-button-secondary {
  background: transparent;
  color: var(--teal);
  border: 1px solid rgba(var(--accent-rgb), 0.3);
}
.c-button-secondary:hover { background: rgba(var(--accent-rgb), 0.08); border-color: rgba(var(--accent-rgb), 0.5); }
.c-button-ghost {
  background: transparent;
  color: var(--light-muted);
  border: 1px solid transparent;
}
.c-button-ghost:hover { color: var(--snow); background: rgba(var(--text-rgb), 0.04); }

/* Definition List */
.c-definition-list {
  display: flex; flex-direction: column;
  border: 1px solid var(--card-border);
  border-radius: 10px;
  overflow: hidden;
}
.c-dl-row {
  display: grid;
  grid-template-columns: 200px 1fr;
  gap: 20px;
  padding: 16px 20px;
  border-bottom: 1px solid var(--card-border);
  background: var(--card-bg);
}
.c-dl-row:last-child { border-bottom: none; }
.c-dl-term {
  font-weight: 600;
  color: var(--snow);
  font-size: 14px;
}
.c-dl-def {
  margin: 0;
  color: var(--light-muted);
  font-size: 14px;
  line-height: 1.6;
}
@media (max-width: 640px) {
  .c-dl-row { grid-template-columns: 1fr; gap: 4px; }
}

/* Blockquote */
.c-blockquote {
  margin: 0;
  padding: 24px 32px;
  background: rgba(var(--accent-rgb), 0.03);
  border-left: 3px solid var(--teal);
  border-radius: 0 10px 10px 0;
}
.c-blockquote blockquote {
  margin: 0;
  border: none;
  padding: 0;
}
.c-blockquote blockquote p {
  font-size: 18px;
  font-style: italic;
  color: var(--snow);
  line-height: 1.6;
  margin: 0;
}
.c-blockquote-attribution {
  margin-top: 12px;
  font-size: 13px;
  color: var(--light-muted);
  font-style: normal;
}

/* Avatar */
.c-avatar {
  display: inline-flex; align-items: center; justify-content: center;
  border-radius: 50%;
  background: rgba(var(--accent-rgb), 0.12);
  color: var(--teal);
  font-weight: 600;
  flex-shrink: 0;
  overflow: hidden;
  border: 1px solid rgba(var(--accent-rgb), 0.2);
}
.c-avatar img { width: 100%; height: 100%; object-fit: cover; display: block; }
.c-avatar-sm { width: 24px; height: 24px; font-size: 10px; }
.c-avatar-md { width: 40px; height: 40px; font-size: 13px; }
.c-avatar-lg { width: 56px; height: 56px; font-size: 18px; }
.c-avatar-xl { width: 80px; height: 80px; font-size: 24px; }
.c-avatar-row { display: inline-flex; align-items: center; gap: 12px; }
.c-avatar-meta { display: flex; flex-direction: column; gap: 2px; }
.c-avatar-name { font-size: 14px; font-weight: 600; color: var(--snow); line-height: 1.3; }
.c-avatar-sub { font-size: 12px; color: var(--muted); line-height: 1.3; }

/* Avatar Group */
.c-avatar-group { display: inline-flex; align-items: center; }
.c-avatar-group .c-avatar {
  margin-left: -8px;
  border: 2px solid var(--bg);
  box-sizing: content-box;
}
.c-avatar-group .c-avatar:first-child { margin-left: 0; }
.c-avatar-more {
  background: rgba(var(--text-rgb), 0.06) !important;
  color: var(--light-muted) !important;
  border-color: rgba(var(--text-rgb), 0.1) !important;
}

/* Progress Bar */
.c-progress { display: flex; flex-direction: column; gap: 8px; }
.c-progress-labels {
  display: flex; justify-content: space-between; align-items: baseline;
  font-size: 13px;
}
.c-progress-label { color: var(--light-muted); font-weight: 500; }
.c-progress-value { color: var(--snow); font-weight: 600; font-variant-numeric: tabular-nums; }
.c-progress-track {
  height: 8px;
  background: rgba(var(--text-rgb), 0.06);
  border-radius: 100px;
  overflow: hidden;
}
.c-progress-fill {
  height: 100%;
  border-radius: 100px;
  transition: width 0.3s ease;
}
.c-progress-fill-default { background: var(--teal); }
.c-progress-fill-green { background: var(--green); }
.c-progress-fill-yellow { background: var(--yellow); }
.c-progress-fill-red { background: var(--red); }
.c-progress-fill-teal { background: var(--teal); }
.c-progress-detail { font-size: 12px; color: var(--muted); }

/* Empty State */
.c-empty-state {
  text-align: center;
  padding: 48px 24px;
  background: var(--card-bg);
  border: 1px dashed var(--card-border);
  border-radius: 12px;
  display: flex; flex-direction: column; align-items: center; gap: 12px;
}
.c-empty-state-icon {
  color: var(--muted);
  margin-bottom: 4px;
}
.c-empty-state-title {
  font-size: 16px; font-weight: 600;
  color: var(--snow);
  margin: 0;
}
.c-empty-state-body {
  font-size: 14px;
  color: var(--light-muted);
  margin: 0;
  max-width: 400px;
}
.c-empty-state .c-button { margin-top: 4px; }

/* Icon component (standalone) */
/* SVG styling is handled via the render; no extra CSS needed */

/* View source link (bottom-right floating pill) */
.view-source {
  position: fixed;
  bottom: 20px;
  right: 20px;
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 8px 14px;
  font-size: 12px;
  font-weight: 500;
  color: var(--light-muted);
  background: var(--card-bg);
  border: 1px solid var(--card-border);
  border-radius: 100px;
  text-decoration: none;
  backdrop-filter: blur(8px);
  transition: all 0.15s;
  z-index: 50;
}
.view-source:hover {
  color: var(--teal);
  border-color: var(--card-hover-border);
}
@media print {
  .view-source { display: none !important; }
}

/* ──────────────────── Print ──────────────────── */

/* Default @page — portrait content pages; deck forces landscape full-bleed */
@page { margin: 0.5in; }
@page deck-page { size: landscape; margin: 0; }
body.shell-deck { page: deck-page; }

@media print {
  .no-print { display: none !important; }
  .view-source { display: none !important; }

  /* ── Shared: preserve accent colors, drop the site bar ── */
  *, *::before, *::after { -webkit-print-color-adjust: exact !important; print-color-adjust: exact !important; }
  .site-bar { display: none !important; }

  /* ── Standard shell ── (dark theme preserved) ── */
  html, body.shell-standard { background: var(--bg) !important; }
  body.shell-standard { min-height: auto !important; }
  body.shell-standard .main-content { padding-top: 20px !important; padding-bottom: 20px !important; }
  /* Avoid awkward page breaks inside structured blocks */
  body.shell-standard .c-card,
  body.shell-standard .c-stat,
  body.shell-standard .c-callout,
  body.shell-standard .c-step,
  body.shell-standard .c-ba-card,
  body.shell-standard .c-meta-item,
  body.shell-standard .c-empty-state { break-inside: avoid; page-break-inside: avoid; }
  body.shell-standard h1, body.shell-standard h2, body.shell-standard h3 { break-after: avoid; page-break-after: avoid; }

  /* ── Document shell ── (clean white-paper print) ── */
  body.shell-document { background: #fff !important; color: #000 !important; min-height: auto !important; }
  body.shell-document .doc-root { padding: 0 !important; max-width: 100% !important; }
  body.shell-document .doc-card { box-shadow: none !important; border: none !important; background: transparent !important; max-width: 100% !important; padding: 0 !important; }
  body.shell-document .doc-body { color: #000 !important; }
  body.shell-document .doc-body strong { color: #000 !important; }
  body.shell-document .doc-body h1, body.shell-document .doc-body h2 { color: #000 !important; }
  body.shell-document .c-step,
  body.shell-document .c-callout,
  body.shell-document .c-card { break-inside: avoid; page-break-inside: avoid; }

  /* ── Deck shell ── (already working — landscape full-bleed via @page deck-page) ── */
  html:has(body.shell-deck), body.shell-deck { background: var(--bg) !important; }
  body.shell-deck { height: auto !important; overflow: visible !important; }
  body.shell-deck .deck-root { position: static !important; height: auto !important; }
  body.shell-deck .deck-viewport { overflow: visible !important; height: auto !important; }
  body.shell-deck .deck-track { transform: none !important; flex-direction: column !important; height: auto !important; }
  body.shell-deck .deck-slide { min-width: 100% !important; height: auto !important; page-break-after: always; overflow: visible !important; }
  body.shell-deck .deck-slide:last-child { page-break-after: avoid; }
  body.shell-deck .deck-inner { min-height: 0 !important; padding: 0.4in 0.5in !important; }
  body.shell-deck .deck-nav { display: none !important; }
}
"#;
