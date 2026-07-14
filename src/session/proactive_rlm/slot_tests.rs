//! Tests for latest-generation coalescing semantics.

use super::slot::Slot;

#[test]
fn newer_pending_value_replaces_older_value() {
    let mut slot = Slot::default();
    assert!(slot.enqueue(|generation| (generation, "old")));
    assert!(!slot.enqueue(|generation| (generation, "new")));
    assert_eq!(slot.take(), Some((2, "new")));
    assert!(slot.is_current(2));
}

#[test]
fn settle_waits_for_pending_work() {
    let mut slot = Slot::default();
    slot.enqueue(|generation| generation);
    assert!(!slot.settle());
    slot.take();
    assert!(slot.settle());
}
