//! Discover work, seed empty queues and apply the per-cycle selection limit.
use super::*;
pub(super) async fn discover<H: Host>(
    repo: &OkrRepository,
    moonshot_rubric: &MoonshotRubric,
    execute: bool,
    top: usize,
) -> Result<Vec<ForageOpportunity>> {
    let mut opportunities = build_opportunities::<H>(repo, moonshot_rubric).await?;
    if opportunities.is_empty() {
        // First try the moonshot seed (execute mode only), then fall back to
        // interactive goal clarification so forage never silently dead-ends.
        if execute
            && let Some(seed_okr_id) =
                seed_moonshot_okr_if_no_opportunities(repo, moonshot_rubric).await?
        {
            tracing::info!(target: "codetether_agent::forage",
                okr_id = %seed_okr_id,
                moonshot_count = moonshot_rubric.goals.len(),
                "Seeded moonshot-derived OKR because forage found no opportunities"
            );
            opportunities = build_opportunities::<H>(repo, moonshot_rubric).await?;
        }
        if opportunities.is_empty() {
            let okr_count = repo.list_okrs().await?.len();
            if H::clarify(repo, okr_count).await? {
                opportunities = build_opportunities::<H>(repo, moonshot_rubric).await?;
            }
        }
    }
    let selected: Vec<ForageOpportunity> = opportunities.into_iter().take(top).collect();

    Ok(selected)
}
