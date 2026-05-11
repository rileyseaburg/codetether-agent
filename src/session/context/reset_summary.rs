//! RLM summarisation for the prefix dropped by [`DerivePolicy::Reset`].

use std::sync::Arc;

use anyhow::{Context as _, Result};
use tokio::sync::mpsc;
use uuid::Uuid;

use crate::provider::Message;
use crate::rlm::RlmRouter;
use crate::rlm::router::AutoProcessContext;
use crate::session::SessionEvent;
use crate::session::helper::error::messages_to_rlm_context;
use crate::session::index_produce::summary_text::bounded_summary;

/// Broadcast capacity for the throwaway [`SessionBus`](crate::session::SessionBus)
/// that bridges `event_tx` into the RLM router. A single reset emits
/// a handful of progress/completion events; 16 is plenty of headroom.
const RESET_BUS_CAPACITY: usize = 16;

/// RLM-summarise the prefix dropped by
/// [`derive_reset`](super::reset::derive_reset).
///
/// When `event_tx` is provided, an ephemeral [`SessionBus`](crate::session::SessionBus)
/// is bridged onto it so the RLM router's `RlmProgress`/`RlmComplete`
/// events surface in the TUI. The Reset policy does not emit
/// `CompactionStarted`/`Completed`, so a fresh `trace_id` is minted
/// here purely to correlate this run's own progress + completion
/// events with each other — not with any parent compaction trace.
#[allow(clippy::too_many_arguments)]
pub(super) async fn summarise_prefix_for_reset(
    prefix: &[Message],
    session_id: &str,
    provider: Arc<dyn crate::provider::Provider>,
    model: &str,
    rlm_config: &crate::rlm::RlmConfig,
    subcall_provider: Option<Arc<dyn crate::provider::Provider>>,
    subcall_model: Option<String>,
    event_tx: Option<&mpsc::Sender<SessionEvent>>,
) -> Result<String> {
    let context = messages_to_rlm_context(prefix);
    let bus = event_tx
        .cloned()
        .map(|tx| crate::session::SessionBus::new(RESET_BUS_CAPACITY).with_legacy_mpsc(tx));
    let trace_id = Uuid::new_v4();
    let auto_ctx = AutoProcessContext {
        tool_id: "context_reset",
        tool_args: serde_json::json!({"policy": "reset"}),
        session_id,
        abort: None,
        on_progress: None,
        provider,
        model: model.to_string(),
        bus,
        trace_id: Some(trace_id),
        subcall_provider,
        subcall_model,
    };
    let result = RlmRouter::auto_process(&context, auto_ctx, rlm_config)
        .await
        .context("RLM summarisation for DerivePolicy::Reset failed")?;
    bounded_summary(result, 512)
}
