//! Map Ralph story completion ratios to all objective key results.

use codetether_okr::{KrOutcomeType, Okr, OkrRun};
use codetether_ralph_core::Prd;
pub(crate) fn map_stories_to_kr_outcomes(
    okr: &mut Okr,
    run: &mut OkrRun,
    prd: &Prd,
    state: &codetether_ralph_core::RalphState,
) {
    let passed = prd.passed_count();
    let total = prd.user_stories.len();
    let ratio = if total > 0 {
        passed as f64 / total as f64
    } else {
        0.0
    };

    // Build evidence from story results
    let story_evidence = crate::evidence::collect(prd);
    let outcome_type = if ratio >= 1.0 {
        KrOutcomeType::FeatureDelivered
    } else {
        KrOutcomeType::Evidence
    };

    let metrics = crate::metrics::Metrics {
        passed,
        total,
        ratio,
        outcome_type,
        story_evidence,
    };
    // For each KR, create an outcome with story-mapped evidence
    for kr in &mut okr.key_results {
        crate::apply_outcome::apply(kr, run, prd, state, &metrics);
    }
}
