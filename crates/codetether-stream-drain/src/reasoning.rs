//! Separation of visible reasoning deltas from opaque provider signatures.

use super::drain_state::DrainState;
use anyhow::Result;
use codetether_session_events::SessionEvent;

pub(super) fn apply<S>(
    state: &mut DrainState,
    delta: String,
    event_tx: Option<&tokio::sync::mpsc::Sender<SessionEvent<S>>>,
) -> Result<()> {
    if codetether_codex_protocol::reasoning::is_signature(&delta) {
        state.reasoning_signature = Some(delta);
        if let Some(tx) = event_tx {
            let _ = tx.try_send(SessionEvent::Thinking);
        }
        return Ok(());
    }
    super::text_acc::on_thinking(&mut state.thinking, &delta, event_tx)
}
