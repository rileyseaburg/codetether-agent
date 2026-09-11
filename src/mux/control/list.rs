//! Mux session discovery across every workspace server.

use anyhow::Result;

use super::MuxSessionSummary;

/// Return every hosted session with a bounded per-server reachability check.
pub(crate) async fn list_sessions() -> Result<Vec<MuxSessionSummary>> {
    let mut summaries = Vec::new();
    for record in crate::mux::registry::list().await? {
        let connected = tokio::time::timeout(
            std::time::Duration::from_millis(300),
            crate::mux::client::probe(&record),
        )
        .await
        .is_ok_and(|result| result.is_ok());
        summaries.extend(MuxSessionSummary::from_record(&record, connected));
    }
    Ok(summaries)
}
