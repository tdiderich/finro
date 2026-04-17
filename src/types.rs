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
    pub components: Option<Vec<Component>>,
    pub slides: Option<Vec<Slide>>,
    /// Exclude this page from llms.txt. Useful for drafts.
    #[serde(default)]
    pub unlisted: bool,
}

#[derive(Deserialize)]
pub struct Slide {
    pub label: String,
    pub components: Vec<Component>,
    /// Hide the top-left "OVERVIEW"-style label on this slide. Typically used
    /// for a cover/title slide where the centered title is the only text.
    #[serde(default)]
    pub hide_label: bool,
}

// ── Components ───────────────────────────────────────

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Component {
    Header {
        title: String,
        subtitle: Option<String>,
        eyebrow: Option<String>,
        #[serde(default)]
        align: Align,
    },
    Meta {
        fields: Vec<MetaField>,
    },
    CardGrid {
        cards: Vec<Card>,
        #[serde(default)]
        min_width: Option<u32>,
        #[serde(default)]
        connector: Connector,
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
        links: Option<Vec<ButtonConfig>>,
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
        #[serde(default)]
        align: Align,
    },
    Columns {
        columns: Vec<Vec<Component>>,
        #[serde(default)]
        equal_heights: bool,
    },
    Accordion {
        items: Vec<AccordionItem>,
    },
    Image {
        src: String,
        alt: Option<String>,
        caption: Option<String>,
        max_width: Option<u32>,
        #[serde(default)]
        align: Align,
    },
    Badge {
        label: String,
        #[serde(default)]
        color: SemColor,
    },
    Tag {
        label: String,
        #[serde(default)]
        color: SemColor,
    },
    Divider {
        label: Option<String>,
    },
    Kbd {
        keys: Vec<String>,
    },
    Status {
        label: String,
        #[serde(default)]
        color: SemColor,
    },
    Breadcrumb {
        items: Vec<BreadcrumbItem>,
    },
    ButtonGroup {
        buttons: Vec<ButtonConfig>,
    },
    DefinitionList {
        items: Vec<DefinitionItem>,
    },
    Blockquote {
        body: String,
        attribution: Option<String>,
    },
    Avatar {
        name: String,
        src: Option<String>,
        #[serde(default)]
        size: AvatarSize,
        subtitle: Option<String>,
    },
    AvatarGroup {
        avatars: Vec<AvatarConfig>,
        #[serde(default)]
        size: AvatarSize,
        #[serde(default = "default_avatar_max")]
        max: usize,
    },
    ProgressBar {
        value: u8,
        label: Option<String>,
        #[serde(default)]
        color: SemColor,
        detail: Option<String>,
    },
    EmptyState {
        title: String,
        body: Option<String>,
        action: Option<EmptyStateAction>,
        #[serde(default)]
        icon: Option<String>,
    },
    Icon {
        name: String,
        #[serde(default)]
        size: IconSize,
        #[serde(default)]
        color: SemColor,
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
    #[serde(default)]
    pub color: SemColor,
}

#[derive(Deserialize)]
pub struct Badge {
    pub label: String,
    #[serde(default)]
    pub color: SemColor,
}

/// Unified semantic color palette used by badge, tag, status, progress_bar,
/// and the stat color accents. Keeps all colored decoration consistent.
#[derive(Deserialize, Default, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum SemColor {
    #[default]
    Default,
    Green,
    Yellow,
    Red,
    Teal,
}

impl SemColor {
    pub fn class_suffix(&self) -> &'static str {
        match self {
            SemColor::Default => "default",
            SemColor::Green => "green",
            SemColor::Yellow => "yellow",
            SemColor::Red => "red",
            SemColor::Teal => "teal",
        }
    }

    pub fn hex(&self) -> &'static str {
        match self {
            SemColor::Default => "#3CCECE",
            SemColor::Green => "#34D399",
            SemColor::Yellow => "#FBBF24",
            SemColor::Red => "#F87171",
            SemColor::Teal => "#3CCECE",
        }
    }
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
    #[serde(default)]
    pub color: SemColor,
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
    Arrow,
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
    pub color: SemColor,
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
            CalloutVariant::Info => "c-callout-info",
            CalloutVariant::Warn => "c-callout-warn",
            CalloutVariant::Success => "c-callout-success",
            CalloutVariant::Danger => "c-callout-danger",
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

// ── New component supporting types ───────────────────

#[derive(Deserialize)]
pub struct BreadcrumbItem {
    pub label: String,
    pub href: Option<String>,
}

#[derive(Deserialize, Default, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum ButtonVariant {
    #[default]
    Primary,
    Secondary,
    Ghost,
}

#[derive(Deserialize)]
pub struct ButtonConfig {
    pub label: String,
    pub href: String,
    #[serde(default)]
    pub variant: ButtonVariant,
    #[serde(default)]
    pub external: bool,
    pub icon: Option<String>,
}

#[derive(Deserialize, Default, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum IconSize {
    Xs,
    Sm,
    #[default]
    Md,
    Lg,
    Xl,
}

impl IconSize {
    pub fn pixels(&self) -> u32 {
        match self {
            IconSize::Xs => 12,
            IconSize::Sm => 14,
            IconSize::Md => 16,
            IconSize::Lg => 20,
            IconSize::Xl => 24,
        }
    }
}

#[derive(Deserialize)]
pub struct DefinitionItem {
    pub term: String,
    pub definition: String,
}

#[derive(Deserialize)]
pub struct AvatarConfig {
    pub name: String,
    pub src: Option<String>,
}

#[derive(Deserialize, Default, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum AvatarSize {
    Sm,
    #[default]
    Md,
    Lg,
    Xl,
}

impl AvatarSize {
    pub fn class_suffix(&self) -> &'static str {
        match self {
            AvatarSize::Sm => "sm",
            AvatarSize::Md => "md",
            AvatarSize::Lg => "lg",
            AvatarSize::Xl => "xl",
        }
    }
}

#[derive(Deserialize)]
pub struct EmptyStateAction {
    pub label: String,
    pub href: String,
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
    pub theme: Option<String>,
    #[serde(default)]
    pub colors: std::collections::HashMap<String, String>,
    pub nav: Option<Vec<NavLink>>,
    pub favicon: Option<Favicon>,
    /// When true, each page gets a companion `*.source.html` rendering of its
    /// YAML source, and a "View source" pill links to it. Off by default —
    /// useful for docs/examples sites, noise for most end-user sites.
    #[serde(default)]
    pub view_source: bool,
}

/// Favicon config: either a single path, or a struct with named slots.
#[derive(Deserialize)]
#[serde(untagged)]
pub enum Favicon {
    Simple(String),
    Full {
        svg: Option<String>,
        png: Option<String>,
        ico: Option<String>,
        apple_touch_icon: Option<String>,
    },
}

impl Favicon {
    /// Render <link> tags (already resolved against base path).
    pub fn render(&self, base: &str) -> String {
        let resolve = |p: &str| crate::render::resolve_href(p, base);
        match self {
            Favicon::Simple(path) => {
                let mime = mime_for(path);
                format!(r#"<link rel="icon" type="{}" href="{}">"#, mime, resolve(path))
            }
            Favicon::Full { svg, png, ico, apple_touch_icon } => {
                let mut out = String::new();
                if let Some(p) = svg { out.push_str(&format!(r#"<link rel="icon" type="image/svg+xml" href="{}">"#, resolve(p))); }
                if let Some(p) = png { out.push_str(&format!(r#"<link rel="icon" type="image/png" href="{}">"#, resolve(p))); }
                if let Some(p) = ico { out.push_str(&format!(r#"<link rel="icon" type="image/x-icon" href="{}">"#, resolve(p))); }
                if let Some(p) = apple_touch_icon { out.push_str(&format!(r#"<link rel="apple-touch-icon" href="{}">"#, resolve(p))); }
                out
            }
        }
    }
}

fn mime_for(path: &str) -> &'static str {
    let lower = path.to_lowercase();
    if lower.ends_with(".svg") { "image/svg+xml" }
    else if lower.ends_with(".png") { "image/png" }
    else if lower.ends_with(".ico") { "image/x-icon" }
    else if lower.ends_with(".jpg") || lower.ends_with(".jpeg") { "image/jpeg" }
    else { "image/png" }
}

impl SiteConfig {
    pub fn resolved_theme(&self) -> crate::theme::Theme {
        let base = self.theme.as_deref().unwrap_or("dark");
        crate::theme::Theme::named(base).with_overrides(&self.colors)
    }
}

impl Default for SiteConfig {
    fn default() -> Self {
        SiteConfig {
            name: String::from("My Site"),
            theme: None,
            colors: std::collections::HashMap::new(),
            nav: None,
            favicon: None,
            view_source: false,
        }
    }
}

// ── Defaults ─────────────────────────────────────────

fn default_stat_columns() -> u32 { 3 }
fn default_true() -> bool { true }
fn default_avatar_max() -> usize { 5 }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_to_string_handles_all_scalar_types() {
        use serde_yaml::Value;
        assert_eq!(value_to_string(&Value::Null), "");
        assert_eq!(value_to_string(&Value::Bool(true)), "true");
        assert_eq!(value_to_string(&Value::Number(42.into())), "42");
        assert_eq!(value_to_string(&Value::String("hi".into())), "hi");
    }

    #[test]
    fn sem_color_class_suffix() {
        assert_eq!(SemColor::Default.class_suffix(), "default");
        assert_eq!(SemColor::Green.class_suffix(), "green");
        assert_eq!(SemColor::Yellow.class_suffix(), "yellow");
        assert_eq!(SemColor::Red.class_suffix(), "red");
        assert_eq!(SemColor::Teal.class_suffix(), "teal");
    }

    #[test]
    fn sem_color_hex_values() {
        assert_eq!(SemColor::Green.hex(), "#34D399");
        assert_eq!(SemColor::Red.hex(), "#F87171");
        assert_eq!(SemColor::Default.hex(), "#3CCECE");
    }
}
