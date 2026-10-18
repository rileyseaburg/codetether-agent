//! Native-source specialization of shared session listing entry points.
use crate::session::SessionSummary;
use anyhow::Result;
use std::path::Path;
pub async fn list_sessions() -> Result<Vec<SessionSummary>> {
    codetether_session_listing::list_sessions::<super::ApplicationSource>().await
}
pub async fn list_sessions_for_directory(dir: &Path) -> Result<Vec<SessionSummary>> {
    codetether_session_listing::list_sessions_for_directory::<super::ApplicationSource>(dir).await
}
pub async fn list_sessions_paged(
    dir: &Path,
    limit: usize,
    offset: usize,
) -> Result<Vec<SessionSummary>> {
    codetether_session_listing::list_sessions_paged::<super::ApplicationSource>(dir, limit, offset)
        .await
}
