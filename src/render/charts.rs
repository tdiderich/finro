//! Build-time SVG chart renderer. Three kinds — pie, bar, timeseries — with an
//! optional second dimension via `series:`. No JS, no runtime deps.
//!
//! Chart colors use the canonical `SemColor` hexes (teal/green/yellow/red)
//! rather than the theme's CSS vars on purpose: themes like `dark` remap
//! `--teal` and `--green` onto the same sage tone, which destroys stack
//! contrast. Charts must stay distinguishable across every theme, so they
//! bypass theme overrides even when the user writes `color: green`.

use super::{esc, Rendered};
use crate::types::{ChartKind, ChartOrientation, ChartPoint, ChartSeries, SemColor};

const VB_W: f64 = 720.0;

/// Bundle of chart render inputs. Mirrors the `Component::Chart` fields —
/// passed as one arg so the entry point isn't a 7-positional-parameter blob.
pub struct ChartSpec<'a> {
    pub kind: ChartKind,
    pub title: &'a Option<String>,
    pub height: Option<u32>,
    pub x_label: &'a Option<String>,
    pub y_label: &'a Option<String>,
    pub orientation: ChartOrientation,
    pub data: &'a Option<Vec<ChartPoint>>,
    pub series: &'a Option<Vec<ChartSeries>>,
}

pub fn render(spec: ChartSpec<'_>) -> Rendered {
    let series_vec = coerce_series(spec.data, spec.series);
    let h = spec.height.unwrap_or_else(|| default_height(spec.kind));
    let aria = spec
        .title
        .clone()
        .unwrap_or_else(|| default_aria(spec.kind));

    let svg = match spec.kind {
        ChartKind::Pie => render_pie(&series_vec, h),
        ChartKind::Bar => render_bar(&series_vec, h, spec.orientation, spec.x_label, spec.y_label),
        ChartKind::Timeseries => render_timeseries(&series_vec, h, spec.x_label, spec.y_label),
    };

    let title_html = spec
        .title
        .as_deref()
        .filter(|s| !s.is_empty())
        .map(|t| {
            format!(
                r#"<figcaption class="c-chart-title">{}</figcaption>"#,
                esc(t)
            )
        })
        .unwrap_or_default();

    let legend = render_legend(spec.kind, &series_vec);

    let html = format!(
        r#"<figure class="c-chart c-chart-{k}" role="img" aria-label="{aria}">{title}{svg}{legend}</figure>"#,
        k = kind_class(spec.kind),
        aria = esc(&aria),
        title = title_html,
        svg = svg,
        legend = legend,
    );

    Rendered::new(html)
}

fn kind_class(k: ChartKind) -> &'static str {
    match k {
        ChartKind::Pie => "pie",
        ChartKind::Bar => "bar",
        ChartKind::Timeseries => "timeseries",
    }
}

fn default_height(k: ChartKind) -> u32 {
    match k {
        ChartKind::Pie => 280,
        ChartKind::Bar => 300,
        ChartKind::Timeseries => 280,
    }
}

fn default_aria(k: ChartKind) -> String {
    match k {
        ChartKind::Pie => "Pie chart".into(),
        ChartKind::Bar => "Bar chart".into(),
        ChartKind::Timeseries => "Time series chart".into(),
    }
}

/// Normalize `data:` / `series:` into a single `Vec<Series>` so the renderers
/// only ever deal with one shape. A lone `data:` becomes a nameless series of
/// index 0; a `series:` list passes through as-is.
struct NormSeries<'a> {
    label: &'a str,
    color: Option<SemColor>,
    points: &'a [ChartPoint],
}

fn coerce_series<'a>(
    data: &'a Option<Vec<ChartPoint>>,
    series: &'a Option<Vec<ChartSeries>>,
) -> Vec<NormSeries<'a>> {
    if let Some(s) = series {
        return s
            .iter()
            .map(|s| NormSeries {
                label: &s.label,
                color: s.color,
                points: &s.points,
            })
            .collect();
    }
    if let Some(d) = data {
        return vec![NormSeries {
            label: "",
            color: None,
            points: d,
        }];
    }
    Vec::new()
}

fn series_color(series_color: Option<SemColor>, idx: usize) -> &'static str {
    let c = series_color.unwrap_or_else(|| cycle_color(idx));
    c.hex()
}

/// Default palette for multi-series charts when the author hasn't set colors.
/// Skips the `Default` alias (which is the same hex as `Teal`) so four
/// consecutive series land on four distinct tones.
fn cycle_color(idx: usize) -> SemColor {
    match idx % 4 {
        0 => SemColor::Teal,
        1 => SemColor::Green,
        2 => SemColor::Yellow,
        _ => SemColor::Red,
    }
}

// ── Pie ──────────────────────────────────────────────

fn render_pie(series: &[NormSeries], height: u32) -> String {
    // Pie is always single-series. If the user accidentally passed `series:`
    // with multiple entries, flatten the first one — matches the "pie = one
    // ring of slices" mental model rather than silently dropping data.
    let Some(first) = series.first() else {
        return empty_svg(height);
    };
    let slices = first.points;
    let total: f64 = slices.iter().map(|p| p.value.max(0.0)).sum();
    if total <= 0.0 || slices.is_empty() {
        return empty_svg(height);
    }

    let h = height as f64;
    let cx = h / 2.0 + 20.0;
    let cy = h / 2.0;
    let r = (h / 2.0) - 16.0;

    let mut out = format!(
        r#"<svg viewBox="0 0 {vb_w} {h}" preserveAspectRatio="xMidYMid meet" class="c-chart-svg">"#,
        vb_w = VB_W,
        h = h,
    );

    // Single 100% slice can't be drawn with an arc (start==end). Draw a
    // circle instead so the fill shows up.
    if slices.len() == 1 {
        let p = &slices[0];
        let fill = series_color(p.color.or(first.color), 0);
        out.push_str(&format!(
            r#"<circle cx="{cx}" cy="{cy}" r="{r}" fill="{fill}"><title>{t}</title></circle>"#,
            cx = cx,
            cy = cy,
            r = r,
            fill = fill,
            t = esc(&format!("{}: {}", p.label, fmt_num(p.value))),
        ));
        out.push_str("</svg>");
        return out;
    }

    let mut angle_acc = 0.0_f64;
    for (i, p) in slices.iter().enumerate() {
        let v = p.value.max(0.0);
        if v <= 0.0 {
            continue;
        }
        let frac = v / total;
        let angle = frac * std::f64::consts::TAU;
        let a0 = angle_acc;
        let a1 = angle_acc + angle;
        let (x0, y0) = polar(cx, cy, r, a0);
        let (x1, y1) = polar(cx, cy, r, a1);
        let large = if angle > std::f64::consts::PI { 1 } else { 0 };
        let fill = series_color(p.color.or(first.color), i);
        let title = format!("{}: {} ({:.1}%)", p.label, fmt_num(v), frac * 100.0);
        out.push_str(&format!(
            r#"<path d="M {cx} {cy} L {x0:.2} {y0:.2} A {r} {r} 0 {large} 1 {x1:.2} {y1:.2} Z" fill="{fill}" class="c-chart-slice"><title>{title}</title></path>"#,
            cx = cx,
            cy = cy,
            r = r,
            x0 = x0,
            y0 = y0,
            x1 = x1,
            y1 = y1,
            large = large,
            fill = fill,
            title = esc(&title),
        ));
        angle_acc = a1;
    }

    out.push_str("</svg>");
    out
}

/// Convert (angle from 12 o'clock, clockwise) to (x, y) on the circle.
fn polar(cx: f64, cy: f64, r: f64, theta: f64) -> (f64, f64) {
    (cx + r * theta.sin(), cy - r * theta.cos())
}

// ── Bar ──────────────────────────────────────────────

fn render_bar(
    series: &[NormSeries],
    height: u32,
    orientation: ChartOrientation,
    x_label: &Option<String>,
    y_label: &Option<String>,
) -> String {
    if series.is_empty() || series.iter().all(|s| s.points.is_empty()) {
        return empty_svg(height);
    }
    match orientation {
        ChartOrientation::Vertical => render_bar_vertical(series, height, x_label, y_label),
        ChartOrientation::Horizontal => render_bar_horizontal(series, height, x_label, y_label),
    }
}

fn render_bar_vertical(
    series: &[NormSeries],
    height: u32,
    _x_label: &Option<String>,
    _y_label: &Option<String>,
) -> String {
    let h = height as f64;
    let left = 56.0;
    let right = 20.0;
    let top = 16.0;
    let bottom = 40.0;
    let plot_w = VB_W - left - right;
    let plot_h = h - top - bottom;

    let buckets = collect_buckets(series);
    if buckets.is_empty() {
        return empty_svg(height);
    }

    // Max value across stacked sums per bucket.
    let max_val = buckets
        .iter()
        .map(|b| stacked_total(series, b))
        .fold(0.0_f64, f64::max);
    if max_val <= 0.0 {
        return empty_svg(height);
    }
    let (_nmin, nmax, step) = nice_scale(0.0, max_val, 5);
    let axis_max = nmax.max(step);

    let mut out = format!(
        r#"<svg viewBox="0 0 {vb_w} {h}" preserveAspectRatio="xMidYMid meet" class="c-chart-svg">"#,
        vb_w = VB_W,
        h = h,
    );

    // Gridlines + y tick labels
    let mut v = 0.0_f64;
    while v <= axis_max + 1e-9 {
        let y = top + plot_h - (v / axis_max) * plot_h;
        out.push_str(&format!(
            r#"<line x1="{x1}" y1="{y:.2}" x2="{x2}" y2="{y:.2}" class="c-chart-grid"/>"#,
            x1 = left,
            x2 = left + plot_w,
            y = y,
        ));
        out.push_str(&format!(
            r#"<text x="{x:.2}" y="{y:.2}" class="c-chart-axis c-chart-axis-y">{t}</text>"#,
            x = left - 8.0,
            y = y + 3.5,
            t = esc(&fmt_num(v)),
        ));
        v += step;
    }

    // Bars per bucket
    let n = buckets.len() as f64;
    let bucket_w = plot_w / n;
    let bar_w = (bucket_w * 0.62).min(60.0);

    for (i, bucket) in buckets.iter().enumerate() {
        let cx = left + bucket_w * (i as f64 + 0.5);
        let x = cx - bar_w / 2.0;
        // Stack from bottom → up.
        let mut stacked = 0.0_f64;
        for (s_idx, s) in series.iter().enumerate() {
            let v = s
                .points
                .iter()
                .find(|p| p.label == *bucket)
                .map(|p| p.value.max(0.0))
                .unwrap_or(0.0);
            if v <= 0.0 {
                continue;
            }
            let seg_h = (v / axis_max) * plot_h;
            let y = top + plot_h - ((stacked + v) / axis_max) * plot_h;
            let fill = series_color(s.color, s_idx);
            let title = if s.label.is_empty() {
                format!("{}: {}", bucket, fmt_num(v))
            } else {
                format!("{} — {}: {}", s.label, bucket, fmt_num(v))
            };
            out.push_str(&format!(
                r#"<rect x="{x:.2}" y="{y:.2}" width="{w:.2}" height="{seg_h:.2}" fill="{fill}" class="c-chart-bar"><title>{title}</title></rect>"#,
                x = x,
                y = y,
                w = bar_w,
                seg_h = seg_h,
                fill = fill,
                title = esc(&title),
            ));
            stacked += v;
        }
        // X tick label
        out.push_str(&format!(
            r#"<text x="{x:.2}" y="{y:.2}" class="c-chart-axis c-chart-axis-x">{t}</text>"#,
            x = cx,
            y = top + plot_h + 20.0,
            t = esc(bucket),
        ));
    }

    out.push_str("</svg>");
    out
}

fn render_bar_horizontal(
    series: &[NormSeries],
    height: u32,
    _x_label: &Option<String>,
    _y_label: &Option<String>,
) -> String {
    let h = height as f64;
    let left = 120.0;
    let right = 24.0;
    let top = 16.0;
    let bottom = 32.0;
    let plot_w = VB_W - left - right;
    let plot_h = h - top - bottom;

    let buckets = collect_buckets(series);
    if buckets.is_empty() {
        return empty_svg(height);
    }

    let max_val = buckets
        .iter()
        .map(|b| stacked_total(series, b))
        .fold(0.0_f64, f64::max);
    if max_val <= 0.0 {
        return empty_svg(height);
    }
    let (_nmin, nmax, step) = nice_scale(0.0, max_val, 5);
    let axis_max = nmax.max(step);

    let mut out = format!(
        r#"<svg viewBox="0 0 {vb_w} {h}" preserveAspectRatio="xMidYMid meet" class="c-chart-svg">"#,
        vb_w = VB_W,
        h = h,
    );

    // Gridlines + x-axis labels (values on the bottom)
    let mut v = 0.0_f64;
    while v <= axis_max + 1e-9 {
        let x = left + (v / axis_max) * plot_w;
        out.push_str(&format!(
            r#"<line x1="{x:.2}" y1="{y1:.2}" x2="{x:.2}" y2="{y2:.2}" class="c-chart-grid"/>"#,
            x = x,
            y1 = top,
            y2 = top + plot_h,
        ));
        out.push_str(&format!(
            r#"<text x="{x:.2}" y="{y:.2}" class="c-chart-axis c-chart-axis-x">{t}</text>"#,
            x = x,
            y = top + plot_h + 18.0,
            t = esc(&fmt_num(v)),
        ));
        v += step;
    }

    let n = buckets.len() as f64;
    let bucket_h = plot_h / n;
    let bar_h = (bucket_h * 0.62).min(38.0);

    for (i, bucket) in buckets.iter().enumerate() {
        let cy = top + bucket_h * (i as f64 + 0.5);
        let y = cy - bar_h / 2.0;
        let mut stacked = 0.0_f64;
        for (s_idx, s) in series.iter().enumerate() {
            let val = s
                .points
                .iter()
                .find(|p| p.label == *bucket)
                .map(|p| p.value.max(0.0))
                .unwrap_or(0.0);
            if val <= 0.0 {
                continue;
            }
            let seg_w = (val / axis_max) * plot_w;
            let x = left + (stacked / axis_max) * plot_w;
            let fill = series_color(s.color, s_idx);
            let title = if s.label.is_empty() {
                format!("{}: {}", bucket, fmt_num(val))
            } else {
                format!("{} — {}: {}", s.label, bucket, fmt_num(val))
            };
            out.push_str(&format!(
                r#"<rect x="{x:.2}" y="{y:.2}" width="{w:.2}" height="{seg_h:.2}" fill="{fill}" class="c-chart-bar"><title>{title}</title></rect>"#,
                x = x,
                y = y,
                w = seg_w,
                seg_h = bar_h,
                fill = fill,
                title = esc(&title),
            ));
            stacked += val;
        }
        out.push_str(&format!(
            r#"<text x="{x:.2}" y="{y:.2}" class="c-chart-axis c-chart-axis-y-right">{t}</text>"#,
            x = left - 10.0,
            y = cy + 3.5,
            t = esc(bucket),
        ));
    }

    out.push_str("</svg>");
    out
}

// ── Timeseries ───────────────────────────────────────

fn render_timeseries(
    series: &[NormSeries],
    height: u32,
    _x_label: &Option<String>,
    _y_label: &Option<String>,
) -> String {
    if series.is_empty() || series.iter().all(|s| s.points.is_empty()) {
        return empty_svg(height);
    }
    let h = height as f64;
    let left = 56.0;
    let right = 20.0;
    let top = 16.0;
    let bottom = 40.0;
    let plot_w = VB_W - left - right;
    let plot_h = h - top - bottom;

    let buckets = collect_buckets(series);
    if buckets.is_empty() {
        return empty_svg(height);
    }

    // Timeseries = multi-line (not stacked). Max is the largest single value
    // across all series — each line needs its own y-space, not a summed one.
    let max_val = series
        .iter()
        .flat_map(|s| s.points.iter())
        .map(|p| p.value)
        .fold(0.0_f64, f64::max);
    if max_val <= 0.0 {
        return empty_svg(height);
    }
    let (_nmin, nmax, step) = nice_scale(0.0, max_val, 5);
    let axis_max = nmax.max(step);

    let mut out = format!(
        r#"<svg viewBox="0 0 {vb_w} {h}" preserveAspectRatio="xMidYMid meet" class="c-chart-svg">"#,
        vb_w = VB_W,
        h = h,
    );

    // Gridlines + y tick labels
    let mut v = 0.0_f64;
    while v <= axis_max + 1e-9 {
        let y = top + plot_h - (v / axis_max) * plot_h;
        out.push_str(&format!(
            r#"<line x1="{x1}" y1="{y:.2}" x2="{x2}" y2="{y:.2}" class="c-chart-grid"/>"#,
            x1 = left,
            x2 = left + plot_w,
            y = y,
        ));
        out.push_str(&format!(
            r#"<text x="{x:.2}" y="{y:.2}" class="c-chart-axis c-chart-axis-y">{t}</text>"#,
            x = left - 8.0,
            y = y + 3.5,
            t = esc(&fmt_num(v)),
        ));
        v += step;
    }

    // X-axis labels (one per bucket). If there are many buckets, thin them
    // out so labels don't collide.
    let n = buckets.len();
    let stride = (n as f64 / 10.0).ceil() as usize;
    let stride = stride.max(1);
    let x_of = |i: usize| -> f64 {
        if n == 1 {
            left + plot_w / 2.0
        } else {
            left + plot_w * (i as f64) / ((n - 1) as f64)
        }
    };
    for (i, bucket) in buckets.iter().enumerate() {
        if i % stride != 0 && i != n - 1 {
            continue;
        }
        out.push_str(&format!(
            r#"<text x="{x:.2}" y="{y:.2}" class="c-chart-axis c-chart-axis-x">{t}</text>"#,
            x = x_of(i),
            y = top + plot_h + 20.0,
            t = esc(bucket),
        ));
    }

    // One polyline per series, plus point markers with tooltips.
    for (s_idx, s) in series.iter().enumerate() {
        let stroke = series_color(s.color, s_idx);
        let mut points_attr = String::new();
        let mut dots = String::new();
        for (i, bucket) in buckets.iter().enumerate() {
            let Some(p) = s.points.iter().find(|p| p.label == *bucket) else {
                continue;
            };
            let x = x_of(i);
            let y = top + plot_h - (p.value / axis_max) * plot_h;
            if !points_attr.is_empty() {
                points_attr.push(' ');
            }
            points_attr.push_str(&format!("{:.2},{:.2}", x, y));
            let title = if s.label.is_empty() {
                format!("{}: {}", bucket, fmt_num(p.value))
            } else {
                format!("{} — {}: {}", s.label, bucket, fmt_num(p.value))
            };
            dots.push_str(&format!(
                r#"<circle cx="{x:.2}" cy="{y:.2}" r="3" fill="{stroke}" class="c-chart-dot"><title>{title}</title></circle>"#,
                x = x,
                y = y,
                stroke = stroke,
                title = esc(&title),
            ));
        }
        if !points_attr.is_empty() {
            out.push_str(&format!(
                r#"<polyline points="{pts}" fill="none" stroke="{stroke}" stroke-width="2" stroke-linejoin="round" stroke-linecap="round" class="c-chart-line"/>"#,
                pts = points_attr,
                stroke = stroke,
            ));
        }
        out.push_str(&dots);
    }

    out.push_str("</svg>");
    out
}

// ── Shared helpers ───────────────────────────────────

/// Union of bucket labels across all series, preserving first-seen order so
/// months stay in January→December even when one series has holes.
fn collect_buckets(series: &[NormSeries]) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for s in series {
        for p in s.points {
            if !out.iter().any(|b| b == &p.label) {
                out.push(p.label.clone());
            }
        }
    }
    out
}

fn stacked_total(series: &[NormSeries], bucket: &str) -> f64 {
    series
        .iter()
        .flat_map(|s| s.points.iter())
        .filter(|p| p.label == bucket)
        .map(|p| p.value.max(0.0))
        .sum()
}

fn render_legend(kind: ChartKind, series: &[NormSeries]) -> String {
    // Pie legend = slice labels. Bar/timeseries legend = series labels, but
    // only when there's more than one (single-series charts don't need one).
    let items: Vec<(String, String)> = match kind {
        ChartKind::Pie => {
            let Some(first) = series.first() else {
                return String::new();
            };
            first
                .points
                .iter()
                .enumerate()
                .map(|(i, p)| {
                    let color = series_color(p.color.or(first.color), i).to_string();
                    (p.label.clone(), color)
                })
                .collect()
        }
        _ => {
            if series.len() < 2 {
                return String::new();
            }
            series
                .iter()
                .enumerate()
                .map(|(i, s)| (s.label.to_string(), series_color(s.color, i).to_string()))
                .collect()
        }
    };
    if items.is_empty() {
        return String::new();
    }
    let mut out = String::from(r#"<ul class="c-chart-legend">"#);
    for (label, color) in items {
        out.push_str(&format!(
            r#"<li class="c-chart-legend-item"><span class="c-chart-swatch" style="background:{c}"></span><span>{l}</span></li>"#,
            c = color,
            l = esc(&label),
        ));
    }
    out.push_str("</ul>");
    out
}

fn empty_svg(height: u32) -> String {
    format!(
        r#"<svg viewBox="0 0 {vb_w} {h}" preserveAspectRatio="xMidYMid meet" class="c-chart-svg"><text x="50%" y="50%" class="c-chart-empty" text-anchor="middle">No data</text></svg>"#,
        vb_w = VB_W,
        h = height,
    )
}

// ── Numeric helpers ──────────────────────────────────

fn fmt_num(v: f64) -> String {
    if v.is_nan() || v.is_infinite() {
        return "—".into();
    }
    if v.fract().abs() < 1e-9 {
        return format!("{}", v as i64);
    }
    let s = format!("{:.2}", v);
    let trimmed = s.trim_end_matches('0').trim_end_matches('.');
    trimmed.to_string()
}

/// Round `raw` to a "nice" number — one of {1,2,5} * 10^n. `round=true` picks
/// the nearest nice step for tick spacing; `round=false` picks the next nice
/// number ≥ raw for the axis extent.
fn nice_number(raw: f64, round: bool) -> f64 {
    if raw <= 0.0 {
        return 1.0;
    }
    let exp = raw.log10().floor();
    let f = raw / 10_f64.powf(exp);
    let nf = if round {
        if f < 1.5 {
            1.0
        } else if f < 3.0 {
            2.0
        } else if f < 7.0 {
            5.0
        } else {
            10.0
        }
    } else if f <= 1.0 {
        1.0
    } else if f <= 2.0 {
        2.0
    } else if f <= 5.0 {
        5.0
    } else {
        10.0
    };
    nf * 10_f64.powf(exp)
}

fn nice_scale(min: f64, max: f64, target_ticks: usize) -> (f64, f64, f64) {
    let range = nice_number((max - min).max(1e-9), false);
    let ticks = target_ticks.max(2);
    let step = nice_number(range / (ticks as f64 - 1.0), true);
    let nice_min = (min / step).floor() * step;
    let nice_max = (max / step).ceil() * step;
    (nice_min, nice_max, step)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fmt_num_trims_integer_and_decimal() {
        assert_eq!(fmt_num(42.0), "42");
        assert_eq!(fmt_num(42.5), "42.5");
        assert_eq!(fmt_num(0.0), "0");
        assert_eq!(fmt_num(1.10), "1.1");
    }

    #[test]
    fn nice_scale_produces_round_steps() {
        let (_, max, step) = nice_scale(0.0, 125.0, 5);
        assert!(max >= 125.0);
        assert!(step > 0.0);
        assert!((max / step).round() * step - max < 1e-6);
    }

    #[test]
    fn polar_places_zero_angle_at_twelve_oclock() {
        let (x, y) = polar(100.0, 100.0, 50.0, 0.0);
        assert!((x - 100.0).abs() < 1e-6);
        assert!((y - 50.0).abs() < 1e-6);
    }

    #[test]
    fn collect_buckets_preserves_first_seen_order() {
        let a_points = vec![
            ChartPoint {
                label: "Jan".into(),
                value: 1.0,
                color: None,
            },
            ChartPoint {
                label: "Feb".into(),
                value: 2.0,
                color: None,
            },
        ];
        let b_points = vec![
            ChartPoint {
                label: "Feb".into(),
                value: 3.0,
                color: None,
            },
            ChartPoint {
                label: "Mar".into(),
                value: 4.0,
                color: None,
            },
        ];
        let series = vec![
            NormSeries {
                label: "A",
                color: None,
                points: &a_points,
            },
            NormSeries {
                label: "B",
                color: None,
                points: &b_points,
            },
        ];
        let buckets = collect_buckets(&series);
        assert_eq!(buckets, vec!["Jan", "Feb", "Mar"]);
    }
}
