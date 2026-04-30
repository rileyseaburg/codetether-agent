//! Stream-completion collection with incremental event forwarding.
//!
//! [`collect_stream_completion_with_events`] drains a provider stream,
//! accumulates text and tool-call deltas into a final
//! [`CompletionResponse`](crate::provider::CompletionResponse), and (optionally)
//! forwards incremental [`SessionEvent::TextChunk`] snapshots to a UI layer.
//!
//! ## Snapshot truncation
//!
//! Each text chunk forwarded over `event_tx` is a **full snapshot** of the
//! accumulated assistant text so far. For extremely long replies this would be
//! O(n²) in memory; to bound the worst case the snapshot is capped at
//! [`MAX_STREAM_SNAPSHOT_BYTES`] with a trailing `" …[truncated]"` marker.
//! The full text is still returned in the final [`CompletionResponse`]; only
//! the streamed previews are truncated.

use super::super::SessionEvent;
use crate::provider::{ContentPart, FinishReason, Message, Role, StreamChunk, Usage};
use anyhow::Result;
use futures::StreamExt;
use futures::stream::BoxStream;
use std::collections::HashMap;

/// Maximum bytes forwarded per [`SessionEvent::TextChunk`] snapshot.
///
/// Bounds worst-case memory for runaway providers to O(n) rather than O(n²)
/// across the streaming lifetime. The final response is **not** truncated.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::session::helper::stream::MAX_STREAM_SNAPSHOT_BYTES;
/// assert_eq!(MAX_STREAM_SNAPSHOT_BYTES, 256 * 1024);
/// ```
pub const MAX_STREAM_SNAPSHOT_BYTES: usize = 256 * 1024;

#[derive(Default)]
struct ToolAccumulator {
    id: String,
    name: String,
    arguments: String,
}

/// Collect a streaming completion into a [`CompletionResponse`](crate::provider::CompletionResponse),
/// optionally forwarding incremental events.
///
/// Reads [`StreamChunk`]s from `stream`, accumulates assistant text and
/// tool-call argument deltas keyed by tool-call id, and tracks the final
/// [`FinishReason`] and [`Usage`]. When `event_tx` is `Some`, each text delta
/// triggers a [`SessionEvent::TextChunk`] carrying the full accumulated text
/// up to that point — truncated to [`MAX_STREAM_SNAPSHOT_BYTES`] with a
/// `" …[truncated]"` suffix when exceeded.
///
/// # Arguments
///
/// * `stream` — Boxed async stream of [`StreamChunk`]s from a provider.
/// * `event_tx` — Optional channel for UI preview events; pass `None` for
///   headless/non-interactive callers.
///
/// # Returns
///
/// A fully materialized [`CompletionResponse`](crate::provider::CompletionResponse)
/// containing the complete assistant text and any accumulated tool calls.
///
/// # Errors
///
/// Returns [`anyhow::Error`] if the stream yields a terminal error chunk or if
/// response assembly fails.
///
/// # Examples
///
/// ```rust,no_run
/// # tokio::runtime::Runtime::new().unwrap().block_on(async {
/// use codetether_agent::session::helper::stream::collect_stream_completion_with_events;
/// use futures::stream;
///
/// // In practice the stream comes from a Provider::stream() call.
/// let s = Box::pin(stream::empty());
/// let response = collect_stream_completion_with_events(s, None).await.unwrap();
/// // `response` is a CompletionResponse; inspect it as needed.
/// let _ = response;
/// # });
/// ```
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
                    let to_send = if text.len() > MAX_STREAM_SNAPSHOT_BYTES {
                        let mut t =
                            crate::util::truncate_bytes_safe(&text, MAX_STREAM_SNAPSHOT_BYTES)
                                .to_string();
                        t.push_str(" …[truncated]");
                        t
                    } else {
                        text.clone()
                    };
                    let _ = tx.send(SessionEvent::TextChunk(to_send)).await;
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
            StreamChunk::ToolCallEnd { .. } | StreamChunk::Thinking(_) => {}
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
