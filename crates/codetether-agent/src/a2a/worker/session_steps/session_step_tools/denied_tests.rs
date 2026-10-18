//! Synthetic approval denials retain their original text and carry no images.
use crate::{
    a2a::worker::AutoApprove,
    provider::{ContentPart, Role},
};
#[tokio::test]
async fn worker_image_recording_denial_is_text_only() {
    let cwd = tempfile::tempdir().unwrap();
    let registry = crate::tool::ToolRegistry::new();
    let mut session = crate::session::Session::new().await.unwrap();
    super::execute_tool_call(
        &mut session,
        &registry,
        AutoApprove::None,
        cwd.path(),
        "mock",
        &None,
        ("denied-image".into(), "write".into(), serde_json::json!({})),
    )
    .await;
    assert_eq!(session.messages.len(), 1);
    let message = &session.messages[0];
    assert!(matches!(message.role, Role::Tool));
    assert_eq!(message.content.len(), 1);
    assert!(
        matches!(&message.content[0], ContentPart::ToolResult { tool_call_id, content }
        if tool_call_id == "denied-image" && content ==
        "Tool 'write' requires approval but auto-approve policy is None")
    );
}
