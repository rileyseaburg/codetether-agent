//! Separation of visible reasoning deltas from opaque provider signatures.

use super::drain_state::DrainState;
use crate::session::SessionEvent;
use anyhow::Result;

pub(super) fn apply(
    state: &mut DrainState,
    delta: String,
    event_tx: Option<&tokio::sync::mpsc::Sender<SessionEvent>>,
) -> Result<()> {
    if crate::provider::codex_reasoning::is_signature(&delta) {
        state.reasoning_signature = Some(delta);
        if let Some(tx) = event_tx {
            let _ = tx.try_send(SessionEvent::Thinking);
        }
        return Ok(());
    }
    super::text_acc::on_thinking(&mut state.thinking, &delta, event_tx)
}

#[cfg(test)]
#[path = "reasoning_signature_tests.rs"]
mod tests;
