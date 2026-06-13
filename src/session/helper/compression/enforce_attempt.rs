//! Per-attempt predicates and logging for the keep-last cascade.

use super::context::CompressContext;

/// Scalar inputs threaded through the cascade loop.
pub(super) struct CascadeEnv<'a> {
    pub trace_id: uuid::Uuid,
    pub trigger_reason: &'a str,
    pub initial_est: usize,
    pub safety_budget: usize,
    pub ctx_window: usize,
}

/// Whether the current buffer already satisfies the budget and the
/// history-length trigger for this `keep_last` candidate.
pub(super) fn attempt_fits(
    message_count: usize,
    est: usize,
    keep_last: usize,
    ctx: &CompressContext,
    env: &CascadeEnv<'_>,
) -> bool {
    let history_trigger = ctx.rlm_config.history_trigger_messages;
    let still_long =
        history_trigger > 0 && message_count > history_trigger.saturating_sub(keep_last);
    est <= env.safety_budget && !still_long
}

/// Structured log line for one compression attempt.
pub(super) fn log_attempt(est: usize, keep_last: usize, env: &CascadeEnv<'_>) {
    tracing::info!(
        est_tokens = est,
        ctx_window = env.ctx_window,
        safety_budget = env.safety_budget,
        keep_last,
        "Context window approaching limit; compressing older session history"
    );
}
