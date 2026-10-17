//! Update a single key result and append its evidence-backed outcome.

use crate::metrics::Metrics;
use codetether_okr::{KeyResult, KrOutcome, OkrRun};
use codetether_ralph_core::{Prd, RalphState};
pub(crate) fn apply(
    kr: &mut KeyResult,
    run: &mut OkrRun,
    prd: &Prd,
    state: &RalphState,
    metrics: &Metrics,
) {
    let Metrics {
        passed,
        total,
        ratio,
        outcome_type,
        story_evidence,
    } = metrics;
    // Map KR progress based on story completion ratio
    let kr_value = ratio * kr.target_value;
    kr.update_progress(kr_value);
    run.update_kr_progress(&kr.id.to_string(), kr_value);

    let mut evidence = story_evidence.clone();
    evidence.push(format!("prd:{}", prd.feature));
    evidence.push(format!("iterations:{}", state.current_iteration));
    evidence.push(format!("status:{:?}", state.status));
    if !prd.branch_name.is_empty() {
        evidence.push(format!("branch:{}", prd.branch_name));
    }

    let mut outcome = KrOutcome::new(
        kr.id,
        format!(
            "Ralph PRD execution: {}/{} stories passed for '{}'",
            passed, total, prd.feature
        ),
    )
    .with_value(kr_value);
    outcome.run_id = Some(run.id);
    outcome.outcome_type = *outcome_type;
    outcome.evidence = evidence;
    outcome.source = "go_ralph".to_string();

    kr.add_outcome(outcome.clone());
    run.outcomes.push(outcome);
}
