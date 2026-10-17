//! Persist goals captured by an interactive forage host.
use super::*;
/// Create an active objective from the supplied goal lines.
///
/// # Arguments
/// `repo` stores the objective; `goals` supplies one key-result title per entry.
/// # Returns
/// Completes after storing the objective and its key results.
/// # Errors
/// Propagates objective persistence errors.
/// # Examples
/// ```rust,no_run
/// async fn example(repo: &codetether_okr::OkrRepository) -> anyhow::Result<()> {
/// codetether_forage_core::seed::persist_clarified_goals(repo, &["Ship the parser".into()]).await?;
/// Ok(())
/// }
/// ```
pub async fn persist_clarified_goals(repo: &OkrRepository, goals: &[String]) -> Result<()> {
    let mut okr = Okr::new(
        "User-Clarified Goals",
        "Goals captured interactively when forage found no opportunities.",
    );
    okr.status = OkrStatus::Active;
    let okr_id = okr.id;
    for goal in goals {
        okr.add_key_result(KeyResult::new(okr_id, goal.clone(), 100.0, "%"));
    }
    repo.create_okr(okr).await?;
    Ok(())
}
