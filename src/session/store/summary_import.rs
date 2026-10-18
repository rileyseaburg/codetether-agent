//! Native summary-index adapter for legacy session import.
use crate::session::Session;
use anyhow::Result;
use std::path::Path;
pub(super) fn run(path: &Path, session: &Session) -> Result<()> {
    codetether_session_store::summaries::import(path, &session.id, &session.summary_index)
}
