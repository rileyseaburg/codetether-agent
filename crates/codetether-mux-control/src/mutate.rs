//! Window mutations initiated by an in-process user interface.

use std::path::PathBuf;

use anyhow::{Result, bail};

use super::MuxSessionSummary;
use codetether_mux_client::MuxConnection;
use codetether_mux_protocol::{ClientRequest, ServerResponse};

/// Add a workspace window to a named mux session.
///
/// # Arguments
/// name identifies the session; workspace is the directory to open.
/// # Returns
/// The updated session summary.
/// # Errors
/// Returns registry, connection or response errors.
/// # Examples
/// ```no_run
/// # async fn example() -> anyhow::Result<()> {
/// let session = codetether_mux_control::create_window("work", ".".into()).await?;
/// # Ok(()) }
/// ```
pub async fn create_window(name: &str, workspace: PathBuf) -> Result<MuxSessionSummary> {
    request(name, ClientRequest::CreateWindow { workspace }).await
}

/// Make a window active in a named mux session.
///
/// # Arguments
/// name identifies the session; id identifies the window.
/// # Returns
/// The updated session summary.
/// # Errors
/// Returns registry, connection or response errors.
/// # Examples
/// ```no_run
/// # async fn example() -> anyhow::Result<()> {
/// let session = codetether_mux_control::select_window("work", 1).await?;
/// # Ok(()) }
/// ```
pub async fn select_window(name: &str, id: u64) -> Result<MuxSessionSummary> {
    request(name, ClientRequest::SelectWindow { id }).await
}

/// Close a window in a named mux session.
///
/// # Arguments
/// name identifies the session; id identifies the window.
/// # Returns
/// The updated session summary.
/// # Errors
/// Returns registry, connection or response errors.
/// # Examples
/// ```no_run
/// # async fn example() -> anyhow::Result<()> {
/// let session = codetether_mux_control::close_window("work", 1).await?;
/// # Ok(()) }
/// ```
pub async fn close_window(name: &str, id: u64) -> Result<MuxSessionSummary> {
    request(name, ClientRequest::CloseWindow { id }).await
}

async fn request(name: &str, request: ClientRequest) -> Result<MuxSessionSummary> {
    let target = codetether_mux_registry::load(name).await?;
    let mut connection = MuxConnection::connect(&target).await?;
    match connection.request(request).await? {
        ServerResponse::Snapshot { state } => Ok(MuxSessionSummary::from_state(
            &target.record,
            &state,
            name,
            true,
        )),
        ServerResponse::Error { message } => bail!(message),
        _ => bail!("mux server returned an invalid mutation response"),
    }
}
