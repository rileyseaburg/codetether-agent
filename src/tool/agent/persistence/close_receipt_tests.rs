use super::super::{collaboration_runtime::message_queue, execution_state, params::Params, store};
use super::test_support;
use serde_json::json;

#[tokio::test]
async fn close_releases_an_inflight_mailbox_receipt() {
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
    let submission = message_queue::enqueue(&child.id, "queued work".into(), &params)
        .await
        .unwrap();
    let receipt = message_queue::claim_for_test(&child.id)
        .await
        .unwrap()
        .unwrap()
        .0;
    assert_eq!(receipt, submission.id);
    let guard = execution_state::try_start(&child.id).unwrap();
    execution_state::close(&child.id);
    message_queue::finished(child.id.clone(), Some(receipt.clone()), guard).await;

    let retried = message_queue::claim_for_test(&child.id)
        .await
        .unwrap()
        .unwrap()
        .0;
    assert_eq!(retried, receipt);
    message_queue::clear(&child.id).await.unwrap();
    super::remove(&entry).await.unwrap();
    store::remove(&child.id);
    execution_state::reopen(&child.id);
}
