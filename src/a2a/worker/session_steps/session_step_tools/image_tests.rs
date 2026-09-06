//! Mocked local worker results retain images for both success and failure.
use super::image_fixture::{IMAGE, ImageTool, OUTPUT};
use crate::{
    a2a::worker::AutoApprove,
    provider::{ContentPart, Role},
};
use std::sync::Arc;
#[tokio::test]
async fn worker_image_recording_preserves_success_and_failure() {
    let cwd = tempfile::tempdir().unwrap();
    let mut registry = crate::tool::ToolRegistry::new();
    registry.register(Arc::new(ImageTool));
    let mut session = crate::session::Session::new().await.unwrap();
    for success in [true, false] {
        let id = format!("worker-image-{success}");
        super::execute_tool_call(
            &mut session,
            &registry,
            AutoApprove::All,
            cwd.path(),
            "mock",
            &None,
            (
                id.clone(),
                "read".into(),
                serde_json::json!({"success": success}),
            ),
        )
        .await;
        let message = session.messages.last().unwrap();
        assert!(matches!(message.role, Role::Tool));
        assert_eq!(message.content.len(), 3);
        assert!(matches!(&message.content[0], ContentPart::ToolResult {
            tool_call_id, content
        } if tool_call_id == &id && content == OUTPUT));
        assert!(
            matches!(&message.content[1], ContentPart::Image { url, mime_type }
            if url == IMAGE && mime_type.as_deref() == Some("image/png"))
        );
        assert!(
            matches!(&message.content[2], ContentPart::Image { url, mime_type }
            if url == IMAGE && mime_type.is_none())
        );
    }
    assert_eq!(session.messages.len(), 2);
}
