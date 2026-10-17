//! try receive operations for the shared bus runtime.
use super::*;
impl BusHandle {
    /// Try to receive without blocking. Returns `None` if no message is
    /// queued.
    pub fn try_recv(&mut self) -> Option<BusEnvelope> {
        loop {
            match self.rx.try_recv() {
                Ok(env) => return Some(env),
                Err(broadcast::error::TryRecvError::Lagged(n)) => {
                    tracing::warn!(
                        agent_id = %self.agent_id,
                        skipped = n,
                        "Bus handle lagged (try_recv), skipping"
                    );
                    continue;
                }
                Err(broadcast::error::TryRecvError::Empty)
                | Err(broadcast::error::TryRecvError::Closed) => return None,
            }
        }
    }
}
