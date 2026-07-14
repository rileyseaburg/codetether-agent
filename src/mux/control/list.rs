//! Named mux server discovery for user interfaces.

use anyhow::Result;

use super::MuxSessionSummary;
use crate::mux::client::MuxConnection;

/// Return all registered mux servers with a bounded reachability check.
pub(crate) async fn list_sessions() -> Result<Vec<MuxSessionSummary>> {
    let mut summaries = Vec::new();
    for record in crate::mux::registry::list().await? {
        let connected = tokio::time::timeout(
            std::time::Duration::from_millis(300),
            MuxConnection::connect(&record),
        )
        .await
        .is_ok_and(|result| result.is_ok());
        summaries.push(MuxSessionSummary::from_record(&record, connected));
    }
    Ok(summaries)
}
