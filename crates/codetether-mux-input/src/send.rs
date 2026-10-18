//! Forward PTY input through bounded mux protocol frames.
use anyhow::{Result, bail};
use codetether_mux_client::MuxConnection;
use codetether_mux_protocol::{ClientRequest, ProgramRequest, ServerResponse};

const INPUT_CHUNK_BYTES: usize = 64 * 1024;

/// Send transformed terminal input using bounded protocol chunks.
/// # Arguments
/// `connection` supplies authenticated transport; `id` selects the window;
/// `data` contains bytes after local detach and paste processing.
/// # Returns
/// Unit once all input chunks have been acknowledged.
/// # Errors
/// Reports transport failures, server errors or unexpected response types.
/// # Examples
/// ```no_run
/// # async fn example(connection: &mut codetether_mux_client::MuxConnection) -> anyhow::Result<()> {
/// codetether_mux_input::send(connection, 0, b"ls\r".to_vec()).await?;
/// # Ok(()) }
/// ```
pub async fn send(connection: &mut MuxConnection, id: u64, data: Vec<u8>) -> Result<()> {
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
#[path = "input_tests.rs"]
mod tests;
