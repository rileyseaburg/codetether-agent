//! Forage objective seeding: initial.
use super::*;

/// Apply the seed initial okr if empty policy to the supplied repository.
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
/// let _ = codetether_forage_core::seed::seed_initial_okr_if_empty(repo, &Default::default()).await?;
/// # Ok(()) }
/// ```
pub async fn seed_initial_okr_if_empty(
    repo: &OkrRepository,
    moonshots: &MoonshotRubric,
) -> Result<()> {
    if moonshots.goals.is_empty() {
        return seed_default_okr_if_empty(repo).await;
    }
    let _ = seed_moonshot_okr_if_empty(repo, moonshots).await?;
    Ok(())
}
