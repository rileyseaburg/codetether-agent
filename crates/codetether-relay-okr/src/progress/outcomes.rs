//! Add one linked outcome for each valid key-result identity.
use super::{Report, parse::parse_uuid_guarded};
use codetether_okr::OkrRun;
use std::collections::HashMap;
pub(super) fn apply(run: &mut OkrRun, kr_progress: &HashMap<String, f64>, report: &Report<'_>) {
    // Create outcomes per KR with progress (link to actual KR IDs)
    let base_evidence = super::evidence::evidence(report);

    let outcome_type = if report.status == "converged" {
        codetether_okr::KrOutcomeType::FeatureDelivered
    } else {
        codetether_okr::KrOutcomeType::Evidence
    };

    // Create one outcome per KR, linked to the actual KR ID
    for (kr_id_str, value) in kr_progress {
        // Parse KR ID with guardrail to prevent NIL UUID linkage
        if let Some(kr_uuid) = parse_uuid_guarded(kr_id_str, "cli_relay_outcome_kr_link") {
            let kr_description = format!(
                "CLI relay outcome for KR {}: {} agents, {} turns, status={}",
                kr_id_str, report.agent_count, report.turns, report.status
            );
            run.outcomes.push({
                let mut outcome =
                    codetether_okr::KrOutcome::new(kr_uuid, kr_description).with_value(*value);
                outcome.run_id = Some(run.id);
                outcome.outcome_type = outcome_type;
                outcome.evidence = base_evidence.clone();
                outcome.source = "cli relay".to_string();
                outcome
            });
        }
    }
}
