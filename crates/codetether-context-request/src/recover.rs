//! Resolve effective policy and the next request options after a failure.
use crate::{RequestOptions, Source};
pub(super) fn next<H: Source>(
    session: &H,
    model: &str,
    error: &anyhow::Error,
    attempt: usize,
    opts: RequestOptions,
) -> Option<RequestOptions> {
    let policy = opts
        .policy_override
        .unwrap_or_else(|| session.effective_policy(model));
    codetether_context_policy::request::recovery::next_options(error, attempt, policy, opts)
}
