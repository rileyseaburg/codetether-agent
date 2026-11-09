//! Validate actual provider-neutral stream events and assemble assistant history.
use anyhow::{Result, ensure};
use codetether_provider_core::{ContentPart, Message, Role, StreamChunk};
use futures::{StreamExt, stream::BoxStream};

pub(super) async fn collect(mut stream: BoxStream<'static, StreamChunk>) -> Result<Message> {
    let mut content = Vec::new();
    let mut pending: Option<(String, String, String)> = None;
    let mut done = false;
    while let Some(chunk) = stream.next().await {
        ensure!(!done, "stream emitted an event after Done");
        match chunk {
            StreamChunk::KeepAlive | StreamChunk::Thinking(_) => {}
            StreamChunk::Text(text) => content.push(ContentPart::Text { text }),
            StreamChunk::ToolCallStart { id, name } => {
                ensure!(pending.is_none() && !id.is_empty(), "invalid tool start");
                pending = Some((id, name, String::new()));
            }
            StreamChunk::ToolCallDelta {
                id,
                arguments_delta,
            } => {
                let Some((active, _, args)) = &mut pending else {
                    anyhow::bail!("tool delta without start");
                };
                ensure!(*active == id, "tool delta ID mismatch");
                args.push_str(&arguments_delta);
            }
            StreamChunk::ToolCallEnd { id } => {
                let Some((active, name, arguments)) = pending.take() else {
                    anyhow::bail!("tool end without start");
                };
                ensure!(active == id, "tool end ID mismatch");
                content.push(ContentPart::ToolCall {
                    id,
                    name,
                    arguments,
                    thought_signature: None,
                });
            }
            StreamChunk::Done { .. } => {
                ensure!(pending.is_none(), "unfinished tool call");
                done = true;
            }
            StreamChunk::Error(_) => anyhow::bail!("provider emitted a stream error"),
            StreamChunk::OutputItemDone { .. } => {
                anyhow::bail!("unexpected Responses item from Gemini")
            }
        }
    }
    ensure!(done, "stream omitted Done");
    Ok(Message {
        role: Role::Assistant,
        content,
    })
}
