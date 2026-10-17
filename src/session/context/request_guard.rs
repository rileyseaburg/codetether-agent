//! Application estimators and logging for shared request-budget policy.
use crate::provider::{CompletionRequest, Message};
use crate::session::helper::token::{estimate_request_tokens, estimate_tokens_for_messages};
use anyhow::Result;
pub(crate) fn finish(
    request: CompletionRequest,
    extras: Vec<Message>,
) -> Result<CompletionRequest> {
    codetether_context_policy::request::guard::finish(
        request,
        extras,
        |messages, tools| estimate_request_tokens("", messages, tools),
        estimate_tokens_for_messages,
        |cost, used, budget| {
            tracing::debug!(
                cost,
                used,
                budget,
                "Omitting optional context that exceeds input budget"
            );
        },
    )
}
#[cfg(test)]
#[path = "request_guard_tests.rs"]
mod tests;
