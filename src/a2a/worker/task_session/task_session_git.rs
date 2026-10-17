//! Adapt session repository metadata to shared task preparation.
use super::super::TaskContext;
use crate::session::Session;
use anyhow::Result;
pub(super) async fn prepare_git(
    task_id: &str,
    session: &mut Session,
    context: &TaskContext,
) -> Result<()> {
    codetether_a2a_worker_core::preparation::prepare_git(
        task_id,
        session.metadata.directory.as_deref(),
        session.metadata.provenance.as_ref(),
        context,
    )
    .await
}
