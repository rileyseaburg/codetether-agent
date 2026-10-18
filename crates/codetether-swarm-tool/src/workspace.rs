//! Parent workspace selection and required-isolation enforcement.
use std::path::PathBuf;
/// Select the inherited workspace or process current directory.
/// # Arguments
/// * `params` — Tool invocation JSON.
/// # Returns
/// Inherited path, current directory, or a relative dot fallback.
/// # Examples
/// ```no_run
/// assert_eq!(codetether_swarm_tool::support::workspace(&serde_json::json!({"__ct_parent_workspace":"/tmp/repo"})), std::path::PathBuf::from("/tmp/repo"));
/// ```
pub fn workspace(params: &serde_json::Value) -> PathBuf {
    params
        .get("__ct_parent_workspace")
        .and_then(serde_json::Value::as_str)
        .map(PathBuf::from)
        .or_else(|| std::env::current_dir().ok())
        .unwrap_or_else(|| PathBuf::from("."))
}

/// Require isolation for mutating tasks and allow parent checkout for read-only tasks.
/// # Arguments
/// * `isolated` — Allocated worktree, if any.
/// * `read_only` — Whether the task is read-only.
/// * `parent` — Parent repository path.
/// # Returns
/// The selected working directory.
/// # Errors
/// Fails when a mutating task has no worktree.
/// # Examples
/// ```no_run
/// assert!(codetether_swarm_tool::support::working_directory(None, false, std::path::Path::new(".")).is_err());
/// ```
pub fn working_directory(
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
