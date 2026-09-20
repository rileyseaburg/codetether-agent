//! PTY input request forwarding.

#[path = "input/clipboard.rs"]
mod clipboard;
pub(super) use clipboard::Resolver;
#[path = "input/pipeline.rs"]
mod pipeline;
pub(super) use pipeline::Pipeline;

use anyhow::{Result, bail};

use crate::mux::protocol::{ClientRequest, ProgramRequest, ServerResponse};

use super::super::connection::MuxConnection;

const INPUT_CHUNK_BYTES: usize = 64 * 1024;

pub(super) async fn send(connection: &mut MuxConnection, id: u64, data: Vec<u8>) -> Result<()> {
    for chunk in chunks(&data) {
        send_chunk(connection, id, chunk.to_vec()).await?;
    }
    Ok(())
}

async fn send_chunk(connection: &mut MuxConnection, id: u64, data: Vec<u8>) -> Result<()> {
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

fn chunks(data: &[u8]) -> std::slice::Chunks<'_, u8> {
    data.chunks(INPUT_CHUNK_BYTES)
}

#[cfg(test)]
#[path = "input/input_tests.rs"]
mod tests;
