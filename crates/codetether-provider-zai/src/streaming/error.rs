//! Preserve application streaming error previews and terminal chunks.
use crate::StreamChunk;
pub(super) async fn chunks(response: reqwest::Response) -> Vec<StreamChunk> {
    let status = response.status();
    let text = response.text().await.unwrap_or_default();
    tracing::warn!(status = %status, body = %crate::preview::text(&text, 500), "Z.AI streaming error response");
    crate::stream_request::error_chunks(crate::error_wire::message(status, &text))
}
