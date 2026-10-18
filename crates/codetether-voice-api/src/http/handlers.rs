//! HTTP extraction for voice session creation and deletion.
use super::HttpState;
use crate::CreateVoiceSessionRequest;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

pub(super) async fn create_voice_session_rest<S: HttpState>(
    State(state): State<S>,
    Json(req): Json<CreateVoiceSessionRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    crate::create_voice_session_rest(state.bus(), req).await
}

pub(super) async fn delete_voice_session_rest<S: HttpState>(
    State(state): State<S>,
    Path(room_name): Path<String>,
) -> Json<serde_json::Value> {
    crate::delete_voice_session_rest(state.bus(), Path(room_name)).await
}
