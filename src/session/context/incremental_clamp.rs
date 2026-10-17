//! Session token-estimator adapter for shared overflow clamping.
use super::incremental_types::MessageOrigin;
use crate::provider::{Message, ToolDefinition};
use crate::session::ResidencyLevel;
use crate::session::helper::token::estimate_request_tokens;
pub(super) fn clamp_and_recompute(
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
