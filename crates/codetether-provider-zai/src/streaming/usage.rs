//! Application SSE usage retains total input and optional zero cache counts.
use crate::{Usage, ZaiUsage};
pub(super) fn total_prompt(usage: &ZaiUsage) -> Usage {
    Usage {
        prompt_tokens: usage.prompt_tokens,
        completion_tokens: usage.completion_tokens,
        total_tokens: usage.total_tokens,
        cache_read_tokens: usage
            .prompt_tokens_details
            .as_ref()
            .map(|d| d.cached_tokens),
        cache_write_tokens: None,
    }
}
