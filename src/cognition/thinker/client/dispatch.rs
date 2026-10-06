//! Backend dispatch with fresh identity on direct native requests.

use super::super::{ThinkerOutput, bedrock_backend, candle_dispatch, openai_backend, provider};
use super::{ThinkerClient, ThinkerClientBackend};
use anyhow::Result;

impl ThinkerClient {
    /// Generate a thinking completion using the configured backend.
    ///
    /// # Errors
    ///
    /// Propagates backend transport, decoding, and inference failures.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use codetether_agent::cognition::{ThinkerClient, ThinkerConfig};
    /// # async fn demo() {
    /// let client = ThinkerClient::new(ThinkerConfig::default()).unwrap();
    /// let output = client.think("You are helpful.", "Explain Rust").await.unwrap();
    /// assert!(!output.text.is_empty());
    /// # }
    /// ```
    pub async fn think(&self, system_prompt: &str, user_prompt: &str) -> Result<ThinkerOutput> {
        match &self.backend {
            ThinkerClientBackend::OpenAICompat { http } => {
                openai_backend::think(&self.config, http, system_prompt, user_prompt).await
            }
            ThinkerClientBackend::Registry => {
                provider::think(&self.config, system_prompt, user_prompt).await
            }
            ThinkerClientBackend::Bedrock { provider } => {
                bedrock_backend::think(&self.config, provider, system_prompt, user_prompt).await
            }
            ThinkerClientBackend::Candle { runtime } => {
                candle_dispatch::think(runtime, "candle", system_prompt, user_prompt).await
            }
        }
    }
}
