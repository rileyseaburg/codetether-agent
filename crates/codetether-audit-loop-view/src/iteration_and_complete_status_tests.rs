//! Existing audit-loop state transition regression.
use super::*;

#[test]
fn iteration_and_complete_status() {
    let mut s = run();
    s.apply(AuditLoopEvent::IterationFinished { passed: true });
    assert_eq!(s.iterations[0].status, IterationStatus::Passed);
    s.apply(AuditLoopEvent::Complete {
        passed: true,
        iterations: 1,
    });
    assert!(s.complete && s.passed);
}
