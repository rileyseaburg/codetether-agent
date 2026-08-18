//! Tests for thought output normalization.

use super::ThoughtPhase;
use super::normalize::normalize_thought_output;
use super::tests_support::sample_work_item;

#[test]
fn normalize_rejects_placeholder_process_line() {
    let work = sample_work_item(ThoughtPhase::Compress);
    let output = normalize_thought_output(
        &work,
        &[],
        "Phase: Compress | State_Summary: ... | Retained_Facts: ... | Open_Risks: ... | Next_Process_Step: ...",
    );
    assert!(output.starts_with("Phase: Compress | State_Summary: reliability monitoring active"));
    assert!(!output.contains("State_Summary: ..."));
}

#[test]
fn normalize_accepts_concrete_process_line() {
    let work = sample_work_item(ThoughtPhase::Test);
    let concrete = "Phase: Test | Check: inspect ingress 5xx over last 15m | Procedure: query error-rate dashboard and compare baseline | Expected_Result: pass if <=0.5% 5xx, fail otherwise | Evidence_Quality: high from direct telemetry | Escalation_Trigger: >2% 5xx for 5 minutes";
    assert_eq!(normalize_thought_output(&work, &[], concrete), concrete);
}
