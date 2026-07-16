//! Event-driven proxy between a terminal and server-owned PTY.

mod detach;
mod input;
mod output;

use std::io::Write;

use anyhow::Result;
use tokio::io::AsyncReadExt;

use super::connection::MuxConnection;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Outcome {
    Detached,
    Exited,
}

pub(super) async fn run(
    connection: &mut MuxConnection,
    id: u64,
    offset: u64,
    alternate_screen: bool,
) -> Result<Outcome> {
    let mut terminal = super::terminal::ProxyTerminal::enter(alternate_screen)?;
    let mut stdin = tokio::io::stdin();
    let mut input = [0_u8; 4096];
    let mut detector = detach::Detector::new();
    let output_connection = connection.secondary().await?;
    let mut output = output::start(output_connection, id, offset);
    loop {
        tokio::select! {
            count = stdin.read(&mut input) => {
                let count = count?;
                if count == 0 { return Ok(Outcome::Detached); }
                let filtered = detector.filter(&input[..count]);
                if !filtered.data.is_empty() { input::send(connection, id, filtered.data).await?; }
                if filtered.detach {
                    terminal.clear_after_detach();
                    return Ok(Outcome::Detached);
                }
            }
            event = output.recv() => {
                let event = event.ok_or_else(|| anyhow::anyhow!("mux output connection closed"))??;
                terminal.observe_output(&event.data);
                std::io::stdout().write_all(&event.data)?;
                std::io::stdout().flush()?;
                if event.exited { return Ok(Outcome::Exited); }
            }
        }
    }
}
