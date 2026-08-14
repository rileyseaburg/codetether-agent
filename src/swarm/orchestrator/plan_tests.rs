use super::{single, validate};
use crate::swarm::SubTask;

#[test]
fn parallel_plan_requires_multiple_root_workers() {
    let first = SubTask::new("inspect", "inspect");
    let second = SubTask::new("edit", "edit").with_dependencies(vec![first.id.clone()]);
    assert!(validate(vec![first, second], true).is_err());
}

#[test]
fn parallel_plan_accepts_independent_root_workers() {
    let first = SubTask::new("api", "inspect api");
    let second = SubTask::new("tests", "inspect tests");
    let plan = validate(vec![first, second], true).expect("parallel roots");
    assert_eq!(plan.values().filter(|task| task.stage == 0).count(), 2);
}

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
    assert!(validate(vec![first, second], false).is_err());
}

#[test]
fn validation_rejects_empty_and_duplicate_plans() {
    assert!(validate(Vec::new(), false).is_err());
    assert!(
        validate(
            vec![SubTask::new("same", "one"), SubTask::new("same", "two")],
            false,
        )
        .is_err()
    );
}
