//! Streaming completion request setup, independent of SSE decoding.
use crate::{CompletionRequest, StreamChunk, ZaiProvider};
use anyhow::{Context, Result};
use codetether_provider_core::retry::send_response_with_retry;
use futures::{StreamExt, stream::BoxStream};

impl ZaiProvider {
    /// Send a streaming completion using the injected HTTP client.
    ///
    /// # Arguments
    /// * `request` — Model, messages, tools and generation limits.
    /// # Returns
    /// A stream of reasoning, text, tool calls and terminal usage.
    /// # Errors
    /// Returns request transport errors; API errors are emitted as stream chunks.
    /// # Examples
    /// ```rust,no_run
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// use codetether_provider_zai::{ZaiProvider, CompletionRequest};
    /// use futures::StreamExt;
    /// let provider = ZaiProvider::with_client(reqwest::Client::new(),
    ///     std::env::var("ZAI_API_KEY").unwrap(), "https://api.z.ai/api/paas/v4".into());
    /// let request = CompletionRequest { model: "glm-5.2".into(), messages: vec![],
    ///     tools: vec![], temperature: None, top_p: None, max_tokens: Some(256), stop: vec![] };
    /// assert!(provider.complete_stream(request).await?.next().await.is_some());
    /// # Ok::<(), anyhow::Error>(()) }).unwrap();
    /// ```
    pub async fn complete_stream(
        &self,
        request: CompletionRequest,
    ) -> Result<BoxStream<'static, StreamChunk>> {
        let tool_stream = ["glm-5", "glm-4.7", "glm-4.6"]
            .iter()
            .any(|model| request.model.contains(model));
        let body = crate::stream_request::build_body(
            &request,
            crate::messages::convert(&request.messages, false),
            crate::request::tools(&request),
            crate::thinking::thinking_config_for_request(&request),
            tool_stream,
        );
        let response = send_response_with_retry(|| async {
            self.client
                .post(format!("{}/chat/completions", self.base_url))
                .bearer_auth(&self.api_key)
                .json(&body)
                .send()
                .await
                .context("Failed to send streaming request to Z.AI")
        })
        .await?;
        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Ok(futures::stream::iter(crate::stream_request::error_chunks(
                crate::error_wire::message(status, &text),
            ))
            .boxed());
        }
        Ok(crate::stream_decode::decode(response))
    }
}
