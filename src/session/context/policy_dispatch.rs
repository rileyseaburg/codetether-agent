//! Match-arm bodies for [`super::policy::derive_with_policy`].
//!
//! Lives in its own file purely to keep `policy.rs` under the
//! 50-line code-budget ratchet.

use std::sync::Arc;

use anyhow::Result;
use tokio::sync::mpsc;

use crate::provider::{Provider, ToolDefinition};
use crate::session::{Session, SessionEvent};

use super::helpers::DerivedContext;
use super::incremental::{DEFAULT_INCREMENTAL_BUDGET, derive_incremental};
use super::reset::derive_reset;

/// Run the [`DerivePolicy::Reset`] arm.
#[allow(clippy::too_many_arguments)]
pub(super) async fn dispatch_reset(
    session: &Session,
    provider: Arc<dyn Provider>,
    model: &str,
    system_prompt: &str,
    tools: &[ToolDefinition],
    threshold_tokens: usize,
    event_tx: Option<&mpsc::Sender<SessionEvent>>,
) -> Result<DerivedContext> {
    derive_reset(session, provider, model, system_prompt, tools, threshold_tokens, event_tx).await
}

/// Run the [`DerivePolicy::Incremental`] arm.
#[allow(clippy::too_many_arguments)]
pub(super) async fn dispatch_incremental(
    session: &Session,
    provider: Arc<dyn Provider>,
    model: &str,
    system_prompt: &str,
    tools: &[ToolDefinition],
    budget_tokens: usize,
    event_tx: Option<&mpsc::Sender<SessionEvent>>,
) -> Result<DerivedContext> {
    let budget = if budget_tokens == 0 { DEFAULT_INCREMENTAL_BUDGET } else { budget_tokens };
    derive_incremental(session, provider, model, system_prompt, tools, budget, event_tx).await
}
