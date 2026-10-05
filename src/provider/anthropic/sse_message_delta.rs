//! Accumulate Anthropic SSE usage without losing prompt-cache accounting.

use crate::provider::{StreamChunk, Usage};
use serde_json::Value;

/// Retain start-event input usage and merge cumulative delta counters.
pub(crate) fn parse(event: &Value, accumulated: &mut Option<Usage>) -> Option<StreamChunk> {
    let start = event["type"] == "message_start";
    let wire = if start {
        &event["message"]["usage"]
    } else {
        &event["usage"]
    };
    if wire.is_object() {
        let usage = accumulated.get_or_insert_with(Usage::default);
        if let Some(tokens) = wire["input_tokens"].as_u64() {
            usage.prompt_tokens = tokens as usize;
        }
        if let Some(tokens) = wire["output_tokens"].as_u64() {
            usage.completion_tokens = tokens as usize;
        }
        if let Some(tokens) = wire["cache_read_input_tokens"].as_u64() {
            usage.cache_read_tokens = Some(tokens as usize);
        }
        if let Some(tokens) = wire["cache_creation_input_tokens"].as_u64() {
            usage.cache_write_tokens = Some(tokens as usize);
        }
        usage.total_tokens = usage
            .prompt_tokens
            .saturating_add(usage.completion_tokens)
            .saturating_add(usage.cache_read_tokens.unwrap_or(0))
            .saturating_add(usage.cache_write_tokens.unwrap_or(0));
    }
    (!start).then(|| StreamChunk::Done {
        usage: accumulated.clone(),
    })
}
