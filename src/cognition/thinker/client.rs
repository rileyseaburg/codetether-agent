//! Backend-agnostic thinker client facade.

use anyhow::Result;

use super::client_backend::ThinkerClientBackend;
use super::{ThinkerConfig, client_build};

#[path = "client/dispatch.rs"]
mod dispatch;
#[path = "client/native.rs"]
mod native;

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
}
