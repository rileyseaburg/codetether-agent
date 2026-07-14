//! Prompt-too-long retry policy.

mod derive_plan;
mod plan;
mod retry_plan;

pub(crate) use plan::Plan;

use anyhow::Error;

use super::error::is_prompt_too_long_error;

/// Return the forced keep-last value for this prompt-too-long attempt.
pub(crate) use derive_plan::resolve as derivation;

pub(crate) fn recovery(
    error: &Error,
    attempt: usize,
    policy: crate::session::DerivePolicy,
) -> Option<Plan> {
    if !is_prompt_too_long_error(error) {
        return None;
    }
    plan::for_attempt(attempt, policy)
}

#[cfg(test)]
mod tests;
