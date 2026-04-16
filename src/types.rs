use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Layout {
    Dashboard,
    Article,
    Agenda,
    Phases,
    Qbr,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum BadgeColor {
    Green,
    Yellow,
    Red,
    #[default]
    #[serde(other)]
    Default,
}

#[derive(Deserialize)]
pub struct PageLink {
    pub label: String,
    pub href: String,
}

#[derive(Deserialize)]
pub struct Card {
    pub name: String,
    pub badge: Option<String>,
    #[serde(default)]
    pub badge_color: BadgeColor,
    pub description: Option<String>,
    pub links: Option<Vec<PageLink>>,
}

#[derive(Deserialize)]
pub struct MetaField {
    pub key: String,
    pub value: String,
}

#[derive(Deserialize)]
pub struct NavBack {
    pub label: String,
    pub href: String,
}

#[derive(Deserialize)]
pub struct Phase {
    pub number: u32,
    pub name: String,
    pub bullets: Vec<String>,
}

#[derive(Deserialize)]
pub struct SuccessOutcome {
    pub title: String,
    pub before: String,
    pub now_highlight: String,
    pub now_context: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PhaseStatus {
    Completed,
    Active,
    Upcoming,
}

#[derive(Deserialize)]
pub struct TimelinePhase {
    pub name: String,
    pub status: PhaseStatus,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HealthColor {
    Green,
    Yellow,
    Red,
}

#[derive(Deserialize)]
pub struct HealthCard {
    pub label: String,
    pub value: String,
    pub detail: String,
    pub color: HealthColor,
}

#[derive(Deserialize)]
pub struct QbrNextStep {
    pub title: String,
    pub detail: String,
}

#[derive(Deserialize)]
pub struct Page {
    pub title: String,
    pub layout: Layout,
    pub subtitle: Option<String>,
    pub customer: Option<String>,
    pub date: Option<String>,
    pub nav_back: Option<NavBack>,
    // dashboard
    pub cards: Option<Vec<Card>>,
    // article + agenda
    pub meta: Option<Vec<MetaField>>,
    pub body: Option<String>,
    // phases
    pub phases: Option<Vec<Phase>>,
    // qbr
    pub success_outcomes: Option<Vec<SuccessOutcome>>,
    pub phase_timeline: Option<Vec<TimelinePhase>>,
    pub health_cards: Option<Vec<HealthCard>>,
    pub next_steps: Option<Vec<QbrNextStep>>,
}

#[derive(Deserialize)]
pub struct NavLink {
    pub label: String,
    pub href: String,
}

#[derive(Deserialize)]
pub struct SiteConfig {
    pub name: String,
    #[allow(dead_code)]
    pub theme: Option<String>,
    pub nav: Option<Vec<NavLink>>,
}

impl Default for SiteConfig {
    fn default() -> Self {
        SiteConfig { name: String::from("My Site"), theme: None, nav: None }
    }
}
