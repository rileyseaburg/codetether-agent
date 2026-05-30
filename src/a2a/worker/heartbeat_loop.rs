//! Worker heartbeat loop.
mod heartbeat_payload;
mod heartbeat_post;
use reqwest::Client;
use std::{collections::HashSet, sync::Arc, time::Duration};
use tokio::{sync::Mutex, task::JoinHandle, time::Instant};
use super::{CognitionHeartbeatConfig, HeartbeatState, persistent_worker_enabled,
    persistent_worker_lease_seconds, send_extended_task_heartbeat};
use heartbeat_payload::build_heartbeat_payload;
use heartbeat_post::post_heartbeat;
pub fn start_heartbeat(
    client: Client, server: String, token: Option<String>,
    heartbeat_state: HeartbeatState, processing: Arc<Mutex<HashSet<String>>>,
    cognition_config: CognitionHeartbeatConfig,
    task_progress: Arc<Mutex<super::task_timeline::TaskProgressState>>,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        let mut failures = 0u32;
        let mut cognition_disabled_until: Option<Instant> = None;
        let persistent = persistent_worker_enabled();
        let lease = persistent_worker_lease_seconds();
        let mut interval = tokio::time::interval(Duration::from_secs(30));
        interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
        loop {
            interval.tick().await;
            let active_count = processing.lock().await.len();
            let tick = build_heartbeat_payload(
                &client, &heartbeat_state, &cognition_config,
                &task_progress, active_count, cognition_disabled_until,
            ).await;
            match post_heartbeat(&client, &server, &token, &heartbeat_state.worker_id, &tick.payload).await {
                true => failures = 0,
                false if tick.cognition_included
                    && post_heartbeat(&client, &server, &token, &heartbeat_state.worker_id, &tick.base_payload).await =>
                { cognition_disabled_until = Some(Instant::now() + Duration::from_secs(300)); failures = 0; }
                false => failures += 1,
            }
            if persistent && let Some(progress_json) = tick.task_progress_payload
                && let Err(error) = send_extended_task_heartbeat(
                    &client, &server, &token, &heartbeat_state.worker_id, progress_json, lease,
                ).await {
                tracing::warn!(worker_id = %heartbeat_state.worker_id, error = %error, "Extended task heartbeat failed");
            }
            if failures >= 3 {
                tracing::error!(worker_id = %heartbeat_state.worker_id, failures, "Heartbeat failed 3 consecutive times - worker will continue running and attempt reconnection via SSE loop");
                failures = 0;
            }
        }
    })
}
