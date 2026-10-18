//! Borrowed event inputs for one selection.
use crate::Timers;
use tokio::sync::mpsc::Receiver;
/// Streams, channels and timers polled by [crate::select].
/// # Examples
/// ```no_run
/// # async fn example() {
/// let mut reader = futures::stream::pending::<()>();
/// let (_, mut shutdown) = tokio::sync::mpsc::channel(1);
/// let (_, mut events) = tokio::sync::mpsc::channel::<()>(1);
/// let (_, mut notices) = tokio::sync::mpsc::channel::<()>(1);
/// let mut timers = codetether_tui_reactor::Timers::new(std::time::Duration::from_secs(60));
/// let _sources = codetether_tui_reactor::Sources { reader: &mut reader, shutdown: &mut shutdown, events: &mut events, notices: &mut notices, timers: &mut timers };
/// # }
/// ```
pub struct Sources<'a, S, E, N> {
    /// Terminal input stream, polled first.
    pub reader: &'a mut S,
    /// Shutdown signal receiver.
    pub shutdown: &'a mut Receiver<()>,
    /// Session event receiver.
    pub events: &'a mut Receiver<E>,
    /// Background notice receiver.
    pub notices: &'a mut Receiver<N>,
    /// Timer intervals used by this selection.
    pub timers: &'a mut Timers,
}
