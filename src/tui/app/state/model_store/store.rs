//! Filesystem-backed local store for the TUI model picker list.
//!
//! Loading models from every provider hits the network and can take many
//! seconds. This store persists the last successful list under the agent data
//! directory so the picker opens instantly while a fresh list is fetched in
//! the background.
//!
//! The store location follows the same resolution as the rest of the agent:
//! `CODETETHER_DATA_DIR` → workspace `.codetether-agent` → platform data dir.

use std::path::PathBuf;

/// Filesystem-backed cache of the available model identifiers.
#[derive(Debug, Clone)]
pub struct ModelStore {
    path: PathBuf,
}

impl ModelStore {
    /// Create a store at the default agent data location, if resolvable.
    pub fn open_default() -> Option<Self> {
        let base = crate::config::Config::data_dir()?;
        Some(Self::with_base(base))
    }

    /// Create a store rooted at `base` (primarily for tests).
    pub fn with_base(base: impl Into<PathBuf>) -> Self {
        let path = base.into().join("model-store").join("models.json");
        Self { path }
    }

    /// Path to the backing JSON file.
    pub fn path(&self) -> &std::path::Path {
        &self.path
    }
}
