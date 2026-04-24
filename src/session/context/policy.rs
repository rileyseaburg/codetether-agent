//! Policy dispatcher — routes to the chosen [`DerivePolicy`] implementation.

use std::env;
use std::sync::Arc;

use anyhow::Result;
use tokio::sync::mpsc;

use crate::provider::ToolDefinition;
use crate::session::Session;
use crate::session::SessionEvent;
use crate::session::derive_policy::DerivePolicy;

use super::derive::derive_context;
use super::helpers::DerivedContext;
use super::reset::derive_reset;
use super::reset_helpers::latest_reset_marker_index;

const DEFAULT_RESET_THRESHOLD_TOKENS: usize = 32_000;

/// Resolve the effective derivation policy for `session`.
///
/// The persisted session policy is the baseline. Operators can override it
/// process-wide via:
///
/// - `CODETETHER_CONTEXT_POLICY=legacy`
/// - `CODETETHER_CONTEXT_POLICY=reset`
/// - `CODETETHER_CONTEXT_RESET_THRESHOLD_TOKENS=<usize>`
///
/// When there is no explicit override and the persisted policy is still
/// legacy, a recorded `[CONTEXT RESET]` marker auto-promotes the effective
/// policy to [`DerivePolicy::Reset`] so `context_reset` markers become live
/// immediately on the next turn.
pub fn effective_policy(session: &Session) -> DerivePolicy {
    let persisted = session.metadata.context_policy;
    let Ok(raw) = env::var("CODETETHER_CONTEXT_POLICY") else {
        if matches!(persisted, DerivePolicy::Legacy)
            && latest_reset_marker_index(&session.messages).is_some()
        {
            return DerivePolicy::Reset {
                threshold_tokens: resolve_reset_threshold(persisted),
            };
        }
        return persisted;
    };
    let normalized = raw.trim().to_ascii_lowercase();
    match normalized.as_str() {
        "" => persisted,
        "legacy" => DerivePolicy::Legacy,
        "reset" => DerivePolicy::Reset {
            threshold_tokens: resolve_reset_threshold(persisted),
        },
        _ => {
            tracing::warn!(raw = %raw, "Unknown CODETETHER_CONTEXT_POLICY override; using persisted session policy");
            persisted
        }
    }
}

fn resolve_reset_threshold(persisted: DerivePolicy) -> usize {
    let default_threshold = match persisted {
        DerivePolicy::Reset { threshold_tokens } => threshold_tokens,
        DerivePolicy::Legacy => DEFAULT_RESET_THRESHOLD_TOKENS,
    };
    env::var("CODETETHER_CONTEXT_RESET_THRESHOLD_TOKENS")
        .ok()
        .and_then(|value| value.trim().parse::<usize>().ok())
        .unwrap_or(default_threshold)
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
/// * `force_keep_last` — When `Some(n)`, bypass policy selection and fall
///   back to the legacy keep-last derivation used for prompt-too-long
///   recovery.
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
    force_keep_last: Option<usize>,
) -> Result<DerivedContext> {
    if force_keep_last.is_some() {
        return derive_context(
            session,
            provider,
            model,
            system_prompt,
            tools,
            event_tx,
            force_keep_last,
        )
        .await;
    }
    match policy {
        DerivePolicy::Legacy => {
            derive_context(
                session,
                provider,
                model,
                system_prompt,
                tools,
                event_tx,
                force_keep_last,
            )
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
