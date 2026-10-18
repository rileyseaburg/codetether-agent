//! Checkout isolation must not weaken same-checkout write protection.

use super::super::{CoordinationReply, LeaseRegistry};
use std::path::Path;

#[test]
fn renewing_parent_file_lease_does_not_block_managed_checkouts() {
    let registry = LeaseRegistry::new();
    let parent = Path::new("/workspace");
    let first = parent.join(".codetether-worktrees/first");
    let second = parent.join(".codetether-worktrees/second");
    assert!(matches!(
        registry.acquire("parent", "parent", parent, vec!["src/lib.rs".into()]),
        CoordinationReply::Acquired { .. }
    ));
    registry.renew("parent");
    for (owner, checkout) in [("first", &first), ("second", &second)] {
        let reply = registry.acquire(owner, owner, checkout, vec!["src/lib.rs".into()]);
        let CoordinationReply::Acquired { leases, .. } = reply else {
            panic!("separate checkout was blocked");
        };
        assert_eq!(&leases[0].workspace, checkout);
    }
    for checkout in [parent, first.as_path(), second.as_path()] {
        let reply = registry.acquire("other", "other", checkout, vec!["src/lib.rs".into()]);
        assert!(matches!(reply, CoordinationReply::Blocked { .. }));
    }
}
