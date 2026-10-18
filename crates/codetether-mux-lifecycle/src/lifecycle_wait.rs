//! Bounded polling for the managed TUI runtime report.
use super::MuxSessionSummary;
use anyhow::{Result, bail};

pub(super) async fn wait_runtime(name: &str) -> Result<MuxSessionSummary> {
    // Preserves the original 200 x 50ms budget while polling far more eagerly
    // at the start, where the TUI usually registers within a few milliseconds.
    let mut backoff = crate::backoff::Backoff::with_budget(std::time::Duration::from_secs(10));
    while backoff.remaining() {
        let sessions = super::list_sessions().await?;
        if let Some(session) = sessions
            .into_iter()
            .find(|item| item.name == name && item.runtime.is_some())
        {
            return Ok(session);
        }
        backoff.sleep().await;
    }
    bail!("mux TUI did not register semantic state")
}
