//! Stop one whole mux server, including every session it hosts.

use anyhow::{Result, bail};

use codetether_mux_protocol::ServerResponse;
use codetether_mux_registry::MuxRecord;

/// Gracefully shut a server down, forcing termination if it will not answer.
pub async fn shutdown(record: MuxRecord) -> Result<()> {
    let sessions: Vec<_> = record.session_names().map(str::to_string).collect();
    match super::shutdown::request(&record).await {
        Ok(ServerResponse::ShuttingDown) => {}
        Ok(ServerResponse::Error { message }) => bail!("{message}"),
        Ok(_) => bail!("mux server returned an invalid shutdown response"),
        Err(error) => {
            tracing::warn!(server = %record.key, %error, "Graceful mux shutdown failed");
            super::terminate::run(&record).await?;
            codetether_mux_registry::remove_key(&record.key).await?;
        }
    }
    println!("stopped mux sessions: {}", sessions.join(", "));
    Ok(())
}
