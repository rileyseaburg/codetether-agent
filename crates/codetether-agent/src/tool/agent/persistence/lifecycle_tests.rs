use super::super::{collaboration_runtime::message_queue, params::Params, store, thread_lifecycle};
use super::{manifest::Lifecycle, manifest_scan, test_support};
use serde_json::json;

#[tokio::test]
async fn close_and_resume_preserve_identity_and_mailbox() {
    let (_temp, _guard) = test_support::isolate();
    let child = crate::session::Session::new().await.unwrap();
    child.save().await.unwrap();
    let owner = format!("parent-{}", uuid::Uuid::new_v4());
    let name = format!("child-{}", uuid::Uuid::new_v4());
    let entry = test_support::entry(&name, child.clone(), &owner);
    super::save(&name, &entry).await.unwrap();
    assert_eq!(super::hydrate_parent(Some(&owner)).await.unwrap(), 1);
    let params: Params = serde_json::from_value(json!({
        "action":"message", "__ct_session_id":owner
    }))
    .unwrap();
    message_queue::enqueue(&child.id, "queued work".into(), &params)
        .await
        .unwrap();

    let closed = thread_lifecycle::close(&child.id, Some(&owner))
        .await
        .unwrap();
    assert!(closed.success);
    assert!(store::get(&child.id).is_none());
    assert_eq!(message_queue::pending(&child.id).await, 1);
    let manifest = manifest_scan::find(Some(&owner), &child.id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(manifest.lifecycle, Lifecycle::Closed);

    message_queue::clear(&child.id).await.unwrap();
    let resumed = thread_lifecycle::resume(&child.id, Some(&owner), Default::default())
        .await
        .unwrap();
    assert!(resumed.success);
    assert_eq!(store::get(&child.id).unwrap().session.id, child.id);
    let manifest = manifest_scan::find(Some(&owner), &child.id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(manifest.lifecycle, Lifecycle::Open);

    message_queue::clear(&child.id).await.unwrap();
    super::remove(&entry).await.unwrap();
    store::remove(&child.id);
}
