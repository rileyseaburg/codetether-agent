//! Native session specialization of revision-aware external imports.
use crate::session::Session;
use std::path::Path;
pub(crate) async fn persist(session: Session, path: &Path) -> anyhow::Result<bool> {
    codetether_session_store::legacy::persist(session, path).await
}
