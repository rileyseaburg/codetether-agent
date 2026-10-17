//! Construct a persisted key-result outcome after progress has been applied.
use super::ExecutionEvidence;
use crate::ForageOpportunity;
use codetether_okr::{KeyResult, KrOutcome, KrOutcomeType};
pub(super) fn build(
    kr: &KeyResult,
    item: &ForageOpportunity,
    evidence: &ExecutionEvidence<'_>,
    cycle: usize,
    (should_increment, before_progress, after_progress): (bool, f64, f64),
) -> KrOutcome {
    let progress_impact = if evidence.quality_gates_passed {
        if should_increment {
            "advanced this KR"
        } else {
            "no concrete changed-file evidence was found; KR progress not incremented"
        }
    } else {
        "quality gates FAILED; KR progress not incremented"
    };

    let mut kr_outcome = KrOutcome::new(
        kr.id,
        format!(
            "Forage cycle {cycle} execution via '{}' {}.",
            evidence.engine, progress_impact
        ),
    );
    kr_outcome.outcome_type = KrOutcomeType::CodeChange;
    kr_outcome.value = Some((after_progress * 100.0).clamp(0.0, 100.0));
    kr_outcome.source = "forage-runtime".to_string();
    kr_outcome.evidence =
        super::evidence::build(item, evidence, cycle, before_progress, after_progress);
    kr_outcome
}
