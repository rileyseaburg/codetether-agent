//! Native transform and background model-resolution callbacks.
use codetether_context_policy::transforms::ExperimentalStats;
use codetether_provider_core::Message;
/// Application transforms applied to the cloned history.
/// # Examples
/// Implementations supply the existing transform pipeline and pairing repair.
pub struct Hooks {
    /// Pre-compression transformations; the reported stats are not used here.
    pub apply: fn(&mut Vec<Message>) -> ExperimentalStats,
    /// Pairing repair applied after compression.
    pub repair: fn(&mut Vec<Message>) -> ExperimentalStats,
    /// Resolver used by oversized-message background summarization.
    pub resolver: codetether_rlm_background::Resolver,
}
