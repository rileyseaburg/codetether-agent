//! Task-channel readiness response.
use super::*;

/// Readiness check - returns OK when the worker can accept task notifications
pub(super) async fn ready(State(state): State<WorkerServerState>) -> (StatusCode, String) {
    if state.is_ready().await {
        (StatusCode::OK, "ready".to_string())
    } else {
        (
            StatusCode::SERVICE_UNAVAILABLE,
            "task notification channel not ready".to_string(),
        )
    }
}
