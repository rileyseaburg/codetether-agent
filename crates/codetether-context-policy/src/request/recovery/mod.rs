//! Context-overflow classification and bounded recovery planning.
//!
//! [recovery] prioritizes prepared context, then emergency keep-last attempts.
//! [next_options] applies that choice while retaining unrelated request options.
use crate::derivation::DerivePolicy;
use anyhow::Error;
mod plan;
pub use plan::Plan;
mod classification;
mod retry_plan;
pub use classification::is_prompt_too_long_error;
mod options;
pub use options::next_options;

/// Choose a retry only when the error identifies a context limit.
///
/// # Arguments
/// * `error` - Provider or request-guard error.
/// * `attempt` - Current retry attempt index.
/// * `policy` - Active derivation strategy.
///
/// # Returns
/// A bounded recovery action, or `None` for unrelated/exhausted errors.
///
/// # Examples
/// ```
/// use codetether_context_policy::{derivation::DerivePolicy, request::recovery::{recovery, Plan}};
/// let error = anyhow::anyhow!("context window exceeded");
/// assert_eq!(recovery(&error, 1, DerivePolicy::Legacy), Some(Plan::Emergency { keep_last: 6 }));
/// ```
pub fn recovery(error: &Error, attempt: usize, policy: DerivePolicy) -> Option<Plan> {
    if !is_prompt_too_long_error(error) {
        return None;
    }
    plan::for_attempt(attempt, policy)
}
#[cfg(test)]
mod tests;
