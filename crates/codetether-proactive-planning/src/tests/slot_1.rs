use crate::Slot;

#[test]
fn settle_waits_for_pending_work() {
    let mut slot = Slot::default();
    slot.enqueue(|generation| generation);
    assert!(!slot.settle());
    slot.take();
    assert!(slot.settle());
}
