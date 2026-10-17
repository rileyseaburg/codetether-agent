//! Cached and uncached response token accounting.
use crate::*;
pub(super) fn usage(completion: &ChatCompletion) -> Usage {
    Usage {
        prompt_tokens: completion
            .usage
            .as_ref()
            .map(|u| u.prompt_tokens.saturating_sub(u.cached()))
            .unwrap_or(0),
        completion_tokens: completion
            .usage
            .as_ref()
            .map(|u| u.completion_tokens)
            .unwrap_or(0),
        total_tokens: completion
            .usage
            .as_ref()
            .map(|u| u.total_tokens)
            .unwrap_or(0),
        cache_read_tokens: completion
            .usage
            .as_ref()
            .map(ApiUsage::cached)
            .filter(|&n| n > 0),
        cache_write_tokens: None,
    }
}
