//! Existing pairing regression.
use super::*;

#[test]
fn orphan_result_is_dropped() {
    let mut msgs = vec![user("q"), result("nonexistent"), user("next")];
    repair_orphans(&mut msgs);
    // The orphaned result is removed.
    assert_eq!(msgs.len(), 2);
    assert_eq!(msgs[0].role, Role::User);
    assert_eq!(msgs[1].role, Role::User);
}
