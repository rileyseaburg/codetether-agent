//! Streaming event collection for sub-agent replies.
//!
//! This module reads session events until completion or timeout and updates the
//! event-loop state incrementally.
//!
//! # Examples
//!
//! ```ignore
//! let state = collect_events(&mut rx).await;
//! ```

use super::state::EventLoopState;
use crate::session::SessionEvent;
use serde_json::json;
use tokio::sync::mpsc;

/// Collects streamed session events until completion or timeout.
///
/// # Examples
///
/// ```ignore
/// let state = collect_events(&mut rx).await;
/// ```
pub(super) async fn collect_events(rx: &mut mpsc::Receiver<SessionEvent>) -> EventLoopState {
    let mut state = EventLoopState::default();
    let timeout_fut = tokio::time::sleep(std::time::Duration::from_secs(300));
    tokio::pin!(timeout_fut);
    while !state.done {
        tokio::select! {
            res = rx.recv() => handle_recv_result(res, &mut state),
            _ = &mut timeout_fut => {
                state.error = Some("Agent timed out after 5 minutes".into());
                state.done = true;
            }
        }
    }
    state
}

fn handle_recv_result(result: Option<SessionEvent>, state: &mut EventLoopState) {
    match result {
        Some(event) => handle_event(event, state),
        None => state.done = true,
    }
}

fn handle_event(event: SessionEvent, state: &mut EventLoopState) {
    match event {
        SessionEvent::TextComplete(t) => state.response.push_str(&t),
        SessionEvent::ThinkingComplete(t) => state.thinking.push_str(&t),
        SessionEvent::ToolCallComplete {
            name,
            output,
            success,
            duration_ms: _,
        } => state.tools.push(json!({
            "tool": name,
            "success": success,
            "output_preview": super::super::text::truncate_preview(&output, 200),
        })),
        SessionEvent::Error(e) => {
            state.response.push_str(&format!("\n[Error: {e}]"));
            state.error = Some(e);
        }
        SessionEvent::Done => state.done = true,
        _ => {}
    }
}
