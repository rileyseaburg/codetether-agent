//! Record successful forage execution evidence and bounded key-result progress.
use crate::ForageOpportunity;
use anyhow::Result;
use codetether_okr::{OkrRepository, OkrStatus};
mod evidence;
mod outcome;
mod policy;
pub use policy::success_progress_increment_ratio;
/// Borrowed execution facts supplied by a CLI or another orchestration host.
///
/// # Examples
/// ```
/// let evidence = codetether_forage_core::progress::ExecutionEvidence {
///     engine: "run", model: None, detail: "completed", changed_files: &[], quality_gates_passed: true,
/// };
/// assert_eq!(evidence.engine, "run");
/// ```
pub struct ExecutionEvidence<'a> {
    /// Execution engine used for file-evidence requirements.
    pub engine: &'a str,
    /// Optional model label recorded with the outcome.
    pub model: Option<&'a str>,
    /// Execution detail summarized into the persisted evidence.
    pub detail: &'a str,
    /// Concrete changed-file paths supplied by the execution adapter.
    pub changed_files: &'a [String],
    /// Whether the caller reported successful quality gates.
    pub quality_gates_passed: bool,
}
/// Apply progress credit and persist ordered evidence for one execution.
///
/// # Arguments
/// * `repo` — Repository containing the selected objective.
/// * `item` — Selected opportunity and key-result identity.
/// * `evidence` — Borrowed facts reported by the execution adapter.
/// * `cycle` — Cycle number recorded in the outcome.
/// # Returns
/// Unit after the updated objective is persisted.
/// # Errors
/// Returns missing objective/key-result errors or repository I/O failures.
/// # Examples
/// ```no_run
/// # async fn example(repo: &codetether_okr::OkrRepository, item: &codetether_forage_core::ForageOpportunity, evidence: &codetether_forage_core::progress::ExecutionEvidence<'_>) -> anyhow::Result<()> {
/// codetether_forage_core::progress::record(repo, item, evidence, 1).await
/// # }
/// ```
pub async fn record(
    repo: &OkrRepository,
    item: &ForageOpportunity,
    evidence: &ExecutionEvidence<'_>,
    cycle: usize,
) -> Result<()> {
    let Some(mut okr) = repo.get_okr(item.okr_id).await? else {
        anyhow::bail!("OKR {} not found", item.okr_id);
    };
    let Some(kr) = okr
        .key_results
        .iter_mut()
        .find(|kr| kr.id == item.key_result_id)
    else {
        anyhow::bail!("KR {} not found in OKR {}", item.key_result_id, item.okr_id);
    };
    let progress = policy::apply(kr, evidence);
    let kr_outcome = outcome::build(kr, item, evidence, cycle, progress);
    kr.add_outcome(kr_outcome);
    if matches!(okr.status, OkrStatus::Draft | OkrStatus::OnHold) && progress.2 > 0.0 {
        okr.status = OkrStatus::Active;
    }
    if okr.is_complete() {
        okr.status = OkrStatus::Completed;
    }
    let _ = repo.update_okr(okr).await?;
    Ok(())
}
