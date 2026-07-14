//! Support helpers for `swarm_execute`: sub-agent tool filtering and
//! shared tool filtering for autonomous sub-agents.
//!
use crate::swarm::SubTask;
use std::path::PathBuf;

pub(crate) fn is_read_only(name: &str, instruction: &str, specialty: Option<&str>) -> bool {
    let mut task = SubTask::new(name, instruction);
    task.specialty = specialty.map(String::from);
    task.is_read_only()
}

pub(crate) fn expects_changes(name: &str, instruction: &str, specialty: Option<&str>) -> bool {
    let mut task = SubTask::new(name, instruction);
    task.specialty = specialty.map(String::from);
    task.expects_file_changes()
}

pub(super) fn workspace(params: &serde_json::Value) -> PathBuf {
    params
        .get("__ct_parent_workspace")
        .and_then(serde_json::Value::as_str)
        .map(PathBuf::from)
        .or_else(|| std::env::current_dir().ok())
        .unwrap_or_else(|| PathBuf::from("."))
}

pub(super) fn working_directory(
    isolated: Option<PathBuf>,
    read_only: bool,
    parent: &std::path::Path,
) -> anyhow::Result<PathBuf> {
    match (isolated, read_only) {
        (Some(path), _) => Ok(path),
        (None, true) => Ok(parent.to_path_buf()),
        (None, false) => anyhow::bail!("required swarm worktree allocation failed"),
    }
}

#[cfg(test)]
#[path = "support_tests.rs"]
mod tests;
