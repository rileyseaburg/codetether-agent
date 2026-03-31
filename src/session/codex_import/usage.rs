use super::payloads::CodexTokenEnvelope;
use crate::provider::Usage;
use anyhow::Result;
use serde_json::Value;

pub(crate) fn parse_event_msg_usage(payload: Value) -> Result<Option<Usage>> {
    let Some(event_type) = payload.get("type").and_then(Value::as_str) else {
        return Ok(None);
    };
    if event_type != "token_count" {
        return Ok(None);
    }

    let payload: CodexTokenEnvelope = serde_json::from_value(payload)?;
    let Some(info) = payload.info else {
        return Ok(None);
    };
    Ok(Some(Usage {
        prompt_tokens: info.total_token_usage.input_tokens,
        completion_tokens: info.total_token_usage.output_tokens,
        total_tokens: info.total_token_usage.total_tokens,
        cache_read_tokens: info.total_token_usage.cached_input_tokens,
        cache_write_tokens: None,
    }))
}
