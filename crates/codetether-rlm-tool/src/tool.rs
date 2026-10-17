//! RLM tool construction and provider configuration.
use crate::Host;
use codetether_provider_core::Provider;
use codetether_rlm::RlmConfig;
use std::{marker::PhantomData, sync::Arc};
/// Large-context analysis tool bound to the application's RLM router.
///
/// # Examples
/// ```
/// use codetether_rlm_tool::{RlmTool,Host};
/// fn construct<H:Host>(provider:std::sync::Arc<dyn codetether_provider_core::Provider>) -> RlmTool<H> {
///     RlmTool::new(provider,"model".into(),codetether_rlm::RlmConfig::default())
/// }
/// ```
pub struct RlmTool<H: Host> {
    pub(crate) provider: Arc<dyn Provider>,
    pub(crate) model: String,
    pub(crate) config: RlmConfig,
    host: PhantomData<fn() -> H>,
}
impl<H: Host> RlmTool<H> {
    /// Build a tool with a configured provider, model and processing limits.
    pub fn new(provider: Arc<dyn Provider>, model: String, config: RlmConfig) -> Self {
        Self {
            provider,
            model,
            config,
            host: PhantomData,
        }
    }
}
