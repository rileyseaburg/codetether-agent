//! Worker status survives both progress accounting and canonical checkpointing.
use super::image_fixture::ImageTool;
use crate::provider::{ContentPart, Message, Role};
use crate::session::Session;
#[tokio::test]
async fn worker_progress_preserves_failure_status_without_double_counting() {
    let cwd = tempfile::tempdir().unwrap();
    let mut registry = crate::tool::ToolRegistry::new();
    registry.register(std::sync::Arc::new(ImageTool));
    let mut session = Session::new().await.unwrap();
    for index in 0..3 {
        let id = index.to_string();
        session.add_message(Message {
            role: Role::Assistant,
            content: vec![ContentPart::ToolCall {
                id: id.clone(),
                name: "read".into(),
                arguments: "{\"success\":false}".into(),
                thought_signature: None,
            }],
        });
        super::execute_tool_call(
            &mut session,
            &registry,
            crate::a2a::worker::AutoApprove::All,
            cwd.path(),
            "mock",
            &None,
            (id, "read".into(), serde_json::json!({"success":false})),
        )
        .await
        .unwrap();
    }
    assert_eq!(session.metadata.progress.observations, 3);
    let checkpoint = codetether_run_checkpoint::RunCheckpoint::from_session_messages(
        "task",
        10,
        &session.id,
        None,
        session.message_count(),
        &session.messages,
    );
    assert_eq!(checkpoint.actions.len(), 3);
    assert!(
        checkpoint
            .actions
            .iter()
            .all(|action| action.status == codetether_run_checkpoint::ActionStatus::Failed)
    );
}
