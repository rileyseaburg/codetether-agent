//! Serialize an OpenAI event payload into SSE data.
pub(crate) fn event(payload: &serde_json::Value) -> axum::response::sse::Event {
    axum::response::sse::Event::default().data(payload.to_string())
}
