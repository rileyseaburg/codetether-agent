//! Policy dispatcher — routes to the chosen [`DerivePolicy`] implementation.

use std::env;
use std::sync::Arc;

use anyhow::Result;
use tokio::sync::mpsc;

use crate::provider::ToolDefinition;
use crate::session::Session;
use crate::session::SessionEvent;
use crate::session::derive_policy::DerivePolicy;

#[path = "policy_implicit.rs"]
mod policy_implicit;

use super::derive::derive_context;
use super::helpers::DerivedContext;
use super::reset_helpers::latest_reset_marker_index;

/// Resolve the effective derivation policy for `session`.
///
/// The persisted session policy is the baseline. Operators can override it
/// process-wide via:
///
/// - `CODETETHER_CONTEXT_POLICY=legacy`
/// - `CODETETHER_CONTEXT_POLICY=reset`
/// - `CODETETHER_CONTEXT_RESET_THRESHOLD_TOKENS=<usize>`
/// - `CODETETHER_CONTEXT_RESET_THRESHOLD_FRACTION=<f64>` (of the model's
///   context window; see [`super::reset_threshold`])
///
/// When there is no explicit override and the persisted policy is still
/// legacy, a recorded `[CONTEXT RESET]` marker auto-promotes the effective
/// policy to [`DerivePolicy::Reset`] so `context_reset` markers become live
/// immediately on the next turn.
///
/// `model` scales the Reset threshold to the model's context window so a
/// 1M-context model is not compressed at 3% utilisation.
pub fn effective_policy(session: &Session, model: &str) -> DerivePolicy {
    let persisted = session.metadata.context_policy;
    let implicit = policy_implicit::resolve(policy_implicit::Inputs {
        persisted,
        has_reset_marker: latest_reset_marker_index(&session.messages).is_some(),
        reset_threshold: resolve_reset_threshold(persisted, model),
        incremental_budget: super::incremental_budget_config::resolve(persisted, model),
    });
    let Ok(raw) = env::var("CODETETHER_CONTEXT_POLICY") else {
        return implicit;
    };
    match raw.trim().to_ascii_lowercase().as_str() {
        "" => implicit,
        "legacy" => DerivePolicy::Legacy,
        "reset" => DerivePolicy::Reset {
            threshold_tokens: resolve_reset_threshold(persisted, model),
        },
        "incremental" => DerivePolicy::Incremental {
            budget_tokens: super::incremental_budget_config::resolve(persisted, model),
        },
        _ => {
            tracing::warn!(raw = %raw, "Unknown CODETETHER_CONTEXT_POLICY override; using proactive default");
            implicit
        }
    }
}

fn resolve_reset_threshold(persisted: DerivePolicy, model: &str) -> usize {
    let persisted_tokens = match persisted {
        DerivePolicy::Reset { threshold_tokens } => Some(threshold_tokens),
        DerivePolicy::Legacy
        | DerivePolicy::Incremental { .. }
        | DerivePolicy::OracleReplay { .. } => None,
    };
    super::reset_threshold::resolve_for_model(model, persisted_tokens)
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
    let mut derived = if force_keep_last.is_some() {
        derive_context(
            session,
            provider,
            model,
            system_prompt,
            tools,
            event_tx,
            force_keep_last,
        )
        .await?
    } else {
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
                super::policy_dispatch::dispatch_reset(
                    session,
                    provider,
                    model,
                    system_prompt,
                    tools,
                    threshold_tokens,
                    event_tx,
                )
                .await
            }
            DerivePolicy::Incremental { budget_tokens } => {
                super::policy_dispatch::dispatch_incremental(
                    session,
                    provider,
                    model,
                    system_prompt,
                    tools,
                    budget_tokens,
                    event_tx,
                )
                .await
            }
            // OracleReplay is evaluation-only; at runtime fall back to Legacy.
            DerivePolicy::OracleReplay { .. } => {
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
        }?
    };
    super::state_header::prepend_state_header(session, &mut derived.messages);
    Ok(derived)
}
