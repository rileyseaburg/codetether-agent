//! Graceful mux session close for in-process user interfaces.

use anyhow::{Result, bail};

use crate::mux::client::MuxConnection;
use crate::mux::protocol::{ClientRequest, ServerResponse};

/// Close a named session; its server persists state and exits with its last session.
pub(crate) async fn stop_session(name: &str) -> Result<()> {
    let target = crate::mux::registry::load(name).await?;
    let mut connection = MuxConnection::connect_server(&target.record).await?;
    let request = ClientRequest::CloseSession {
        name: name.to_string(),
    };
    match connection.request(request).await? {
        ServerResponse::Snapshot { .. } | ServerResponse::ShuttingDown => Ok(()),
        ServerResponse::Error { message } => bail!(message),
        _ => bail!("mux server returned an invalid close response"),
    }
}
