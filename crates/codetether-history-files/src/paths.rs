//! Select the history directory and construct canonical per-turn filenames.
use anyhow::Result;
use std::path::{Path, PathBuf};

/// Construct a stable path for one absolute turn index.
///
/// # Arguments
/// * `session_dir` - Session history directory.
/// * `turn` - Absolute zero-based transcript index.
/// * `role` - Stable role label.
/// # Returns
/// A path named `turn-NNNN-role.md` under the directory.
/// # Examples
/// ```rust
/// let path=codetether_history_files::format_turn_path(std::path::Path::new("history"),7,"user");
/// assert!(path.ends_with("turn-0007-user.md"));
/// ```
pub fn format_turn_path(session_dir: &Path, turn: usize, role: &str) -> PathBuf {
    session_dir.join(format!("turn-{turn:04}-{role}.md"))
}

/// Resolve the session history directory using the configured data location.
///
/// # Arguments
/// * `session_id` - Durable session identifier.
/// * `workspace` - Optional workspace whose Git root locates local data.
/// # Returns
/// Data directory joined with `history` and the session identifier.
/// # Errors
/// No errors are currently produced; the result preserves the caller contract.
/// # Examples
/// ```rust
/// let path=codetether_history_files::history_dir("session",None)?;
/// assert!(path.ends_with("history/session"));
/// # Ok::<(),anyhow::Error>(())
/// ```
pub fn history_dir(session_id: &str, workspace: Option<&Path>) -> Result<PathBuf> {
    let data_dir = if let Ok(explicit) = std::env::var("CODETETHER_DATA_DIR") {
        let explicit = explicit.trim();
        if !explicit.is_empty() {
            PathBuf::from(explicit)
        } else {
            workspace_data_dir(workspace)
        }
    } else {
        workspace_data_dir(workspace)
    };
    Ok(data_dir.join("history").join(session_id))
}

fn workspace_data_dir(workspace: Option<&Path>) -> PathBuf {
    workspace
        .map(|start| {
            start
                .ancestors()
                .find(|path| path.join(".git").exists())
                .unwrap_or(start)
                .join(".codetether-agent")
        })
        .or_else(codetether_config::Config::data_dir)
        .unwrap_or_else(|| PathBuf::from(".codetether-agent"))
}
