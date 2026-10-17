//! Persist successful execution, update the monitor, and notify callers.

use crate::Host;
use codetether_go_monitor::{ACTIVE_GO_RUNS, GoRunPhase};
use codetether_okr::{OkrRepository, OkrRun};
use std::sync::Arc;
pub(crate) async fn complete<H: Host>(
    run: OkrRun,
    result: H::Output,
    bg_task: String,
    bg_okr_id: String,
    bg_callback: Option<Arc<dyn Fn(String) + Send + Sync>>,
) {
    // Persist final state
    if let Ok(repo) = OkrRepository::from_config().await {
        let _ = repo.update_run(run).await;
    }

    let result = H::summarize(result, &bg_task);
    let summary = result.summary;

    if let Ok(mut runs) = ACTIVE_GO_RUNS.lock()
        && let Some(r) = runs.get_mut(&bg_okr_id)
    {
        r.phase = GoRunPhase::Completed {
            passed: result.passed,
            total: result.total,
            all_passed: result.all_passed,
            feature_branch: result.feature_branch,
            summary,
        };
    }

    tracing::info!(target: "codetether_agent::tool::go",
        okr_id = %bg_okr_id,
        passed = result.passed,
        total = result.total,
        "Go pipeline completed"
    );

    // Notify the LLM session that the pipeline finished
    if let Some(ref cb) = bg_callback {
        let phase_str = crate::callback::completed(&bg_okr_id);
        cb(phase_str);
    }
}
