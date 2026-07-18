use super::super::{persistence, residency, store};
use super::subtree_test_support as support;
use std::time::Duration;

#[tokio::test]
async fn close_waits_for_descendant_lifecycle_transition() {
    let (_temp, _guard) = persistence::test_support::isolate();
    let owner = format!("owner-{}", uuid::Uuid::new_v4());
    let child = support::entry("child", &owner, 1).await;
    let grandchild = support::entry("grandchild", child.id(), 2).await;
    support::register(&child).await;
    support::register(&grandchild).await;
    let child_id = child.id().to_string();
    let grandchild_id = grandchild.id().to_string();
    let held = residency::acquire_transition(&grandchild_id).await;
    let closing = super::close(&child_id, Some(&owner));
    tokio::pin!(closing);

    assert!(
        tokio::time::timeout(Duration::from_millis(20), closing.as_mut())
            .await
            .is_err()
    );
    assert!(store::get(&child_id).is_none());
    assert!(store::get(&grandchild_id).is_some());
    drop(held);
    assert!(
        tokio::time::timeout(Duration::from_secs(1), closing)
            .await
            .unwrap()
            .unwrap()
            .success
    );
    support::cleanup(&[child, grandchild]).await;
}
