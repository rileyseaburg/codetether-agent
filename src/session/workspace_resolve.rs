//! Resolve the workspace directory a durable session was last recorded in.
//!
//! `codetether mux resume --session <id>` must start the mux server in the same
//! workspace the session belongs to, otherwise the resumed TUI would look for
//! the transcript in an unrelated directory. The session snapshot already
//! records its own workspace in `metadata.directory`, so the snapshot is the
//! authoritative source rather than the caller's current directory.
//!
//! Only the workspace field is parsed: session files reach multiple megabytes
//! and the whole transcript is not needed to read one path.

use anyhow::{Context, Result};
use serde::Deserialize;
use std::path::{Path, PathBuf};

use crate::session::types::Session;

/// Minimal projection of a session snapshot: only the recorded workspace.
///
/// Deliberately not [`crate::session::header::SessionHeader`], which embeds the
/// full [`SessionMetadata`](crate::session::types::SessionMetadata) and so fails
/// on snapshots predating any later-added required field. Resume must keep
/// working for old sessions, so this reads one optional path and nothing else.
#[derive(Deserialize)]
struct WorkspaceProjection {
    #[serde(default)]
    metadata: ProjectedMetadata,
}

#[derive(Default, Deserialize)]
struct ProjectedMetadata {
    #[serde(default)]
    directory: Option<PathBuf>,
}

impl Session {
    /// Returns the workspace directory recorded for session `id`.
    ///
    /// When the snapshot records no workspace, the directory is derived from the
    /// snapshot's own location so sessions written before `metadata.directory`
    /// existed remain resumable.
    ///
    /// # Errors
    ///
    /// Returns an error if `id` is malformed, the snapshot is missing, the JSON
    /// cannot be parsed, or no directory can be determined.
    pub async fn recorded_workspace(id: &str) -> Result<PathBuf> {
        let path = Self::session_path(id)?;
        let body = tokio::fs::read_to_string(&path)
            .await
            .with_context(|| format!("read session {id}"))?;
        let projection: WorkspaceProjection =
            serde_json::from_str(&body).with_context(|| format!("parse session {id}"))?;
        if let Some(directory) = projection.metadata.directory.filter(|item| item.is_dir()) {
            return Ok(directory);
        }
        enclosing_workspace(&path).with_context(|| format!("resolve workspace for session {id}"))
    }
}

/// Derives the workspace from the snapshot path, which is laid out as
/// `<workspace>/.codetether-agent/sessions/<id>.json`.
fn enclosing_workspace(session_file: &Path) -> Option<PathBuf> {
    session_file
        .ancestors()
        .nth(3)
        .filter(|path| path.is_dir())
        .map(PathBuf::from)
}

#[cfg(test)]
#[path = "workspace_resolve_tests.rs"]
mod tests;
