//! Shared event bus state; clones retain the same transport handles.
use crate::{SessionEvent, SharedSink};
use tokio::sync::{broadcast, mpsc};
/// Event bus parameterized by the application's snapshot type.
/// # Examples
/// ```
/// let bus = codetether_session_bus::SessionBus::<()>::new(8);
/// assert_eq!(bus.subscriber_count(), 0);
/// ```
#[derive(Clone)]
pub struct SessionBus<S: Clone> {
    pub(crate) tx: broadcast::Sender<SessionEvent<S>>,
    pub(crate) sink: SharedSink<S>,
    pub(crate) legacy: Option<mpsc::Sender<SessionEvent<S>>>,
}
