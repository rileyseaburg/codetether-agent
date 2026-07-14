//! Window mutations initiated by an in-process user interface.

use std::path::PathBuf;

use anyhow::{Context, Result, bail};

use super::MuxSessionSummary;
use crate::mux::client::MuxConnection;
use crate::mux::protocol::{ClientRequest, ServerResponse};

/// Add a workspace window to a named mux session.
pub(crate) async fn create_window(name: &str, workspace: PathBuf) -> Result<MuxSessionSummary> {
    request(name, ClientRequest::CreateWindow { workspace }).await
}

/// Make a window active in a named mux session.
pub(crate) async fn select_window(name: &str, id: u64) -> Result<MuxSessionSummary> {
    request(name, ClientRequest::SelectWindow { id }).await
}

/// Close a window in a named mux session.
pub(crate) async fn close_window(name: &str, id: u64) -> Result<MuxSessionSummary> {
    request(name, ClientRequest::CloseWindow { id }).await
}

async fn request(name: &str, request: ClientRequest) -> Result<MuxSessionSummary> {
    crate::mux::registry::validate_name(name)?;
    let record = crate::mux::registry::load(name)
        .await
        .with_context(|| format!("mux session '{name}' was not found"))?;
    let mut connection = MuxConnection::connect(&record).await?;
    match connection.request(request).await? {
        ServerResponse::Snapshot { state } => Ok(MuxSessionSummary::from_state(&record, &state)),
        ServerResponse::Error { message } => bail!(message),
        _ => bail!("mux server returned an invalid mutation response"),
    }
}
