//! Graceful mux session close for in-process user interfaces.

use anyhow::{Result, bail};

use codetether_mux_client::MuxConnection;
use codetether_mux_protocol::{ClientRequest, ServerResponse};

/// Close a named session; its server exits with its last session.
///
/// # Arguments
/// name identifies the hosted session.
/// # Returns
/// Unit after the server acknowledges closure.
/// # Errors
/// Returns registry, connection or response errors.
/// # Examples
/// ```no_run
/// # async fn example() -> anyhow::Result<()> {
/// codetether_mux_control::stop_session("work").await?;
/// # Ok(()) }
/// ```
pub async fn stop_session(name: &str) -> Result<()> {
    let target = codetether_mux_registry::load(name).await?;
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
