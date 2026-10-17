//! Cached-input-aware token accounting.
use super::wire::*;
use crate::*;
pub(super) fn convert(usage: Option<&ApiUsage>) -> Usage {
    Usage {
        prompt_tokens: usage
            .map(|u| u.prompt_tokens.saturating_sub(u.cached_input_tokens()))
            .unwrap_or(0),
        completion_tokens: usage.map(|u| u.completion_tokens).unwrap_or(0),
        total_tokens: usage.map(|u| u.total_tokens).unwrap_or(0),
        cache_read_tokens: usage.map(ApiUsage::cached_input_tokens).filter(|&n| n > 0),
        cache_write_tokens: None,
    }
}
