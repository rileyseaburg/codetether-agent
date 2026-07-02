//! OKR struct, run workflow, and outcome tests.

use super::*;

#[test]
fn test_okr_creation() {
    let okr = Okr::new("Test Objective", "Description");
    assert_eq!(okr.title, "Test Objective");
    assert_eq!(okr.status, OkrStatus::Draft);
    assert!(okr.validate().is_err());
}

#[test]
fn test_okr_with_key_results() {
    let mut okr = Okr::new("Test Objective", "Description");
    let kr = KeyResult::new(okr.id, "KR1", 100.0, "%");
    okr.add_key_result(kr);
    assert!(okr.validate().is_ok());
}

#[test]
fn test_okr_run_workflow() {
    let okr_id = Uuid::new_v4();
    let mut run = OkrRun::new(okr_id, "Q1 2024 Run");

    run.submit_for_approval().unwrap();
    assert_eq!(run.status, OkrRunStatus::PendingApproval);

    run.record_decision(ApprovalDecision::approve(run.id, "Looks good"));
    assert_eq!(run.status, OkrRunStatus::Approved);

    run.start().unwrap();
    assert_eq!(run.status, OkrRunStatus::Running);

    run.update_kr_progress("kr-1", 0.5);
    run.complete();
    assert_eq!(run.status, OkrRunStatus::Completed);
}

#[test]
fn test_outcome_creation() {
    let outcome = KrOutcome::new(Uuid::new_v4(), "Fixed bug in auth")
        .with_value(1.0)
        .add_evidence("commit:abc123");

    assert_eq!(outcome.value, Some(1.0));
    assert!(outcome.evidence.contains(&"commit:abc123".to_string()));
}
