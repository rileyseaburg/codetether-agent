//! Ralph view event rx.
use crate::RalphViewState;
use tokio::sync::mpsc;
impl RalphViewState {
    /// Drain any pending events from [`Self::event_rx`] without blocking.
    ///
    /// Returns `true` when at least one event was applied, so callers can
    /// cheap-gate a redraw. When the upstream sender is dropped, the
    /// receiver is closed automatically via `None` from `try_recv`.
    pub fn drain_events(&mut self) -> bool {
        let Some(mut rx) = self.event_rx.take() else {
            return false;
        };
        let mut any = false;
        loop {
            match rx.try_recv() {
                Ok(evt) => {
                    any = true;
                    self.handle_event(evt);
                }
                Err(mpsc::error::TryRecvError::Empty) => {
                    self.event_rx = Some(rx);
                    break;
                }
                Err(mpsc::error::TryRecvError::Disconnected) => {
                    // The Ralph task finished or panicked. Leave the
                    // slot None so subsequent ticks are O(1) no-ops.
                    break;
                }
            }
        }
        any
    }
}
