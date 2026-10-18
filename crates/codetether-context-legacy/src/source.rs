//! Borrow only the native fields needed for context derivation.
use codetether_provider_core::Message;
use codetether_rlm::RlmConfig;
/// Read-only session input for legacy context derivation.
/// # Implementors
/// Expose the stored transcript, stable session identity and RLM settings.
pub trait Source {
    /// Stored messages, never mutated by derivation.
    fn messages(&self) -> &[Message];
    /// Stable identity used in compression traces.
    fn session_id(&self) -> &str;
    /// Configuration copied into each compression pass.
    fn rlm_config(&self) -> &RlmConfig;
}
