//! Server-owned program startup from the mux command prompt.

use anyhow::{Result, bail};

use crate::mux::protocol::{ClientRequest, ProgramRequest, ServerResponse};

use super::connection::MuxConnection;

pub(super) async fn start(
    connection: &mut MuxConnection,
    window_id: u64,
    command: String,
) -> Result<u64> {
    let (columns, rows) = crossterm::terminal::size().unwrap_or((80, 24));
    let response = connection
        .request(ClientRequest::Program {
            request: ProgramRequest::Start {
                window_id,
                command,
                columns,
                rows,
            },
        })
        .await?;
    match response {
        ServerResponse::ProgramAttached { offset, .. } => Ok(offset),
        ServerResponse::Error { message } => bail!(message),
        _ => bail!("mux server returned an invalid program-start response"),
    }
}
