//! HTTP request half of the Bedrock converse-stream adapter.
//!
//! Owns URL construction, SigV4/bearer dispatch, and non-200 error mapping;
//! the byte-to-chunk pump lives in the protocol crate.

use crate::{BedrockClient, recovery as auth_recover};
use anyhow::{Context, Result};
use codetether_bedrock_protocol::stream::event_chunk_stream;
use codetether_provider_core::StreamChunk;

impl BedrockClient<'_> {
    /// POST to `/model/{id}/converse-stream` and yield `StreamChunk`s as
    /// eventstream frames arrive.
    ///
    /// # Errors
    ///
    /// Returns [`anyhow::Error`] if the initial HTTP request fails or the
    /// server responds non-200. Per-frame decode errors are emitted as
    /// [`StreamChunk::Error`] but do not abort the stream.
    ///
    /// # Arguments
    /// Accepts the resolved model ID, serialized body, and application recovery handler.
    ///
    /// # Returns
    /// The completion result or its provider-neutral stream.
    ///
    /// # Errors
    /// Returns an error when request execution or response decoding fails.
    ///
    /// # Examples
    /// Construct [BedrockClient] and pass an implementation of
    /// [crate::recovery::CredentialRecovery] to supply application refresh.
    pub async fn converse_stream(
        &self,
        model_id: &str,
        body: Vec<u8>,
        recovery: &dyn crate::recovery::CredentialRecovery,
    ) -> Result<futures::stream::BoxStream<'static, StreamChunk>> {
        let url = self.runtime_model_url(model_id, "converse-stream");
        tracing::debug!("Bedrock stream URL: {}", url);

        let mut response = self
            .send_request("POST", &url, Some(&body), "bedrock")
            .await?;

        // On a mid-session auth failure, attempt a silent token refresh and
        // retry once so an active TUI recovers without restarting.
        if auth_recover::is_auth_failure(response.status()) && recovery.recover().await {
            response = self
                .send_request("POST", &url, Some(&body), "bedrock")
                .await?;
        }

        let status = response.status();
        if !status.is_success() {
            let text = response.text().await.context("failed to read error body")?;
            let body = codetether_util::truncate_bytes_safe(&text, 500);
            anyhow::bail!(super::auth_error::message(status, body));
        }

        Ok(event_chunk_stream(response))
    }
}
