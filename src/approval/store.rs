use crate::config::Config;
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

/// Append-only JSONL store for approval requests and decisions.
#[derive(Debug, Clone)]
pub struct ApprovalStore {
    pub(crate) root: PathBuf,
}

impl ApprovalStore {
    /// Open an approval store at a caller-supplied directory.
    ///
    /// # Errors
    ///
    /// Returns an error if the directory cannot be created.
    pub fn open(root: impl Into<PathBuf>) -> Result<Self> {
        let store = Self { root: root.into() };
        std::fs::create_dir_all(&store.root)?;
        Ok(store)
    }

    /// Open the default store under `Config::data_dir()/approvals`.
    ///
    /// # Errors
    ///
    /// Returns an error if no data directory is available or if the approval
    /// directory cannot be created.
    pub fn open_default() -> Result<Self> {
        let data_dir = Config::data_dir().context("no CodeTether data directory")?;
        Self::open(data_dir.join("approvals"))
    }

    pub(crate) fn log_path(&self) -> PathBuf {
        self.root.join("approvals.jsonl")
    }

    /// Return the directory used by this store.
    pub fn root(&self) -> &Path {
        &self.root
    }
}
