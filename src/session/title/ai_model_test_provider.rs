use crate::provider::*;
use anyhow::Result;
use async_trait::async_trait;
use futures::stream::BoxStream;

pub(super) struct SelectedProvider;

#[async_trait]
impl Provider for SelectedProvider {
    fn name(&self) -> &str {
        "selected"
    }

    async fn list_models(&self) -> Result<Vec<ModelInfo>> {
        Ok(Vec::new())
    }

    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse> {
        assert_eq!(request.model, "selected-model");
        Ok(response())
    }

    async fn complete_stream(
        &self,
        _: CompletionRequest,
    ) -> Result<BoxStream<'static, StreamChunk>> {
        anyhow::bail!("streaming is not used for title generation")
    }
}

fn response() -> CompletionResponse {
    CompletionResponse {
        message: Message {
            role: Role::Assistant,
            content: vec![ContentPart::Text {
                text: "Generated title".into(),
            }],
        },
        usage: Usage::default(),
        finish_reason: FinishReason::Stop,
    }
}
