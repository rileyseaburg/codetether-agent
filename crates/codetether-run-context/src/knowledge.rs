//! Opt-in workspace snapshot preparation for a run.
use std::path::{Path, PathBuf};

const ENV_RUN_KNOWLEDGE_SNAPSHOT: &str = "CODETETHER_RUN_KNOWLEDGE_SNAPSHOT";

/// Refresh the workspace snapshot when explicitly enabled by the environment.
///
/// `CODETETHER_RUN_KNOWLEDGE_SNAPSHOT` accepts 1, true, yes and on, ignoring
/// case and surrounding whitespace. Failures are logged and remain nonfatal.
/// # Arguments
/// * `workspace_dir` - Workspace to index.
/// # Returns
/// The written snapshot path, or none when disabled or refresh fails.
/// # Examples
/// ```no_run
/// # async fn example() {
/// let snapshot = codetether_run_context::refresh_knowledge(std::path::Path::new(".")).await;
/// # }
/// ```
pub async fn refresh(workspace_dir: &Path) -> Option<PathBuf> {
    if !enabled() {
        tracing::debug!(
            env = ENV_RUN_KNOWLEDGE_SNAPSHOT,
            "Skipping workspace knowledge snapshot for run"
        );
        return None;
    }

    match codetether_indexer::refresh_workspace_knowledge_snapshot(workspace_dir).await {
        Ok(path) => {
            tracing::info!(
                workspace = %workspace_dir.display(),
                output = %path.display(),
                "Refreshed workspace knowledge snapshot for run"
            );
            Some(path)
        }
        Err(error) => {
            tracing::warn!(
                workspace = %workspace_dir.display(),
                error = %error,
                "Failed to refresh workspace knowledge snapshot"
            );
            None
        }
    }
}

fn enabled() -> bool {
    std::env::var(ENV_RUN_KNOWLEDGE_SNAPSHOT).is_ok_and(|value| enabled_value(&value))
}

fn enabled_value(value: &str) -> bool {
    matches!(
        value.trim().to_ascii_lowercase().as_str(),
        "1" | "true" | "yes" | "on"
    )
}

#[cfg(test)]
#[path = "knowledge_tests.rs"]
mod tests;
