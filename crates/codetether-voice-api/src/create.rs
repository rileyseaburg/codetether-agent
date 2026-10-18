//! Voice REST create operation.
use crate::CreateVoiceSessionRequest;
use axum::Json;
use axum::http::StatusCode;

// ── Voice REST bridge ───────────────────────────────────────────────────
// Bridges the REST /v1/voice/* surface the dashboard expects to the
// existing gRPC VoiceService implementation via the AgentBus.

/// Create a voice session (REST bridge for VoiceService::CreateVoiceSession)
/// # Arguments
/// * `bus` - Shared bus used to publish session notifications.
/// * `req` - Optional voice and mode settings.
/// # Returns
/// The existing REST compatibility response as JSON.
/// # Examples
/// ```no_run
/// # async fn example(bus: &std::sync::Arc<codetether_bus_core::AgentBus>, req: codetether_voice_api::CreateVoiceSessionRequest) {
/// let _ = codetether_voice_api::create_voice_session_rest(bus, req).await;
/// # }
/// ```
pub async fn create_voice_session_rest(
    bus: &std::sync::Arc<codetether_bus_core::AgentBus>,
    req: CreateVoiceSessionRequest,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let voice_id = req.voice.unwrap_or_else(|| "960f89fc".to_string());
    let room_name = format!("voice-{}", uuid::Uuid::new_v4());

    // Publish session started event to bus
    let handle = bus.handle("voice-rest");
    handle.send_voice_session_started(&room_name, &voice_id);

    let livekit_url = std::env::var("LIVEKIT_URL").unwrap_or_default();

    Ok(Json(serde_json::json!({
        "room_name": room_name,
        "voice": voice_id,
        "mode": req.mode.unwrap_or_else(|| "chat".to_string()),
        "access_token": "",
        "livekit_url": livekit_url,
        "expires_at": (chrono::Utc::now() + chrono::Duration::hours(1)).to_rfc3339(),
    })))
}
