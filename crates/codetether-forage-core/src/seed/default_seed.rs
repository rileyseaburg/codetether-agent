//! Forage objective seeding: default_seed.
use super::*;

/// Apply the seed default okr if empty policy to the supplied repository.
///
/// # Arguments
/// The repository owns persistence; a supplied rubric provides mission goals.
/// # Returns
/// The original seeding outcome, including the new objective ID where applicable.
/// # Errors
/// Returns repository lookup or persistence errors without replacing them.
/// # Examples
/// ```no_run
/// # async fn example(repo: &codetether_okr::OkrRepository) -> anyhow::Result<()> {
/// let _ = codetether_forage_core::seed::seed_default_okr_if_empty(repo).await?;
/// # Ok(()) }
/// ```
pub async fn seed_default_okr_if_empty(repo: &OkrRepository) -> Result<()> {
    let existing = repo.list_okrs().await?;
    if !existing.is_empty() {
        return Ok(());
    }

    let mut okr = Okr::new(
        "Mission: Autonomous Business-Aligned Execution",
        "Autonomously execute concrete, behavior-preserving code changes that align to business goals and produce measurable progress.",
    );
    okr.status = OkrStatus::Active;
    let okr_id = okr.id;
    okr.add_key_result(KeyResult::new(okr_id, "Key Result 1", 100.0, "%"));
    okr.add_key_result(KeyResult::new(
        okr_id,
        "Team produces actionable handoff",
        100.0,
        "%",
    ));
    okr.add_key_result(KeyResult::new(okr_id, "No critical errors", 100.0, "%"));

    let _ = repo.create_okr(okr).await?;
    tracing::info!(target: "codetether_agent::forage", okr_id = %okr_id, "Seeded default OKR for forage execution");
    Ok(())
}
