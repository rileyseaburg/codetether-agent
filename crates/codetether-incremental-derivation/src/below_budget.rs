//! Session token-estimator adapter for the below-budget path.
use codetether_context_policy::projection::helpers::DerivedContext;
use codetether_provider_core::{Message, ToolDefinition};
use codetether_request_tokens::estimate_request_tokens;
pub fn try_pass_through(
    clone: &mut Vec<Message>,
    system_prompt: &str,
    tools: &[ToolDefinition],
    budget_tokens: usize,
    origin_len: usize,
) -> Option<DerivedContext> {
    codetether_context_policy::projection::incremental_below_budget::try_pass_through(
        clone,
        budget_tokens,
        origin_len,
        |messages| estimate_request_tokens(system_prompt, messages, tools),
    )
}
