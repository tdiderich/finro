use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TaskStore {
    pub tasks: Vec<Task>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: String,
    pub title: String,
    #[serde(default)]
    pub status: TaskStatus,
    #[serde(default = "default_priority")]
    pub priority: u8,
    #[serde(default)]
    pub task_type: TaskType,
    #[serde(default, skip_serializing_if = "is_agent")]
    pub owner: TaskOwner,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assignee: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub blocks: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    pub created: String,
    pub updated: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub closed: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub close_reason: Option<String>,
}

fn default_priority() -> u8 {
    2
}

fn is_agent(o: &TaskOwner) -> bool {
    *o == TaskOwner::Agent
}

#[derive(Serialize, Deserialize, Clone, Copy, Default, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    #[default]
    Open,
    Active,
    Closed,
    Blocked,
    Deferred,
}

impl TaskStatus {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Active => "active",
            Self::Closed => "closed",
            Self::Blocked => "blocked",
            Self::Deferred => "deferred",
        }
    }
}

impl std::fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.label())
    }
}

impl std::str::FromStr for TaskStatus {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "open" => Ok(Self::Open),
            "active" => Ok(Self::Active),
            "closed" => Ok(Self::Closed),
            "blocked" => Ok(Self::Blocked),
            "deferred" => Ok(Self::Deferred),
            _ => Err(format!("unknown status: {s}")),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Default, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaskType {
    #[default]
    Task,
    Bug,
    Feature,
    Epic,
}

#[derive(Serialize, Deserialize, Clone, Copy, Default, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaskOwner {
    #[default]
    Agent,
    Human,
}

impl TaskOwner {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Agent => "agent",
            Self::Human => "human",
        }
    }
}

impl std::fmt::Display for TaskOwner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.label())
    }
}

impl std::str::FromStr for TaskOwner {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "agent" => Ok(Self::Agent),
            "human" => Ok(Self::Human),
            _ => Err(format!("unknown owner: {s} (expected: agent, human)")),
        }
    }
}

impl TaskType {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Task => "task",
            Self::Bug => "bug",
            Self::Feature => "feature",
            Self::Epic => "epic",
        }
    }
}

impl std::fmt::Display for TaskType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.label())
    }
}

// ── Log ──────────────────────────────────────────

#[derive(Serialize, Deserialize)]
pub struct LogStore {
    pub events: Vec<LogEntry>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LogEntry {
    pub date: String,
    pub title: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[serde(default)]
    pub severity: LogSeverity,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Default)]
#[serde(rename_all = "snake_case")]
pub enum LogSeverity {
    Major,
    Minor,
    #[default]
    Info,
}

// ── Ready result (agent-first: includes full context) ──

#[derive(Serialize)]
pub struct ReadyTask {
    #[serde(flatten)]
    pub task: Task,
    pub parent_title: Option<String>,
    pub blocker_titles: Vec<String>,
}
