//! Native session specialization of shared commit and checkpoint sequencing.
use crate::session::Session;
use anyhow::Result;
pub(super) use codetether_session_store::clean;
use std::path::Path;
pub(crate) async fn save(session: &Session, path: &Path) -> Result<bool> {
    codetether_session_store::save(session, path).await
}
