//! Text and thinking accumulation helpers for the stream collector.
//!
//! Both buffers are size-capped via
//! [`stream_caps`](codetether_stream_bounds); text deltas
//! additionally forward snapshot previews to the UI event channel.

use anyhow::Result;
use codetether_session_events::SessionEvent;

/// Append a text delta and forward a capped snapshot to `event_tx`.
///
/// # Errors
///
/// Returns an error when the accumulated text would exceed the cap.
pub(super) async fn on_text<S>(
    text: &mut String,
    delta: &str,
    event_tx: Option<&tokio::sync::mpsc::Sender<SessionEvent<S>>>,
) -> Result<()> {
    if delta.is_empty() {
        return Ok(());
    }
    codetether_stream_bounds::ensure_text_room(text.len(), delta.len())?;
    text.push_str(delta);
    if let Some(tx) = event_tx {
        let to_send = codetether_stream_bounds::snapshot_for_send(text);
        let _ = tx.send(SessionEvent::TextChunk(to_send)).await;
    }
    Ok(())
}

/// Append readable reasoning and emit a bounded cumulative preview without blocking.
///
/// # Errors
///
/// Returns an error when the accumulated thinking text would exceed the cap.
pub(super) fn on_thinking<S>(
    thinking: &mut String,
    delta: &str,
    event_tx: Option<&tokio::sync::mpsc::Sender<SessionEvent<S>>>,
) -> Result<()> {
    if delta.is_empty() {
        if let Some(tx) = event_tx {
            let _ = tx.try_send(SessionEvent::Thinking);
        }
        return Ok(());
    }
    codetether_stream_bounds::ensure_text_room(thinking.len(), delta.len())?;
    thinking.push_str(delta);
    if let Some(tx) = event_tx {
        let preview = codetether_stream_bounds::snapshot_for_send(thinking);
        let _ = tx.try_send(SessionEvent::ThinkingChunk(preview));
    }
    Ok(())
}
