//! Owned event selected from the reactor inputs.
/// One event selected in terminal-first priority order.
/// # Examples
/// ```
/// let event: codetether_tui_reactor::Ready<(), (), ()> = codetether_tui_reactor::Ready::Shutdown;
/// assert!(matches!(event, codetether_tui_reactor::Ready::Shutdown));
/// ```
pub enum Ready<T, E, N> {
    /// Terminal stream item, including end-of-stream.
    Terminal(Option<T>),
    /// Shutdown signal received.
    Shutdown,
    /// Session event received.
    Session(E),
    /// Background notice received.
    Notice(N),
    /// Watchdog interval elapsed.
    Watchdog,
    /// Audit refresh interval elapsed.
    Audit,
    /// General UI interval elapsed.
    Tick,
}
