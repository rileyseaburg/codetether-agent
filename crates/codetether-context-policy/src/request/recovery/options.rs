//! Request-option transitions after provider context rejection.

use anyhow::Error;

use super::{Plan, recovery};
use crate::derivation::DerivePolicy;
use crate::derivation::options::RequestOptions;

pub fn next_options(
    error: &Error,
    attempt: usize,
    policy: DerivePolicy,
    opts: RequestOptions,
) -> Option<RequestOptions> {
    match recovery(error, attempt, policy)? {
        Plan::Prepared { budget_tokens } => Some(RequestOptions {
            policy_override: Some(DerivePolicy::Incremental { budget_tokens }),
            force_keep_last: None,
            ..opts
        }),
        prompt_too_long::Plan::Emergency { keep_last } => Some(RequestOptions {
            force_keep_last: Some(keep_last),
            ..opts
        }),
    }
}
