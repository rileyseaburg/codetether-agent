//! Event-driven terminal input and remote PTY output attachment.
use crate::{Outcome, output, presentation};
use anyhow::Result;
use codetether_mux_client::MuxConnection;
use tokio::io::AsyncReadExt;

/// Attach the local terminal to an authenticated server-owned PTY.
/// # Arguments
/// `connection` and `id` select the PTY; `offset` and `replay_until` bound initial
/// replay; `alternate_screen` initializes screen tracking; `clipboard` supplies
/// local paste content only when the input pipeline needs it.
/// # Returns
/// Whether the attachment detached or the remote program exited.
/// # Errors
/// Reports terminal setup, input, output, transport and protocol failures.
/// # Examples
/// ```no_run
/// # async fn example(connection: &mut codetether_mux_client::MuxConnection) -> anyhow::Result<()> {
/// let _outcome = codetether_mux_attachment::run(connection, 0, 0, 0, false, || None).await?;
/// # Ok(()) }
/// ```
pub async fn run(
    connection: &mut MuxConnection,
    id: u64,
    offset: u64,
    replay_until: u64,
    alternate_screen: bool,
    clipboard: impl Fn() -> Option<String>,
) -> Result<Outcome> {
    let mut terminal =
        crate::terminal::ProxyTerminal::enter(alternate_screen, replay_until > offset)?;
    let mut stdin = tokio::io::stdin();
    let mut input = [0_u8; 64 * 1024];
    let mut pipeline = codetether_mux_input::Pipeline::new();
    let output_connection = connection.secondary().await?;
    let mut output = output::start(output_connection, id, offset);
    loop {
        tokio::select! {
            count = stdin.read(&mut input) => {
                let count = count?;
                if count == 0 { return Ok(Outcome::Detached); }
                let filtered = pipeline.filter(&input[..count], &clipboard);
                if !filtered.data.is_empty() { codetether_mux_input::send(connection, id, filtered.data).await?; }
                if filtered.detach {
                    terminal.clear_after_detach();
                    return Ok(Outcome::Detached);
                }
            }
            event = output.recv() => {
                let event = event.ok_or_else(|| anyhow::anyhow!("mux output connection closed"))??;
                terminal.observe_output(&event.data);
                presentation::write(&event.data)?;
                if event.next_offset >= replay_until {
                    terminal.finish_replay()?;
                }
                if event.exited { return Ok(Outcome::Exited); }
            }
        }
    }
}
