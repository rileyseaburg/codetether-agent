//! Existing audit-loop state transition regression.
use super::*;

#[test]
fn gate_started_then_failed() {
    let mut s = run();
    s.apply(AuditLoopEvent::GateFinished {
        name: "test".into(),
        passed: false,
        detail: Some("2 failed".into()),
    });
    let g = &s.iterations[0].gates[0];
    assert_eq!(g.status, GateStatus::Failed);
    assert_eq!(g.detail.as_deref(), Some("2 failed"));
}
