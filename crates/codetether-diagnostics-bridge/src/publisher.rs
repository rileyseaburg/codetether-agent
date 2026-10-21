//! One producer's snapshot ownership and atomic replacement.
use crate::{directory, uri};
use anyhow::Result;
use std::{
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
};

/// Owns one private snapshot; dropping it removes only its own file.
/// See the [crate-level example](crate) for construction and publication.
pub struct Publisher {
    pub(super) root: PathBuf,
    pub(super) workspace_uri: String,
    pub(super) server: String,
    pub(super) path: PathBuf,
}

impl Publisher {
    /// Use the per-user cache or CODETETHER_DIAGNOSTICS_DIR override.
    /// # Errors
    /// Returns errors for invalid roots or an unavailable/private cache directory.
    pub fn new(workspace_uri: &str, server: &str) -> Result<Self> {
        Self::in_directory(&directory::default_path()?, workspace_uri, server)
    }

    /// Create an isolated producer in an explicit directory (also useful in tests).
    /// # Errors
    /// Returns directory ownership, permission, URI or canonicalization errors.
    pub fn in_directory(directory: &Path, workspace_uri: &str, server: &str) -> Result<Self> {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        let (root, workspace_uri) = uri::workspace(workspace_uri)?;
        directory::prepare(directory)?;
        let path = directory.join(format!(
            "producer-{}-{}.json",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        Ok(Self {
            root,
            workspace_uri,
            server: server.into(),
            path,
        })
    }

    /// Snapshot filename, for consumers/tests; callers must not write it directly.
    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for Publisher {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.path);
    }
}
