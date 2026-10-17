//! Stable lock filenames for workspace clones.
use std::path::PathBuf;
/// Derive a lock path from a sanitized workspace ID and configured base directory.
///
/// # Arguments
/// * `workspace_id` - Workspace identifier; unsafe filename characters become underscores.
/// * `resolve_base` - Resolve the directory after sanitizing the identifier.
/// # Returns
/// A hidden per-workspace clone lock path.
/// # Examples
/// ```rust
/// use codetether_worktree::repository::clone_lock::workspace_clone_lock_path;
/// let path = workspace_clone_lock_path("a/b", || "/tmp/clones".into());
/// assert_eq!(path.file_name().unwrap(), ".a_b.clone.lock");
/// ```

pub fn workspace_clone_lock_path(
    workspace_id: &str,
    resolve_base: impl FnOnce() -> PathBuf,
) -> PathBuf {
    let safe_id: String = workspace_id
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_') {
                ch
            } else {
                '_'
            }
        })
        .collect();
    resolve_base().join(format!(".{safe_id}.clone.lock"))
}
