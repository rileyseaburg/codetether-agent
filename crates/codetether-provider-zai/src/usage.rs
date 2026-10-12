//! Conversion of provider usage without charging cached input twice.
use crate::{Usage, ZaiUsage};

pub(crate) fn convert(usage: &ZaiUsage) -> Usage {
    let cached = usage
        .prompt_tokens_details
        .as_ref()
        .map(|details| details.cached_tokens)
        .unwrap_or(0);
    Usage {
        prompt_tokens: usage.prompt_tokens.saturating_sub(cached),
        completion_tokens: usage.completion_tokens,
        total_tokens: usage.total_tokens,
        cache_read_tokens: (cached > 0).then_some(cached),
        cache_write_tokens: None,
    }
}
