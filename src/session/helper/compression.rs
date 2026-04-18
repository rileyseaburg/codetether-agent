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

use std::sync::Arc;

use anyhow::Result;
use chrono::Utc;

use crate::provider::{ContentPart, Message, Role, ToolDefinition};
use crate::rlm::router::AutoProcessContext;
use crate::rlm::{RlmChunker, RlmConfig, RlmRouter};

use super::error::messages_to_rlm_context;
use super::token::{
    context_window_for_model, estimate_request_tokens, session_completion_max_tokens,
};
use crate::session::Session;

/// Progressively smaller `keep_last` values tried by [`enforce_context_window`].
const KEEP_LAST_CANDIDATES: [usize; 4] = [16, 12, 8, 6];

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

    let rlm_config = RlmConfig::default();
    let auto_ctx = AutoProcessContext {
        tool_id: "session_context",
        tool_args: serde_json::json!({"reason": reason}),
        session_id: &session.id,
        abort: None,
        on_progress: None,
        provider,
        model: model.to_string(),
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
/// `keep_last` values until the estimate is under budget or nothing more
/// can be compressed.
pub(crate) async fn enforce_context_window(
    session: &mut Session,
    provider: Arc<dyn crate::provider::Provider>,
    model: &str,
    system_prompt: &str,
    tools: &[ToolDefinition],
) -> Result<()> {
    let ctx_window = context_window_for_model(model);
    let reserve = session_completion_max_tokens().saturating_add(RESERVE_OVERHEAD_TOKENS);
    let budget = ctx_window.saturating_sub(reserve);
    let safety_budget = (budget as f64 * SAFETY_BUDGET_RATIO) as usize;

    for keep_last in KEEP_LAST_CANDIDATES {
        let est = estimate_request_tokens(system_prompt, &session.messages, tools);
        if est <= safety_budget {
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

    Ok(())
}
