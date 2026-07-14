//! Graceful mux shutdown for in-process user interfaces.

use anyhow::{Context, Result, bail};

use crate::mux::client::MuxConnection;
use crate::mux::protocol::{ClientRequest, ServerResponse};

/// Ask a named mux server to persist and terminate.
pub(crate) async fn stop_session(name: &str) -> Result<()> {
    crate::mux::registry::validate_name(name)?;
    let record = crate::mux::registry::load(name)
        .await
        .with_context(|| format!("mux session '{name}' was not found"))?;
    let mut connection = MuxConnection::connect(&record).await?;
    match connection.request(ClientRequest::Shutdown).await? {
        ServerResponse::ShuttingDown => Ok(()),
        ServerResponse::Error { message } => bail!(message),
        _ => bail!("mux server returned an invalid shutdown response"),
    }
}
