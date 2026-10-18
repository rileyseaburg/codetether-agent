//! Optional workspace-only deserialization compatible with older snapshots.
use serde::Deserialize;
use std::path::PathBuf;

/// Minimal projection of a session snapshot: only the recorded workspace.
///
/// Deliberately not the full native session header, which embeds the
/// full native session metadata and so fails
/// on snapshots predating any later-added required field. Resume must keep
/// working for old sessions, so this reads one optional path and nothing else.
#[derive(Deserialize)]
pub(super) struct WorkspaceProjection {
    #[serde(default)]
    pub(super) metadata: ProjectedMetadata,
}

#[derive(Default, Deserialize)]
pub(super) struct ProjectedMetadata {
    #[serde(default)]
    pub(super) directory: Option<PathBuf>,
}
