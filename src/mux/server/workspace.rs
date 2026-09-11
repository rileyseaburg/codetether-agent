//! Workspace resolution and isolation for session window mutations.

use std::path::{Path, PathBuf};

use crate::mux::protocol::ClientRequest;

use super::context::ServerContext;

pub(super) async fn resolve(
    context: &ServerContext,
    session: &str,
    request: &ClientRequest,
) -> Result<Option<PathBuf>, String> {
    match request {
        ClientRequest::CreateWindow { workspace } => {
            isolate(context, session, workspace).await.map(Some)
        }
        ClientRequest::ChangeDirectory { workspace } => canonical(workspace).await.map(Some),
        _ => Ok(None),
    }
}

/// Apply the server's isolation policy to a requested directory for `session`.
///
/// Shared servers hand back the canonical path; worktree servers allocate a
/// managed checkout named after the session and the next server-wide slot.
pub(super) async fn isolate(
    context: &ServerContext,
    session: &str,
    requested: &Path,
) -> Result<PathBuf, String> {
    let (slot, isolation) = {
        let state = context.state.read().await;
        (state.next_window_id(), state.isolation)
    };
    crate::mux::isolation::workspace(session, slot, requested, isolation)
        .await
        .map_err(|error| error.to_string())
}

async fn canonical(path: &Path) -> Result<PathBuf, String> {
    tokio::fs::canonicalize(path)
        .await
        .map_err(|error| format!("invalid workspace: {error}"))
}
