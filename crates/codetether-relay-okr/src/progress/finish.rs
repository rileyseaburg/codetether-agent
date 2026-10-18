//! Persist final key-result progress and relay outcomes on a best-effort basis.
use super::Report;
use std::collections::HashMap;
use uuid::Uuid;
/// Record final progress, per-KR evidence and the relay outcome.
/// # Arguments
/// * `okr_run_id` - Associated execution run.
/// * `kr_progress` - Final progress keyed by KR UUID strings.
/// * `report` - Relay execution facts to record.
/// # Returns
/// Unit after a best-effort update; unavailable runs and write failures are ignored.
/// # Examples
/// ```no_run
/// # async fn example() {
/// let report = codetether_relay_okr::progress::Report { relay_id: "r1", turns: 2, agent_count: 2, status: "converged", rlm_handoff_count: 0, dynamic_spawn_count: 0 };
/// codetether_relay_okr::progress::finish(None, &Default::default(), report).await;
/// # }
/// ```
pub async fn finish(
    okr_run_id: Option<Uuid>,
    kr_progress: &HashMap<String, f64>,
    report: Report<'_>,
) {
    if let Some(run_id) = okr_run_id
        && let Ok(repo) = codetether_okr::persistence::OkrRepository::from_config().await
        && let Ok(Some(mut run)) = repo.get_run(run_id).await
    {
        // Update KR progress from execution
        for (kr_id, value) in kr_progress {
            run.update_kr_progress(kr_id, *value);
        }

        super::outcomes::apply(&mut run, kr_progress, &report);

        // Mark complete or update status based on execution result
        if report.status == "converged" {
            run.complete();
        } else if report.status == "agent_error" || report.status == "bus_error" {
            run.status = codetether_okr::OkrRunStatus::Failed;
        } else {
            run.status = codetether_okr::OkrRunStatus::Completed;
        }
        let _ = repo.update_run(run).await;
    }
}
