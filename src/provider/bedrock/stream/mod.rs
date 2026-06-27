//! Real streaming adapter for Bedrock's `/converse-stream` endpoint.
//!
//! Parses AWS eventstream frames into [`StreamChunk`]s so callers see text
//! deltas, thinking deltas, and tool-call argument deltas as they arrive,
//! rather than waiting for the full response.
//!
//! Module layout: [`request`] performs the signed HTTP call (mapping auth
//! failures via [`auth_error`]), [`pump`] turns response bytes into frames,
//! and the handlers in [`delta`] and [`events`] map frames to chunks.

mod auth_error;
mod delta;
#[cfg(test)]
mod delta_tests;
mod events;
mod pump;
mod request;

use delta::handle_block_delta;
use events::{extract_usage, handle_block_start, handle_block_stop};

use super::eventstream::EventMessage;
use crate::provider::{StreamChunk, Usage};
use serde_json::Value;
use std::collections::HashMap;

/// Per-stream state: tracks which content-block indexes are open tool-use
/// blocks so we can emit matching `ToolCallEnd` events, and accumulates usage
/// for the final `Done` chunk.
#[derive(Debug, Default)]
struct StreamState {
    /// Map of contentBlockIndex → tool_use_id for active tool-use blocks.
    open_tool_blocks: HashMap<u64, String>,
    usage: Option<Usage>,
    /// `stopReason` from the `messageStop` event (e.g. `max_tokens`).
    stop_reason: Option<String>,
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
        "messageStop" => {
            state.stop_reason = body
                .get("stopReason")
                .and_then(|v| v.as_str())
                .map(String::from);
            Vec::new()
        }
        "metadata" => {
            state.usage = extract_usage(&body);
            Vec::new()
        }
        // messageStart — no session-observable effect
        _ => Vec::new(),
    }
}
