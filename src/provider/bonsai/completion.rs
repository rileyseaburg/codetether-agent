//! Complete responses consume the same generation stream, not a second inference path.
use crate::provider::{CompletionResponse, ContentPart, FinishReason, Message, Role, StreamChunk};
use anyhow::{Result, bail};
use futures::{StreamExt, stream::BoxStream};
pub(super) async fn collect(
    mut stream: BoxStream<'static, StreamChunk>,
) -> Result<CompletionResponse> {
    let mut text = String::new();
    while let Some(chunk) = stream.next().await {
        match chunk {
            StreamChunk::Text(delta) => text.push_str(&delta),
            StreamChunk::Done { usage } => {
                return Ok(CompletionResponse {
                    message: Message {
                        role: Role::Assistant,
                        content: vec![ContentPart::Text { text }],
                    },
                    usage: usage.unwrap_or_default(),
                    finish_reason: FinishReason::Stop,
                });
            }
            StreamChunk::Error(error) => bail!(error),
            StreamChunk::KeepAlive => {}
            other => bail!("Unexpected native Bonsai chunk: {other:?}"),
        }
    }
    bail!("Native Bonsai stream closed without completion")
}
