//! Lifecycle event routing and ordered state transitions.
use super::{Host, Session};
use crate::SessionEvent;
/// Apply thinking, completion or restored-session lifecycle transitions.
/// # Arguments
/// * `host` — Frontend worker, timing, completion and session bindings.
/// * `evt` — Owned event received from the runtime.
/// # Returns
/// None for handled lifecycle events; unrelated events pass through unchanged.
/// # Examples
/// ```no_run
/// # async fn example<H: codetether_session_events::lifecycle::Host>(host: &mut H) {
/// let _ = codetether_session_events::lifecycle::handle_event(host, codetether_session_events::SessionEvent::Thinking).await;
/// # }
/// ```
pub async fn handle_event<H: Host>(
    host: &mut H,
    evt: SessionEvent<H::Session>,
) -> Option<SessionEvent<H::Session>> {
    match evt {
        SessionEvent::Thinking => thinking(host).await,
        SessionEvent::Done => done(host).await,
        SessionEvent::SessionSync(updated) => sync(host, updated),
        other => return Some(other),
    }
    None
}

async fn thinking(host: &mut impl Host) {
    host.processing_started().await;
    if !host.request_timing_started() {
        host.begin_request_timing();
    }
    if !host.streaming_started() {
        host.begin_streaming();
    }
    host.set_status("Thinking…".to_string());
}

async fn done(host: &mut impl Host) {
    host.processing_stopped().await;
    // Done precedes return of the owned session. Settle replies only after
    // the completion notice restores that final session.
    host.clear_streaming_text();
    host.complete_turn_timing();
    host.clear_streaming_start();
    host.clear_interlude();
    // NOTE: do NOT reset main_watchdog_restart_count here. The count must
    // survive a completed turn so a pathological "completes-then-immediately-
    // stalls" storm still climbs to WATCHDOG_MAX_RESTARTS instead of being
    // zeroed on every completion. The count is reset only when a genuine new
    // user prompt is dispatched (see dispatch_prompt).
    // Clear any watchdog notice now that a turn has completed; otherwise the
    // retry guard (notification.is_some() && !processing) keeps resubmitting the
    // same prompt forever.
    host.clear_watchdog_notification();
    host.clear_inflight_prompt();
    host.set_status("Ready".to_string());
}

fn sync<H: Host>(host: &mut H, updated: Box<H::Session>) {
    host.restore(*updated);
    if let Some(session) = host.current_session() {
        session.attach_bus();
        let id = session.session_id();
        host.set_session_id(id);
    }
}
