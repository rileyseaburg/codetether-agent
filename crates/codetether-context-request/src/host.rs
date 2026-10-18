//! Async context derivation and optional recall supplied by native sessions.
use crate::{Derivation, Source};
use anyhow::Result;
use async_trait::async_trait;
use codetether_context_policy::projection::helpers::DerivedContext;
use codetether_provider_core::Message;
/// Native operations required to build a complete request.
/// # Implementors
/// Preserve the session's existing derivation and recall behavior.
#[async_trait]
pub trait Host: Source {
    /// Derive the request transcript.
    /// # Errors
    /// Propagates context derivation failures.
    async fn derive(&self, input: Derivation<'_>) -> Result<DerivedContext>;
    /// Retrieve optional prior-session context after derivation.
    async fn prefetch(&self) -> Option<Message>;
}
