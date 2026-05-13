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
//!    via a ready background RLM summary or deterministic chunker fallback, keeping the
//!    most recent `keep_last` messages verbatim.
//! 3. Progressively shrink `keep_last` (16 → 12 → 8 → 6 → 3 → 1) until the budget
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
use crate::rlm::RlmChunker;

use super::error::messages_to_rlm_context;
use super::token::{
    context_window_for_model, estimate_request_tokens, session_completion_max_tokens,
};
use crate::session::event_compaction::{
    CompactionFailure, CompactionOutcome, CompactionStart, ContextTruncation, FallbackStrategy,
};
use crate::session::{Session, SessionEvent};

/// Progressively smaller `keep_last` values tried by [`enforce_context_window`].
const KEEP_LAST_CANDIDATES: [usize; 6] = [16, 12, 8, 6, 3, 1];

/// Number of most-recent messages retained by the terminal truncation
/// fallback when every RLM compaction attempt has failed to bring the
/// request under budget. Must be small enough to leave room for the
/// active turn, but large enough to keep the model's immediate context
/// coherent.
const TRUNCATE_KEEP_LAST: usize = 4;

/// Byte ceilings used by terminal truncation when the retained tail still
/// exceeds the provider context budget. The cascade preserves generous
/// head/tail snippets first, then gets progressively stricter.
const TERMINAL_PAYLOAD_CAPS: [usize; 8] = [
    262_144, 131_072, 65_536, 32_768, 16_384, 8_192, 4_096, 2_048,
];

/// Final emergency ceiling if the normal cascade still cannot bring the
/// prompt under budget. This should only affect pathological single-turn
/// tool dumps; the newest user instruction is already protected by the
/// oversized-message compressor before this fallback runs.
const TERMINAL_EMERGENCY_CAP_BYTES: usize = 1_024;

/// Fraction of the usable budget we target after compression.
const SAFETY_BUDGET_RATIO: f64 = 0.90;

/// Reserve (in tokens) added on top of `session_completion_max_tokens()` for
/// tool schemas, protocol framing, and provider-specific wrappers.
const RESERVE_OVERHEAD_TOKENS: usize = 2048;

/// Non-buffer inputs for [`compress_messages_keep_last`] and
/// [`enforce_on_messages`].
///
/// Cloned from a [`Session`] by [`CompressContext::from_session`] so the
/// core compression functions can operate on `&mut Vec<Message>` without
/// holding any borrow of the owning [`Session`]. This is the pivot that
/// lets [`derive_context`](crate::session::context::derive_context) run
/// the compression pipeline on a *clone* of the history rather than
/// mutating it in place — Phase A of the history/context split.
///
/// All fields are owned (not borrowed) so the context can be constructed
/// up front and re-used across multiple compression passes without
/// holding an outer borrow of the session.
#[derive(Clone)]
pub(crate) struct CompressContext {
    /// RLM configuration for the session (thresholds, model selectors,
    /// iteration limits).
    pub rlm_config: crate::rlm::RlmConfig,
    /// UUID of the owning session, propagated to RLM traces.
    pub session_id: String,
    /// Optional pre-resolved subcall provider from
    /// [`SessionMetadata::subcall_provider`](crate::session::types::SessionMetadata).
    pub subcall_provider: Option<Arc<dyn crate::provider::Provider>>,
    /// Model name resolved alongside [`Self::subcall_provider`].
    pub subcall_model: Option<String>,
    /// CADMAS-CTX routing state used to rank fallback RLM compaction models.
    pub delegation: crate::session::delegation::DelegationState,
    /// Context bucket for the current transcript projection.
    pub bucket: crate::session::relevance::Bucket,
    /// RLM observability: bus + trace id. See `compression_bus`.
    pub observability: super::compression_bus::Observability,
}

impl CompressContext {
    /// Snapshot the non-buffer fields of `session` into an owned context.
    pub(crate) fn from_session(session: &Session) -> Self {
        Self {
            rlm_config: session.metadata.rlm.clone(),
            session_id: session.id.clone(),
            subcall_provider: session.metadata.subcall_provider.clone(),
            subcall_model: session.metadata.subcall_model_name.clone(),
            delegation: session.metadata.delegation.clone(),
            bucket: crate::session::relevance::bucket_for_messages(&session.messages),
            observability: super::compression_bus::Observability::default(),
        }
    }
}

/// Compress everything older than the last `keep_last` messages in
/// `messages` into a single synthetic `[AUTO CONTEXT COMPRESSION]`
/// assistant message, in-place.
///
/// This is the `&mut Vec<Message>` core that powers both the legacy
/// [`compress_history_keep_last`] wrapper (which still takes
/// `&mut Session`) and the Phase B
/// [`derive_context`](crate::session::context::derive_context) pipeline
/// (which runs on a history clone).
///
/// # Arguments
///
/// * `messages` — The message buffer to rewrite in place.
/// * `ctx` — Session-level configuration snapshot (RLM config, session
///   id, subcall provider/model).
/// * `provider` — Caller's primary provider. Used as a fallback when the
///   dedicated RLM model cannot be resolved.
/// * `model` — Caller's primary model identifier (used for ctx-window
///   sizing and fallback routing).
/// * `keep_last` — Number of most-recent messages to keep verbatim. Must
///   be ≥ 0; when `messages.len() <= keep_last` the function returns
///   `Ok(false)` without touching the buffer.
/// * `reason` — Short diagnostic string recorded on the RLM trace.
///
/// # Returns
///
/// `Ok(true)` if the buffer was rewritten, `Ok(false)` if it was already
/// short enough to skip compression.
///
/// # Errors
///
/// Propagates any error from the compression pipeline that cannot be
/// recovered by the deterministic fallback.
pub(crate) async fn compress_messages_keep_last(
    messages: &mut Vec<Message>,
    ctx: &CompressContext,
    provider: Arc<dyn crate::provider::Provider>,
    model: &str,
    keep_last: usize,
    reason: &str,
) -> Result<bool> {
    if messages.len() <= keep_last {
        return Ok(false);
    }

    let split_idx = crate::session::context::active_tail::active_tail_start(messages, keep_last);
    let tail = messages.split_off(split_idx);
    let prefix = std::mem::take(messages);

    let context = messages_to_rlm_context(&prefix);
    let ctx_window = context_window_for_model(model);
    if context.trim().is_empty() {
        *messages = prefix;
        messages.extend(tail);
        return Ok(false);
    }
    if let Some(summary) = super::compression_defer::context_summary(
        &context,
        ctx_window,
        &ctx.session_id,
        reason,
        model,
        Arc::clone(&provider),
        &ctx.rlm_config,
    ) {
        super::compression_summary::install(messages, tail, summary);
        return Ok(true);
    }
    Ok(false)
}

/// Ratio of the model's context window a single "last" message can occupy
/// before [`compress_last_message_if_oversized`] replaces it with an RLM
/// summary.
const OVERSIZED_LAST_MESSAGE_RATIO: f64 = 0.35;

/// Character budget retained verbatim from the original message as a
/// prefix, after RLM compression, so the model can see the literal
/// opening of the request even when the body was summarised.
const OVERSIZED_LAST_MESSAGE_PREFIX_CHARS: usize = 500;

/// Compress the **last** message in `messages` in-place when its text
/// content exceeds [`OVERSIZED_LAST_MESSAGE_RATIO`] of the model's
/// context window.
///
/// This is the buffer-taking core that replaces the former
/// `compress_user_message_if_oversized` helpers in `prompt.rs` and
/// `prompt_events.rs`. Those called `session.messages.last_mut()`
/// destructively on the canonical history; that was the last remaining
/// mutator that broke the Phase A invariant (*history stays pure*).
/// The derivation pipeline now calls this helper on a clone, so the
/// original user text remains in [`Session::messages`] while the LLM
/// sees the compressed projection.
///
/// Behaviour matches the legacy pair:
///
/// * When the last message contains ≤ threshold tokens, returns
///   `Ok(false)` without touching the buffer.
/// * If a background RLM summary is already cached, rewrites the last
///   message with that summary and a literal request prefix.
/// * Otherwise schedules background RLM and returns a deterministic
///   [`RlmChunker::compress`] projection immediately.
///
/// # Arguments
///
/// * `messages` — The message buffer to rewrite in place. A no-op when
///   empty.
/// * `ctx` — Session-level configuration snapshot.
/// * `provider` — Caller's primary provider (used as subcall fallback).
/// * `model` — Caller's primary model identifier (governs ctx window).
///
/// # Returns
///
/// `Ok(true)` when the last message was rewritten, `Ok(false)` when no
/// compression was needed.
///
/// # Errors
///
/// Never returns `Err` in the current implementation — RLM failures
/// fall back to chunk-based compression, which is infallible. The
/// `Result` return shape is preserved for forward-compatibility with
/// future pipeline wiring.
pub(crate) async fn compress_last_message_if_oversized(
    messages: &mut [Message],
    ctx: &CompressContext,
    provider: Arc<dyn crate::provider::Provider>,
    model: &str,
) -> Result<bool> {
    let Some(last) = messages.last_mut() else {
        return Ok(false);
    };

    let original = extract_message_text(last);
    if original.is_empty() {
        return Ok(false);
    }

    let ctx_window = context_window_for_model(model);
    let msg_tokens = RlmChunker::estimate_tokens(&original);
    let threshold = (ctx_window as f64 * OVERSIZED_LAST_MESSAGE_RATIO) as usize;
    if msg_tokens <= threshold {
        return Ok(false);
    }

    tracing::info!(
        msg_tokens,
        threshold,
        ctx_window,
        "RLM: Last message exceeds context threshold, compressing"
    );

    let cached = super::rlm_background::context_summary(
        &original,
        "oversized_last_message",
        &ctx.session_id,
        model,
        provider,
        &ctx.rlm_config,
    );
    let replacement = cached
        .map(|summary| {
            super::compression_last_message::wrap_cached(
                summary,
                &original,
                msg_tokens,
                OVERSIZED_LAST_MESSAGE_PREFIX_CHARS,
            )
        })
        .unwrap_or_else(|| RlmChunker::compress(&original, threshold, None));

    last.content = vec![ContentPart::Text { text: replacement }];
    Ok(true)
}

/// Concatenate the textual content of `msg`, skipping non-text parts.
///
/// Used by [`compress_last_message_if_oversized`] to recover the string
/// form of a message without assuming the content vector has exactly one
/// text part.
fn extract_message_text(msg: &Message) -> String {
    let mut buf = String::new();
    for part in &msg.content {
        if let ContentPart::Text { text } = part {
            if !buf.is_empty() {
                buf.push('\n');
            }
            buf.push_str(text);
        }
    }
    buf
}

/// Back-compat wrapper over [`compress_messages_keep_last`] that operates
/// on an owning [`Session`]. Bumps [`Session::updated_at`] when the
/// buffer was rewritten.
///
/// Prefer [`compress_messages_keep_last`] for new code — it lets you run
/// compression on a history clone without mutating the canonical
/// [`Session::messages`] buffer.
///
/// # Errors
///
/// Propagates any error from the underlying core.
pub async fn compress_history_keep_last(
    session: &mut Session,
    provider: Arc<dyn crate::provider::Provider>,
    model: &str,
    keep_last: usize,
    reason: &str,
) -> Result<bool> {
    let ctx = CompressContext::from_session(session);
    let did = compress_messages_keep_last(
        &mut session.messages,
        &ctx,
        provider,
        model,
        keep_last,
        reason,
    )
    .await?;
    if did {
        session.updated_at = Utc::now();
    }
    Ok(did)
}

/// Ensure the estimated request token cost of `messages` fits within the
/// model's safety budget, in-place.
///
/// This is the `&mut Vec<Message>` core that powers both the legacy
/// [`enforce_context_window`] wrapper (which takes `&mut Session`) and
/// the Phase B [`derive_context`](crate::session::context::derive_context)
/// pipeline (which runs on a history clone and never mutates the
/// canonical [`Session::messages`] buffer).
///
/// # Arguments
///
/// * `messages` — The message buffer to rewrite in place.
/// * `ctx` — Session-level configuration snapshot.
/// * `provider` — Caller's primary provider.
/// * `model` — Caller's primary model identifier (governs ctx window).
/// * `system_prompt` — Current system prompt, included in token estimates.
/// * `tools` — Tool definitions, included in token estimates.
/// * `event_tx` — Optional channel for emitting
///   [`SessionEvent::CompactionStarted`] /
///   [`SessionEvent::CompactionCompleted`] /
///   [`SessionEvent::ContextTruncated`] /
///   [`SessionEvent::CompactionFailed`]. When `None`, compaction runs
///   silently.
///
/// # Errors
///
/// Returns `Err` only if an underlying RLM pass errors in a way the
/// fallback cascade cannot recover from. Terminal truncation itself is
/// infallible.
pub(crate) async fn enforce_on_messages(
    messages: &mut Vec<Message>,
    ctx: &CompressContext,
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

    let initial_est = estimate_request_tokens(system_prompt, messages, tools);
    let history_trigger = ctx.rlm_config.history_trigger_messages;
    let history_over = history_trigger > 0 && messages.len() >= history_trigger;
    if initial_est <= safety_budget && !history_over {
        return Ok(());
    }

    let trigger_reason = if history_over && initial_est <= safety_budget {
        "history_length"
    } else {
        "context_budget"
    };

    let trace_id = Uuid::new_v4();
    emit(
        event_tx,
        SessionEvent::CompactionStarted(CompactionStart {
            trace_id,
            reason: trigger_reason.to_string(),
            before_tokens: initial_est,
            budget: safety_budget,
        }),
    )
    .await;

    let rlm_ctx = super::compression_bus::observability_ctx(ctx, event_tx, trace_id);

    for keep_last in KEEP_LAST_CANDIDATES {
        let est = estimate_request_tokens(system_prompt, messages, tools);
        let still_long =
            history_trigger > 0 && messages.len() > history_trigger.saturating_sub(keep_last);
        if est <= safety_budget && !still_long {
            emit(
                event_tx,
                SessionEvent::CompactionCompleted(CompactionOutcome {
                    trace_id,
                    strategy: FallbackStrategy::Rlm,
                    before_tokens: initial_est,
                    after_tokens: est,
                    kept_messages: messages.len(),
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

        let did = compress_messages_keep_last(
            messages,
            &rlm_ctx,
            Arc::clone(&provider),
            model,
            keep_last,
            trigger_reason,
        )
        .await?;

        if !did {
            break;
        }
    }

    // Re-estimate one last time after the final RLM pass.
    let last_est = estimate_request_tokens(system_prompt, messages, tools);
    if last_est <= safety_budget {
        emit(
            event_tx,
            SessionEvent::CompactionCompleted(CompactionOutcome {
                trace_id,
                strategy: FallbackStrategy::Rlm,
                before_tokens: initial_est,
                after_tokens: last_est,
                kept_messages: messages.len(),
            }),
        )
        .await;
        return Ok(());
    }

    // Every RLM / chunk attempt still leaves us over budget.
    // Apply terminal truncation as the last-resort fallback.
    let dropped_tokens = terminal_truncate_messages(
        messages,
        system_prompt,
        tools,
        TRUNCATE_KEEP_LAST,
        safety_budget,
    );
    let after_tokens = estimate_request_tokens(system_prompt, messages, tools);

    tracing::warn!(
        before_tokens = initial_est,
        after_tokens,
        dropped_tokens,
        kept_messages = messages.len(),
        safety_budget,
        "All RLM compaction attempts exhausted; applied terminal truncation fallback"
    );

    emit(
        event_tx,
        SessionEvent::ContextTruncated(ContextTruncation {
            trace_id,
            dropped_tokens,
            kept_messages: messages.len(),
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
            kept_messages: messages.len(),
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

/// Back-compat wrapper over [`enforce_on_messages`] that operates on an
/// owning [`Session`]. Bumps [`Session::updated_at`] when the buffer was
/// rewritten.
///
/// Prefer [`enforce_on_messages`] for new code — it lets you run
/// context-window enforcement on a history clone without mutating the
/// canonical [`Session::messages`] buffer (the Phase A history/context
/// split).
///
/// # Errors
///
/// Propagates any error from the underlying core.
pub async fn enforce_context_window(
    session: &mut Session,
    provider: Arc<dyn crate::provider::Provider>,
    model: &str,
    system_prompt: &str,
    tools: &[ToolDefinition],
    event_tx: Option<&mpsc::Sender<SessionEvent>>,
) -> Result<()> {
    let ctx = CompressContext::from_session(session);
    let before_len = session.messages.len();
    enforce_on_messages(
        &mut session.messages,
        &ctx,
        provider,
        model,
        system_prompt,
        tools,
        event_tx,
    )
    .await?;
    if session.messages.len() != before_len {
        session.updated_at = Utc::now();
    }
    Ok(())
}

/// Drop everything older than the last `keep_last` messages in
/// `messages` and return an approximate count of the tokens removed.
///
/// Unlike [`compress_messages_keep_last`] this keeps **no** summary of
/// the dropped prefix — the caller is expected to have already attempted
/// RLM-based compaction and exhausted it. A `[CONTEXT TRUNCATED]`
/// assistant marker is prepended so the model is aware that older turns
/// were silently removed.
///
/// This is the `&mut Vec<Message>` core for terminal truncation. It is
/// called from [`enforce_on_messages`] as the last-resort fallback when
/// every RLM / chunk attempt has left the buffer over budget.
///
/// # Arguments
///
/// * `messages` — The message buffer to rewrite in place.
/// * `system_prompt` — Included in the before/after token estimates so
///   the caller sees the net effect on request size, not just message
///   count.
/// * `tools` — Tool definitions, included in token estimates.
/// * `keep_last` — Number of most-recent messages to retain.
///
/// # Returns
///
/// An approximate count of tokens removed (saturating subtraction).
/// Returns `0` when `messages.len() <= keep_last`, the request already fits
/// `target_tokens`, and no work is done.
pub(crate) fn terminal_truncate_messages(
    messages: &mut Vec<Message>,
    system_prompt: &str,
    tools: &[ToolDefinition],
    keep_last: usize,
    target_tokens: usize,
) -> usize {
    let before = estimate_request_tokens(system_prompt, messages, tools);
    if messages.len() <= keep_last && before <= target_tokens {
        return 0;
    }

    let split_idx = if messages.len() > keep_last {
        crate::session::context::active_tail::active_tail_start(messages, keep_last)
    } else {
        0
    };
    let dropped_prefix = split_idx > 0;
    let tail = if dropped_prefix {
        messages.split_off(split_idx)
    } else {
        std::mem::take(messages)
    };

    let marker = Message {
        role: Role::Assistant,
        content: vec![ContentPart::Text {
            text: if dropped_prefix {
                "[CONTEXT TRUNCATED]\nOlder conversation was dropped to keep the request \
                 under the model's context window. Some retained tool output may also be \
                 shortened with head/tail snippets."
                    .to_string()
            } else {
                "[CONTEXT TRUNCATED]\nRecent tool output was too large for the model's \
                 context window and was shortened with head/tail snippets."
                    .to_string()
            },
        }],
    };

    let mut new_messages = Vec::with_capacity(1 + tail.len());
    new_messages.push(marker);
    new_messages.extend(tail);
    *messages = new_messages;

    let shrunk_parts =
        shrink_retained_payloads_to_budget(messages, system_prompt, tools, target_tokens);
    if shrunk_parts > 0 {
        tracing::warn!(
            shrunk_parts,
            target_tokens,
            after_tokens = estimate_request_tokens(system_prompt, messages, tools),
            "Terminal truncation shortened retained message payloads"
        );
    }

    let after = estimate_request_tokens(system_prompt, messages, tools);
    before.saturating_sub(after)
}

fn shrink_retained_payloads_to_budget(
    messages: &mut [Message],
    system_prompt: &str,
    tools: &[ToolDefinition],
    target_tokens: usize,
) -> usize {
    if target_tokens == usize::MAX
        || estimate_request_tokens(system_prompt, messages, tools) <= target_tokens
    {
        return 0;
    }

    let mut changed_parts = 0;
    for cap in TERMINAL_PAYLOAD_CAPS {
        changed_parts += shrink_payloads_over_cap(messages, cap, false);
        if estimate_request_tokens(system_prompt, messages, tools) <= target_tokens {
            return changed_parts;
        }
    }

    changed_parts += shrink_payloads_over_cap(messages, TERMINAL_EMERGENCY_CAP_BYTES, true);
    changed_parts
}

fn shrink_payloads_over_cap(
    messages: &mut [Message],
    cap_bytes: usize,
    include_latest_user: bool,
) -> usize {
    let message_count = messages.len();
    let mut changed = 0;
    for (msg_idx, msg) in messages.iter_mut().enumerate() {
        let role = msg.role;
        let is_latest_user = matches!(role, Role::User) && msg_idx + 1 == message_count;
        for part in &mut msg.content {
            let cap = match part {
                ContentPart::ToolResult { .. } | ContentPart::Thinking { .. } => cap_bytes,
                ContentPart::ToolCall { .. } => cap_bytes,
                ContentPart::Text { text } => {
                    if msg_idx == 0 && text.starts_with("[CONTEXT TRUNCATED]") {
                        continue;
                    }
                    if is_latest_user && !include_latest_user {
                        cap_bytes.max(16_384)
                    } else {
                        cap_bytes
                    }
                }
                ContentPart::Image { .. } | ContentPart::File { .. } => continue,
            };
            if shrink_content_part(part, cap) {
                changed += 1;
            }
        }
    }
    changed
}

fn shrink_content_part(part: &mut ContentPart, cap_bytes: usize) -> bool {
    match part {
        ContentPart::Text { text } => shrink_string_payload(text, cap_bytes, "text"),
        ContentPart::ToolResult { content, .. } => {
            shrink_string_payload(content, cap_bytes, "tool_result")
        }
        ContentPart::ToolCall { arguments, .. } => {
            shrink_string_payload(arguments, cap_bytes, "tool_call_arguments")
        }
        ContentPart::Thinking { text } => shrink_string_payload(text, cap_bytes, "thinking"),
        ContentPart::Image { .. } | ContentPart::File { .. } => false,
    }
}

fn shrink_string_payload(value: &mut String, cap_bytes: usize, label: &str) -> bool {
    if value.len() <= cap_bytes {
        return false;
    }
    *value = terminal_head_tail(value, cap_bytes, label);
    true
}

fn terminal_head_tail(value: &str, cap_bytes: usize, label: &str) -> String {
    let marker = format!(
        "\n\n[terminal context fallback truncated {label}; original_bytes={}]\n\n",
        value.len()
    );
    if cap_bytes <= marker.len() + 32 {
        return format!(
            "[terminal context fallback truncated {label}; original_bytes={}]",
            value.len()
        );
    }

    let available = cap_bytes - marker.len();
    let head_budget = available / 2;
    let tail_budget = available - head_budget;
    let head = crate::util::truncate_bytes_safe(value, head_budget);
    let tail = suffix_bytes_safe(value, tail_budget);
    format!("{head}{marker}{tail}")
}

fn suffix_bytes_safe(s: &str, max_bytes: usize) -> &str {
    if s.len() <= max_bytes {
        return s;
    }
    let mut start = s.len().saturating_sub(max_bytes);
    while start < s.len() && !s.is_char_boundary(start) {
        start += 1;
    }
    &s[start..]
}

async fn emit(event_tx: Option<&mpsc::Sender<SessionEvent>>, ev: SessionEvent) {
    if let Some(tx) = event_tx {
        let _ = tx.send(ev).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn user(text: &str) -> Message {
        Message {
            role: Role::User,
            content: vec![ContentPart::Text {
                text: text.to_string(),
            }],
        }
    }

    fn tool_result(content: String) -> Message {
        Message {
            role: Role::Tool,
            content: vec![ContentPart::ToolResult {
                tool_call_id: "call_1".to_string(),
                content,
            }],
        }
    }

    #[test]
    fn terminal_truncate_noop_when_short_enough() {
        let mut messages = vec![user("a"), user("b")];
        let before_len = messages.len();
        let dropped = terminal_truncate_messages(&mut messages, "", &[], 4, usize::MAX);
        assert_eq!(dropped, 0);
        assert_eq!(messages.len(), before_len);
    }

    #[test]
    fn terminal_truncate_drops_prefix_and_prepends_marker() {
        let mut messages: Vec<Message> = (0..10).map(|i| user(&format!("msg-{i}"))).collect();
        let _ = terminal_truncate_messages(&mut messages, "", &[], 3, usize::MAX);

        // 1 synthetic marker + 3 most-recent messages.
        assert_eq!(messages.len(), 4);

        // First message is the [CONTEXT TRUNCATED] assistant marker.
        assert!(matches!(messages[0].role, Role::Assistant));
        if let ContentPart::Text { text } = &messages[0].content[0] {
            assert!(text.starts_with("[CONTEXT TRUNCATED]"));
        } else {
            panic!("expected text content on the synthetic marker");
        }

        // Remaining three messages are the most-recent user messages,
        // preserved verbatim and in their original order.
        let kept_texts: Vec<&str> = messages[1..]
            .iter()
            .map(|m| match &m.content[0] {
                ContentPart::Text { text } => text.as_str(),
                _ => panic!("expected text content"),
            })
            .collect();
        assert_eq!(kept_texts, vec!["msg-7", "msg-8", "msg-9"]);
    }

    #[test]
    fn terminal_truncate_shrinks_oversized_tail_when_message_count_is_short() {
        let huge_output = format!("head\n{}\ntail", "x".repeat(200_000));
        let mut messages = vec![user("inspect the logs"), tool_result(huge_output)];
        let before = estimate_request_tokens("", &messages, &[]);

        let dropped = terminal_truncate_messages(&mut messages, "", &[], 4, 6_000);
        let after = estimate_request_tokens("", &messages, &[]);

        assert!(dropped > 0);
        assert!(after < before);
        assert!(after <= 6_000, "after={after}");
        assert_eq!(messages.len(), 3);
        assert!(matches!(messages[0].role, Role::Assistant));

        let ContentPart::ToolResult { content, .. } = &messages[2].content[0] else {
            panic!("expected tool result");
        };
        assert!(content.contains("terminal context fallback truncated tool_result"));
        assert!(content.contains("head"));
        assert!(content.contains("tail"));
    }

    #[test]
    fn compress_context_from_session_snapshot_is_independent() {
        // Changing the session's metadata after snapshotting must not
        // leak through the captured CompressContext. This lets the
        // Phase B derive_context pipeline hold a CompressContext
        // alongside a `&mut Vec<Message>` borrowed from the same
        // Session without running into borrow conflicts.
        let session = Session {
            id: "session-42".to_string(),
            title: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            metadata: Default::default(),
            agent: "test".to_string(),
            messages: Vec::new(),
            pages: Vec::new(),
            summary_index: crate::session::index::SummaryIndex::new(),
            tool_uses: Vec::new(),
            usage: Default::default(),
            max_steps: None,
            bus: None,
        };
        let snapshot = CompressContext::from_session(&session);
        assert_eq!(snapshot.session_id, "session-42");
        assert!(snapshot.subcall_provider.is_none());
        assert!(snapshot.subcall_model.is_none());
    }
}
