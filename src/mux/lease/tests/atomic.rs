use super::super::{CoordinationReply, LeaseRegistry};
use std::path::Path;

#[test]
fn multi_path_acquisition_is_all_or_nothing() {
    let registry = LeaseRegistry::new();
    let root = Path::new("/workspace");
    registry.acquire("one", "a", root, vec!["src/a.rs".into()]);
    let reply = registry.acquire(
        "two",
        "b",
        root,
        vec!["src/a.rs".into(), "src/free.rs".into()],
    );
    assert!(matches!(reply, CoordinationReply::Blocked { .. }));

    let snapshot = registry.snapshot();
    let CoordinationReply::Snapshot { leases } = snapshot else {
        panic!("expected snapshot");
    };
    assert_eq!(leases.len(), 1);
    assert_eq!(leases[0].owner, "one");
}
