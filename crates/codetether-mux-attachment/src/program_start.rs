//! Start and attach to a new server-owned terminal program.
use anyhow::Result;
use codetether_mux_client::MuxConnection;

/// Start a server-owned program and attach from its returned output offset.
/// # Arguments
/// Connection/window, program command and the local clipboard callback.
/// # Returns
/// The new program exit or detach outcome.
/// # Errors
/// Propagates start-request, terminal and attachment failures.
/// # Examples
/// ```no_run
/// # async fn example(connection: &mut codetether_mux_client::MuxConnection) -> anyhow::Result<()> {
/// let _outcome = codetether_mux_attachment::program::start(connection, 0, "sh".into(), || None).await?;
/// # Ok(()) }
/// ```
pub async fn start(
    connection: &mut MuxConnection,
    id: u64,
    command: String,
    clipboard: impl Fn() -> Option<String>,
) -> Result<crate::Outcome> {
    let offset = crate::program_request::start(connection, id, command).await?;
    crate::run(connection, id, offset, offset, false, clipboard).await
}
