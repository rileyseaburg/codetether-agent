//! Session event and completion-notice channel allocation.
use tokio::sync::mpsc;
/// Channel endpoints handed from startup to the session runtime.
/// # Examples
/// ```
/// let channels = codetether_tui_startup::channels::session::<(), ()>();
/// assert_eq!(channels.event_tx.max_capacity(), 64);
/// assert_eq!(channels.notice_tx.max_capacity(), 8);
/// ```
pub struct SessionChannels<E, N> {
    /// Sender used by session event producers.
    pub event_tx: mpsc::Sender<E>,
    /// Receiver drained by the frontend.
    pub event_rx: mpsc::Receiver<E>,
    /// Sender used for session completion notices.
    pub notice_tx: mpsc::Sender<N>,
    /// Receiver drained before ownership transitions.
    pub notice_rx: mpsc::Receiver<N>,
}
/// Allocate the existing bounded event and notice channels.
/// # Returns
/// Both endpoint pairs, with capacities 64 and 8 respectively.
pub fn session<E, N>() -> SessionChannels<E, N> {
    let (event_tx, event_rx) = mpsc::channel::<E>(64);
    let (notice_tx, notice_rx) = mpsc::channel::<N>(8);
    SessionChannels {
        event_tx,
        event_rx,
        notice_tx,
        notice_rx,
    }
}
