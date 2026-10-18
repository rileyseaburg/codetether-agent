//! Native-host specialization retained for import integration tests.
use super::{super::Session, info::PersistOutcome};
use anyhow::Result;
pub(crate) async fn persist_imported_session(
    session: Session,
    data_dir: &std::path::Path,
) -> Result<PersistOutcome> {
    codetether_codex_import::persist_imported_session::<super::host::ApplicationHost>(
        session, data_dir,
    )
    .await
}
