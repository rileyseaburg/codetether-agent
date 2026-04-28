use axum::{
    Json,
    body::Bytes,
    extract::{Path, State},
    http::HeaderMap,
    response::IntoResponse,
};
use serde_json::{Value, json};

use crate::state::{VoiceStubState, record};

pub async fn health() -> Json<Value> {
    Json(json!({"status":"ok","tts_model_loaded":true,"whisper_model_loaded":true}))
}

pub async fn voices() -> Json<Value> {
    Json(
        json!({"voices":[{"voice_id":"960f89fc","name":"Stub Voice","duration_seconds":1.2,"created_at":"2026-04-23T00:00:00Z"}]}),
    )
}

pub async fn transcribe(
    state: State<VoiceStubState>,
    headers: HeaderMap,
    body: Bytes,
) -> Json<Value> {
    record(state, headers, "/transcribe".into(), body).await;
    Json(json!({"transcription":"stub transcript"}))
}

pub async fn speak_voice(
    state: State<VoiceStubState>,
    Path(voice_id): Path<String>,
    headers: HeaderMap,
    body: Bytes,
) -> impl IntoResponse {
    record(state, headers, format!("/voices/{voice_id}/speak"), body).await;
    (
        [("x-job-id", "voice-job-1"), ("content-type", "audio/wav")],
        b"RIFFstubWAVE".to_vec(),
    )
}

pub async fn tts_speak(
    State(state): State<VoiceStubState>,
    headers: HeaderMap,
    body: Bytes,
) -> Json<Value> {
    record(State(state.clone()), headers, "/tts/speak".into(), body).await;
    Json(
        json!({"job_id":"stream-job-1","output_url": format!("{}/outputs/stream-job-1", state.base_url)}),
    )
}

pub async fn output_head(Path(_job_id): Path<String>) -> impl IntoResponse {
    axum::http::StatusCode::OK
}
