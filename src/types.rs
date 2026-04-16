use std::collections::HashMap;
use serde::Deserialize;

// ── Shell ────────────────────────────────────────────

#[derive(Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum Shell {
    Standard,
    Document,
    Deck,
}

impl Shell {
    pub fn class(&self) -> &'static str {
        match self {
            Shell::Standard => "shell-standard",
            Shell::Document => "shell-document",
            Shell::Deck => "shell-deck",
        }
    }
}

// ── Page ─────────────────────────────────────────────

#[derive(Deserialize)]
pub struct Page {
    pub title: String,
    pub shell: Shell,
    pub eyebrow: Option<String>,
    pub subtitle: Option<String>,
    pub nav_back: Option<NavBack>,
    pub components: Option<Vec<Component>>,
    pub slides: Option<Vec<Slide>>,
}

#[derive(Deserialize)]
pub struct NavBack {
    pub label: String,
    pub href: String,
}

#[derive(Deserialize)]
pub struct Slide {
    pub label: String,
    pub components: Vec<Component>,
}

// ── Components ───────────────────────────────────────

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Component {
    Header {
        title: String,
        subtitle: Option<String>,
        eyebrow: Option<String>,
    },
    Meta {
        fields: Vec<MetaField>,
    },
    CardGrid {
        cards: Vec<Card>,
        #[serde(default)]
        min_width: Option<u32>,
    },
    SelectableGrid {
        cards: Vec<SelectableCard>,
        #[serde(default)]
        interaction: Interaction,
        #[serde(default)]
        connector: Connector,
    },
    Timeline {
        items: Vec<TimelineItem>,
    },
    StatGrid {
        stats: Vec<Stat>,
        #[serde(default = "default_stat_columns")]
        columns: u32,
    },
    BeforeAfter {
        items: Vec<BeforeAfterItem>,
    },
    Steps {
        items: Vec<Step>,
        #[serde(default = "default_true")]
        numbered: bool,
    },
    Markdown {
        body: String,
    },
    Table {
        columns: Vec<TableColumn>,
        rows: Vec<HashMap<String, serde_yaml::Value>>,
        #[serde(default)]
        filterable: bool,
    },
    Callout {
        #[serde(default)]
        variant: CalloutVariant,
        title: Option<String>,
        body: String,
    },
    Code {
        language: Option<String>,
        code: String,
    },
    Tabs {
        tabs: Vec<Tab>,
    },
    Section {
        heading: Option<String>,
        eyebrow: Option<String>,
        components: Vec<Component>,
    },
    Columns {
        columns: Vec<Vec<Component>>,
    },
    Accordion {
        items: Vec<AccordionItem>,
    },
    Image {
        src: String,
        alt: Option<String>,
        caption: Option<String>,
        max_width: Option<u32>,
    },
}

// ── Supporting types ─────────────────────────────────

#[derive(Deserialize)]
pub struct MetaField {
    pub key: String,
    pub value: String,
}

#[derive(Deserialize)]
pub struct Card {
    pub title: String,
    pub badge: Option<Badge>,
    pub description: Option<String>,
    pub links: Option<Vec<Link>>,
    pub href: Option<String>,
}

#[derive(Deserialize)]
pub struct Badge {
    pub label: String,
    #[serde(default)]
    pub color: BadgeColor,
}

#[derive(Deserialize, Default, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum BadgeColor {
    Green,
    Yellow,
    Red,
    #[default]
    Default,
}

#[derive(Deserialize)]
pub struct Link {
    pub label: String,
    pub href: String,
}

#[derive(Deserialize)]
pub struct SelectableCard {
    pub title: String,
    pub eyebrow: Option<String>,
    pub bullets: Option<Vec<String>>,
    pub body: Option<String>,
}

#[derive(Deserialize, Default, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum Interaction {
    #[default]
    SingleSelect,
    MultiSelect,
    None,
}

#[derive(Deserialize, Default, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum Connector {
    #[default]
    None,
    DotsLine,
}

#[derive(Deserialize)]
pub struct TimelineItem {
    pub name: String,
    pub status: TimelineStatus,
}

#[derive(Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum TimelineStatus {
    Completed,
    Active,
    Upcoming,
}

#[derive(Deserialize)]
pub struct Stat {
    pub label: String,
    pub value: String,
    pub detail: Option<String>,
    #[serde(default)]
    pub color: StatColor,
}

#[derive(Deserialize, Default, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum StatColor {
    Green,
    Yellow,
    Red,
    #[default]
    Default,
}

#[derive(Deserialize)]
pub struct BeforeAfterItem {
    pub title: String,
    pub before: String,
    pub after: String,
    pub after_context: Option<String>,
}

#[derive(Deserialize)]
pub struct Step {
    pub title: String,
    pub detail: Option<String>,
}

#[derive(Deserialize)]
pub struct TableColumn {
    pub key: String,
    pub label: String,
    #[serde(default)]
    pub sortable: bool,
    #[serde(default)]
    pub align: Align,
}

#[derive(Deserialize, Default, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum Align {
    #[default]
    Left,
    Right,
    Center,
}

impl Align {
    pub fn class(&self) -> &'static str {
        match self {
            Align::Left => "align-left",
            Align::Right => "align-right",
            Align::Center => "align-center",
        }
    }
}

#[derive(Deserialize, Default, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum CalloutVariant {
    #[default]
    Info,
    Warn,
    Success,
    Danger,
}

impl CalloutVariant {
    pub fn class(&self) -> &'static str {
        match self {
            CalloutVariant::Info => "callout-info",
            CalloutVariant::Warn => "callout-warn",
            CalloutVariant::Success => "callout-success",
            CalloutVariant::Danger => "callout-danger",
        }
    }
}

#[derive(Deserialize)]
pub struct Tab {
    pub label: String,
    pub components: Vec<Component>,
}

#[derive(Deserialize)]
pub struct AccordionItem {
    pub title: String,
    pub components: Vec<Component>,
}

// ── Site config ──────────────────────────────────────

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

// ── Defaults ─────────────────────────────────────────

fn default_stat_columns() -> u32 { 3 }
fn default_true() -> bool { true }

pub fn value_to_string(v: &serde_yaml::Value) -> String {
    match v {
        serde_yaml::Value::Null => String::new(),
        serde_yaml::Value::Bool(b) => b.to_string(),
        serde_yaml::Value::Number(n) => n.to_string(),
        serde_yaml::Value::String(s) => s.clone(),
        serde_yaml::Value::Sequence(_) | serde_yaml::Value::Mapping(_) => {
            serde_yaml::to_string(v).unwrap_or_default()
        }
        serde_yaml::Value::Tagged(t) => value_to_string(&t.value),
    }
}
