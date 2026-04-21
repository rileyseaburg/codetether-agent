//! Derive the per-step LLM context from an append-only chat history.
//!
//! ## Hypothesis
//!
//! *Chat/session history is not chat context and should not be treated as
//! such.* See the plan at
//! `~/.claude/plans/hypothesis-chat-session-history-is-snuggly-dijkstra.md`
//! and the load-bearing references cited there (Liu et al. arXiv:2512.22087,
//! Rafique & Bindschaedler arXiv:2604.10352, Lu et al. arXiv:2510.06727).
//!
//! ## Role in the Phase A refactor
//!
//! Today `session.messages` is destructively mutated by compression,
//! experimental dedup / snippet strategies, and pairing repair before
//! every LLM call, and then persisted to disk — so the pure record of
//! *what happened* is lost the moment a compaction fires.
//!
//! This module produces an ephemeral [`DerivedContext`] per prompt step
//! by cloning `session.messages` and running the existing pipeline on
//! the clone. The canonical buffer stays untouched and append-only; the
//! LLM sees the derived projection.
//!
//! In Phase A the derivation policy is fixed ("legacy clone + pipeline").
//! Phase B adds `Incremental`, `Reset` (Lu et al.), and `OracleReplay`
//! (ClawVM) variants on this same signature.
//!
//! ## Examples
//!
//! ```rust,no_run
//! # tokio::runtime::Runtime::new().unwrap().block_on(async {
//! use codetether_agent::session::Session;
//! use codetether_agent::session::context::derive_context;
//! use std::sync::Arc;
//!
//! let session = Session::new().await.unwrap();
//! # let provider: Arc<dyn codetether_agent::provider::Provider> = todo!();
//! let derived = derive_context(
//!     &session,
//!     provider,
//!     "openai/gpt-5",
//!     "You are a helpful assistant.",
//!     &[],
//!     None,
//!     None,
//! )
//! .await
//! .unwrap();
//!
//! // Phase A invariant: the canonical history is never mutated.
//! assert_eq!(session.messages.len(), derived.origin_len);
//! # });
//! ```

use std::sync::Arc;

use anyhow::{Context as _, Result};
use tokio::sync::mpsc;

use crate::provider::{ContentPart, Message, Role, ToolDefinition};
use crate::rlm::router::AutoProcessContext;
use crate::rlm::RlmRouter;
use crate::session::{Session, SessionEvent};

use super::derive_policy::DerivePolicy;
use super::helper::compression::{
    CompressContext, compress_last_message_if_oversized, compress_messages_keep_last,
    enforce_on_messages,
};
use super::helper::error::messages_to_rlm_context;
use super::helper::experimental;
use super::helper::token::estimate_request_tokens;

/// The per-step LLM context, derived from an append-only chat history.
///
/// This is the object the prompt loop should hand to the provider — *not*
/// [`Session::messages`] directly.
///
/// # Fields
///
/// * `messages` — The message list to include in the completion request.
///   Produced by running the Phase A pipeline on a clone of
///   [`Session::messages`].
/// * `origin_len` — Length of the source history at the moment of
///   derivation. Lets Phase B's incremental policies detect a stale
///   cache without comparing full message contents.
/// * `compressed` — Whether compression (RLM or terminal truncation)
///   fired during this derivation. Purely informational for the TUI.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::session::context::DerivedContext;
///
/// let derived = DerivedContext {
///     messages: Vec::new(),
///     origin_len: 0,
///     compressed: false,
/// };
/// assert_eq!(derived.origin_len, 0);
/// assert!(!derived.compressed);
/// ```
#[derive(Debug, Clone)]
pub struct DerivedContext {
    /// Messages to send to the provider this turn.
    pub messages: Vec<Message>,
    /// `session.messages.len()` at the moment of derivation.
    pub origin_len: usize,
    /// `true` when any compression / truncation pass rewrote the clone.
    pub compressed: bool,
}

/// Derive an ephemeral [`DerivedContext`] from `session`'s pure history.
///
/// Phase A (legacy) policy:
///
/// 1. Clone [`Session::messages`] into a scratch buffer.
/// 2. Run [`experimental::apply_all`] — dedup, snippet-stale, entropy
///    prune, streaming-llm trim. Ends with a [`pairing::repair_orphans`]
///    pass so no orphan `tool_call`/`tool_result` pairs leak through.
/// 3. Run [`enforce_on_messages`] — progressively shrink the prefix via
///    RLM compaction until the token estimate fits within the model's
///    safety budget, falling back to terminal truncation if needed.
/// 4. Run [`pairing::repair_orphans`] once more as a safety net, in case
///    the compaction pass severed a `tool_call`/`tool_result` pair.
/// 5. Return a [`DerivedContext`] wrapping the mutated clone.
///
/// The canonical [`Session::messages`] buffer is never touched —
/// `session` is borrowed immutably.
///
/// # Arguments
///
/// * `session` — The owning session (read-only borrow).
/// * `provider` — Caller's primary provider.
/// * `model` — Caller's primary model identifier.
/// * `system_prompt` — Included in token estimates.
/// * `tools` — Tool definitions, included in token estimates.
/// * `event_tx` — Optional channel for emitting compaction lifecycle
///   events. When `None`, compaction runs silently.
/// * `force_keep_last` — When `Some(n)`, skip the adaptive
///   [`enforce_on_messages`] budget cascade and force a single
///   [`compress_messages_keep_last`] call with `keep_last = n`. Used by
///   the prompt-too-long retry path in `prompt.rs:244` / `prompt_events.rs`.
///
/// # Returns
///
/// The derived context. `messages` may be shorter than `session.messages`
/// when compression fires; `origin_len` always reflects the untouched
/// source length.
///
/// # Errors
///
/// Propagates any error from the underlying compression pipeline that
/// the fallback cascade cannot recover from.
///
/// [`pairing::repair_orphans`]: super::helper::experimental::pairing::repair_orphans
pub async fn derive_context(
    session: &Session,
    provider: Arc<dyn crate::provider::Provider>,
    model: &str,
    system_prompt: &str,
    tools: &[ToolDefinition],
    event_tx: Option<&mpsc::Sender<SessionEvent>>,
    force_keep_last: Option<usize>,
) -> Result<DerivedContext> {
    let origin_len = session.messages.len();
    let mut messages = session.messages.clone();
    let ctx = CompressContext::from_session(session);

    // Step 0: if the freshly-appended message (typically a user prompt)
    // is larger than 35 % of the ctx window, replace it in the clone
    // with an RLM summary. The canonical `session.messages` still holds
    // the original — preserving the Phase A invariant.
    let _ = compress_last_message_if_oversized(
        &mut messages,
        &ctx,
        Arc::clone(&provider),
        model,
    )
    .await?;

    // Step 1: run experimental strategies on the clone so the prompt
    // loop never observes the compressed form mutating its own input.
    experimental::apply_all(&mut messages);

    let before_compression = messages.len();

    match force_keep_last {
        Some(keep_last) => {
            let _ = compress_messages_keep_last(
                &mut messages,
                &ctx,
                Arc::clone(&provider),
                model,
                keep_last,
                "prompt_too_long_retry",
            )
            .await?;
        }
        None => {
            enforce_on_messages(
                &mut messages,
                &ctx,
                Arc::clone(&provider),
                model,
                system_prompt,
                tools,
                event_tx,
            )
            .await?;
        }
    }

    // Safety net: compaction may have dropped a tool_result without its
    // matching tool_call (or vice versa). Re-run the pairing repair so
    // the provider never sees an orphaned tool_use id.
    experimental::pairing::repair_orphans(&mut messages);

    Ok(DerivedContext {
        messages,
        origin_len,
        compressed: messages_len_changed(before_compression, &messages),
    })
}

/// Compare two message counts and return whether compression fired.
///
/// Separated out so the comparison is testable without a full provider
/// round-trip. Compression can either shrink the buffer (RLM / chunk /
/// truncation) or leave it exactly the same length (no-op when under
/// budget). Any count change is treated as evidence that compression
/// fired.
fn messages_len_changed(before: usize, after: &[Message]) -> bool {
    before != after.len()
}

/// Derive an ephemeral [`DerivedContext`] under a chosen [`DerivePolicy`].
///
/// Generalisation of [`derive_context`] that accepts a policy selector.
/// [`DerivePolicy::Legacy`] delegates back to `derive_context` so the
/// two signatures co-exist without behaviour drift during rollout.
///
/// # Arguments
///
/// Same as [`derive_context`], plus:
///
/// * `policy` — Which derivation strategy to run. See [`DerivePolicy`].
///
/// # Errors
///
/// Propagates any error from the underlying pipeline.
pub async fn derive_with_policy(
    session: &Session,
    provider: Arc<dyn crate::provider::Provider>,
    model: &str,
    system_prompt: &str,
    tools: &[ToolDefinition],
    event_tx: Option<&mpsc::Sender<SessionEvent>>,
    policy: DerivePolicy,
) -> Result<DerivedContext> {
    match policy {
        DerivePolicy::Legacy => {
            derive_context(session, provider, model, system_prompt, tools, event_tx, None)
                .await
        }
        DerivePolicy::Reset { threshold_tokens } => {
            derive_reset(
                session,
                provider,
                model,
                system_prompt,
                tools,
                threshold_tokens,
            )
            .await
        }
    }
}

/// Index of the last [`Role::User`] message in `messages`, if any.
///
/// Used by [`derive_reset`] to preserve the most-recent user turn
/// verbatim while summarising the prefix.
fn last_user_index(messages: &[Message]) -> Option<usize> {
    messages
        .iter()
        .enumerate()
        .rev()
        .find(|(_, m)| matches!(m.role, Role::User))
        .map(|(i, _)| i)
}

/// Build the synthetic `[CONTEXT RESET]` summary message that replaces
/// the discarded prefix in [`DerivePolicy::Reset`].
///
/// Kept separate from the derivation body so the exact marker text can
/// be asserted in unit tests without a provider round-trip.
fn build_reset_summary_message(summary: &str) -> Message {
    Message {
        role: Role::Assistant,
        content: vec![ContentPart::Text {
            text: format!(
                "[CONTEXT RESET]\nEverything older than the current user turn was \
                 compressed into the summary below. Recent turns were \
                 intentionally discarded — call `session_recall` if you need \
                 a specific dropped detail.\n\n{summary}"
            ),
        }],
    }
}

/// [`DerivePolicy::Reset`] implementation.
///
/// When the token estimate of the cloned history plus system prompt
/// and tools exceeds `threshold_tokens`, summarise everything older
/// than the last user turn via the RLM router and return
/// `[summary_message, last_user_turn]`. Under threshold, return the
/// full clone verbatim. The canonical [`Session::messages`] is always
/// read-only here.
async fn derive_reset(
    session: &Session,
    provider: Arc<dyn crate::provider::Provider>,
    model: &str,
    system_prompt: &str,
    tools: &[ToolDefinition],
    threshold_tokens: usize,
) -> Result<DerivedContext> {
    let origin_len = session.messages.len();
    let mut messages = session.messages.clone();

    let est = estimate_request_tokens(system_prompt, &messages, tools);
    if est <= threshold_tokens {
        experimental::pairing::repair_orphans(&mut messages);
        return Ok(DerivedContext {
            messages,
            origin_len,
            compressed: false,
        });
    }

    let Some(split_idx) = last_user_index(&messages) else {
        // No user turn to preserve — fall through to Legacy behaviour
        // so we still honour the budget via adaptive compaction.
        return derive_context(
            session,
            provider,
            model,
            system_prompt,
            tools,
            None,
            None,
        )
        .await;
    };

    let tail = messages.split_off(split_idx);
    let prefix = std::mem::take(&mut messages);
    if prefix.is_empty() {
        messages = tail;
        experimental::pairing::repair_orphans(&mut messages);
        return Ok(DerivedContext {
            messages,
            origin_len,
            compressed: false,
        });
    }

    let rlm_config = session.metadata.rlm.clone();
    let summary = summarise_prefix_for_reset(
        &prefix,
        &session.id,
        Arc::clone(&provider),
        model,
        &rlm_config,
        session.metadata.subcall_provider.clone(),
        session.metadata.subcall_model_name.clone(),
    )
    .await?;

    let mut reset_messages = Vec::with_capacity(1 + tail.len());
    reset_messages.push(build_reset_summary_message(&summary));
    reset_messages.extend(tail);

    experimental::pairing::repair_orphans(&mut reset_messages);

    Ok(DerivedContext {
        messages: reset_messages,
        origin_len,
        compressed: true,
    })
}

/// RLM-summarise the prefix dropped by [`derive_reset`].
///
/// Extracted so the surrounding control flow stays scannable and the
/// summary prompt can be swapped for an agent-authored one in a future
/// commit (the `context_reset` tool — Phase B step 20 — hands the
/// model-produced summary directly to [`build_reset_summary_message`]
/// without an RLM round-trip).
async fn summarise_prefix_for_reset(
    prefix: &[Message],
    session_id: &str,
    provider: Arc<dyn crate::provider::Provider>,
    model: &str,
    rlm_config: &crate::rlm::RlmConfig,
    subcall_provider: Option<Arc<dyn crate::provider::Provider>>,
    subcall_model: Option<String>,
) -> Result<String> {
    let context = messages_to_rlm_context(prefix);
    let auto_ctx = AutoProcessContext {
        tool_id: "context_reset",
        tool_args: serde_json::json!({"policy": "reset"}),
        session_id,
        abort: None,
        on_progress: None,
        provider,
        model: model.to_string(),
        bus: None,
        trace_id: None,
        subcall_provider,
        subcall_model,
    };
    let result = RlmRouter::auto_process(&context, auto_ctx, rlm_config)
        .await
        .context("RLM summarisation for DerivePolicy::Reset failed")?;
    Ok(result.processed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derived_context_record_round_trips() {
        let ctx = DerivedContext {
            messages: vec![Message {
                role: Role::User,
                content: vec![ContentPart::Text {
                    text: "hi".to_string(),
                }],
            }],
            origin_len: 1,
            compressed: false,
        };
        let cloned = ctx.clone();
        assert_eq!(ctx.origin_len, cloned.origin_len);
        assert_eq!(ctx.compressed, cloned.compressed);
        assert_eq!(ctx.messages.len(), cloned.messages.len());
    }

    #[test]
    fn messages_len_changed_detects_shrink_and_noop() {
        let empty: Vec<Message> = Vec::new();
        assert!(!messages_len_changed(0, &empty));

        let one = vec![Message {
            role: Role::User,
            content: vec![ContentPart::Text {
                text: "x".to_string(),
            }],
        }];
        assert!(messages_len_changed(5, &one));
        assert!(!messages_len_changed(1, &one));
    }

    fn text(role: Role, s: &str) -> Message {
        Message {
            role,
            content: vec![ContentPart::Text {
                text: s.to_string(),
            }],
        }
    }

    #[test]
    fn last_user_index_finds_latest_user_turn() {
        let msgs = vec![
            text(Role::System, "sys"),
            text(Role::User, "first"),
            text(Role::Assistant, "reply"),
            text(Role::User, "second"),
            text(Role::Assistant, "reply2"),
        ];
        assert_eq!(last_user_index(&msgs), Some(3));
    }

    #[test]
    fn last_user_index_is_none_without_user_turn() {
        let msgs = vec![text(Role::System, "sys"), text(Role::Assistant, "noop")];
        assert!(last_user_index(&msgs).is_none());
    }

    #[test]
    fn reset_summary_message_carries_expected_markers() {
        let msg = build_reset_summary_message("the summary body");
        assert!(matches!(msg.role, Role::Assistant));
        if let ContentPart::Text { text } = &msg.content[0] {
            assert!(text.starts_with("[CONTEXT RESET]"));
            assert!(text.contains("the summary body"));
            assert!(text.contains("session_recall"));
        } else {
            panic!("expected text content");
        }
    }
}
