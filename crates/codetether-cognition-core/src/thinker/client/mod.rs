//! Backend-neutral thinker client ownership and inference routing.
//!
//! [Client] owns the selected backend and configuration. Implement [Host] to
//! construct native/provider handles and bind their inference operations.
mod backend;
mod build;
mod dispatch;
mod host;
mod native;
use super::ThinkerConfig;
pub use host::Host;
/// Client facade retaining configuration and one constructed backend.
/// # Examples
/// ```ignore
/// let client = Client::<NativeHost>::new(config)?;
/// let output = client.think("system", "prompt").await?;
/// ```
pub struct Client<H: Host> {
    config: ThinkerConfig,
    backend: backend::Backend<H::Native, H::Bedrock>,
}
impl<H: Host> Clone for Client<H> {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            backend: self.backend.clone(),
        }
    }
}
impl<H: Host> std::fmt::Debug for Client<H> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ThinkerClient")
            .field("backend", &self.config.backend)
            .field("model", &self.config.model)
            .finish()
    }
}
impl<H: Host> Client<H> {
    /// Construct the backend selected by the supplied configuration.
    /// # Arguments
    /// * `config` — Model, credentials source, transport and runtime settings.
    /// # Returns
    /// A client owning its configuration and constructed backend handle.
    /// # Errors
    /// Propagates HTTP client, native model or credentialed provider failures.
    /// # Examples
    /// ```ignore
    /// let client = Client::<NativeHost>::new(config)?;
    /// ```
    pub fn new(config: ThinkerConfig) -> anyhow::Result<Self> {
        let backend = build::build::<H>(&config)?;
        Ok(Self { config, backend })
    }
    /// Borrow the active model configuration.
    pub fn config(&self) -> &ThinkerConfig {
        &self.config
    }
}
