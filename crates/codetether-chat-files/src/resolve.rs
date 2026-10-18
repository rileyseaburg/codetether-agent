//! Resolve the most-recently-referenced file path from the transcript.

use std::path::{Path, PathBuf};

use codetether_chat_model::ChatMessage;

use super::extract::path_from_message;

/// Absolute path of the most recently referenced file, if any.
///
/// Scans the transcript newest-first for a shared `File` message or a
/// file-oriented tool call (`read`/`write`/`edit`/…), returning the first
/// resolved path that exists on disk.
///
/// # Arguments
/// * `messages` — Transcript in chronological order.
/// * `workspace_dir` — Base directory for relative file references.
/// # Returns
/// The newest reference resolving to an existing file, or `None`.
/// # Examples
/// ```rust
/// use codetether_chat_files::latest_shared_file;
/// assert!(latest_shared_file(&[], std::path::Path::new(".")).is_none());
/// ```
pub fn latest_shared_file(messages: &[ChatMessage], workspace_dir: &Path) -> Option<PathBuf> {
    messages
        .iter()
        .rev()
        .filter_map(|m| path_from_message(m, workspace_dir))
        .find(|p| p.is_file())
}
