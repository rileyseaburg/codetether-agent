use super::{TaskState, terminal_state};

#[test]
fn failed_deliverables_are_not_announced_completed() {
    assert_eq!(terminal_state(false), TaskState::Failed);
    assert_ne!(terminal_state(false), TaskState::Completed);
}
