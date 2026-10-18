//! Immutable native session inputs for proactive preparation.
use codetether_provider_core::{Message, Provider};
use codetether_rlm::RlmConfig;
use std::sync::Arc;
/// Session identity, transcript and cached model settings.
/// # Implementors
/// Expose existing session fields without modifying them.
pub trait Source {
    /// Owning session ID.
    fn id(&self) -> &str;
    /// Loaded transcript window.
    fn messages(&self) -> &[Message];
    /// Global offset of that transcript window.
    fn message_offset(&self) -> usize;
    /// RLM configuration for proactive jobs.
    fn rlm_config(&self) -> &RlmConfig;
    /// Cached subcall provider, when present.
    fn subcall_provider(&self) -> Option<&Arc<dyn Provider>>;
    /// Cached provider-local subcall model name.
    fn subcall_model_name(&self) -> Option<&str>;
}
