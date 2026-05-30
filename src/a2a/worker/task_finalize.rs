//! Post-agent task finalization helpers.

use anyhow::Result;

use crate::session::Session;

use super::task_timeline;

pub(super) async fn finalize_task_result(
    session: &mut Session,
    task_id: &str,
    status: &mut &'static str,
    result: &mut Option<String>,
    error: &mut Option<String>,
    is_virtual_task: bool,
    timeline: &mut task_timeline::TaskTimeline,
) -> Result<()> {
    if timeline.is_near_deadline() {
        tracing::warn!(
            task_id,
            elapsed_secs = format!("{:.1}", timeline.elapsed_secs()),
            budget_pct = format!("{:.1}%", timeline.budget_pct_used()),
            "Near deadline after agent completion — will attempt graceful shutdown"
        );
        timeline.checkpoint(task_timeline::TaskCheckpoint::GracefulShutdown);
    }
    if *status == "completed"
        && !is_virtual_task
        && let Some(directory) = session.metadata.directory.as_deref()
    {
        match super::git_commit_push::run(
            directory,
            task_id,
            session.metadata.provenance.as_ref(),
            timeline,
        )
        .await
        {
            Ok(Some(summary)) => {
                *result = Some(match result.take() {
                    Some(existing) if !existing.trim().is_empty() => {
                        format!("{existing}\n\n{summary}")
                    }
                    _ => summary,
                })
            }
            Ok(None) => {}
            Err(err) => {
                tracing::error!(task_id, error = %err, "Failed to commit and push task changes");
                *status = "failed";
                *error = Some(format!("Post-agent git commit/push failed: {err}"));
            }
        }
    }
    timeline.sync_progress().await;
    Ok(())
}
