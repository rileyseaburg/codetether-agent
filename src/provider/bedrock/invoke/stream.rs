//! Stream adapter: run a non-streaming InvokeModel completion and replay it
//! as a synthetic stream of [`StreamChunk`]s.

use crate::provider::bedrock::BedrockProvider;
use crate::provider::{CompletionRequest, ContentPart, StreamChunk};
use anyhow::Result;
use futures::stream;

impl BedrockProvider {
    pub(in crate::provider::bedrock) async fn invoke_model_stream_adapter(
        &self,
        request: &CompletionRequest,
        model_id: &str,
    ) -> Result<futures::stream::BoxStream<'static, StreamChunk>> {
        let response = self.complete_invoke_model(request, model_id).await?;
        let mut chunks = Vec::new();
        for part in response.message.content {
            push_part_chunks(part, &mut chunks);
        }
        chunks.push(StreamChunk::Done {
            usage: Some(response.usage),
        });
        Ok(Box::pin(stream::iter(chunks)))
    }
}

fn push_part_chunks(part: ContentPart, chunks: &mut Vec<StreamChunk>) {
    match part {
        ContentPart::Text { text } if !text.is_empty() => chunks.push(StreamChunk::Text(text)),
        ContentPart::Thinking { text, .. } if !text.is_empty() => {
            chunks.push(StreamChunk::Thinking(text));
        }
        ContentPart::ToolCall {
            id,
            name,
            arguments,
            ..
        } => {
            chunks.push(StreamChunk::ToolCallStart {
                id: id.clone(),
                name,
            });
            if !arguments.is_empty() {
                chunks.push(StreamChunk::ToolCallDelta {
                    id: id.clone(),
                    arguments_delta: arguments,
                });
            }
            chunks.push(StreamChunk::ToolCallEnd { id });
        }
        _ => {}
    }
}
