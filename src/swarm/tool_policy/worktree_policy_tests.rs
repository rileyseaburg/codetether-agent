use crate::swarm::SubTask;

#[test]
fn explicit_readonly_verification_review_uses_current_workspace() {
    let task = SubTask::new(
        "Inspect focused diff and test evidence",
        "Review test evidence only; do not run tests or edit files",
    )
    .with_needs_worktree(false);

    assert!(!task.needs_worktree());
    assert!(task.is_read_only());
}

#[test]
fn verification_without_explicit_choice_stays_isolated() {
    let task = SubTask::new("Run focused tests", "Run the navigation tests");
    assert!(task.needs_worktree());
    assert!(!task.is_read_only());
}
