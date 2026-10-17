//! Pause and cancellation checks between swarm stages.

use super::state::Run;
use crate::tui::swarm_view::SwarmEvent;

pub(super) async fn may_continue(run: &Run<'_>) -> bool {
    let Some(control) = run.executor.control.as_ref() else {
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
