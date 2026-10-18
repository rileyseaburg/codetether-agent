//! Close one mux session; its server exits when no sessions remain.

use anyhow::Result;

use codetether_mux_client::MuxConnection;
use codetether_mux_protocol::{ClientRequest, ServerResponse};
use codetether_mux_registry::{MuxRecord, SessionTarget};

/// Close a named session, forcing its server down after connection failure.
///
/// # Arguments
/// target is the session name.
/// # Returns
/// Unit after the close or verified forced termination.
/// # Errors
/// Returns lookup, server response or verified termination errors.
/// # Examples
/// ```no_run
/// # async fn example() -> anyhow::Result<()> {
/// codetether_mux_lifecycle::kill::run("work").await?;
/// # Ok(()) }
/// ```
pub async fn run(target: &str) -> Result<()> {
    let target = codetether_mux_registry::load(target).await?;
    run_target(target).await
}

/// Close a discovered session with the same fallback as named closure.
///
/// # Arguments
/// target is the authenticated session registry target.
/// # Returns
/// Unit after the close or verified forced termination.
/// # Errors
/// Returns server response, registry removal or verified termination errors.
/// # Examples
/// ```no_run
/// # async fn example() -> anyhow::Result<()> {
/// let target = codetether_mux_registry::load("work").await?;
/// codetether_mux_lifecycle::kill::run_target(target).await?;
/// # Ok(()) }
/// ```
pub async fn run_target(target: SessionTarget) -> Result<()> {
    let name = target.session.as_str();
    let connection = MuxConnection::connect_server(&target.record).await;
    let response = match connection {
        Ok(mut connection) => {
            let request = ClientRequest::CloseSession {
                name: name.to_string(),
            };
            match connection.request(request).await {
                Ok(response) => response,
                Err(error) => return force(&target.record, name, error).await,
            }
        }
        Err(error) => return force(&target.record, name, error).await,
    };
    match response {
        ServerResponse::Snapshot { .. } | ServerResponse::ShuttingDown => {
            println!("stopped mux session '{name}'");
            Ok(())
        }
        ServerResponse::Error { message } => anyhow::bail!("{message}"),
        _ => anyhow::bail!("mux server returned an invalid close response"),
    }
}

async fn force(record: &MuxRecord, name: &str, error: anyhow::Error) -> Result<()> {
    tracing::warn!(session = name, %error, "Graceful mux session close failed");
    super::terminate::run(record).await?;
    codetether_mux_registry::remove_key(&record.key).await?;
    println!("force-stopped mux server hosting '{name}'");
    Ok(())
}
