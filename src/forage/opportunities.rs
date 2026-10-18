//! Test bindings for shared forage opportunity selection.
use super::*;
pub(super) fn collect_opportunities(okrs: &[Okr]) -> Vec<ForageOpportunity> {
    codetether_forage_runtime::collect_opportunities::<super::execution_host::Native>(okrs)
}
pub(super) fn collect_opportunities_with_rubric(
    okrs: &[Okr],
    rubric: &MoonshotRubric,
) -> Vec<ForageOpportunity> {
    codetether_forage_runtime::collect_opportunities_with_rubric::<super::execution_host::Native>(
        okrs, rubric,
    )
}
