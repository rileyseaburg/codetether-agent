//! Native construction and mutable transcript access for indexed restoration.
use crate::Snapshot;
use anyhow::Result;
use codetether_context_policy::{pages::PageKind, summary_index::SummaryIndex};
use codetether_provider_core::Message;
use codetether_tracked_vec::TrackedVec;
/// Native session operations required to restore persisted record windows.
/// # Implementors
/// Keep header decoding, field access and sidecar normalization identical to
/// ordinary native session loading. The store owns record order and checkpoints.
pub trait Restore: Snapshot + Sized {
    /// Decode a persisted header without reading transcript records.
    /// # Errors
    /// Returns native header decoding errors.
    fn decode_header(header: &str) -> Result<Self>;
    /// Mutable tracked messages for ordered hydration.
    fn messages_mut(&mut self) -> &mut TrackedVec<Message>;
    /// Mutable page classifications aligned with messages.
    fn pages_mut(&mut self) -> &mut TrackedVec<PageKind>;
    /// Mutable tracked tool-use records.
    fn tools_mut(&mut self) -> &mut TrackedVec<Self::Tool>;
    /// Replace the summary index after loading the requested window.
    fn set_summary_index(&mut self, index: SummaryIndex);
    /// Apply the native sidecar-normalization rules after restoration.
    fn normalize_sidecars(&mut self);
}
