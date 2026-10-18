//! Session token-estimator adapter for shared overflow clamping.
use codetether_context_policy::pages::ResidencyLevel;
use codetether_context_policy::projection::incremental_types::MessageOrigin;
use codetether_provider_core::{Message, ToolDefinition};
use codetether_request_tokens::estimate_request_tokens;
pub fn clamp_and_recompute(
    messages: &mut Vec<Message>,
    resolutions: &mut Vec<ResidencyLevel>,
    origins: &mut Vec<MessageOrigin>,
    system_prompt: &str,
    tools: &[ToolDefinition],
    budget_tokens: usize,
    clone_len: usize,
    pre_dropped: &[(usize, usize)],
) -> (Vec<(usize, usize)>, Vec<&'static str>) {
    codetether_context_policy::projection::incremental_clamp::clamp_and_recompute(
        messages,
        resolutions,
        origins,
        budget_tokens,
        clone_len,
        pre_dropped,
        |messages| estimate_request_tokens(system_prompt, messages, tools),
    )
}
