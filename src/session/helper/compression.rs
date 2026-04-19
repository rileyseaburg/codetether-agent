//! Session history compression via the RLM router.
//!
//! This module contains the context-window enforcement logic that keeps
//! the prompt under the model's token budget. It is invoked automatically
//! at the start of every agent step by [`Session::run_loop`](crate::session::Session).
//!
//! ## Strategy
//!
//! 1. Estimate the current request token cost (system + messages + tools).
//! 2. If it exceeds 90% of the model's usable budget, compress the prefix
//!    of the conversation via [`RlmRouter::auto_process`], keeping the
//!    most recent `keep_last` messages verbatim.
//! 3. Progressively shrink `keep_last` (16 → 12 → 8 → 6) until the budget
//!    is met or nothing more can be compressed.
//!
//! The compressed prefix is replaced by a single synthetic assistant
//! message tagged `[AUTO CONTEXT COMPRESSION]` so the model sees a
//! coherent summary rather than a truncated tail.
//!
//! ## Fallback decision table
//!
//! ```text
//! ┌────────────────────────────────────┬─────────────────────────────────────┬────────────────────────────────────┐
//! │ State after attempt                │ Action                              │ Events emitted                     │
//! ├────────────────────────────────────┼─────────────────────────────────────┼────────────────────────────────────┤
//! │ RLM keep_last ∈ {16,12,8,6} fits   │ Stop; request is ready              │ CompactionStarted → Completed(Rlm) │
//! │ RLM auto_process errors on prefix  │ Fall back to chunk compression      │ (internal; logged via tracing)     │
//! │ All 4 keep_last values exhausted   │ Apply terminal truncation           │ Completed(Truncate) + Truncated    │
//! │ Terminal truncation still over bud │ Surface error to caller             │ Failed(fell_back_to = Truncate)    │
//! └────────────────────────────────────┴─────────────────────────────────────┴────────────────────────────────────┘
//! ```
//!
//! Terminal truncation drops the oldest messages outright (no summary)
//! and is deliberately a distinct event from `CompactionCompleted` so
//! consumers can warn the user about silent context loss.

use std::sync::Arc;

use anyhow::Result;
use chrono::Utc;
use tokio::sync::mpsc;
use uuid::Uuid;

use crate::provider::{ContentPart, Message, Role, ToolDefinition};
use crate::rlm::router::AutoProcessContext;
use crate::rlm::{RlmChunker, RlmRouter};

use super::error::messages_to_rlm_context;
use super::token::{
    context_window_for_model, estimate_request_tokens, session_completion_max_tokens,
};
use crate::session::event_compaction::{
    CompactionFailure, CompactionOutcome, CompactionStart, ContextTruncation, FallbackStrategy,
};
use crate::session::{Session, SessionEvent};

/// Progressively smaller `keep_last` values tried by [`enforce_context_window`].
const KEEP_LAST_CANDIDATES: [usize; 4] = [16, 12, 8, 6];

/// Number of most-recent messages retained by the terminal truncation
/// fallback when every RLM compaction attempt has failed to bring the
/// request under budget. Must be small enough to leave room for the
/// active turn, but large enough to keep the model's immediate context
/// coherent.
const TRUNCATE_KEEP_LAST: usize = 4;

/// Fraction of the usable budget we target after compression.
const SAFETY_BUDGET_RATIO: f64 = 0.90;

/// Reserve (in tokens) added on top of `session_completion_max_tokens()` for
/// tool schemas, protocol framing, and provider-specific wrappers.
const RESERVE_OVERHEAD_TOKENS: usize = 2048;

/// Fallback chunk-compression target size as a fraction of the ctx window.
const FALLBACK_CHUNK_RATIO: f64 = 0.25;

/// Compress all messages older than the last `keep_last` into a single
/// synthetic `[AUTO CONTEXT COMPRESSION]` assistant message.
///
/// Returns `Ok(true)` if compression ran, `Ok(false)` if the session was
/// already short enough to skip.
pub(crate) async fn compress_history_keep_last(
    session: &mut Session,
    provider: Arc<dyn crate::provider::Provider>,
    model: &str,
    keep_last: usize,
    reason: &str,
) -> Result<bool> {
    if session.messages.len() <= keep_last {
        return Ok(false);
    }

    let split_idx = session.messages.len().saturating_sub(keep_last);
    let tail = session.messages.split_off(split_idx);
    let prefix = std::mem::take(&mut session.messages);

    let context = messages_to_rlm_context(&prefix);
    let ctx_window = context_window_for_model(model);

    let rlm_config = session.metadata.rlm.clone();
    let auto_ctx = AutoProcessContext {
        tool_id: "session_context",
        tool_args: serde_json::json!({"reason": reason}),
        session_id: &session.id,
        abort: None,
        on_progress: None,
        provider,
        model: model.to_string(),
        bus: None,
        trace_id: None,
        subcall_provider: session.metadata.subcall_provider.clone(),
        subcall_model: session.metadata.subcall_model_name.clone(),
    };

    let summary = match RlmRouter::auto_process(&context, auto_ctx, &rlm_config).await {
        Ok(result) => {
            tracing::info!(
                reason,
                input_tokens = result.stats.input_tokens,
                output_tokens = result.stats.output_tokens,
                compression_ratio = result.stats.compression_ratio,
                "RLM: Compressed session history"
            );
            result.processed
        }
        Err(e) => {
            tracing::warn!(
                reason,
                error = %e,
                "RLM: Failed to compress session history; falling back to chunk compression"
            );
            RlmChunker::compress(
                &context,
                (ctx_window as f64 * FALLBACK_CHUNK_RATIO) as usize,
                None,
            )
        }
    };

    let summary_msg = Message {
        role: Role::Assistant,
        content: vec![ContentPart::Text {
            text: format!(
                "[AUTO CONTEXT COMPRESSION]\nOlder conversation + tool output was compressed \
                 to fit the model context window.\n\n{summary}"
            ),
        }],
    };

    let mut new_messages = Vec::with_capacity(1 + tail.len());
    new_messages.push(summary_msg);
    new_messages.extend(tail);
    session.messages = new_messages;
    session.updated_at = Utc::now();

    Ok(true)
}

/// Ensure the estimated request token cost fits within the model's safety budget.
///
/// Invokes [`compress_history_keep_last`] with progressively smaller
/// `keep_last` values until the estimate is under budget. When every
/// RLM attempt leaves the request over budget, falls back to
/// [`terminal_truncate_history`] which drops the oldest messages
/// outright.
///
/// When `event_tx` is `Some`, the full compaction lifecycle is emitted
/// as [`SessionEvent`] records ([`SessionEvent::CompactionStarted`],
/// [`SessionEvent::CompactionCompleted`],
/// [`SessionEvent::ContextTruncated`], [`SessionEvent::CompactionFailed`]).
/// When `None`, compaction runs silently — preserving the historical
/// behaviour of the non-event prompt path.
///
/// # Errors
///
/// Returns `Err` only if an underlying RLM pass errors in a way the
/// fallback cascade cannot recover from; the terminal truncation path
/// itself is infallible (it cannot produce an over-budget request
/// unless `TRUNCATE_KEEP_LAST` messages alone exceed the budget, in
/// which case a [`CompactionFailure`] event is emitted and `Ok(())` is
/// still returned so the caller can surface a clean error from the
/// provider round-trip).
pub(crate) async fn enforce_context_window(
    session: &mut Session,
    provider: Arc<dyn crate::provider::Provider>,
    model: &str,
    system_prompt: &str,
    tools: &[ToolDefinition],
    event_tx: Option<&mpsc::Sender<SessionEvent>>,
) -> Result<()> {
    let ctx_window = context_window_for_model(model);
    let reserve = session_completion_max_tokens().saturating_add(RESERVE_OVERHEAD_TOKENS);
    let budget = ctx_window.saturating_sub(reserve);
    let safety_budget = (budget as f64 * SAFETY_BUDGET_RATIO) as usize;

    let initial_est = estimate_request_tokens(system_prompt, &session.messages, tools);
    if initial_est <= safety_budget {
        return Ok(());
    }

    let trace_id = Uuid::new_v4();
    emit(
        event_tx,
        SessionEvent::CompactionStarted(CompactionStart {
            trace_id,
            reason: "context_budget".to_string(),
            before_tokens: initial_est,
            budget: safety_budget,
        }),
    )
    .await;

    for keep_last in KEEP_LAST_CANDIDATES {
        let est = estimate_request_tokens(system_prompt, &session.messages, tools);
        if est <= safety_budget {
            emit(
                event_tx,
                SessionEvent::CompactionCompleted(CompactionOutcome {
                    trace_id,
                    strategy: FallbackStrategy::Rlm,
                    before_tokens: initial_est,
                    after_tokens: est,
                    kept_messages: session.messages.len(),
                }),
            )
            .await;
            return Ok(());
        }

        tracing::info!(
            est_tokens = est,
            ctx_window,
            safety_budget,
            keep_last,
            "Context window approaching limit; compressing older session history"
        );

        let did = compress_history_keep_last(
            session,
            Arc::clone(&provider),
            model,
            keep_last,
            "context_budget",
        )
        .await?;

        if !did {
            break;
        }
    }

    // Re-estimate one last time after the final RLM pass.
    let last_est = estimate_request_tokens(system_prompt, &session.messages, tools);
    if last_est <= safety_budget {
        emit(
            event_tx,
            SessionEvent::CompactionCompleted(CompactionOutcome {
                trace_id,
                strategy: FallbackStrategy::Rlm,
                before_tokens: initial_est,
                after_tokens: last_est,
                kept_messages: session.messages.len(),
            }),
        )
        .await;
        return Ok(());
    }

    // Every RLM / chunk attempt still leaves us over budget.
    // Apply terminal truncation as the last-resort fallback.
    let dropped_tokens =
        terminal_truncate_history(session, system_prompt, tools, TRUNCATE_KEEP_LAST);
    let after_tokens = estimate_request_tokens(system_prompt, &session.messages, tools);

    tracing::warn!(
        before_tokens = initial_est,
        after_tokens,
        dropped_tokens,
        kept_messages = session.messages.len(),
        safety_budget,
        "All RLM compaction attempts exhausted; applied terminal truncation fallback"
    );

    emit(
        event_tx,
        SessionEvent::ContextTruncated(ContextTruncation {
            trace_id,
            dropped_tokens,
            kept_messages: session.messages.len(),
            archive_ref: None,
        }),
    )
    .await;
    emit(
        event_tx,
        SessionEvent::CompactionCompleted(CompactionOutcome {
            trace_id,
            strategy: FallbackStrategy::Truncate,
            before_tokens: initial_est,
            after_tokens,
            kept_messages: session.messages.len(),
        }),
    )
    .await;

    if after_tokens > safety_budget {
        tracing::error!(
            after_tokens,
            safety_budget,
            "Terminal truncation still over budget; request will likely fail at the provider"
        );
        emit(
            event_tx,
            SessionEvent::CompactionFailed(CompactionFailure {
                trace_id,
                fell_back_to: Some(FallbackStrategy::Truncate),
                reason: "terminal truncation still exceeds safety budget".to_string(),
                after_tokens,
                budget: safety_budget,
            }),
        )
        .await;
    }

    Ok(())
}

/// Drop everything older than the last `keep_last` messages and return an
/// approximate count of the tokens removed.
///
/// Unlike [`compress_history_keep_last`] this keeps **no** summary of the
/// dropped prefix — the caller is expected to have already attempted
/// RLM-based compaction and exhausted it. A `[CONTEXT TRUNCATED]`
/// assistant marker is prepended so the model is aware that older turns
/// were silently removed.
fn terminal_truncate_history(
    session: &mut Session,
    system_prompt: &str,
    tools: &[ToolDefinition],
    keep_last: usize,
) -> usize {
    if session.messages.len() <= keep_last {
        return 0;
    }

    let before = estimate_request_tokens(system_prompt, &session.messages, tools);
    let split_idx = session.messages.len().saturating_sub(keep_last);
    let tail = session.messages.split_off(split_idx);

    let marker = Message {
        role: Role::Assistant,
        content: vec![ContentPart::Text {
            text: "[CONTEXT TRUNCATED]\nOlder conversation was dropped to keep the request \
                   under the model's context window. Ask for details to recall anything \
                   specific."
                .to_string(),
        }],
    };

    let mut new_messages = Vec::with_capacity(1 + tail.len());
    new_messages.push(marker);
    new_messages.extend(tail);
    session.messages = new_messages;
    session.updated_at = Utc::now();

    let after = estimate_request_tokens(system_prompt, &session.messages, tools);
    before.saturating_sub(after)
}

async fn emit(event_tx: Option<&mpsc::Sender<SessionEvent>>, ev: SessionEvent) {
    if let Some(tx) = event_tx {
        let _ = tx.send(ev).await;
    }
}
