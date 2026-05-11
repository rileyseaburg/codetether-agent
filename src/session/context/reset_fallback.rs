//! Early-return paths for [`super::reset::derive_reset`] when there's
//! nothing left to RLM-summarise.

use std::sync::Arc;

use anyhow::Result;
use tokio::sync::mpsc;

use crate::provider::{Message, Provider, ToolDefinition};
use crate::session::helper::experimental;
use crate::session::{ResidencyLevel, Session, SessionEvent};

use super::derive::derive_context;
use super::helpers::DerivedContext;

/// Either return the current `messages` as a Reset context (if we
/// already produced provenance), or fall through to the legacy
/// derivation entirely.
#[allow(clippy::too_many_arguments)]
pub(super) async fn no_active_tail_fallback(
    session: &Session,
    provider: Arc<dyn Provider>,
    model: &str,
    system_prompt: &str,
    tools: &[ToolDefinition],
    event_tx: Option<&mpsc::Sender<SessionEvent>>,
    mut messages: Vec<Message>,
    dropped_ranges: Vec<(usize, usize)>,
    provenance: Vec<String>,
    origin_len: usize,
) -> Result<DerivedContext> {
    if !provenance.is_empty() {
        experimental::pairing::repair_orphans(&mut messages);
        return Ok(DerivedContext {
            resolutions: vec![ResidencyLevel::Full; messages.len()],
            dropped_ranges,
            provenance,
            messages,
            origin_len,
            compressed: true,
        });
    }
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
