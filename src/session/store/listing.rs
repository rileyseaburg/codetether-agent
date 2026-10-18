//! Native summary projection for shared metadata-only listing.
use crate::session::SessionSummary;
use anyhow::Result;
use std::path::{Path, PathBuf};
pub(crate) async fn list(root: &Path, workspace: Option<PathBuf>) -> Result<Vec<SessionSummary>> {
    codetether_session_store::metadata::listing::list(root, workspace, decode).await
}
fn decode(header: &str, count: usize) -> Result<SessionSummary> {
    let session = super::header::decode(header)?;
    let directory = session.metadata.directory;
    Ok(SessionSummary {
        id: session.id,
        title: session.title,
        created_at: session.created_at,
        updated_at: session.updated_at,
        message_count: count,
        agent: session.agent,
        directory,
    })
}
