//! Joining an existing workspace server with a new isolated session.

use std::path::{Path, PathBuf};

use anyhow::{Result, bail};

use crate::mux::client::MuxConnection;
use crate::mux::protocol::{ClientRequest, ServerResponse};
use crate::mux::registry::{MuxRecord, SessionTarget};

/// The reachable server already bound to `workspace`, if any.
pub(in crate::mux) async fn live_server(workspace: &Path) -> Option<MuxRecord> {
    let record = crate::mux::registry::find_workspace(workspace).await?;
    let probe = tokio::time::timeout(
        std::time::Duration::from_millis(500),
        crate::mux::client::probe(&record),
    )
    .await;
    probe.is_ok_and(|result| result.is_ok()).then_some(record)
}

/// Ask a live server to host one more isolated session.
pub(in crate::mux) async fn create_session(
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
