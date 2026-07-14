use super::assign;
use crate::swarm::SubTask;
use std::collections::HashMap;

#[test]
fn assigns_every_dependency_stage() {
    let first = SubTask::new("inspect", "inspect");
    let second = SubTask::new("edit", "edit").with_dependencies(vec![first.id.clone()]);
    let third = SubTask::new("verify", "verify").with_dependencies(vec![second.id.clone()]);
    let mut tasks = [first, second, third]
        .into_iter()
        .map(|task| (task.id.clone(), task))
        .collect::<HashMap<_, _>>();
    assign(&mut tasks).expect("valid graph");
    let mut stages = tasks.values().map(|task| task.stage).collect::<Vec<_>>();
    stages.sort_unstable();
    assert_eq!(stages, vec![0, 1, 2]);
}

#[test]
fn rejects_dependency_cycles() {
    let mut first = SubTask::new("first", "inspect");
    let mut second = SubTask::new("second", "inspect");
    first.dependencies = vec![second.id.clone()];
    second.dependencies = vec![first.id.clone()];
    let mut tasks = [first, second]
        .into_iter()
        .map(|task| (task.id.clone(), task))
        .collect();
    assert!(assign(&mut tasks).is_err());
}
