//! Adapt CLI goals, repository loading and custom scoring to shared selection.
use super::*;

pub(super) async fn load_moonshot_rubric(args: &ForageArgs) -> Result<MoonshotRubric> {
    codetether_forage_core::rubric_load::load(
        &args.moonshots,
        args.moonshot_file.as_deref(),
        args.moonshot_required,
        args.moonshot_min_alignment,
    )
    .await
}

pub(super) async fn build_opportunities<H: Host>(
    repo: &OkrRepository,
    moonshots: &MoonshotRubric,
) -> Result<Vec<ForageOpportunity>> {
    let okrs = repo.list_okrs().await?;
    Ok(collect_opportunities_with_rubric::<H>(&okrs, moonshots))
}

/// Score available objective work using the default moonshot rubric.
/// # Arguments
/// * `okrs` — Loaded objectives; `H` supplies the application scoring hook.
/// # Returns
/// Ranked actionable opportunities from the supplied objectives.
/// # Examples
/// ```
/// # fn select<H: codetether_forage_runtime::Host>() {
/// assert!(codetether_forage_runtime::collect_opportunities::<H>(&[]).is_empty());
/// # }
/// ```
pub fn collect_opportunities<H: Host>(okrs: &[Okr]) -> Vec<ForageOpportunity> {
    collect_opportunities_with_rubric::<H>(okrs, &MoonshotRubric::default())
}

/// Score available objective work against an explicit moonshot rubric.
/// # Arguments
/// * `okrs` — Loaded objectives.
/// * `moonshots` — Alignment goals and thresholds.
/// # Returns
/// Ranked opportunities after applying alignment and the host scoring hook.
/// # Examples
/// ```
/// # fn select<H: codetether_forage_runtime::Host>() {
/// let rubric = codetether_forage_core::MoonshotRubric::default();
/// assert!(codetether_forage_runtime::collect_opportunities_with_rubric::<H>(&[], &rubric).is_empty());
/// # }
/// ```
pub fn collect_opportunities_with_rubric<H: Host>(
    okrs: &[Okr],
    moonshots: &MoonshotRubric,
) -> Vec<ForageOpportunity> {
    codetether_forage_core::collect_opportunities_with_rubric(okrs, moonshots, &mut H::score)
}
