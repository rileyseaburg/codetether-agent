//! The keep-last RLM cascade loop.

use std::sync::Arc;

use anyhow::Result;
use tokio::sync::mpsc;

use crate::provider::{Message, ToolDefinition};
use crate::session::SessionEvent;
use crate::session::helper::token::estimate_request_tokens;

use super::context::CompressContext;
use super::enforce_attempt::{CascadeEnv, attempt_fits, log_attempt};
use super::enforce_events::emit_completed;
use super::keep_last::compress_messages_keep_last;

/// Progressively smaller `keep_last` values tried by the cascade.
const KEEP_LAST_CANDIDATES: [usize; 6] = [16, 12, 8, 6, 3, 1];

/// Run the keep-last RLM cascade.
///
/// Returns `Ok(true)` when the request now fits the safety budget (a
/// completion event has been emitted); `Ok(false)` when every candidate
/// was exhausted and the caller must apply terminal truncation.
pub(super) async fn run_cascade(
    messages: &mut Vec<Message>,
    ctx: &CompressContext,
    provider: Arc<dyn crate::provider::Provider>,
    model: &str,
    system_prompt: &str,
    tools: &[ToolDefinition],
    event_tx: Option<&mpsc::Sender<SessionEvent>>,
    env: &CascadeEnv<'_>,
) -> Result<bool> {
    for keep_last in KEEP_LAST_CANDIDATES {
        let est = estimate_request_tokens(system_prompt, messages, tools);
        if attempt_fits(messages.len(), est, keep_last, ctx, env) {
            emit_completed(event_tx, env.trace_id, env.initial_est, est, messages.len()).await;
            return Ok(true);
        }
        log_attempt(est, keep_last, env);

        let did = compress_messages_keep_last(
            messages,
            ctx,
            Arc::clone(&provider),
            model,
            keep_last,
            env.trigger_reason,
        )
        .await?;
        if !did {
            break;
        }
    }

    let last_est = estimate_request_tokens(system_prompt, messages, tools);
    if last_est <= env.safety_budget {
        let kept = messages.len();
        emit_completed(event_tx, env.trace_id, env.initial_est, last_est, kept).await;
        return Ok(true);
    }
    Ok(false)
}
