use crate::session::Session;
use crate::tool::agent::{persistence, store};

#[tokio::test]
async fn durable_identity_check_is_exactly_owner_scoped() {
    let (_temp, _guard) = persistence::test_support::isolate();
    let name = format!("worker-{}", uuid::Uuid::new_v4());
    let owner_a = format!("owner-{}", uuid::Uuid::new_v4());
    let owner_b = format!("owner-{}", uuid::Uuid::new_v4());
    let session = Session::new().await.unwrap();
    let id = session.id.clone();
    store::insert(store::AgentEntry {
        name: name.clone(),
        instructions: "test".into(),
        session,
        parent: None,
        owner_session_id: Some(owner_a.clone()),
        depth: 0,
        model_id: None,
    });
    let same_owner = super::reserve(&name, Some(&owner_a)).await;
    let Err(result) = same_owner else {
        panic!("same owner should be rejected")
    };
    assert!(!result.success);
    let other_owner = super::reserve(&name, Some(&owner_b)).await;
    assert!(other_owner.is_ok());
    drop(other_owner);
    store::remove(&id);
}
