//! Application credential and provider lookup boundaries.
use crate::{ThinkerConfig, ThinkerOutput};
use anyhow::Result;
use std::future::Future;
/// Application services retained outside thinker backend dispatch.
/// # Implementors
/// Applications construct their credentialed provider and preserve their
/// registry lookup policy; this crate supplies the inference wiring.
pub trait Host {
    /// Shared handle to the signed Bedrock transport.
    type Bedrock: Clone + Send + Sync;
    /// Construct the provider using the application's credential policy.
    fn bedrock(config: &ThinkerConfig) -> Result<Self::Bedrock>;
    /// Resolve a registered model and execute a thought.
    fn registry(
        config: &ThinkerConfig,
        system: &str,
        user: &str,
    ) -> impl Future<Output = Result<ThinkerOutput>> + Send;
    /// Send a prepared request through the existing signed transport.
    fn send(
        provider: &Self::Bedrock,
        url: &str,
        bytes: &[u8],
    ) -> impl Future<Output = Result<reqwest::Response>> + Send;
}
