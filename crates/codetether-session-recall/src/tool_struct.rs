//! Construction and runtime dependencies for `SessionRecallTool`.

use codetether_provider_core::Provider;
use codetether_rlm::RlmConfig;
use std::sync::Arc;

#[cfg(test)]
pub(super) use super::description::DESCRIPTION;

/// Local-first recall tool over persisted session evidence.
///
/// # Examples
/// ```rust
/// # fn example<H:codetether_session_recall::Host>(provider:std::sync::Arc<dyn codetether_provider_core::Provider>) {
/// let _tool=codetether_session_recall::SessionRecallTool::<H>::new(provider,"model".into(),codetether_rlm::RlmConfig::default());
/// # }
/// ```
pub struct SessionRecallTool<H> {
    host: std::marker::PhantomData<fn() -> H>,
    pub(super) provider: Arc<dyn Provider>,
    pub(super) model: String,
    pub(super) config: RlmConfig,
}

impl<H: crate::Host> SessionRecallTool<H> {
    /// Construct recall with the existing synthesis provider, model and RLM settings.
    pub fn new(provider: Arc<dyn Provider>, model: String, config: RlmConfig) -> Self {
        Self {
            host: std::marker::PhantomData,
            provider,
            model,
            config,
        }
    }
}
