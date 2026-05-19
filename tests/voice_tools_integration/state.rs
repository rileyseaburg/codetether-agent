use std::sync::Arc;

use axum::{body::Bytes, extract::State, http::HeaderMap};
use tokio::sync::Mutex;

pub type RequestLog = Arc<Mutex<Vec<(String, String, String)>>>;

#[derive(Clone, Default)]
pub struct VoiceStubState {
    pub base_url: String,
    pub requests: RequestLog,
}

pub async fn record(
    State(state): State<VoiceStubState>,
    headers: HeaderMap,
    path: String,
    body: Bytes,
) {
    let content_type = headers
        .get("content-type")
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default()
        .to_string();
    state.requests.lock().await.push((
        path,
        content_type,
        String::from_utf8_lossy(&body).to_string(),
    ));
}
