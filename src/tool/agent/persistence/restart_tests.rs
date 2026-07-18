use super::super::{collaboration_runtime::message_queue, params::Params, store};
use serde_json::json;

use super::test_support as support;

#[path = "restart_assertions.rs"]
mod assertions;

#[tokio::test]
async fn registry_and_running_mailbox_survive_restart() {
    let (_temp, _guard) = support::isolate();
    let child = crate::session::Session::new().await.unwrap();
    child.save().await.unwrap();
    let owner = format!("parent-{}", uuid::Uuid::new_v4());
    let name = format!("child-{}", uuid::Uuid::new_v4());
    let entry = support::entry(&name, child.clone(), &owner);
    super::save(&name, &entry).await.unwrap();
    assert_eq!(super::hydrate_parent(Some(&owner)).await.unwrap(), 1);
    let restored = store::get_for_parent(&name, Some(&owner)).unwrap();
    assert_eq!(restored.session.id, child.id);
    let params: Params = serde_json::from_value(json!({
        "action":"message", "__ct_session_id":owner
    }))
    .unwrap();
    assertions::mailbox_survives(&child.id, &params).await;
    message_queue::clear(&child.id).await.unwrap();
    super::remove(&entry).await.unwrap();
    store::remove(&child.id);
}
