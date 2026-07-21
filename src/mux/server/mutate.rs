//! Workspace validation and serialized mux state mutations.

use std::sync::Arc;

use crate::mux::protocol::ClientRequest;

use super::context::ServerContext;

pub(super) async fn apply(
    context: &Arc<ServerContext>,
    request: ClientRequest,
) -> Result<(), String> {
    let workspace = super::workspace::resolve(context, &request).await?;
    let mut state = context.state.write().await;
    let mut closed = None;
    match request {
        ClientRequest::CreateWindow { .. } => state.create_window(required(workspace)?),
        ClientRequest::SelectWindow { id } => {
            state.select_window(id).map_err(|error| error.to_string())?
        }
        ClientRequest::CloseWindow { id } => {
            state.close_window(id).map_err(|error| error.to_string())?;
            closed = Some(id);
        }
        ClientRequest::ChangeDirectory { .. } => state.change_directory(required(workspace)?),
        _ => return Err("unsupported mutation".into()),
    }
    drop(state);
    if let Some(id) = closed {
        context.programs.stop(id);
    }
    context.persist().await.map_err(|error| error.to_string())
}

fn required(workspace: Option<std::path::PathBuf>) -> Result<std::path::PathBuf, String> {
    workspace.ok_or_else(|| "workspace is required".into())
}
