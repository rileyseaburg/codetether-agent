//! Store root and constructors.

use anyhow::{Result, anyhow};
use std::path::PathBuf;

use super::path::thread_path;

/// Append-only JSONL storage for thread events.
///
/// # Examples
///
/// ```
/// use codetether_agent::session::thread_store::ThreadStore;
///
/// let store = ThreadStore::new("/tmp/codetether-threads");
/// ```
#[derive(Debug, Clone)]
pub struct ThreadStore {
    pub(super) root: PathBuf,
}

impl ThreadStore {
    /// Create a store rooted at `root`.
    ///
    /// # Arguments
    ///
    /// * `root` - Directory containing `<thread-id>.jsonl` files.
    ///
    /// # Returns
    ///
    /// A store handle. The directory is created lazily on append.
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    /// Create a store rooted at `<Config::data_dir()>/threads`.
    ///
    /// # Errors
    ///
    /// Returns an error when the platform data directory cannot be resolved.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use codetether_agent::session::thread_store::ThreadStore;
    ///
    /// let store = ThreadStore::from_config().expect("data dir should resolve");
    /// ```
    pub fn from_config() -> Result<Self> {
        let data_dir = crate::config::Config::data_dir()
            .ok_or_else(|| anyhow!("Could not determine data directory"))?;
        Ok(Self::new(data_dir.join("threads")))
    }

    pub(super) fn path_for(&self, thread_id: &str) -> Result<PathBuf> {
        thread_path(&self.root, thread_id)
    }
}
