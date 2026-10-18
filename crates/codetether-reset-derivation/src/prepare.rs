//! Clone history and select the shared reset preparation outcome.
use crate::Source;
use codetether_context_policy::derivation::reset::{self, ResetPreparation};
use codetether_provider_core::ToolDefinition;
use codetether_request_tokens::estimate_request_tokens;
pub(super) fn prepare<T: Source>(
    session: &T,
    threshold_tokens: usize,
    system_prompt: &str,
    tools: &[ToolDefinition],
) -> ResetPreparation {
    let origin_len = session.messages().len();
    let messages = session.messages().to_vec();
    reset::prepare(messages, origin_len, threshold_tokens, |messages| {
        estimate_request_tokens(system_prompt, messages, tools)
    })
}
