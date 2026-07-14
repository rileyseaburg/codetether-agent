//! Budget computation and compaction-trigger detection.

use crate::provider::{Message, ToolDefinition};
use crate::session::helper::token::{context_window_for_model, estimate_request_tokens};

use super::context::CompressContext;

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
    let safety_budget = crate::session::context::input_budget::usable(model);

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
