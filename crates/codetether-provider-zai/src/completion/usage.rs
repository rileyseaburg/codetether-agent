//! Application token accounting without double-counting cached input.
use crate::{Usage, response_wire::ZaiResponse};
pub(super) fn usage(response: &ZaiResponse) -> Usage {
    Usage {
        prompt_tokens: {
            // Subtract cached input so the cost estimator does
            // not double-count: cache_read_tokens is billed
            // separately at the discounted rate.
            let u = response.usage.as_ref();
            let total = u.map(|u| u.prompt_tokens).unwrap_or(0);
            let cached = u
                .and_then(|u| u.prompt_tokens_details.as_ref())
                .map(|d| d.cached_tokens)
                .unwrap_or(0);
            total.saturating_sub(cached)
        },
        completion_tokens: response
            .usage
            .as_ref()
            .map(|u| u.completion_tokens)
            .unwrap_or(0),
        total_tokens: response.usage.as_ref().map(|u| u.total_tokens).unwrap_or(0),
        cache_read_tokens: response
            .usage
            .as_ref()
            .and_then(|u| u.prompt_tokens_details.as_ref())
            .map(|d| d.cached_tokens)
            .filter(|&t| t > 0),
        cache_write_tokens: None,
    }
}
