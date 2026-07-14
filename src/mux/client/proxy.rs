//! Bidirectional polling proxy between a terminal and server-owned PTY.

mod detach;
mod input;

use std::io::Write;

use anyhow::{Result, bail};
use tokio::io::AsyncReadExt;

use crate::mux::protocol::{ClientRequest, ServerResponse};

use super::connection::MuxConnection;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Outcome {
    Detached,
    Exited,
}

pub(super) async fn run(
    connection: &mut MuxConnection,
    id: u64,
    mut offset: u64,
) -> Result<Outcome> {
    let _terminal = super::terminal::ProxyTerminal::enter()?;
    let mut stdin = tokio::io::stdin();
    let mut input = [0_u8; 4096];
    let mut detector = detach::Detector::new();
    let mut poll = tokio::time::interval(std::time::Duration::from_millis(16));
    loop {
        tokio::select! {
            count = stdin.read(&mut input) => {
                let count = count?;
                if count == 0 { return Ok(Outcome::Detached); }
                let filtered = detector.filter(&input[..count]);
                if !filtered.data.is_empty() { input::send(connection, id, filtered.data).await?; }
                if filtered.detach { return Ok(Outcome::Detached); }
            }
            _ = poll.tick() => {
                let response = connection.request(ClientRequest::ReadProgram { window_id: id, offset }).await?;
                let ServerResponse::ProgramOutput { data, next_offset, running } = response else {
                    bail!("mux server returned an invalid PTY output response");
                };
                let drained = data.is_empty();
                std::io::stdout().write_all(&data)?;
                std::io::stdout().flush()?;
                offset = next_offset;
                if !running && drained { return Ok(Outcome::Exited); }
            }
        }
    }
}
