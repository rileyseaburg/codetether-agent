//! Request-option transitions after provider context rejection.

use anyhow::Error;

use super::super::options::RequestOptions;
use crate::session::DerivePolicy;
use crate::session::helper::prompt_too_long;

pub(super) fn next(
    error: &Error,
    attempt: usize,
    policy: DerivePolicy,
    opts: RequestOptions,
) -> Option<RequestOptions> {
    match prompt_too_long::recovery(error, attempt, policy)? {
        prompt_too_long::Plan::Prepared { budget_tokens } => Some(RequestOptions {
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
