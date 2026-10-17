//! workflow_state regression.
use super::*;

#[test]
fn state_machine_and_serialization_contract() {
    let mut run = OkrRun::new(Uuid::new_v4(), "run");
    assert!(matches!(run.start(), Err(OkrValidationError::NotApproved)));
    run.submit_for_approval().unwrap();
    assert!(matches!(
        run.submit_for_approval(),
        Err(OkrValidationError::InvalidStatusTransition)
    ));
    run.record_decision(ApprovalDecision::deny(run.id, "denied"));
    assert_eq!(run.status, OkrRunStatus::Denied);
    run.record_decision(ApprovalDecision::approve(run.id, "approved"));
    run.start().unwrap();
    assert!(run.is_resumable());
    run.update_kr_progress("kr", 0.7);
    run.complete();
    assert!(!run.is_resumable());
    assert!(run.completed_at.is_some());
    let value = serde_json::to_value(&run).unwrap();
    assert_eq!(value["status"], "completed");
    assert_eq!(value["kr_progress"]["kr"], 0.7);
    let restored: OkrRun = serde_json::from_value(value).unwrap();
    assert_eq!(restored.id, run.id);
}
