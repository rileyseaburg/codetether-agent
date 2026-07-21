//! Workspace resolution and isolation for mux window mutations.

use std::path::PathBuf;

use crate::mux::protocol::ClientRequest;

use super::context::ServerContext;

pub(super) async fn resolve(
    context: &ServerContext,
    request: &ClientRequest,
) -> Result<Option<PathBuf>, String> {
    match request {
        ClientRequest::CreateWindow { workspace } => isolate(context, workspace).await.map(Some),
        ClientRequest::ChangeDirectory { workspace } => canonical(workspace).await.map(Some),
        _ => Ok(None),
    }
}

async fn isolate(context: &ServerContext, requested: &std::path::Path) -> Result<PathBuf, String> {
    let (session, slot) = {
        let state = context.state.read().await;
        let slot = state
            .windows
            .iter()
            .map(|window| window.id)
            .max()
            .unwrap_or(0)
            + 1;
        (state.name.clone(), slot)
    };
    crate::mux::isolation::workspace(&session, slot, requested)
        .await
        .map_err(|error| error.to_string())
}

async fn canonical(path: &std::path::Path) -> Result<PathBuf, String> {
    tokio::fs::canonicalize(path)
        .await
        .map_err(|error| format!("invalid workspace: {error}"))
}
