use super::super::{ThinkerConfig, ThinkerOutput};
use std::future::Future;
/// Native model construction, provider resolution and transport observation.
/// # Implementors
/// Application adapters retain Candle runtimes, credentialed Bedrock providers
/// and registry lookup; the shared client owns selection and dispatch.
pub trait Host {
    /// Cloneable handle to a serialized native model.
    type Native: Clone + Send + Sync;
    /// Cloneable handle to a signed Bedrock provider.
    type Bedrock: Clone + Send + Sync;
    /// Construct the configured native model.
    /// # Errors
    /// Propagates model loading or unavailable-backend errors.
    fn native(config: &ThinkerConfig) -> anyhow::Result<Self::Native>;
    /// Construct the credentialed Bedrock provider.
    /// # Errors
    /// Propagates missing credentials and provider initialization failures.
    fn bedrock(config: &ThinkerConfig) -> anyhow::Result<Self::Bedrock>;
    /// Execute OpenAI-compatible HTTP inference with native observations.
    /// # Errors
    /// Propagates HTTP transport and response failures.
    fn http(
        config: &ThinkerConfig,
        http: &reqwest::Client,
        system: &str,
        user: &str,
    ) -> impl Future<Output = anyhow::Result<ThinkerOutput>> + Send;
    /// Resolve the configured registry model and execute inference.
    /// # Errors
    /// Propagates registry resolution and provider failures.
    fn registry(
        config: &ThinkerConfig,
        system: &str,
        user: &str,
    ) -> impl Future<Output = anyhow::Result<ThinkerOutput>> + Send;
    /// Execute signed Bedrock inference.
    /// # Errors
    /// Propagates signed request and response failures.
    fn bedrock_think(
        config: &ThinkerConfig,
        provider: &Self::Bedrock,
        system: &str,
        user: &str,
    ) -> impl Future<Output = anyhow::Result<ThinkerOutput>> + Send;
    /// Execute native inference under the supplied provider identity.
    /// # Errors
    /// Propagates native inference failures.
    fn native_think(
        runtime: &Self::Native,
        provider: &str,
        system: &str,
        user: &str,
    ) -> impl Future<Output = anyhow::Result<ThinkerOutput>> + Send;
}
