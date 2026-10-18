//! Send application events and build initial story presentation state.
use super::*;
impl<H: Host> RalphLoop<H> {
    /// Non-blocking send of a Ralph event
    pub(crate) fn try_send_event(&self, event: RalphEvent) {
        if let Some(ref tx) = self.event_tx {
            let _ = tx.try_send(event);
        }
    }
}
