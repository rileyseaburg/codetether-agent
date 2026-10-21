use async_trait::async_trait;
use codetether_provider_core::*;
use codetether_rust_repair::{RepairRequest, completion_request, propose};
use std::sync::atomic::{AtomicUsize, Ordering};
struct Mock {
    content: Vec<ContentPart>,
    finish: FinishReason,
    calls: AtomicUsize,
}
#[async_trait]
impl Provider for Mock {
    fn name(&self) -> &str {
        "fixture"
    }
    async fn list_models(&self) -> anyhow::Result<Vec<ModelInfo>> {
        Ok(vec![])
    }
    async fn complete(&self, request: CompletionRequest) -> anyhow::Result<CompletionResponse> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        assert!(request.tools.is_empty());
        assert_eq!(request.temperature, None);
        assert_eq!(request.max_tokens, Some(8000));
        assert_eq!(request.messages.len(), 2);
        Ok(CompletionResponse {
            message: Message {
                role: Role::Assistant,
                content: self.content.clone(),
            },
            finish_reason: self.finish,
            usage: Usage::default(),
        })
    }
    async fn complete_stream(
        &self,
        _: CompletionRequest,
    ) -> anyhow::Result<futures::stream::BoxStream<'static, StreamChunk>> {
        anyhow::bail!("streaming must not be called")
    }
}
fn snapshot() -> RepairRequest {
    RepairRequest {
        schema_version: 1,
        diagnostics: vec![],
        files: vec![],
    }
}
fn text() -> ContentPart {
    ContentPart::Text {
        text: r#"{"schema_version":1,"summary":"Insufficient context","edits":[]}"#.into(),
    }
}
#[tokio::test]
async fn makes_exactly_one_tool_free_completion() {
    let mock = Mock {
        content: vec![text()],
        finish: FinishReason::Stop,
        calls: AtomicUsize::new(0),
    };
    assert!(
        propose(&mock, "resolved-model".into(), &snapshot())
            .await
            .unwrap()
            .edits
            .is_empty()
    );
    assert_eq!(mock.calls.load(Ordering::SeqCst), 1);
}
#[tokio::test]
async fn rejects_tool_calls_nontext_and_truncation() {
    for (content, finish) in [
        (
            vec![ContentPart::ToolCall {
                id: "x".into(),
                name: "write".into(),
                arguments: "{}".into(),
                thought_signature: None,
            }],
            FinishReason::ToolCalls,
        ),
        (
            vec![
                ContentPart::Thinking {
                    text: "reasoning".into(),
                    signature: None,
                },
                text(),
            ],
            FinishReason::Stop,
        ),
        (vec![text()], FinishReason::Length),
    ] {
        let mock = Mock {
            content,
            finish,
            calls: AtomicUsize::new(0),
        };
        assert!(propose(&mock, "model".into(), &snapshot()).await.is_err());
        assert_eq!(mock.calls.load(Ordering::SeqCst), 1);
    }
}
#[test]
fn invalid_snapshot_is_rejected_before_request_creation() {
    let mut request = snapshot();
    request.schema_version = 2;
    assert!(completion_request("model".into(), &request).is_err());
}
