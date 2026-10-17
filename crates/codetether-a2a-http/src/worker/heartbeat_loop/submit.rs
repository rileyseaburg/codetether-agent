//! Submit a full heartbeat and apply cognition fallback when rejected.
use super::*;
pub(super) async fn submit(
    client: &Client,
    server: &str,
    token: &Option<String>,
    worker_id: &str,
    tick: &HeartbeatPayload,
    failures: &mut u32,
    cognition_disabled_until: &mut Option<Instant>,
) {
    match post_heartbeat(&client, &server, &token, worker_id, &tick.payload).await {
        true => *failures = 0,
        false
            if tick.cognition_included
                && post_heartbeat(&client, &server, &token, worker_id, &tick.base_payload)
                    .await =>
        {
            *cognition_disabled_until = Some(Instant::now() + Duration::from_secs(300));
            *failures = 0;
        }
        false => *failures += 1,
    }
}
