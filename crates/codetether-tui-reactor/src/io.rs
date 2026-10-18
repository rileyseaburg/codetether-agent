//! Borrowed event channels retained for a reactor iteration.
use tokio::sync::mpsc;

/// Event, session-notice and shutdown receivers for one iteration.
///
/// Payload types remain selected by the frontend.
///
/// # Examples
/// ```
/// let (_, mut events) = tokio::sync::mpsc::channel::<String>(1);
/// let (_, mut notices) = tokio::sync::mpsc::channel::<u8>(1);
/// let (_, mut shutdown) = tokio::sync::mpsc::channel::<()>(1);
/// let io = codetether_tui_reactor::LoopIo::new(&mut events, &mut notices, &mut shutdown);
/// assert!(io.shutdown_rx.is_closed());
/// ```
pub struct LoopIo<'a, E, N> {
    /// Session events awaiting dispatch.
    pub event_rx: &'a mut mpsc::Receiver<E>,
    /// Session lifecycle notices awaiting dispatch.
    pub notice_rx: &'a mut mpsc::Receiver<N>,
    /// Signal-driven shutdown notifications.
    pub shutdown_rx: &'a mut mpsc::Receiver<()>,
}

impl<'a, E, N> LoopIo<'a, E, N> {
    /// Borrow the three channels for one reactor iteration.
    ///
    /// # Arguments
    /// Receives session event, session notice and shutdown channels.
    /// # Returns
    /// A container retaining each exclusive borrow for its lifetime.
    /// # Examples
    /// See [`LoopIo`] for channel construction and borrowing.
    pub fn new(
        event_rx: &'a mut mpsc::Receiver<E>,
        notice_rx: &'a mut mpsc::Receiver<N>,
        shutdown_rx: &'a mut mpsc::Receiver<()>,
    ) -> Self {
        Self {
            event_rx,
            notice_rx,
            shutdown_rx,
        }
    }
}
