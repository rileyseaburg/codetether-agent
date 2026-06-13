//! Budget computation and compaction-trigger detection.

use crate::provider::{Message, ToolDefinition};
use crate::session::helper::token::{
    context_window_for_model, estimate_request_tokens, session_completion_max_tokens,
};

use super::context::CompressContext;

/// Fraction of the usable budget we target after compression.
const SAFETY_BUDGET_RATIO: f64 = 0.90;

/// Reserve (in tokens) added on top of `session_completion_max_tokens()` for
/// tool schemas, protocol framing, and provider-specific wrappers.
const RESERVE_OVERHEAD_TOKENS: usize = 2048;

/// Computed budget figures plus the compaction trigger decision.
pub(super) struct BudgetCheck {
    pub ctx_window: usize,
    pub safety_budget: usize,
    pub initial_est: usize,
    /// `None` when no compaction is needed; otherwise the trigger reason.
    pub trigger_reason: Option<&'static str>,
}

/// Estimate the request cost and decide whether compaction must run.
pub(super) fn check_budget(
    messages: &[Message],
    ctx: &CompressContext,
    model: &str,
    system_prompt: &str,
    tools: &[ToolDefinition],
) -> BudgetCheck {
    let ctx_window = context_window_for_model(model);
    let reserve = session_completion_max_tokens().saturating_add(RESERVE_OVERHEAD_TOKENS);
    let budget = ctx_window.saturating_sub(reserve);
    let safety_budget = (budget as f64 * SAFETY_BUDGET_RATIO) as usize;

    let initial_est = estimate_request_tokens(system_prompt, messages, tools);
    let history_trigger = ctx.rlm_config.history_trigger_messages;
    let history_over = history_trigger > 0 && messages.len() >= history_trigger;

    let trigger_reason = if initial_est <= safety_budget && !history_over {
        None
    } else if history_over && initial_est <= safety_budget {
        Some("history_length")
    } else {
        Some("context_budget")
    };
    BudgetCheck {
        ctx_window,
        safety_budget,
        initial_est,
        trigger_reason,
    }
}
