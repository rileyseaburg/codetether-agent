use crate::Slot;

#[test]
fn newer_pending_value_replaces_older_value() {
    let mut slot = Slot::default();
    assert!(slot.enqueue(|generation| (generation, "old")));
    assert!(!slot.enqueue(|generation| (generation, "new")));
    assert_eq!(slot.take(), Some((2, "new")));
    assert!(slot.is_current(2));
}
