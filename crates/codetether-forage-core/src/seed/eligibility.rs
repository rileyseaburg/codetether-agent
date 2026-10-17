//! Forage objective seeding: eligibility.
use super::*;

/// Apply the seed moonshot okr if empty policy to the supplied repository.
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
/// let _ = codetether_forage_core::seed::seed_moonshot_okr_if_empty(repo, &Default::default()).await?;
/// # Ok(()) }
/// ```
pub async fn seed_moonshot_okr_if_empty(
    repo: &OkrRepository,
    moonshots: &MoonshotRubric,
) -> Result<Option<Uuid>> {
    let existing = repo.list_okrs().await?;
    if !existing.is_empty() || moonshots.goals.is_empty() {
        return Ok(None);
    }
    seed_moonshot_okr(repo, moonshots).await.map(Some)
}

/// Apply the seed moonshot okr if no opportunities policy to the supplied repository.
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
/// let _ = codetether_forage_core::seed::seed_moonshot_okr_if_no_opportunities(repo, &Default::default()).await?;
/// # Ok(()) }
/// ```
pub async fn seed_moonshot_okr_if_no_opportunities(
    repo: &OkrRepository,
    moonshots: &MoonshotRubric,
) -> Result<Option<Uuid>> {
    if moonshots.goals.is_empty() {
        return Ok(None);
    }

    // If there is already an open moonshot-derived objective with incomplete work,
    // avoid continuously creating new ones.
    let existing = repo.list_okrs().await?;
    let has_open_moonshot_seed = existing.iter().any(|okr| {
        matches!(
            okr.status,
            OkrStatus::Active | OkrStatus::Draft | OkrStatus::OnHold
        ) && okr.title == "Mission: Moonshot-Derived Autonomous Execution"
            && okr
                .key_results
                .iter()
                .any(|kr| kr.progress().clamp(0.0, 1.0) < 1.0)
    });
    if has_open_moonshot_seed {
        return Ok(None);
    }

    seed_moonshot_okr(repo, moonshots).await.map(Some)
}
