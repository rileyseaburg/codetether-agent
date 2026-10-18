//! Check final budget fit after all keep-last candidates are exhausted.
use super::{enforce_attempt::CascadeEnv, enforce_events::emit_completed};
use codetether_provider_core::{Message, ToolDefinition};
use codetether_request_tokens::estimate_request_tokens;
use codetether_session_events::SessionEvent;
pub(super) async fn finish<S>(
    messages: &[Message],
    system_prompt: &str,
    tools: &[ToolDefinition],
    event_tx: Option<&tokio::sync::mpsc::Sender<SessionEvent<S>>>,
    env: &CascadeEnv<'_>,
) -> anyhow::Result<bool> {
    let last_est = estimate_request_tokens(system_prompt, messages, tools);
    if last_est <= env.safety_budget {
        let kept = messages.len();
        emit_completed(event_tx, env.trace_id, env.initial_est, last_est, kept).await;
        return Ok(true);
    }
    Ok(false)
}
