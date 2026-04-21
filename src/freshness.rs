//! Freshness metadata: staleness check + reporting.
//!
//! Staleness is computed at build time — zero runtime JS. A page is stale
//! when `updated + review_every < today`. "Today" comes from the env var
//! `KAZAM_TODAY` when set (deterministic tests), otherwise from the
//! system clock.
//!
//! Date handling is hand-rolled against ISO `YYYY-MM-DD`. We only care
//! about day-resolution comparisons, so days-since-1970-01-01 via the
//! Julian-day-number algorithm is enough — no chrono / time dep.
//!
//! Duration parsing accepts `Nd` / `Nw` / `Nm` / `Ny` and the word
//! shortcuts `weekly` / `monthly` / `quarterly` / `yearly` / `annually`.

use std::time::{SystemTime, UNIX_EPOCH};

use crate::types::Freshness;

/// Number of days before the review deadline at which a page starts
/// surfacing a yellow "review due soon" banner. Inside this window the
/// page is not yet overdue but reviewers should see the nudge.
pub const DUE_SOON_WINDOW_DAYS: i64 = 7;

/// A page's current freshness state. The renderer picks a banner variant
/// (and the build report picks a tone) based on this.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FreshnessStatus {
    /// No banner. Either no freshness metadata, or comfortably inside the
    /// review window (more than `DUE_SOON_WINDOW_DAYS` to go).
    Fresh,
    /// Yellow banner — review comes due within `DUE_SOON_WINDOW_DAYS`.
    /// `days_until_due` is non-negative.
    DueSoon { days_until_due: i64 },
    /// Red banner — review window has elapsed. `days_overdue` is positive.
    Overdue { days_overdue: i64 },
}

/// Today's date as `YYYY-MM-DD`. Honors `KAZAM_TODAY` for deterministic
/// tests, else reads the system clock.
pub fn today_iso() -> String {
    if let Ok(s) = std::env::var("KAZAM_TODAY") {
        if !s.is_empty() {
            return s;
        }
    }
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);
    let days = secs.div_euclid(86_400);
    iso_from_days_since_epoch(days)
}

/// Structured days-since-epoch info for a page's freshness metadata.
pub struct FreshnessInfo {
    pub updated_days: Option<i64>,
    pub review_days: Option<i64>,
    pub today_days: i64,
}

impl FreshnessInfo {
    pub fn days_since_update(&self) -> Option<i64> {
        self.updated_days.map(|u| self.today_days - u)
    }

    /// True when an `updated` date AND a `review_every` cadence are set AND
    /// the elapsed days exceed the cadence. Pages without both fields set
    /// are never "stale" — they simply have nothing to compare against.
    #[allow(dead_code)]
    pub fn is_stale(&self) -> bool {
        matches!(self.status(), FreshnessStatus::Overdue { .. })
    }

    /// Tri-state freshness state. Fresh = no banner, DueSoon = yellow nudge,
    /// Overdue = red banner. Pages missing either `updated` or `review_every`
    /// are always Fresh — there's nothing to compare against.
    pub fn status(&self) -> FreshnessStatus {
        let (elapsed, cadence) = match (self.days_since_update(), self.review_days) {
            (Some(e), Some(c)) => (e, c),
            _ => return FreshnessStatus::Fresh,
        };
        let days_until_due = cadence - elapsed;
        if days_until_due < 0 {
            FreshnessStatus::Overdue {
                days_overdue: -days_until_due,
            }
        } else if days_until_due <= DUE_SOON_WINDOW_DAYS {
            FreshnessStatus::DueSoon { days_until_due }
        } else {
            FreshnessStatus::Fresh
        }
    }
}

/// Parse a `Freshness` struct into days-since-epoch integers relative to
/// `today_iso` (a `YYYY-MM-DD` string). Returns `None` when there's no
/// freshness metadata at all.
pub fn info_for(f: Option<&Freshness>, today_iso: &str) -> Option<FreshnessInfo> {
    let f = f?;
    let today_days = parse_iso_date(today_iso).unwrap_or(0);
    let updated_days = f.updated.as_deref().and_then(parse_iso_date);
    let review_days = f.review_every.as_deref().and_then(parse_duration_days);
    Some(FreshnessInfo {
        updated_days,
        review_days,
        today_days,
    })
}

/// Parse an ISO `YYYY-MM-DD` date into days since 1970-01-01. Returns
/// `None` on malformed input — the renderer degrades to "not stale."
pub fn parse_iso_date(s: &str) -> Option<i64> {
    let s = s.trim();
    let mut parts = s.split('-');
    let y: i32 = parts.next()?.parse().ok()?;
    let m: u32 = parts.next()?.parse().ok()?;
    let d: u32 = parts.next()?.parse().ok()?;
    if parts.next().is_some() {
        return None;
    }
    if !(1..=12).contains(&m) || !(1..=31).contains(&d) {
        return None;
    }
    Some(days_since_epoch(y, m, d))
}

/// Gregorian (y, m, d) → days since 1970-01-01. JDN formula.
fn days_since_epoch(y: i32, m: u32, d: u32) -> i64 {
    let a = (14 - m as i32) / 12;
    let y = y + 4800 - a;
    let m_adj = m as i32 + 12 * a - 3;
    let jdn = d as i32 + (153 * m_adj + 2) / 5 + 365 * y + y / 4 - y / 100 + y / 400 - 32045;
    (jdn - 2440588) as i64
}

/// Days since 1970-01-01 → ISO `YYYY-MM-DD`. Inverse of `days_since_epoch`.
fn iso_from_days_since_epoch(days: i64) -> String {
    // Offset to JDN, then invert the Gregorian algorithm.
    let jdn = days + 2440588;
    let a = jdn + 32044;
    let b = (4 * a + 3) / 146_097;
    let c = a - (146_097 * b) / 4;
    let d = (4 * c + 3) / 1461;
    let e = c - (1461 * d) / 4;
    let m_ = (5 * e + 2) / 153;
    let day = e - (153 * m_ + 2) / 5 + 1;
    let month = m_ + 3 - 12 * (m_ / 10);
    let year = 100 * b + d - 4800 + m_ / 10;
    format!("{:04}-{:02}-{:02}", year, month, day)
}

/// Parse a duration string into days. Returns `None` on anything we don't
/// recognize — renderer falls back to "not stale."
pub fn parse_duration_days(s: &str) -> Option<i64> {
    let s = s.trim();
    match s.to_ascii_lowercase().as_str() {
        "weekly" => return Some(7),
        "monthly" => return Some(30),
        "quarterly" => return Some(90),
        "yearly" | "annually" => return Some(365),
        _ => {}
    }
    // Numeric + unit suffix: 7d, 12w, 3m, 1y.
    let (num, unit) = s.split_at(s.len().saturating_sub(1));
    let n: i64 = num.trim().parse().ok()?;
    let mult = match unit {
        "d" | "D" => 1,
        "w" | "W" => 7,
        "m" | "M" => 30,
        "y" | "Y" => 365,
        _ => return None,
    };
    Some(n * mult)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_iso_and_back_round_trips() {
        let cases = [
            "1970-01-01",
            "2026-04-21",
            "2000-02-29",
            "2100-01-01",
            "1999-12-31",
        ];
        for c in cases {
            let d = parse_iso_date(c).expect("parse");
            let back = iso_from_days_since_epoch(d);
            assert_eq!(back, c, "round-trip {c}");
        }
    }

    #[test]
    fn parse_iso_rejects_garbage() {
        assert!(parse_iso_date("not a date").is_none());
        assert!(parse_iso_date("2026-13-01").is_none());
        assert!(parse_iso_date("2026-01-32").is_none());
        assert!(parse_iso_date("2026/01/01").is_none());
    }

    #[test]
    fn duration_parses_numeric_and_word_forms() {
        assert_eq!(parse_duration_days("7d"), Some(7));
        assert_eq!(parse_duration_days("12w"), Some(84));
        assert_eq!(parse_duration_days("3m"), Some(90));
        assert_eq!(parse_duration_days("1y"), Some(365));
        assert_eq!(parse_duration_days("weekly"), Some(7));
        assert_eq!(parse_duration_days("Monthly"), Some(30));
        assert_eq!(parse_duration_days("quarterly"), Some(90));
        assert_eq!(parse_duration_days("yearly"), Some(365));
        assert_eq!(parse_duration_days("annually"), Some(365));
        assert_eq!(parse_duration_days("once in a while"), None);
    }

    #[test]
    fn is_stale_triggers_when_cadence_exceeded() {
        // Page updated 2026-01-01, reviewed every 90 days. On 2026-04-21
        // (110 days later) it should be stale.
        let f = Freshness {
            updated: Some("2026-01-01".to_string()),
            review_every: Some("90d".to_string()),
            owner: None,
            sources_of_truth: None,
        };
        let info = info_for(Some(&f), "2026-04-21").unwrap();
        assert_eq!(info.days_since_update(), Some(110));
        assert!(info.is_stale());
    }

    #[test]
    fn is_not_stale_when_within_window() {
        let f = Freshness {
            updated: Some("2026-04-01".to_string()),
            review_every: Some("90d".to_string()),
            owner: None,
            sources_of_truth: None,
        };
        let info = info_for(Some(&f), "2026-04-21").unwrap();
        assert!(!info.is_stale());
    }

    #[test]
    fn is_not_stale_when_metadata_incomplete() {
        // Missing review_every → no cadence → never stale.
        let f = Freshness {
            updated: Some("2020-01-01".to_string()),
            review_every: None,
            owner: None,
            sources_of_truth: None,
        };
        let info = info_for(Some(&f), "2026-04-21").unwrap();
        assert!(!info.is_stale());

        // Missing updated → nothing to compare against.
        let f = Freshness {
            updated: None,
            review_every: Some("90d".to_string()),
            owner: None,
            sources_of_truth: None,
        };
        let info = info_for(Some(&f), "2026-04-21").unwrap();
        assert!(!info.is_stale());
    }

    #[test]
    fn today_honors_env_var() {
        std::env::set_var("KAZAM_TODAY", "2099-06-15");
        assert_eq!(today_iso(), "2099-06-15");
        std::env::remove_var("KAZAM_TODAY");
    }
}
