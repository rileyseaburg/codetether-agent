use super::super::{CoordinationReply, LeaseRegistry};
use std::path::Path;

#[test]
fn release_and_expiry_make_paths_available() {
    let registry = LeaseRegistry::new();
    let root = Path::new("/workspace");
    registry.acquire("one", "a", root, vec!["src/a.rs".into()]);
    assert!(matches!(
        registry.release("one"),
        CoordinationReply::Released { count: 1 }
    ));
    assert!(matches!(
        registry.acquire("two", "b", root, vec!["src/a.rs".into()]),
        CoordinationReply::Acquired { .. }
    ));

    for lease in registry.entries.lock().unwrap().values_mut() {
        lease.expires_at_ms = 0;
    }
    assert!(matches!(
        registry.acquire("three", "c", root, vec!["src/a.rs".into()]),
        CoordinationReply::Acquired { .. }
    ));
}
