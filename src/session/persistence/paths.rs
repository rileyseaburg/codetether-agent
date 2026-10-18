//! Validated session snapshot paths with worktree-independent resolution.

use std::path::PathBuf;

use anyhow::Result;

use crate::session::Session;

impl Session {
    /// Resolve the configured sessions directory.
    pub(crate) fn sessions_dir() -> Result<PathBuf> {
        codetether_session_location::sessions_dir()
    }

    /// Resolve a durable session by ID across mux worktree ownership changes.
    pub(crate) fn session_path(id: &str) -> Result<PathBuf> {
        codetether_session_store::lifecycle::session_path(id)
    }
}
