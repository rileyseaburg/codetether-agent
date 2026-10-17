//! Report consecutive heartbeat failures without stopping the worker.
pub(super) fn report(worker_id: &str, failures: &mut u32) {
    if *failures >= 3 {
        tracing::error!(target: "codetether_agent::a2a::worker::heartbeat_loop", worker_id = %worker_id, failures = *failures, "Heartbeat failed 3 consecutive times - worker will continue running and attempt reconnection via SSE loop");
        *failures = 0;
    }
}
