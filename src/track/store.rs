use anyhow::{Context, Result};
use std::path::Path;

use crate::workspace;

use super::types::{LogEntry, LogStore, TaskStore};

pub fn tasks_path(project: &Path) -> std::path::PathBuf {
    workspace::root(project).join("track/tasks.yaml")
}

pub fn log_path(project: &Path) -> std::path::PathBuf {
    workspace::root(project).join("track/log.yaml")
}

pub fn read_tasks(project: &Path) -> Result<TaskStore> {
    workspace::read_yaml(&tasks_path(project))
}

pub fn write_tasks(project: &Path, store: &TaskStore) -> Result<()> {
    workspace::write_yaml(&tasks_path(project), store)
}

pub fn read_log(project: &Path) -> Result<LogStore> {
    workspace::read_yaml(&log_path(project))
}

pub fn append_log(project: &Path, entry: LogEntry) -> Result<()> {
    let mut store = read_log(project).unwrap_or(LogStore { events: vec![] });
    store.events.push(entry);
    workspace::write_yaml(&log_path(project), &store).context("write log")
}
