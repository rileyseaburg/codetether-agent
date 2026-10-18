//! Persist an approved objective and correlated run when storage is available.
use codetether_okr::{ApprovalDecision, Okr, OkrRepository, OkrRun};
use uuid::Uuid;
/// Persist an approved objective and correlated run when storage is available.
/// # Arguments
/// * `okr_id` - Objective identity for logging.
/// * `okr` - Objective to save.
/// * `approved_run` - Run receiving the approval and correlation identity.
/// # Returns
/// Unit after best-effort writes; storage failures are ignored.
/// # Examples
/// ```no_run
/// # async fn example(okr: &codetether_okr::Okr, run: &mut codetether_okr::OkrRun) {
/// codetether_relay_okr::cli_approval::persist(okr.id, okr, run).await;
/// # }
/// ```
pub async fn persist(okr_id: Uuid, okr: &Okr, approved_run: &mut OkrRun) {
    if let Ok(repo) = OkrRepository::from_config().await {
        let _ = repo.create_okr(okr.clone()).await;
        approved_run.record_decision(ApprovalDecision::approve(
            approved_run.id,
            "User approved via CLI",
        ));
        approved_run.correlation_id = Some(format!("ralph-{}", Uuid::new_v4()));
        let _ = repo.create_run(approved_run.clone()).await;
        tracing::info!(okr_id = %okr_id, okr_run_id = %approved_run.id, "OKR run approved and saved");
    }
}
