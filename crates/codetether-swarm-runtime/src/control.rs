//! Pause and cancellation checks between swarm stages.

use super::state::Run;
use codetether_swarm_core::SwarmEvent;

pub(super) async fn may_continue(run: &Run<'_, impl crate::Host>) -> bool {
    let Some(control) = run.executor.control() else {
        return true;
    };
    match control.ready().await {
        Ok(()) => true,
        Err(error) => {
            run.executor
                .try_send_event(SwarmEvent::Error(format!("Swarm stopped: {error}")));
            false
        }
    }
}
