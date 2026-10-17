//! Input, output, and cache-token accounting.
use crate::*;
pub(super) fn usage(usage: Option<&AnthropicUsage>) -> Usage {
    Usage {
        prompt_tokens: usage.map(|u| u.input_tokens).unwrap_or(0),
        completion_tokens: usage.map(|u| u.output_tokens).unwrap_or(0),
        total_tokens: usage.map(|u| u.input_tokens + u.output_tokens).unwrap_or(0),
        cache_read_tokens: usage.and_then(|u| u.cache_read_input_tokens),
        cache_write_tokens: usage.and_then(|u| u.cache_creation_input_tokens),
    }
}
