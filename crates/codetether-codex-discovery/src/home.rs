//! Resolve the Codex archive home from configuration or rollout ancestry.
use std::path::{Path, PathBuf};

/// Resolve CODEX_HOME first, then HOME/.codex.
/// # Returns
/// The configured archive path, or None if neither variable exists.
/// # Examples
/// ```
/// let _home = codetether_codex_discovery::codex_home_dir();
/// ```
pub fn codex_home_dir() -> Option<PathBuf> {
    std::env::var_os("CODEX_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".codex")))
}

/// Find the nearest .codex ancestor of an archive file.
/// # Arguments
/// * `path` — Path whose parent directories should be searched.
/// # Returns
/// The matching ancestor, if present.
/// # Examples
/// ```
/// use std::path::{Path, PathBuf};
/// assert_eq!(codetether_codex_discovery::codex_home_dir_for_path(Path::new("/tmp/.codex/sessions/a.jsonl")), Some(PathBuf::from("/tmp/.codex")));
/// ```
pub fn codex_home_dir_for_path(path: &Path) -> Option<PathBuf> {
    let mut current = path.parent()?;
    loop {
        if current.file_name().and_then(|name| name.to_str()) == Some(".codex") {
            return Some(current.to_path_buf());
        }
        current = current.parent()?;
    }
}
