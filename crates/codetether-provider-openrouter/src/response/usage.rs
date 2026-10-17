//! Response token accounting.
use crate::*;
pub(super) fn usage(response: &OpenRouterResponse) -> Usage {
    Usage {
        prompt_tokens: response
            .usage
            .as_ref()
            .map(|u| u.prompt_tokens)
            .unwrap_or(0),
        completion_tokens: response
            .usage
            .as_ref()
            .map(|u| u.completion_tokens)
            .unwrap_or(0),
        total_tokens: response.usage.as_ref().map(|u| u.total_tokens).unwrap_or(0),
        ..Default::default()
    }
}
