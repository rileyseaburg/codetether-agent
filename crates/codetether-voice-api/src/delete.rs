//! Voice REST delete operation.
use axum::Json;
use axum::extract::Path;

/// Delete a voice session (REST bridge)
/// # Arguments
/// * `bus` - Shared bus used to publish session notifications.
/// * `room_name` - Requested room identifier from the route.
/// # Returns
/// The existing REST compatibility response as JSON.
/// # Examples
/// ```no_run
/// # async fn example(bus: &std::sync::Arc<codetether_bus_core::AgentBus>) {
/// let _ = codetether_voice_api::delete_voice_session_rest(bus, axum::extract::Path("room".into())).await;
/// # }
/// ```
pub async fn delete_voice_session_rest(
    bus: &std::sync::Arc<codetether_bus_core::AgentBus>,
    Path(room_name): Path<String>,
) -> Json<serde_json::Value> {
    let handle = bus.handle("voice-rest");
    handle.send_voice_session_ended(&room_name, "user_ended");
    Json(serde_json::json!({ "status": "deleted" }))
}
