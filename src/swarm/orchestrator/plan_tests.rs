use super::{single, validate};
use crate::swarm::SubTask;

#[test]
fn fallback_is_one_truthful_task() {
    let plan = single("fix the bug");
    let task = plan.values().next().unwrap();
    assert_eq!(plan.len(), 1);
    assert_eq!(task.instruction, "fix the bug");
}

#[test]
fn validation_rejects_cycles() {
    let mut first = SubTask::new("first", "inspect");
    let mut second = SubTask::new("second", "inspect");
    first.dependencies = vec![second.id.clone()];
    second.dependencies = vec![first.id.clone()];
    assert!(validate(vec![first, second]).is_err());
}

#[test]
fn validation_rejects_empty_and_duplicate_plans() {
    assert!(validate(Vec::new()).is_err());
    assert!(
        validate(vec![
            SubTask::new("same", "one"),
            SubTask::new("same", "two"),
        ])
        .is_err()
    );
}
