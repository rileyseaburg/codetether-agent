use super::{Manifest, find};
use crate::tool::agent::persistence::manifest::Lifecycle;

#[test]
fn resolves_absolute_relative_and_id_targets_within_one_root() {
    let parent = manifest("parent-id", "root-id", "researcher");
    let worker = manifest("worker-id", "parent-id", "worker");
    let foreign = manifest("foreign-id", "other-root", "worker");
    let all = [parent, worker, foreign];
    assert_eq!(
        find(&all, "root-id", "/root/researcher/worker")
            .unwrap()
            .child_session_id,
        "worker-id"
    );
    assert_eq!(
        find(&all, "parent-id", "worker").unwrap().child_session_id,
        "worker-id"
    );
    assert_eq!(
        find(&all, "root-id", "worker-id").unwrap().child_session_id,
        "worker-id"
    );
    assert!(find(&all, "root-id", "foreign-id").is_none());
}

fn manifest(id: &str, owner: &str, name: &str) -> Manifest {
    Manifest {
        version: 1,
        name: name.into(),
        instructions: "test".into(),
        child_session_id: id.into(),
        parent: None,
        owner_session_id: Some(owner.into()),
        depth: 1,
        model_id: None,
        lifecycle: Lifecycle::Open,
    }
}
