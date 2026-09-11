//! Session creation on a workspace mux server.

use std::path::PathBuf;
use std::sync::Arc;

use crate::mux::protocol::ServerResponse;
use crate::mux::pty::TerminalSize;

use super::context::ServerContext;
use super::dispatch::error;

/// Create an isolated session and start its login shell in its first window.
///
/// The server guards uniqueness among its own sessions; global uniqueness
/// across servers is enforced by the client before it asks to join.
pub(super) async fn create(
    context: &Arc<ServerContext>,
    name: String,
    requested: PathBuf,
) -> ServerResponse {
    if let Err(failure) = crate::mux::registry::validate_name(&name) {
        return error(&failure.to_string());
    }
    let workspace = match super::workspace::isolate(context, &name, &requested).await {
        Ok(workspace) => workspace,
        Err(message) => return error(&message),
    };
    let created = context
        .state
        .write()
        .await
        .create_session(name.clone(), workspace.clone());
    let window = match created {
        Ok(window) => window,
        Err(failure) => return error(&failure.to_string()),
    };
    let shell = crate::mux::pty::default_shell::command();
    let size = TerminalSize::new(80, 24);
    if let Err(failure) = context
        .programs
        .start(window, &shell, &workspace, size, &name)
    {
        let _ = context.state.write().await.close_session(&name);
        return error(&failure.to_string());
    }
    super::session_close::persisted(context).await
}
