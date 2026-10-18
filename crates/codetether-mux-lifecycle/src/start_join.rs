//! Joining an existing workspace server with a new isolated session.

use std::path::{Path, PathBuf};

use anyhow::{Result, bail};

use codetether_mux_client::MuxConnection;
use codetether_mux_protocol::{ClientRequest, ServerResponse};
use codetether_mux_registry::{MuxRecord, SessionTarget};

/// Return a reachable server already bound to the workspace, if any.
///
/// # Arguments
/// workspace is the canonical directory to look up.
/// # Returns
/// A server record after a successful bounded probe, or None.
/// # Examples
/// ```no_run
/// # async fn example() {
/// let server = codetether_mux_lifecycle::start_join::live_server(std::path::Path::new(".")).await;
/// # }
/// ```
pub async fn live_server(workspace: &Path) -> Option<MuxRecord> {
    let record = codetether_mux_registry::find_workspace(workspace).await?;
    let probe = tokio::time::timeout(
        std::time::Duration::from_millis(500),
        codetether_mux_client::handshake::probe(&record),
    )
    .await;
    probe.is_ok_and(|result| result.is_ok()).then_some(record)
}

/// Ask a live server to host one more isolated session.
///
/// # Arguments
/// record identifies the server; name is the new session name; workspace is its directory.
/// # Returns
/// The updated server record and selected session.
/// # Errors
/// Returns connection, protocol or server-rejected creation errors.
/// # Examples
/// ```no_run
/// # async fn example() -> anyhow::Result<()> {
/// let record = codetether_mux_registry::list().await?.remove(0);
/// let target = codetether_mux_lifecycle::start_join::create_session(record, "work", ".".into()).await?;
/// # Ok(()) }
/// ```
pub async fn create_session(
    record: MuxRecord,
    name: &str,
    workspace: PathBuf,
) -> Result<SessionTarget> {
    let mut connection = MuxConnection::connect_server(&record).await?;
    let request = ClientRequest::CreateSession {
        name: name.to_string(),
        workspace,
    };
    match connection.request(request).await? {
        ServerResponse::Snapshot { state } => Ok(SessionTarget {
            record: MuxRecord { state, ..record },
            session: name.to_string(),
        }),
        ServerResponse::Error { message } => bail!(message),
        _ => bail!("mux server returned an invalid session-create response"),
    }
}
