//! Pause and cancellation checks between swarm stages.

use super::state::Run;
use crate::tui::swarm_view::SwarmEvent;
use tokio::time::{Duration, sleep};

pub(super) async fn may_continue(run: &Run<'_>) -> bool {
    let Some(control) = run.executor.control.as_ref() else {
        return true;
    };
    while control.is_paused() && !control.is_cancelled() {
        sleep(Duration::from_millis(200)).await;
    }
    if control.is_cancelled() {
        run.executor
            .try_send_event(SwarmEvent::Error("Swarm cancelled by user".into()));
        false
    } else {
        true
    }
}
