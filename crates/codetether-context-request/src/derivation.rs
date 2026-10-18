//! Inputs passed to native context derivation.
use codetether_context_policy::derivation::DerivePolicy;
use codetether_provider_core::{Provider, ToolDefinition};
use std::sync::Arc;
/// Per-request derivation settings supplied to [crate::Host].
pub struct Derivation<'a> {
    /// Primary provider for compression and policy execution.
    pub provider: Arc<dyn Provider>,
    /// Primary model identifier.
    pub model: &'a str,
    /// System instructions included in context estimates.
    pub system_prompt: &'a str,
    /// Tool definitions included in context estimates.
    pub tools: &'a [ToolDefinition],
    /// Effective derivation strategy.
    pub policy: DerivePolicy,
    /// Optional forced keep-last count for context recovery.
    pub force_keep_last: Option<usize>,
}
