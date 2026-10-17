//! Session adapters for the shared materialized-history file implementation.
use super::Session;
use anyhow::Result;
pub(crate) use codetether_history_files::{format_turn_path, render_turn, role_label};
use std::path::PathBuf;
pub(crate) fn history_dir_for_session(session: &Session) -> Result<PathBuf> {
    codetether_history_files::history_dir(&session.id, session.metadata.directory.as_deref())
}
pub(crate) async fn materialize_session_history(session: &Session) -> Result<Vec<PathBuf>> {
    codetether_history_files::materialize_history(
        &session.id,
        session.metadata.directory.as_deref(),
        session.history(),
    )
    .await
}
