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

pub(super) async fn build_opportunities(
    repo: &OkrRepository,
    moonshots: &MoonshotRubric,
) -> Result<Vec<ForageOpportunity>> {
    let okrs = repo.list_okrs().await?;
    Ok(collect_opportunities_with_rubric(&okrs, moonshots))
}

#[cfg(test)]
pub(super) fn collect_opportunities(okrs: &[Okr]) -> Vec<ForageOpportunity> {
    collect_opportunities_with_rubric(okrs, &MoonshotRubric::default())
}

pub(super) fn collect_opportunities_with_rubric(
    okrs: &[Okr],
    moonshots: &MoonshotRubric,
) -> Vec<ForageOpportunity> {
    codetether_forage_core::collect_opportunities_with_rubric(
        okrs,
        moonshots,
        &mut tetherscript_score::apply,
    )
}
