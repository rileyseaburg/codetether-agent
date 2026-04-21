//! Policy dispatcher — routes to the chosen [`DerivePolicy`] implementation.

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
            derive_context(
                session,
                provider,
                model,
                system_prompt,
                tools,
                event_tx,
                None,
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
