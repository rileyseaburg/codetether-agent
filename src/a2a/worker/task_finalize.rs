//! Adapt session workspace metadata to shared task finalization.
use super::task_timeline;
use crate::session::Session;
use anyhow::Result;

pub(super) async fn finalize_task_result(
    session: &mut Session,
    task_id: &str,
    status: &mut &'static str,
    result: &mut Option<String>,
    error: &mut Option<String>,
    is_virtual_task: bool,
    timeline: &mut task_timeline::TaskTimeline,
) -> Result<()> {
    codetether_a2a_worker_core::finalization::finalize_task_result(
        codetether_a2a_worker_core::finalization::FinalizationWorkspace {
            directory: session.metadata.directory.as_deref(),
            provenance: session.metadata.provenance.as_ref(),
        },
        task_id,
        status,
        result,
        error,
        is_virtual_task,
        timeline,
    )
    .await
}
