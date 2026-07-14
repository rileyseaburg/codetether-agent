//! Structured retry diagnostics for local sub-agents.

use super::super::AgentLoopExit;

pub(super) fn incomplete(
    id: &str,
    attempt: u32,
    maximum: u32,
    exit: AgentLoopExit,
    delay_ms: u128,
) {
    tracing::warn!(subtask_id = %id, attempt = attempt + 1, max_retries = maximum,
        exit_reason = ?exit, delay_ms, "Sub-agent incomplete; retrying");
}

pub(super) fn error(id: &str, attempt: u32, maximum: u32, error: &anyhow::Error, delay_ms: u128) {
    tracing::warn!(subtask_id = %id, attempt = attempt + 1, max_retries = maximum,
        %error, delay_ms, "Sub-agent failed; retrying");
}

pub(super) fn exhausted(id: &str, attempt: u32, maximum: u32) {
    tracing::warn!(subtask_id = %id, attempt = attempt + 1, max_retries = maximum,
        "Sub-agent retries exhausted");
}
