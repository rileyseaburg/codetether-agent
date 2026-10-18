//! A mux owner can never obtain or renew a whole-workspace lease.

use super::super::{CoordinationReply, LeaseRegistry};
use std::path::PathBuf;

#[test]
fn workspace_claims_are_rejected_atomically_without_blocking_peers() {
    let root = tempfile::tempdir().unwrap();
    std::fs::create_dir(root.path().join("src")).unwrap();
    let registry = LeaseRegistry::new();
    for broad in [
        PathBuf::new(),
        ".".into(),
        "./".into(),
        "src/..".into(),
        root.path().into(),
    ] {
        let reply = registry.acquire("antisocial", "one", root.path(), vec!["a.rs".into(), broad]);
        assert_eq!(reply, CoordinationReply::WorkspaceScopeForbidden);
        assert_eq!(
            registry.renew("antisocial"),
            CoordinationReply::Renewed { count: 0 }
        );
    }
    let reply = registry.acquire("peer", "two", root.path(), vec!["a.rs".into()]);
    assert!(matches!(reply, CoordinationReply::Acquired { .. }));
    let CoordinationReply::Snapshot { leases } = registry.snapshot() else {
        panic!("expected snapshot");
    };
    assert_eq!(leases.len(), 1);
    assert_eq!(leases[0].owner, "peer");
}

#[cfg(unix)]
#[test]
fn symlink_alias_cannot_claim_the_workspace() {
    let root = tempfile::tempdir().unwrap();
    std::os::unix::fs::symlink(root.path(), root.path().join("alias")).unwrap();
    let registry = LeaseRegistry::new();
    let reply = registry.acquire("one", "one", root.path(), vec!["alias".into()]);
    assert_eq!(reply, CoordinationReply::WorkspaceScopeForbidden);
}
