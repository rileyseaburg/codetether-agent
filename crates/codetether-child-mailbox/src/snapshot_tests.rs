//! Existing crash-recovery regression test.
use super::Snapshot;
use crate::item::{DeliveryState, Item};

#[test]
fn crash_recovery_requeues_running_delivery() {
    let mut snapshot = Snapshot::new("child-1");
    let mut item = Item::new(
        "follow up".into(),
        Vec::new(),
        Some("parent".into()),
        Some(true),
    );
    item.state = DeliveryState::Running;
    snapshot.items.push_back(item);
    assert!(snapshot.recover());
    assert!(
        snapshot
            .items
            .iter()
            .all(|item| item.state == DeliveryState::Pending)
    );
}
