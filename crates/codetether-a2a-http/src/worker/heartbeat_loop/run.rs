//! Heartbeat cadence, failure accounting and renewal ordering.
use super::*;
pub(super) async fn run(
    client: Client,
    server: String,
    token: Option<String>,
    heartbeat_state: HeartbeatState,
    processing: Arc<Mutex<HashSet<String>>>,
    cognition_config: CognitionHeartbeatConfig,
    task_progress: Arc<Mutex<TaskProgressState>>,
) {
    let mut failures = 0u32;
    let mut cognition_disabled_until: Option<Instant> = None;
    let (persistent, lease) = configuration::persistent_settings();
    let mut interval = tokio::time::interval(Duration::from_secs(30));
    interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
    loop {
        interval.tick().await;
        let active_count = processing.lock().await.len();
        let tick = build_heartbeat_payload(
            &client,
            &heartbeat_state,
            &cognition_config,
            &task_progress,
            active_count,
            cognition_disabled_until,
        )
        .await;
        submit::submit(
            &client,
            &server,
            &token,
            &heartbeat_state.worker_id,
            &tick,
            &mut failures,
            &mut cognition_disabled_until,
        )
        .await;
        lease::renew(
            &client,
            &server,
            &token,
            &heartbeat_state.worker_id,
            persistent,
            tick.task_progress_payload,
            lease,
        )
        .await;
        failures::report(&heartbeat_state.worker_id, &mut failures);
    }
}
