//! HTTP request half of the Bedrock converse-stream adapter.
//!
//! Owns URL construction, SigV4/bearer dispatch, and non-200 error mapping;
//! the byte-to-chunk pump lives in [`super::pump`].

use super::pump::event_chunk_stream;
use crate::provider::StreamChunk;
use crate::provider::bedrock::BedrockProvider;
use anyhow::{Context, Result};

#[path = "refresh_retry.rs"]
mod refresh_retry;

impl BedrockProvider {
    /// POST to `/model/{id}/converse-stream` and yield `StreamChunk`s as
    /// eventstream frames arrive.
    ///
    /// # Errors
    ///
    /// Returns [`anyhow::Error`] if the initial HTTP request fails or the
    /// server responds non-200. Per-frame decode errors are emitted as
    /// [`StreamChunk::Error`] but do not abort the stream.
    pub(in crate::provider::bedrock) async fn converse_stream(
        &self,
        model_id: &str,
        body: Vec<u8>,
    ) -> Result<futures::stream::BoxStream<'static, StreamChunk>> {
        let url = format!("{}/model/{}/converse-stream", self.base_url(), model_id);
        tracing::debug!("Bedrock stream URL: {}", url);

        let mut response = self
            .send_request("POST", &url, Some(&body), "bedrock")
            .await?;

        // On a mid-session auth failure, attempt a silent token refresh and
        // retry once so an active TUI recovers without restarting.
        if refresh_retry::is_auth_failure(response.status())
            && refresh_retry::try_refresh(self).await
        {
            response = self
                .send_request("POST", &url, Some(&body), "bedrock")
                .await?;
        }

        let status = response.status();
        if !status.is_success() {
            let text = response.text().await.context("failed to read error body")?;
            let body = crate::util::truncate_bytes_safe(&text, 500);
            anyhow::bail!(super::auth_error::message(status, body));
        }

        Ok(event_chunk_stream(response))
    }
}
