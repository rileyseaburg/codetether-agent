//! Read-only native data needed to prepare and acknowledge a session delta.
use crate::State;
use anyhow::Result;
use codetether_context_policy::pages::PageKind;
use codetether_provider_core::Message;
use codetether_tracked_vec::TrackedVec;
/// Native session data exposed without duplicating tracked vectors or checkpoints.
/// # Implementors
/// Applications return references to their existing state and vectors and keep
/// native metadata/provenance handling in the header encoder.
pub trait Snapshot {
    /// Serializable tool-use record owned by the application.
    type Tool: serde::Serialize;
    /// Current durable session identity.
    fn id(&self) -> &str;
    /// Instance-local optimistic checkpoint.
    fn state(&self) -> &State;
    /// Tracked messages in the loaded window.
    fn messages(&self) -> &TrackedVec<Message>;
    /// Tracked page classifications aligned with loaded messages.
    fn pages(&self) -> &TrackedVec<PageKind>;
    /// Tracked tool-use records in the loaded window.
    fn tools(&self) -> &TrackedVec<Self::Tool>;
    /// Encode the native small header, preserving metadata/provenance behavior.
    /// # Errors
    /// Propagates native header serialization errors.
    fn encode_header(&self) -> Result<String>;
}
