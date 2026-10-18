//! Voice REST get operation.
use axum::Json;
use axum::extract::Path;

/// Get a voice session (REST bridge)
/// # Arguments
/// * `room_name` - Requested room identifier from the route.
/// # Returns
/// The existing REST compatibility response as JSON.
/// # Examples
/// ```no_run
/// # async fn example() {
/// let _ = codetether_voice_api::get_voice_session_rest(axum::extract::Path("room".into())).await;
/// # }
/// ```
pub async fn get_voice_session_rest(Path(room_name): Path<String>) -> Json<serde_json::Value> {
    let livekit_url = std::env::var("LIVEKIT_URL").unwrap_or_default();
    Json(serde_json::json!({
        "room_name": room_name,
        "state": "active",
        "agent_state": "listening",
        "access_token": "",
        "livekit_url": livekit_url,
    }))
}
