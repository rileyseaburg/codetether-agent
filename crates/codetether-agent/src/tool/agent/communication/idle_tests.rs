use super::queue_only;
use crate::provider::ContentPart;
use crate::session::Session;
use crate::tool::agent::store::{self, AgentEntry};

#[tokio::test]
async fn idle_child_persists_message_without_starting_a_turn() {
    let (_temp, _guard) = crate::tool::agent::persistence::test_support::isolate();
    let child = Session::new().await.unwrap();
    let id = child.id.clone();
    store::insert(AgentEntry {
        name: "idle-child".into(),
        instructions: "test".into(),
        session: child,
        parent: None,
        owner_session_id: Some("idle-owner".into()),
        depth: 0,
        model_id: None,
    });
    let result = queue_only("idle-child", Some("idle-owner"), "context only".into())
        .await
        .unwrap();
    assert!(result.success);
    assert!(!super::super::execution_state::is_running(&id));
    let restored = Session::load(&id).await.unwrap();
    assert!(
        restored
            .messages
            .last()
            .unwrap()
            .content
            .iter()
            .any(|part| matches!(part, ContentPart::Text { text } if text == "context only"))
    );
    store::remove(&id);
}
