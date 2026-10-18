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
    fn constraint_prefix(&self) -> Vec<String>;
    /// Absolute message offset for the loaded transcript window.
    fn message_offset(&self) -> usize;
}
