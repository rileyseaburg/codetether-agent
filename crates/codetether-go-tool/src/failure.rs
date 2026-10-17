//! Persist execution failure, update the monitor, and notify callers.

use codetether_go_monitor::{ACTIVE_GO_RUNS, GoRunPhase};
use codetether_okr::{OkrRepository, OkrRun};
use std::sync::Arc;
pub(crate) async fn record(
    mut run: OkrRun,
    err: anyhow::Error,
    bg_okr_id: String,
    bg_callback: Option<Arc<dyn Fn(String) + Send + Sync>>,
) {
    // Mark run as failed
    run.status = codetether_okr::OkrRunStatus::Failed;
    if let Ok(repo) = OkrRepository::from_config().await {
        let _ = repo.update_run(run).await;
    }

    let error_msg = err.to_string();
    if let Ok(mut runs) = ACTIVE_GO_RUNS.lock()
        && let Some(r) = runs.get_mut(&bg_okr_id)
    {
        r.phase = GoRunPhase::Failed {
            error: error_msg.clone(),
        };
    }

    tracing::error!(target: "codetether_agent::tool::go",
        okr_id = %bg_okr_id,
        error = %error_msg,
        "Go pipeline failed"
    );

    // Notify the LLM session that the pipeline failed
    if let Some(ref cb) = bg_callback {
        cb(format!(
            "# Go Pipeline Failed ❌\n\n**OKR ID:** `{}`\n**Error:** {}",
            bg_okr_id, error_msg
        ));
    }
}
