//! TUI reporting into its inherited authenticated mux server.

use anyhow::{Result, bail};

use crate::mux::client::MuxConnection;
use crate::mux::model::MuxRuntimeStatus;
use crate::mux::protocol::{ClientRequest, ServerResponse};

/// Persist semantic TUI state in the owning mux server, when inherited.
pub(crate) async fn report_runtime(status: Option<MuxRuntimeStatus>) -> Result<bool> {
    let Some(name) = std::env::var_os(crate::mux::coordination::SESSION_ENV) else {
        return Ok(false);
    };
    let name = name
        .into_string()
        .map_err(|_| anyhow::anyhow!("mux session name is not UTF-8"))?;
    let record = crate::mux::registry::load(&name).await?;
    let mut connection = MuxConnection::connect(&record).await?;
    match connection
        .request(ClientRequest::ReportRuntime { status })
        .await?
    {
        ServerResponse::Acknowledged => Ok(true),
        ServerResponse::Error { message } => bail!(message),
        _ => bail!("mux server returned an invalid runtime response"),
    }
}
