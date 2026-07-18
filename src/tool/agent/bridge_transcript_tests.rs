use super::super::store::{self, AgentEntry};
use crate::provider::{ContentPart, Message, Role};
use crate::session::Session;

#[tokio::test]
async fn transcript_bridge_is_scoped_to_the_parent_session() {
    let mut child = Session::new().await.expect("child session");
    child.messages.push(Message {
        role: Role::Assistant,
        content: vec![ContentPart::Text {
            text: "visible".into(),
        }],
    });
    let name = format!("transcript-{}", child.id);
    let child_id = child.id.clone();
    store::insert(AgentEntry {
        name: name.clone(),
        instructions: "test".into(),
        session: child,
        parent: None,
        owner_session_id: Some("owner-a".into()),
        depth: 0,
        model_id: None,
    });
    let visible = super::agent_tool_transcript_for_parent(&name, "owner-a").unwrap();
    assert_eq!(visible.len(), 1);
    assert!(super::agent_tool_transcript_for_parent(&name, "owner-b").is_none());
    store::remove(&child_id);
}
