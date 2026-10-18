//! Reconnect to an existing server-owned terminal program.
use anyhow::{Result, bail};
use codetether_mux_client::MuxConnection;
use codetether_mux_protocol::{ClientRequest, ProgramRequest, ServerResponse};

/// Reattach with the existing program replay and screen state.
/// # Arguments
/// Connection/window and the local clipboard callback.
/// # Returns
/// An attachment outcome, or None when no program is running.
/// # Errors
/// Propagates other server, terminal and transport failures.
/// # Examples
/// ```no_run
/// # async fn example(connection: &mut codetether_mux_client::MuxConnection) -> anyhow::Result<()> {
/// let _outcome = codetether_mux_attachment::program::attach(connection, 0, || None).await?;
/// # Ok(()) }
/// ```
pub async fn attach(
    connection: &mut MuxConnection,
    id: u64,
    clipboard: impl Fn() -> Option<String>,
) -> Result<Option<crate::Outcome>> {
    let (columns, rows) = crossterm::terminal::size().unwrap_or((80, 24));
    let response = connection
        .request(ClientRequest::Program {
            request: ProgramRequest::Attach {
                window_id: id,
                columns,
                rows,
            },
        })
        .await?;
    match response {
        ServerResponse::ProgramAttached {
            offset,
            replay_until,
            alternate_screen,
            ..
        } => Ok(Some(
            crate::run(
                connection,
                id,
                offset,
                replay_until,
                alternate_screen,
                clipboard,
            )
            .await?,
        )),
        ServerResponse::Error { message } if message.contains("no running program") => Ok(None),
        ServerResponse::Error { message } => bail!(message),
        _ => bail!("mux server returned an invalid program-attach response"),
    }
}
