//! Native policy and persisted-constraint inputs for shared dispatch.
use codetether_context_policy::derivation::DerivePolicy;
/// Session inputs shared by all three derivation strategies.
/// # Implementors
/// Reuse native legacy/reset/incremental sources and expose persisted state.
pub trait Source:
    codetether_reset_derivation::Source + codetether_incremental_derivation::Source
{
    /// Persisted derivation strategy before process overrides are applied.
    fn saved_policy(&self) -> DerivePolicy;
    /// Persisted hard constraints rendered in original sequence order.
    fn constraint_prefix(&self) -> Vec<codetether_context_policy::state_header::ConstraintRecord>;
    /// Explicit partial source replacements retained in session metadata.
    fn constraint_corrections(
        &self,
    ) -> &[codetether_context_policy::state_header::ConstraintCorrection] {
        &[]
    }
    /// Absolute message offset for the loaded transcript window.
    fn message_offset(&self) -> usize;
}
