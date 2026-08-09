//! PTY input request forwarding.

#[path = "input/clipboard.rs"]
mod clipboard;

use anyhow::{Result, bail};

use crate::mux::protocol::{ClientRequest, ProgramRequest, ServerResponse};

use super::super::connection::MuxConnection;

pub(super) async fn send(connection: &mut MuxConnection, id: u64, raw: Vec<u8>) -> Result<()> {
    let data = clipboard::resolve(raw);
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
