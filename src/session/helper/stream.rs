use super::super::SessionEvent;
use crate::provider::{ContentPart, FinishReason, Message, Role, StreamChunk, Usage};
use anyhow::Result;
use futures::StreamExt;
use futures::stream::BoxStream;
use std::collections::HashMap;

#[derive(Default)]
struct ToolAccumulator {
    id: String,
    name: String,
    arguments: String,
}

pub async fn collect_stream_completion_with_events(
    mut stream: BoxStream<'static, StreamChunk>,
    event_tx: Option<&tokio::sync::mpsc::Sender<SessionEvent>>,
) -> Result<crate::provider::CompletionResponse> {
    let mut text = String::new();
    let mut tools = Vec::<ToolAccumulator>::new();
    let mut tool_index_by_id = HashMap::<String, usize>::new();
    let mut usage = Usage::default();

    while let Some(chunk) = stream.next().await {
        match chunk {
            StreamChunk::Text(delta) => {
                if delta.is_empty() {
                    continue;
                }
                text.push_str(&delta);
                if let Some(tx) = event_tx {
                    let _ = tx.send(SessionEvent::TextChunk(text.clone())).await;
                }
            }
            StreamChunk::ToolCallStart { id, name } => {
                let next_idx = tools.len();
                let idx = *tool_index_by_id.entry(id.clone()).or_insert(next_idx);
                if idx == next_idx {
                    tools.push(ToolAccumulator {
                        id,
                        name,
                        arguments: String::new(),
                    });
                } else if tools[idx].name == "tool" {
                    tools[idx].name = name;
                }
            }
            StreamChunk::ToolCallDelta {
                id,
                arguments_delta,
            } => {
                if let Some(idx) = tool_index_by_id.get(&id).copied() {
                    tools[idx].arguments.push_str(&arguments_delta);
                } else {
                    let idx = tools.len();
                    tool_index_by_id.insert(id.clone(), idx);
                    tools.push(ToolAccumulator {
                        id,
                        name: "tool".to_string(),
                        arguments: arguments_delta,
                    });
                }
            }
            StreamChunk::ToolCallEnd { .. } => {}
            StreamChunk::Done { usage: done_usage } => {
                if let Some(done_usage) = done_usage {
                    usage = done_usage;
                }
            }
            StreamChunk::Error(message) => anyhow::bail!(message),
        }
    }

    let mut content = Vec::new();
    if !text.is_empty() {
        content.push(ContentPart::Text { text });
    }
    for tool in tools {
        content.push(ContentPart::ToolCall {
            id: tool.id,
            name: tool.name,
            arguments: tool.arguments,
            thought_signature: None,
        });
    }

    let finish_reason = if content
        .iter()
        .any(|part| matches!(part, ContentPart::ToolCall { .. }))
    {
        FinishReason::ToolCalls
    } else {
        FinishReason::Stop
    };

    Ok(crate::provider::CompletionResponse {
        message: Message {
            role: Role::Assistant,
            content,
        },
        usage,
        finish_reason,
    })
}
