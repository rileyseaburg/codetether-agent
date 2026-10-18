//! Decode PTY output replies and advance the stream offset.
use anyhow::{Result, bail};
use codetether_mux_protocol::ServerResponse;

pub(super) struct Event {
    pub data: Vec<u8>,
    pub next_offset: u64,
    pub exited: bool,
}
pub(super) fn decode(response: ServerResponse, offset: &mut u64) -> Result<Event> {
    match response {
        ServerResponse::ProgramOutput {
            data,
            next_offset,
            running,
        } => {
            *offset = next_offset;
            Ok(Event {
                next_offset,
                exited: !running && data.is_empty(),
                data,
            })
        }
        ServerResponse::Error { message } => bail!(message),
        _ => bail!("mux server returned an invalid PTY output response"),
    }
}
