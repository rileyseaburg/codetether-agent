//! Swarm view event rx.
use crate::SwarmViewState;
use tokio::sync::mpsc;
impl SwarmViewState {
    /// Drain pending swarm events without blocking.
    ///
    /// Returns `true` when at least one event was applied.
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
                Err(mpsc::error::TryRecvError::Disconnected) => break,
            }
        }
        any
    }
}
