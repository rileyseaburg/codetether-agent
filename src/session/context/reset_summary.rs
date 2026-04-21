//! RLM summarisation for the prefix dropped by [`DerivePolicy::Reset`].

use std::sync::Arc;

use anyhow::{Context as _, Result};

use crate::provider::Message;
use crate::rlm::router::AutoProcessContext;
use crate::rlm::RlmRouter;
use crate::session::helper::error::messages_to_rlm_context;

/// RLM-summarise the prefix dropped by
/// [`derive_reset`](super::reset::derive_reset).
///
/// Extracted so the surrounding control flow stays scannable and the
/// summary prompt can be swapped for an agent-authored one in a future
/// commit.
pub(super) async fn summarise_prefix_for_reset(
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
