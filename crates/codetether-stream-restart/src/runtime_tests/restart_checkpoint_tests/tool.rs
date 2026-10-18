use super::{provider, request};
use crate::{RestartPolicy, run};
use codetether_provider_core::{ContentPart, StreamChunk};

#[tokio::test(start_paused = true)]
async fn completed_tool_returns_without_replaying_side_effect() {
    let tool = ContentPart::ToolCall {
        id: "call_1".into(),
        name: "read".into(),
        arguments: "{}".into(),
        thought_signature: None,
    };
    let (concrete, provider) = provider(vec![vec![
        StreamChunk::OutputItemDone { content: tool },
        StreamChunk::Error("codex-retryable: reset".into()),
    ]]);
    let response = run::<()>(
        &provider,
        request(),
        "session",
        None,
        &RestartPolicy::default(),
    )
    .await
    .unwrap();
    assert!(
        matches!(&response.message.content[..], [ContentPart::ToolCall { id, .. }] if id == "call_1")
    );
    assert_eq!(concrete.requests.lock().unwrap().len(), 1);
}
