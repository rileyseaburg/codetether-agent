//! tree entry for filesystem inspection.
use super::*;

/// Entry with resolved metadata for sorting
pub(crate) struct TreeEntry {
    pub(crate) name: String,
    pub(crate) path: std::path::PathBuf,
    pub(crate) is_dir: bool,
    pub(crate) size: u64,
}
