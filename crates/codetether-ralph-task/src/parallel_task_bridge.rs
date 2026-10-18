//! Build a single-agent event bridge while preserving its join handle.
use super::parallel_task::ParallelTask;
use super::*;
impl<H: Host> ParallelTask<H> {
    pub(crate) fn event_bridge(
        &self,
    ) -> (
        Option<mpsc::Sender<SwarmEvent>>,
        Option<tokio::task::JoinHandle<()>>,
    ) {
        // Create event bridge for this story's sub-agent (used for single-agent mode)
        let (bridge_tx, _bridge_handle) = if !self.relay_enabled {
            if let Some(ref tx) = self.ralph_tx {
                let (btx, handle) = codetether_ralph_agent::event_bridge(tx, self.story.id.clone());
                (Some(btx), Some(handle))
            } else {
                (None, None)
            }
        } else {
            (None, None)
        };

        (bridge_tx, _bridge_handle)
    }
}
