//! Voice REST list operation.
use axum::Json;

/// List available voices (REST bridge)
/// # Arguments
/// This operation takes no arguments.
/// # Returns
/// The existing REST compatibility response as JSON.
/// # Examples
/// ```no_run
/// # async fn example() {
/// let _ = codetether_voice_api::list_voices_rest().await;
/// # }
/// ```
pub async fn list_voices_rest() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "voices": [
            { "voice_id": "960f89fc", "name": "Riley", "language": "english" },
            { "voice_id": "puck", "name": "Puck", "language": "english" },
            { "voice_id": "charon", "name": "Charon", "language": "english" },
            { "voice_id": "kore", "name": "Kore", "language": "english" },
            { "voice_id": "fenrir", "name": "Fenrir", "language": "english" },
            { "voice_id": "aoede", "name": "Aoede", "language": "english" },
        ]
    }))
}
