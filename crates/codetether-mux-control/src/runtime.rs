//! TUI reporting into its inherited authenticated mux server.

use anyhow::{Result, bail};

use codetether_mux_client::MuxConnection;
use codetether_mux_model::MuxRuntimeStatus;
use codetether_mux_protocol::{ClientRequest, ServerResponse};

/// Persist semantic TUI state in the owning mux server, when inherited.
///
/// # Arguments
/// status is the current semantic report, or None to clear it.
/// # Returns
/// True after acknowledgment; false if no mux identity is inherited.
/// # Errors
/// Returns invalid identity, registry, connection or response errors.
/// # Examples
/// ```no_run
/// # async fn example() -> anyhow::Result<()> {
/// let inherited = codetether_mux_control::report_runtime(None).await?;
/// # Ok(()) }
/// ```
pub async fn report_runtime(status: Option<MuxRuntimeStatus>) -> Result<bool> {
    let Some(name) = std::env::var_os(codetether_mux_coordination::SESSION_ENV) else {
        return Ok(false);
    };
    let name = name
        .into_string()
        .map_err(|_| anyhow::anyhow!("mux session name is not UTF-8"))?;
    let target = codetether_mux_registry::load(&name).await?;
    let mut connection = MuxConnection::connect(&target).await?;
    match connection
        .request(ClientRequest::ReportRuntime { status })
        .await?
    {
        ServerResponse::Acknowledged => Ok(true),
        ServerResponse::Error { message } => bail!(message),
        _ => bail!("mux server returned an invalid runtime response"),
    }
}
