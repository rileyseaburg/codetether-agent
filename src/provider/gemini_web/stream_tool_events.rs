//! Construction of provider-neutral tool-call stream events.

use crate::provider::StreamChunk;
use std::time::{SystemTime, UNIX_EPOCH};

pub(super) fn append(chunks: &mut Vec<StreamChunk>, calls: Vec<(String, String)>) {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();
    for (index, (name, arguments)) in calls.into_iter().enumerate() {
        let id = format!("gwsc_{stamp}_{index}");
        chunks.push(StreamChunk::ToolCallStart {
            id: id.clone(),
            name,
        });
        chunks.push(StreamChunk::ToolCallDelta {
            id: id.clone(),
            arguments_delta: arguments,
        });
        chunks.push(StreamChunk::ToolCallEnd { id });
    }
}
