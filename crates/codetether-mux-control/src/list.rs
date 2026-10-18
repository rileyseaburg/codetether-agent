//! Mux session discovery across every workspace server.

use anyhow::Result;

use super::MuxSessionSummary;

/// Return every hosted session with a bounded per-server reachability check.
///
/// # Arguments
/// None.
/// # Returns
/// Summaries of all discovered sessions, including unreachable servers.
/// # Errors
/// Returns registry discovery errors.
/// # Examples
/// ```no_run
/// # async fn example() -> anyhow::Result<()> {
/// let sessions = codetether_mux_control::list_sessions().await?;
/// # Ok(()) }
/// ```
pub async fn list_sessions() -> Result<Vec<MuxSessionSummary>> {
    let mut summaries = Vec::new();
    for record in codetether_mux_registry::list().await? {
        let connected = tokio::time::timeout(
            std::time::Duration::from_millis(300),
            codetether_mux_client::handshake::probe(&record),
        )
        .await
        .is_ok_and(|result| result.is_ok());
        summaries.extend(MuxSessionSummary::from_record(&record, connected));
    }
    Ok(summaries)
}
