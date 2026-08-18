//! Backend-agnostic thinker client facade.

use anyhow::Result;

use super::client_backend::ThinkerClientBackend;
use super::{
    ThinkerConfig, ThinkerOutput, bedrock_backend, candle_dispatch, client_build, openai_backend,
    provider,
};

/// Client for thinker inference across multiple backends.
///
/// # Examples
///
/// ```rust,no_run
/// use codetether_agent::cognition::{ThinkerClient, ThinkerConfig};
/// let client = ThinkerClient::new(ThinkerConfig::default()).unwrap();
/// ```
#[derive(Clone)]
pub struct ThinkerClient {
    config: ThinkerConfig,
    backend: ThinkerClientBackend,
}

impl std::fmt::Debug for ThinkerClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ThinkerClient")
            .field("backend", &self.config.backend)
            .field("model", &self.config.model)
            .finish()
    }
}

impl ThinkerClient {
    /// Create a new thinker client from the given config.
    ///
    /// # Errors
    ///
    /// Returns an error when the configured backend cannot be initialized.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use codetether_agent::cognition::{ThinkerClient, ThinkerConfig};
    /// let client = ThinkerClient::new(ThinkerConfig::default()).unwrap();
    /// ```
    pub fn new(config: ThinkerConfig) -> Result<Self> {
        let backend = client_build::build(&config)?;
        Ok(Self { config, backend })
    }

    /// Return a reference to the active config.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use codetether_agent::cognition::{ThinkerClient, ThinkerConfig};
    /// let client = ThinkerClient::new(ThinkerConfig::default()).unwrap();
    /// assert!(!client.config().enabled);
    /// ```
    pub fn config(&self) -> &ThinkerConfig {
        &self.config
    }

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
                candle_dispatch::think(runtime, system_prompt, user_prompt).await
            }
        }
    }
}
