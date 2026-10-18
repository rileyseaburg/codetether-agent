use crate::provider::ContentPart;
use crate::session::Session;

#[tokio::test]
async fn agent_guard_rejects_recall_after_persistent_denial() {
    let mut session = Session::new().await.expect("session");
    session.add_human_message(crate::provider::Message {
        role: crate::provider::Role::User,
        content: vec![ContentPart::Text {
            text: "never use session recall".into(),
        }],
    });
    assert_eq!(session.metadata.prior_context_allowed, Some(false));
    let call = ContentPart::ToolCall {
        id: "recall-1".into(),
        name: "session_recall".into(),
        arguments: "{}".into(),
        thought_signature: None,
    };
    let result = super::tool_calls_policy::blocked(&session, &call).expect("blocked");
    assert_eq!(
        result.metadata["error_code"],
        "PRIOR_CONTEXT_DISABLED_BY_USER"
    );
}

#[tokio::test]
async fn inherited_denial_cannot_be_overridden_inside_child() {
    let mut session = Session::new().await.expect("session");
    session.metadata.inherited_prior_context_allowed = Some(false);
    session.add_human_message(crate::provider::Message {
        role: crate::provider::Role::User,
        content: vec![ContentPart::Text {
            text: "enable session recall".into(),
        }],
    });
    assert_eq!(session.metadata.prior_context_allowed, Some(true));
    let call = ContentPart::ToolCall {
        id: "recall-2".into(),
        name: "session_recall".into(),
        arguments: "{}".into(),
        thought_signature: None,
    };
    assert!(super::tool_calls_policy::blocked(&session, &call).is_some());
}
