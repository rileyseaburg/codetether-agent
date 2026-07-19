use super::super::{CoordinationReply, LeaseRegistry};
use std::path::{Path, PathBuf};

#[test]
fn parent_path_blocks_another_owners_child_path() {
    let registry = LeaseRegistry::new();
    let root = Path::new("/workspace");
    let first = registry.acquire("one", "agent-one", root, vec![PathBuf::from("src/auth")]);
    assert!(matches!(first, CoordinationReply::Acquired { .. }));

    let second = registry.acquire(
        "two",
        "agent-two",
        root,
        vec![PathBuf::from("src/auth/mod.rs")],
    );
    let CoordinationReply::Blocked { conflicts } = second else {
        panic!("overlapping path was not blocked");
    };
    assert_eq!(conflicts[0].owner, "one");
}

#[test]
fn disjoint_paths_can_be_owned_concurrently() {
    let registry = LeaseRegistry::new();
    let root = Path::new("/workspace");
    registry.acquire("one", "a", root, vec!["src/a.rs".into()]);
    let reply = registry.acquire("two", "b", root, vec!["src/b.rs".into()]);
    assert!(matches!(reply, CoordinationReply::Acquired { .. }));
}
