//! Budget computation and compaction-trigger detection.

use codetether_provider_core::limits::context_window_for_model;
use codetether_provider_core::{Message, ToolDefinition};
use codetether_request_tokens::estimate_request_tokens;

use codetether_context_compression::CompressContext;
pub(super) use codetether_context_policy::compression::budget::BudgetCheck;

/// Estimate the request cost and decide whether compaction must run.
pub(super) fn check_budget(
    messages: &[Message],
    ctx: &CompressContext,
    model: &str,
    system_prompt: &str,
    tools: &[ToolDefinition],
) -> BudgetCheck {
    let ctx_window = context_window_for_model(model);
    let safety_budget = codetether_context_policy::derivation::input_budget::usable(model);

    let initial_est = estimate_request_tokens(system_prompt, messages, tools);
    let history_trigger = ctx.rlm_config.history_trigger_messages;
    codetether_context_policy::compression::budget::check_budget(
        ctx_window,
        safety_budget,
        initial_est,
        history_trigger,
        messages.len(),
    )
}
