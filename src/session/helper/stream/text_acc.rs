//! Text and thinking accumulation helpers for the stream collector.
//!
//! Both buffers are size-capped via
//! [`stream_caps`](crate::session::helper::stream_caps); text deltas
//! additionally forward snapshot previews to the UI event channel.

use crate::session::SessionEvent;
use anyhow::Result;

/// Append a text delta and forward a capped snapshot to `event_tx`.
///
/// # Errors
///
/// Returns an error when the accumulated text would exceed the cap.
pub(super) async fn on_text(
    text: &mut String,
    delta: &str,
    event_tx: Option<&tokio::sync::mpsc::Sender<SessionEvent>>,
) -> Result<()> {
    if delta.is_empty() {
        return Ok(());
    }
    super::super::stream_caps::ensure_text_room(text.len(), delta.len())?;
    text.push_str(delta);
    if let Some(tx) = event_tx {
        let to_send = super::super::stream_caps::snapshot_for_send(text);
        let _ = tx.send(SessionEvent::TextChunk(to_send)).await;
    }
    Ok(())
}

/// Append a thinking/reasoning delta and emit a nonblocking activity signal.
///
/// # Errors
///
/// Returns an error when the accumulated thinking text would exceed the cap.
pub(super) fn on_thinking(
    thinking: &mut String,
    delta: &str,
    event_tx: Option<&tokio::sync::mpsc::Sender<SessionEvent>>,
) -> Result<()> {
    if let Some(tx) = event_tx {
        let _ = tx.try_send(SessionEvent::Thinking);
    }
    if delta.is_empty() {
        return Ok(());
    }
    super::super::stream_caps::ensure_text_room(thinking.len(), delta.len())?;
    thinking.push_str(delta);
    Ok(())
}
