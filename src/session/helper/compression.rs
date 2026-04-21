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

/// Default cheap model used for RLM context compression when neither
/// [`crate::rlm::RlmConfig::root_model`] nor the
/// `CODETETHER_RLM_MODEL` environment variable is set.
///
/// RLM compaction can consume a surprisingly large share of session
/// cost because it runs on large prefixes of history at every step
/// past the threshold. Using the caller's main model (often Claude
/// Opus or GPT-5) for this work quietly multiplies spend. We default
/// to a cheap general-purpose model instead; users who want higher
/// fidelity summaries can override via config or env.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::session::helper::compression::DEFAULT_RLM_MODEL;
/// assert!(DEFAULT_RLM_MODEL.contains('/'));
/// ```
pub const DEFAULT_RLM_MODEL: &str = "zai/glm-5.1";

/// Resolve the model string that should be used for RLM compaction.
///
/// Precedence (highest first):
///
/// 1. [`crate::rlm::RlmConfig::root_model`] on the session.
/// 2. The `CODETETHER_RLM_MODEL` environment variable.
/// 3. [`DEFAULT_RLM_MODEL`].
///
/// The caller's main model is never used unless all three above are
/// absent (returns `None`, signalling the caller should keep its own
/// model).
fn resolve_rlm_model(rlm_config: &crate::rlm::RlmConfig) -> Option<String> {
    if let Some(m) = rlm_config.root_model.as_ref() {
        return Some(m.clone());
    }
    if let Ok(env_model) = std::env::var("CODETETHER_RLM_MODEL")
        && !env_model.trim().is_empty()
    {
        return Some(env_model);
    }
    Some(DEFAULT_RLM_MODEL.to_string())
}

/// Candidate model pool that [`resolve_rlm_model_bandit`] can rank
/// when the rule-based precedence returns no winner and delegation is
/// enabled.
///
/// Hand-picked per CADMAS-CTX Section 5.9 — a small set of cheap
/// general-purpose models with stable cost profiles so the LCB scoring
/// actually has a meaningful dynamic range. Add models here rather
/// than at call sites so the candidate list stays reviewable.
const RLM_MODEL_CANDIDATES: &[&str] = &[
    "zai/glm-5.1",
    "glm5/glm-5",
    "openrouter/openai/gpt-oss-120b:free",
];

/// CADMAS-CTX-aware variant of [`resolve_rlm_model`] (Phase C step 30).
///
/// When `state.config.enabled` is `true`, ranks [`RLM_MODEL_CANDIDATES`]
/// by the LCB score `μ − γ·√u` under the supplied `bucket` (skill
/// key: `"rlm_compact"`) and returns the highest-scoring candidate.
/// When delegation is disabled, this is exactly [`resolve_rlm_model`].
///
/// The *update* half of the bandit loop lives at the compaction call
/// site — a `session.state.update(model, "rlm_compact", bucket,
/// produced_under_budget)` after each pass. That wiring is deferred
/// to a follow-up commit to keep this one scoped to the selection
/// primitive.
///
/// # Arguments
///
/// * `rlm_config` — Session-level RLM configuration.
/// * `state` — CADMAS-CTX sidecar. Read-only here.
/// * `bucket` — Context bucket from the current turn's
///   [`RelevanceMeta`](crate::session::relevance::RelevanceMeta).
///
/// # Returns
///
/// Same shape as [`resolve_rlm_model`]: `Some(model)` to target a
/// dedicated cheap model, or `None` to signal the caller should reuse
/// its own model.
pub(crate) fn resolve_rlm_model_bandit(
    rlm_config: &crate::rlm::RlmConfig,
    state: &crate::session::delegation::DelegationState,
    bucket: crate::session::relevance::Bucket,
) -> Option<String> {
    if !state.config.enabled {
        return resolve_rlm_model(rlm_config);
    }
    // Explicit configuration still wins — the bandit only disambiguates
    // the otherwise-static fallback list.
    if let Some(m) = rlm_config.root_model.as_ref() {
        return Some(m.clone());
    }
    if let Ok(env_model) = std::env::var("CODETETHER_RLM_MODEL")
        && !env_model.trim().is_empty()
    {
        return Some(env_model);
    }

    let mut best: Option<(&str, f64)> = None;
    for candidate in RLM_MODEL_CANDIDATES {
        let score = state
            .score(candidate, "rlm_compact", bucket)
            .unwrap_or(0.0);
        match best {
            Some((_, current)) if current >= score => {}
            _ => best = Some((candidate, score)),
        }
    }
    best.map(|(m, _)| m.to_string())
        .or_else(|| Some(DEFAULT_RLM_MODEL.to_string()))
}

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
}

impl CompressContext {
    /// Snapshot the non-buffer fields of `session` into an owned context.
    pub(crate) fn from_session(session: &Session) -> Self {
        Self {
            rlm_config: session.metadata.rlm.clone(),
            session_id: session.id.clone(),
            subcall_provider: session.metadata.subcall_provider.clone(),
            subcall_model: session.metadata.subcall_model_name.clone(),
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
/// Propagates any error from [`RlmRouter::auto_process`] that cannot be
/// recovered by the chunk-compression fallback.
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

    let split_idx = messages.len().saturating_sub(keep_last);
    let tail = messages.split_off(split_idx);
    let prefix = std::mem::take(messages);

    let context = messages_to_rlm_context(&prefix);
    let ctx_window = context_window_for_model(model);

    // Prefer a cheap dedicated RLM model over the caller's main model
    // to keep compaction cost bounded. Falls back to the caller's
    // provider/model if the dedicated model cannot be resolved.
    let (rlm_provider, rlm_model) = match resolve_rlm_model(&ctx.rlm_config) {
        Some(target_model) if target_model != model => {
            match crate::provider::ProviderRegistry::from_vault().await {
                Ok(registry) => match registry.resolve_model(&target_model) {
                    Ok((p, m)) => {
                        tracing::info!(
                            rlm_model = %m,
                            caller_model = %model,
                            "RLM: using cheap dedicated model for compression"
                        );
                        (p, m)
                    }
                    Err(err) => {
                        tracing::warn!(
                            rlm_model = %target_model,
                            error = %err,
                            "RLM: dedicated model resolution failed; falling back to caller model"
                        );
                        (Arc::clone(&provider), model.to_string())
                    }
                },
                Err(err) => {
                    tracing::warn!(
                        error = %err,
                        "RLM: provider registry unavailable; falling back to caller model"
                    );
                    (Arc::clone(&provider), model.to_string())
                }
            }
        }
        _ => (Arc::clone(&provider), model.to_string()),
    };

    let auto_ctx = AutoProcessContext {
        tool_id: "session_context",
        tool_args: serde_json::json!({"reason": reason}),
        session_id: &ctx.session_id,
        abort: None,
        on_progress: None,
        provider: rlm_provider,
        model: rlm_model.clone(),
        bus: None,
        trace_id: None,
        subcall_provider: ctx.subcall_provider.clone(),
        subcall_model: ctx.subcall_model.clone(),
    };

    let summary = match RlmRouter::auto_process(&context, auto_ctx, &ctx.rlm_config).await {
        Ok(result) => {
            tracing::info!(
                reason,
                rlm_model = %rlm_model,
                input_tokens = result.stats.input_tokens,
                output_tokens = result.stats.output_tokens,
                compression_ratio = result.stats.compression_ratio,
                "RLM: Compressed session history"
            );
            // Attribute RLM token spend against the RLM model so the
            // TUI cost badge reflects it instead of silently inflating
            // the caller's main model's reported usage.
            crate::telemetry::TOKEN_USAGE.record_model_usage(
                &rlm_model,
                result.stats.input_tokens as u64,
                result.stats.output_tokens as u64,
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
                 to fit the model context window.\n\n{summary}\n\n\
                 [RECOVERY] If you need specific details that this summary \
                 dropped (exact file paths, prior tool output, earlier user \
                 instructions, numeric values), call the `session_recall` \
                 tool with a targeted query instead of guessing or asking \
                 the user to repeat themselves."
            ),
        }],
    };

    let mut new_messages = Vec::with_capacity(1 + tail.len());
    new_messages.push(summary_msg);
    new_messages.extend(tail);
    *messages = new_messages;

    Ok(true)
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
/// * On successful RLM compression, rewrites the last message's content
///   to a single text part with an `[Original message: N tokens,
///   compressed via RLM]` prefix, the compressed body, and the first
///   [`OVERSIZED_LAST_MESSAGE_PREFIX_CHARS`] characters of the original
///   as a literal suffix.
/// * On RLM failure, falls back to [`RlmChunker::compress`].
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

    let auto_ctx = AutoProcessContext {
        tool_id: "session_context",
        tool_args: serde_json::json!({}),
        session_id: &ctx.session_id,
        abort: None,
        on_progress: None,
        provider: Arc::clone(&provider),
        model: model.to_string(),
        bus: None,
        trace_id: None,
        subcall_provider: ctx.subcall_provider.clone(),
        subcall_model: ctx.subcall_model.clone(),
    };

    let replacement = match RlmRouter::auto_process(&original, auto_ctx, &ctx.rlm_config).await {
        Ok(result) => {
            tracing::info!(
                input_tokens = result.stats.input_tokens,
                output_tokens = result.stats.output_tokens,
                "RLM: Last message compressed"
            );
            format!(
                "[Original message: {msg_tokens} tokens, compressed via RLM]\n\n{}\n\n---\nOriginal request prefix:\n{}",
                result.processed,
                original
                    .chars()
                    .take(OVERSIZED_LAST_MESSAGE_PREFIX_CHARS)
                    .collect::<String>()
            )
        }
        Err(e) => {
            tracing::warn!(error = %e, "RLM: Failed to compress last message, using truncation");
            let max_chars = threshold * 4;
            RlmChunker::compress(&original, max_chars / 4, None)
        }
    };

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
pub(crate) async fn compress_history_keep_last(
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
            ctx,
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
    let dropped_tokens =
        terminal_truncate_messages(messages, system_prompt, tools, TRUNCATE_KEEP_LAST);
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
pub(crate) async fn enforce_context_window(
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
/// Returns `0` when `messages.len() <= keep_last` and no work is done.
pub(crate) fn terminal_truncate_messages(
    messages: &mut Vec<Message>,
    system_prompt: &str,
    tools: &[ToolDefinition],
    keep_last: usize,
) -> usize {
    if messages.len() <= keep_last {
        return 0;
    }

    let before = estimate_request_tokens(system_prompt, messages, tools);
    let split_idx = messages.len().saturating_sub(keep_last);
    let tail = messages.split_off(split_idx);

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
    *messages = new_messages;

    let after = estimate_request_tokens(system_prompt, messages, tools);
    before.saturating_sub(after)
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

    #[test]
    fn terminal_truncate_noop_when_short_enough() {
        let mut messages = vec![user("a"), user("b")];
        let before = messages.clone();
        let dropped = terminal_truncate_messages(&mut messages, "", &[], 4);
        assert_eq!(dropped, 0);
        assert_eq!(messages, before);
    }

    #[test]
    fn terminal_truncate_drops_prefix_and_prepends_marker() {
        let mut messages: Vec<Message> = (0..10).map(|i| user(&format!("msg-{i}"))).collect();
        let _ = terminal_truncate_messages(&mut messages, "", &[], 3);

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
