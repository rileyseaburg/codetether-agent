//! Real streaming adapter for Bedrock's `/converse-stream` endpoint.
//!
//! Parses AWS eventstream frames into [`StreamChunk`]s so callers see text
//! deltas and tool-call argument deltas as they arrive, rather than waiting
//! for the full response.
//!
//! Bedrock event types handled:
//! - `messageStart`         → ignored (role is always assistant)
//! - `contentBlockStart`    → `StreamChunk::ToolCallStart` (only for tool blocks)
//! - `contentBlockDelta`    → `StreamChunk::Text` or `StreamChunk::ToolCallDelta`
//! - `contentBlockStop`     → `StreamChunk::ToolCallEnd` (only for open tool blocks)
//! - `messageStop`          → ignored (finish reason is carried by `Done`)
//! - `metadata`             → captured into usage, emitted at `Done`
//! - `exception` / `error`  → `StreamChunk::Error`

use super::BedrockProvider;
use super::eventstream::{EventMessage, FrameBuffer};
use crate::provider::{StreamChunk, Usage};
use anyhow::{Context, Result};
use futures::StreamExt;
use serde_json::Value;
use std::collections::HashMap;

impl BedrockProvider {
    /// POST to `/model/{id}/converse-stream` and yield `StreamChunk`s as
    /// eventstream frames arrive.
    ///
    /// # Errors
    ///
    /// Returns [`anyhow::Error`] if the initial HTTP request fails or the
    /// server responds non-200. Per-frame decode errors are emitted as
    /// [`StreamChunk::Error`] but do not abort the stream.
    pub(super) async fn converse_stream(
        &self,
        model_id: &str,
        body: Vec<u8>,
    ) -> Result<futures::stream::BoxStream<'static, StreamChunk>> {
        let url = format!("{}/model/{}/converse-stream", self.base_url(), model_id);
        tracing::debug!("Bedrock stream URL: {}", url);

        let response = self
            .send_request("POST", &url, Some(&body), "bedrock")
            .await?;

        let status = response.status();
        if !status.is_success() {
            let text = response.text().await.context("failed to read error body")?;
            anyhow::bail!(
                "Bedrock stream error ({status}): {}",
                crate::util::truncate_bytes_safe(&text, 500)
            );
        }

        let mut byte_stream = response.bytes_stream();
        let mut framer = FrameBuffer::new();
        let mut state = StreamState::default();

        let stream = async_stream::stream! {
            while let Some(chunk) = byte_stream.next().await {
                match chunk {
                    Ok(bytes) => framer.extend(&bytes),
                    Err(e) => {
                        yield StreamChunk::Error(format!("transport error: {e}"));
                        return;
                    }
                }

                loop {
                    match framer.next_frame() {
                        Ok(Some(msg)) => {
                            for out in handle_event(&mut state, msg) {
                                yield out;
                            }
                        }
                        Ok(None) => break,
                        Err(e) => {
                            yield StreamChunk::Error(format!("frame decode: {e}"));
                            return;
                        }
                    }
                }
            }

            yield StreamChunk::Done { usage: state.usage };
        };

        Ok(Box::pin(stream))
    }
}

/// Per-stream state: tracks which content-block indexes are open tool-use
/// blocks so we can emit matching `ToolCallEnd` events, and accumulates usage
/// for the final `Done` chunk.
#[derive(Debug, Default)]
struct StreamState {
    /// Map of contentBlockIndex → tool_use_id for active tool-use blocks.
    open_tool_blocks: HashMap<u64, String>,
    usage: Option<Usage>,
}

fn handle_event(state: &mut StreamState, msg: EventMessage) -> Vec<StreamChunk> {
    let message_type = msg.message_type().unwrap_or("event");
    if matches!(message_type, "exception" | "error") {
        let event_type = msg.event_type().unwrap_or("unknown");
        let payload = String::from_utf8_lossy(&msg.payload);
        return vec![StreamChunk::Error(format!("{event_type}: {payload}"))];
    }

    let event_type = msg.event_type().unwrap_or("");
    let body: Value = match serde_json::from_slice(&msg.payload) {
        Ok(v) => v,
        Err(_) if msg.payload.is_empty() => Value::Null,
        Err(e) => return vec![StreamChunk::Error(format!("bad {event_type} json: {e}"))],
    };

    match event_type {
        "contentBlockStart" => handle_block_start(state, &body),
        "contentBlockDelta" => handle_block_delta(state, &body),
        "contentBlockStop" => handle_block_stop(state, &body),
        "metadata" => {
            state.usage = extract_usage(&body);
            Vec::new()
        }
        // messageStart / messageStop — no session-observable effect
        _ => Vec::new(),
    }
}

fn handle_block_start(state: &mut StreamState, body: &Value) -> Vec<StreamChunk> {
    let Some(idx) = body.get("contentBlockIndex").and_then(|v| v.as_u64()) else {
        return Vec::new();
    };
    let Some(tool_use) = body.get("start").and_then(|v| v.get("toolUse")) else {
        return Vec::new();
    };
    let id = tool_use
        .get("toolUseId")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let name = tool_use
        .get("name")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    if id.is_empty() {
        return Vec::new();
    }
    state.open_tool_blocks.insert(idx, id.clone());
    vec![StreamChunk::ToolCallStart { id, name }]
}

fn handle_block_delta(state: &StreamState, body: &Value) -> Vec<StreamChunk> {
    let idx = body.get("contentBlockIndex").and_then(|v| v.as_u64());
    let Some(delta) = body.get("delta") else {
        return Vec::new();
    };

    if let Some(text) = delta.get("text").and_then(|v| v.as_str())
        && !text.is_empty()
    {
        return vec![StreamChunk::Text(text.to_string())];
    }

    if let Some(tool) = delta.get("toolUse")
        && let Some(partial) = tool.get("input").and_then(|v| v.as_str())
        && let Some(idx) = idx
        && let Some(id) = state.open_tool_blocks.get(&idx)
    {
        return vec![StreamChunk::ToolCallDelta {
            id: id.clone(),
            arguments_delta: partial.to_string(),
        }];
    }

    // Reasoning/thinking deltas — no matching StreamChunk variant; drop silently.
    Vec::new()
}

fn handle_block_stop(state: &mut StreamState, body: &Value) -> Vec<StreamChunk> {
    let Some(idx) = body.get("contentBlockIndex").and_then(|v| v.as_u64()) else {
        return Vec::new();
    };
    if let Some(id) = state.open_tool_blocks.remove(&idx) {
        return vec![StreamChunk::ToolCallEnd { id }];
    }
    Vec::new()
}

fn extract_usage(body: &Value) -> Option<Usage> {
    let u = body.get("usage")?;
    Some(Usage {
        prompt_tokens: u.get("inputTokens").and_then(|v| v.as_u64()).unwrap_or(0) as usize,
        completion_tokens: u.get("outputTokens").and_then(|v| v.as_u64()).unwrap_or(0) as usize,
        total_tokens: u.get("totalTokens").and_then(|v| v.as_u64()).unwrap_or(0) as usize,
        cache_read_tokens: u
            .get("cacheReadInputTokens")
            .and_then(|v| v.as_u64())
            .map(|n| n as usize),
        cache_write_tokens: u
            .get("cacheWriteInputTokens")
            .and_then(|v| v.as_u64())
            .map(|n| n as usize),
    })
}
