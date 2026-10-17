//! Native-event specialization for existing timeout regression tests.
use crate::session::SessionEvent;
use std::time::Duration;
use tokio::sync::mpsc;
pub(super) async fn collect_events_with_idle_timeout(
    agent_name: &str,
    rx: &mut mpsc::Receiver<SessionEvent>,
    idle_timeout: Duration,
) -> codetether_child_events::EventLoopState {
    codetether_child_events::collect_events_with_idle_timeout::<super::host::ApplicationHost>(
        agent_name,
        rx,
        idle_timeout,
    )
    .await
}
