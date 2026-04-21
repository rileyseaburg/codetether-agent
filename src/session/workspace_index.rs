//! Sidecar index mapping workspace directory -> most-recent session id.
//!
//! Eliminates the O(N) directory scan on resume: we just look up the
//! canonical workspace path in a small JSON map and tail-load that one
//! file. On miss (or stale entry), callers fall back to the full scan
//! and repair the index by writing the winner back.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::types::Session;

pub(super) const INDEX_FILENAME: &str = ".workspace_index.json";

#[derive(Debug, Default, Serialize, Deserialize)]
pub(super) struct WorkspaceIndex {
    /// Canonical workspace path (as a string) -> session UUID.
    pub entries: HashMap<String, String>,
}

impl WorkspaceIndex {
    pub(super) fn path() -> Result<PathBuf> {
        Ok(Session::sessions_dir()?.join(INDEX_FILENAME))
    }

    /// Load the index, returning an empty one on any error (missing,
    /// malformed, permission denied). This is a cache — never fatal.
    pub(super) fn load_sync() -> Self {
        let Ok(path) = Self::path() else {
            return Self::default();
        };
        std::fs::read(&path)
            .ok()
            .and_then(|bytes| serde_json::from_slice(&bytes).ok())
            .unwrap_or_default()
    }

    /// Look up `workspace` (canonical path) and return the session id.
    pub(super) fn get(&self, workspace: &Path) -> Option<&str> {
        self.entries
            .get(workspace.to_string_lossy().as_ref())
            .map(String::as_str)
    }

    /// Record a session id for a canonical workspace path.
    pub(super) fn set(&mut self, workspace: &Path, session_id: &str) {
        self.entries.insert(
            workspace.to_string_lossy().into_owned(),
            session_id.to_string(),
        );
    }
}
