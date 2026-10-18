use crate::{outcome::TaskInner, preparation::session::SessionState, task_timeline};
pub(super) async fn finish<S: SessionState>(
    session: &mut S,
    task_id: &str,
    mut status: &'static str,
    mut result: Option<String>,
    mut error: Option<String>,
    session_id: Option<String>,
    is_virtual_task: bool,
    timeline: &mut task_timeline::TaskTimeline,
) -> TaskInner {
    crate::finalization::finalize_task_result(
        crate::finalization::FinalizationWorkspace {
            directory: session.metadata().directory.as_deref(),
            provenance: session.metadata().provenance.as_ref(),
        },
        task_id,
        &mut status,
        &mut result,
        &mut error,
        is_virtual_task,
        timeline,
    )
    .await?;
    Ok((
        status,
        result,
        error,
        Some(session_id.unwrap_or_else(|| session.id().to_owned())),
    ))
}
