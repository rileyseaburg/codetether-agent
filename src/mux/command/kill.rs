//! Graceful mux server shutdown command.

use anyhow::{Context, Result};

use crate::mux::protocol::ServerResponse;

pub(super) async fn run(target: &str) -> Result<()> {
    crate::mux::registry::validate_name(target)?;
    let record = crate::mux::registry::load(target)
        .await
        .with_context(|| format!("mux session '{target}' was not found"))?;
    let response = match super::shutdown::request(&record).await {
        Ok(response) => response,
        Err(error) => {
            tracing::warn!(session = target, %error, "Graceful mux shutdown failed");
            super::terminate::run(&record).await?;
            crate::mux::registry::remove(target).await?;
            println!("force-stopped mux session '{target}'");
            return Ok(());
        }
    };
    match response {
        ServerResponse::ShuttingDown => {
            println!("stopped mux session '{target}'");
            Ok(())
        }
        ServerResponse::Error { message } => anyhow::bail!("{message}"),
        _ => anyhow::bail!("mux server returned an invalid shutdown response"),
    }
}
