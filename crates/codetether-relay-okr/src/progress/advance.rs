//! Advance and persist monotonic relay progress after a turn.
use std::collections::HashMap;
use uuid::Uuid;
/// Advance monotonic target progress and persist a resumable run when available.
/// # Arguments
/// * `kr_targets` - Target values loaded for the objective.
/// * `kr_progress` - Monotonic progress accumulated so far.
/// * `turns` - Completed relay turns.
/// * `agent_count` - Current team size.
/// * `max_rounds` - Configured relay round limit.
/// * `okr_run_id` - Optional execution run to update.
/// # Returns
/// Unit after the in-memory update and best-effort persistence.
/// # Examples
/// ```no_run
/// # async fn example() {
/// let targets = std::collections::HashMap::new();
/// let mut progress = std::collections::HashMap::new();
/// codetether_relay_okr::progress::advance(&targets, &mut progress, 1, 2, 3, None).await;
/// # }
/// ```
pub async fn advance(
    kr_targets: &HashMap<String, f64>,
    kr_progress: &mut HashMap<String, f64>,
    turns: usize,
    agent_count: usize,
    max_rounds: usize,
    okr_run_id: Option<Uuid>,
) {
    if !kr_targets.is_empty() {
        let max_turns = agent_count * max_rounds;
        let progress_ratio = (turns as f64 / max_turns as f64).min(1.0);

        for (kr_id, target) in kr_targets {
            let current = progress_ratio * target;
            let existing = kr_progress.get(kr_id).copied().unwrap_or(0.0);
            if current > existing {
                kr_progress.insert(kr_id.clone(), current);
            }
        }

        // Persist mid-run (best-effort)
        if let Some(run_id) = okr_run_id
            && let Ok(repo) = codetether_okr::persistence::OkrRepository::from_config().await
            && let Ok(Some(mut run)) = repo.get_run(run_id).await
            && run.is_resumable()
        {
            run.iterations = turns as u32;
            for (kr_id, value) in kr_progress.iter() {
                run.update_kr_progress(kr_id, *value);
            }
            run.status = codetether_okr::OkrRunStatus::Running;
            let _ = repo.update_run(run).await;
        }
    }
}
