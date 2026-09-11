//! Workspace validation and serialized window mutations for one session.

use std::sync::Arc;

use crate::mux::protocol::ClientRequest;

use super::context::ServerContext;

pub(super) async fn apply(
    context: &Arc<ServerContext>,
    session: &str,
    request: ClientRequest,
) -> Result<(), String> {
    let workspace = super::workspace::resolve(context, session, &request).await?;
    let mut state = context.state.write().await;
    let next = state.next_window_id();
    let item = state
        .session_mut(session)
        .ok_or_else(|| format!("unknown mux session '{session}'"))?;
    let mut closed = None;
    match request {
        ClientRequest::CreateWindow { .. } => item.create_window(next, required(workspace)?),
        ClientRequest::SelectWindow { id } => {
            item.select_window(id).map_err(|error| error.to_string())?
        }
        ClientRequest::CloseWindow { id } => {
            item.close_window(id).map_err(|error| error.to_string())?;
            closed = Some(id);
        }
        ClientRequest::ChangeDirectory { .. } => item.change_directory(required(workspace)?),
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
