//! Start and reconnect flows for server-owned terminal programs.

use anyhow::{Result, bail};

use crate::mux::protocol::{ClientRequest, ServerResponse};

use super::connection::MuxConnection;

pub(super) async fn start(
    connection: &mut MuxConnection,
    id: u64,
    command: String,
) -> Result<super::proxy::Outcome> {
    let offset = super::exec::start(connection, id, command).await?;
    super::proxy::run(connection, id, offset).await
}

pub(super) async fn attach(
    connection: &mut MuxConnection,
    id: u64,
) -> Result<Option<super::proxy::Outcome>> {
    let (columns, rows) = crossterm::terminal::size().unwrap_or((80, 24));
    let response = connection
        .request(ClientRequest::AttachProgram {
            window_id: id,
            columns,
            rows,
        })
        .await?;
    match response {
        ServerResponse::ProgramAttached { offset, .. } => {
            Ok(Some(super::proxy::run(connection, id, offset).await?))
        }
        ServerResponse::Error { message } if message.contains("no running program") => Ok(None),
        ServerResponse::Error { message } => bail!(message),
        _ => bail!("mux server returned an invalid program-attach response"),
    }
}
