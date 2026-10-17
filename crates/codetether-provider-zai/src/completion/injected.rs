//! Buffered completion transport using the provider-neutral retry policy.
use crate::{CompletionRequest, CompletionResponse, ZaiProvider};
use anyhow::{Context, Result};
use codetether_provider_core::retry::send_with_retry;
impl ZaiProvider {
    /// Send a buffered chat completion using the injected HTTP client.
    ///
    /// # Arguments
    /// * `request` — Model, messages, tools and generation limits.
    /// # Returns
    /// The assistant message, finish reason and token accounting.
    /// # Errors
    /// Returns transport, API, or response decoding errors.
    /// # Examples
    /// ```rust,no_run
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// use codetether_provider_zai::{CompletionRequest, ZaiProvider, DEFAULT_BASE_URL};
    /// let provider = ZaiProvider::with_client(reqwest::Client::new(),
    ///     std::env::var("ZAI_API_KEY").unwrap(), DEFAULT_BASE_URL.into());
    /// let request = CompletionRequest { model: "glm-5.2".into(), messages: vec![],
    ///     tools: vec![], temperature: None, top_p: None, max_tokens: Some(256), stop: vec![] };
    /// let result = provider.complete(request).await?;
    /// assert!(!result.message.content.is_empty());
    /// # Ok::<(), anyhow::Error>(()) }).unwrap();
    /// ```
    pub async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse> {
        let body = crate::request::body(&request);
        let (text, status) = send_with_retry(|| async {
            let response = self
                .client
                .post(format!("{}/chat/completions", self.base_url))
                .bearer_auth(&self.api_key)
                .json(&body)
                .send()
                .await
                .context("Failed to send request to Z.AI")?;
            let status = response.status();
            let text = response
                .text()
                .await
                .context("Failed to read Z.AI response")?;
            Ok((text, status))
        })
        .await?;
        if !status.is_success() {
            anyhow::bail!("{}", crate::error_wire::message(status, &text));
        }
        crate::response::parse(&text)
    }
}
