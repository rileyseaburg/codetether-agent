//! Graceful mux server shutdown command.

use anyhow::{Context, Result};

use crate::mux::protocol::ServerResponse;

pub(super) async fn run(target: &str) -> Result<()> {
    crate::mux::registry::validate_name(target)?;
    let record = crate::mux::registry::load(target)
        .await
        .with_context(|| format!("mux session '{target}' was not found"))?;
    match super::shutdown::request(&record).await? {
        ServerResponse::ShuttingDown => {
            println!("stopped mux session '{target}'");
            Ok(())
        }
        ServerResponse::Error { message } => anyhow::bail!("{message}"),
        _ => anyhow::bail!("mux server returned an invalid shutdown response"),
    }
}
