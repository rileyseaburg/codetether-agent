//! Validated session snapshot paths with worktree-independent resolution.

use std::path::PathBuf;

use anyhow::Result;

use crate::session::Session;

impl Session {
    /// Resolve the configured sessions directory.
    pub(crate) fn sessions_dir() -> Result<PathBuf> {
        crate::config::Config::data_dir()
            .map(|directory| directory.join("sessions"))
            .ok_or_else(|| anyhow::anyhow!("Could not determine data directory"))
    }

    /// Resolve a durable session by ID across mux worktree ownership changes.
    pub(crate) fn session_path(id: &str) -> Result<PathBuf> {
        if id.is_empty()
            || id.len() > 128
            || id.contains(|character: char| {
                !character.is_alphanumeric() && character != '-' && character != '_'
            })
        {
            anyhow::bail!("Invalid session ID:rejecting path traversal risk");
        }
        let local = Self::sessions_dir()?.join(format!("{id}.json"));
        super::location::resolve(id, local)
    }
}
