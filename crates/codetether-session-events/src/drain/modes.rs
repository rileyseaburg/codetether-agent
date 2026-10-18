//! Standard bounded delivery modes for rendering and completion notices.
use super::{Handler, run};
use crate::SessionEvent;
use tokio::sync::mpsc;

/// Drain one rendering batch without waiting for new events.
/// # Arguments
/// * `handler` - Applies each delivered event.
/// * `event_rx` - Supplies queued session events.
/// # Returns
/// Completes after at most [super::BATCH_LIMIT] events and flushes pending text.
/// # Examples
/// ```no_run
/// # async fn example<S: Send>(h: &mut impl codetether_session_events::drain::Handler<S>, rx: &mut tokio::sync::mpsc::Receiver<codetether_session_events::SessionEvent<S>>) {
/// codetether_session_events::drain::batch(h, rx).await;
/// # }
/// ```
pub async fn batch<S: Send>(
    handler: &mut impl Handler<S>,
    event_rx: &mut mpsc::Receiver<SessionEvent<S>>,
) {
    run(handler, event_rx, super::BATCH_LIMIT, false).await;
}

/// Flush queued events before applying a completion or ownership notice.
/// # Arguments
/// * `handler` - Applies each delivered event.
/// * `event_rx` - Supplies queued session events.
/// # Returns
/// Completes at [super::NOTICE_LIMIT] events or the first completion event,
/// flushing pending text before the caller handles the notice.
/// # Examples
/// ```no_run
/// # async fn example<S: Send>(h: &mut impl codetether_session_events::drain::Handler<S>, rx: &mut tokio::sync::mpsc::Receiver<codetether_session_events::SessionEvent<S>>) {
/// codetether_session_events::drain::before_notice(h, rx).await;
/// # }
/// ```
pub async fn before_notice<S: Send>(
    handler: &mut impl Handler<S>,
    event_rx: &mut mpsc::Receiver<SessionEvent<S>>,
) {
    run(handler, event_rx, super::NOTICE_LIMIT, true).await;
}
