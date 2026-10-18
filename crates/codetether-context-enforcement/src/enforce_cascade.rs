//! The keep-last RLM cascade loop.

use std::sync::Arc;

use anyhow::Result;
use tokio::sync::mpsc;

use codetether_provider_core::{Message, ToolDefinition};
use codetether_request_tokens::estimate_request_tokens;
use codetether_session_events::SessionEvent;

use super::enforce_attempt::{CascadeEnv, attempt_fits, log_attempt};
use super::enforce_events::emit_completed;
use codetether_context_compression::CompressContext;
use codetether_context_compression::compress_messages_keep_last;
use codetether_context_policy::compression::budget::KEEP_LAST_CANDIDATES;

/// Run the keep-last RLM cascade.
///
/// Returns `Ok(true)` when the request now fits the safety budget (a
/// completion event has been emitted); `Ok(false)` when every candidate
/// was exhausted and the caller must apply terminal truncation.
pub(super) async fn run_cascade<S>(
    messages: &mut Vec<Message>,
    ctx: &CompressContext,
    provider: Arc<dyn codetether_provider_core::Provider>,
    model: &str,
    system_prompt: &str,
    tools: &[ToolDefinition],
    event_tx: Option<&mpsc::Sender<SessionEvent<S>>>,
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

    super::cascade_finish::finish(messages, system_prompt, tools, event_tx, env).await
}
