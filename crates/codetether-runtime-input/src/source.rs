//! Immutable session fields needed for authoritative tool-input enrichment.
use codetether_provider_core::Message;
use codetether_session_metadata::SessionMetadata;
/// Native session identity, metadata and current transcript.
/// # Implementors
/// Expose the existing fields without copying or changing their ownership.
pub trait Source {
    /// Owning session ID.
    fn id(&self) -> &str;
    /// Active agent identity.
    fn agent(&self) -> &str;
    /// Persisted metadata and runtime provenance.
    fn metadata(&self) -> &SessionMetadata;
    /// Current messages for prior-context policy fallback.
    fn messages(&self) -> &[Message];
}
