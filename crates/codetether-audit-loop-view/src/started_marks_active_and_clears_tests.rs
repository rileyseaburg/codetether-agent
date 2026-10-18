//! Existing audit-loop state transition regression.
use super::*;

#[test]
fn started_marks_active_and_clears() {
    let s = run();
    assert!(s.active);
    assert_eq!(s.task, "tables");
    assert_eq!(s.iterations.len(), 1);
}
