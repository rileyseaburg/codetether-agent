//! Error cleanup, failover scheduling and chat presentation.
use codetether_chat_model::{ChatMessage, MessageType};
use codetether_session_events::{
    SessionEvent,
    lifecycle::{Completion, Processing},
};
/// Error-specific effects over the existing lifecycle bindings.
/// # Implementors
/// Frontends preserve inbox, failover and timing state through these callbacks.
pub trait View: Processing + Completion {
    /// Settle pending inbox entries with the error.
    fn settle_error(&mut self, error: String);
    /// Complete request timing without promoting a successful turn.
    fn complete_request_timing(&mut self);
    /// Select a retry using the current model and available catalog.
    fn schedule_retry(&mut self, error: &str);
    /// Borrow chronological chat messages.
    fn messages(&mut self) -> &mut Vec<ChatMessage>;
    /// Whether new messages should scroll into view.
    fn auto_follow(&self) -> bool;
    /// Scroll to the latest message.
    fn scroll_to_bottom(&mut self);
}
/// Apply cleanup, retry scheduling and presentation for a session error.
/// # Arguments
/// * `view` — Frontend lifecycle and error bindings.
/// * `evt` — Owned runtime event.
/// # Returns
/// Consumes errors and returns unrelated events unchanged.
/// # Examples
/// ```no_run
/// # async fn example<S>(view: &mut impl codetether_chat_events::error::View, event: codetether_session_events::SessionEvent<S>) {
/// let _ = codetether_chat_events::error::handle_event(view, event).await;
/// # }
/// ```
pub async fn handle_event<S>(
    view: &mut impl View,
    evt: SessionEvent<S>,
) -> Option<SessionEvent<S>> {
    match evt {
        SessionEvent::Error(err) => handle(view, err).await,
        other => return Some(other),
    }
    None
}

async fn handle(view: &mut impl View, err: String) {
    view.processing_stopped().await;
    view.settle_error(err.clone());
    view.clear_streaming_text();
    view.complete_request_timing();
    view.clear_interlude();
    view.schedule_retry(&err);
    view.messages()
        .push(ChatMessage::new(MessageType::Error, err));
    view.set_status("Error".to_string());
    if view.auto_follow() {
        view.scroll_to_bottom();
    }
}
