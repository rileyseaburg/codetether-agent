//! Local token-usage estimates for the undocumented Gemini Web protocol.
//!
//! The browser endpoint does not return token counts. These estimates keep
//! throughput telemetry useful without claiming provider-supplied precision.

use crate::provider::Usage;

pub(super) fn estimate(prompt: &str, answer: &str, calls: &[(String, String)]) -> Usage {
    let prompt_tokens = tokens(prompt);
    let call_tokens = calls
        .iter()
        .map(|(name, arguments)| tokens(name) + tokens(arguments))
        .sum::<usize>();
    let completion_tokens = tokens(answer) + call_tokens;
    Usage {
        prompt_tokens,
        completion_tokens,
        total_tokens: prompt_tokens + completion_tokens,
        cache_read_tokens: None,
        cache_write_tokens: None,
    }
}

fn tokens(text: &str) -> usize {
    let characters = text.chars().count();
    characters.saturating_add(3) / 4
}

#[cfg(test)]
#[path = "usage_tests.rs"]
mod tests;
