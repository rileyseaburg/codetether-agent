//! PTY input request forwarding.

use anyhow::{Result, bail};

use crate::mux::protocol::{ClientRequest, ProgramRequest, ServerResponse};

use super::super::connection::MuxConnection;

pub(super) async fn send(connection: &mut MuxConnection, id: u64, data: Vec<u8>) -> Result<()> {
    match connection
        .request(ClientRequest::Program {
            request: ProgramRequest::Input {
                window_id: id,
                data,
            },
        })
        .await?
    {
        ServerResponse::Acknowledged => Ok(()),
        ServerResponse::Error { message } => bail!(message),
        _ => bail!("mux server returned an invalid PTY input response"),
    }
}
