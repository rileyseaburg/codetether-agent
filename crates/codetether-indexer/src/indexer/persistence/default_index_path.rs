//! default index path for codebase indexing.

use super::*;

/// Derive the persistent index path from the workspace identity and data directory.
///
/// # Arguments
/// * `root` - Workspace root whose path is hashed into the filename.
/// # Returns
/// A JSON path under the configured data directory, with a workspace-local fallback.
/// # Examples
/// ```
/// let path = codetether_indexer::default_index_path(std::path::Path::new("/workspace"));
/// assert_eq!(path.extension().and_then(|value| value.to_str()), Some("json"));
/// ```
pub fn default_index_path(root: &Path) -> PathBuf {
    let mut hasher = Sha256::new();
    hasher.update(root.to_string_lossy().as_bytes());
    let digest = hasher.finalize();
    let short = hex::encode(digest);
    let short = &short[..16];

    let base = codetether_paths::data_dir().unwrap_or_else(|| root.join(".codetether-agent"));
    base.join("indexes")
        .join(format!("codebase-index-{short}.json"))
}
