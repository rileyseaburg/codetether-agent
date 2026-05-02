//! Prompt-too-long retry policy.

mod retry_plan;

use anyhow::Error;

use super::error::is_prompt_too_long_error;

/// Return the forced keep-last value for this prompt-too-long attempt.
pub(crate) fn keep_last(error: &Error, attempt: usize) -> Option<usize> {
    if !is_prompt_too_long_error(error) {
        return None;
    }

    retry_plan::keep_last_for_attempt(attempt)
}

#[cfg(test)]
mod tests;
