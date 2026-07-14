use super::partition;
use crate::swarm::SubTask;
use std::collections::HashSet;

#[test]
fn failed_prerequisite_blocks_its_dependents() {
    let dependency = SubTask::new("first", "inspect");
    let dependent = SubTask::new("second", "edit").with_dependencies(vec![dependency.id.clone()]);
    let failed = HashSet::from([dependency.id]);
    let (runnable, blocked) = partition(&[dependent], &failed);
    assert!(runnable.is_empty());
    assert_eq!(blocked.len(), 1);
    assert!(!blocked[0].success);
    assert!(
        blocked[0]
            .error
            .as_deref()
            .unwrap()
            .contains("failed dependencies")
    );
}
