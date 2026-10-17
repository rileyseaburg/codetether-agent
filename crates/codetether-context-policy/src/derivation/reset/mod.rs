//! Reset preparation and reconstruction around host summary generation.
//!
//! [prepare] returns a ready projection, a legacy fallback, or a prefix/tail
//! plan. The host retrieves a summary before [rebuild] and [finalize] finish it.
use crate::pages::ResidencyLevel;
use crate::projection::helpers::DerivedContext;
use crate::selection::active_tail::active_user_tail_start;
use crate::transforms::pairing;
use codetether_provider_core::Message;
/// Outcome of preparing a reset projection.
///
/// # Examples
/// ```
/// use codetether_context_policy::derivation::reset::{prepare, ResetPreparation};
/// match prepare(Vec::new(), 0, 100, |_| 0) {
///     ResetPreparation::Ready(context) => assert!(context.messages.is_empty()),
///     ResetPreparation::Legacy => panic!("unexpected legacy fallback"),
///     ResetPreparation::Summarize(_) => panic!("unexpected summary request"),
/// }
/// ```
pub enum ResetPreparation {
    /// A complete projection that needs no summary request.
    Ready(DerivedContext),
    /// No reset context exists; use the host's legacy derivation.
    Legacy,
    /// Retrieve a summary for this prefix before finalization.
    Summarize(ResetSummaryPlan),
}
mod plan;
pub use plan::ResetSummaryPlan;
mod state;
use state::ResetState;
mod prepare;
pub use prepare::prepare;
mod rebuild;
mod split;
mod threshold;
pub use rebuild::rebuild;
mod finalize;
pub use finalize::finalize;
/// Reset-marker rendering and detection.
pub mod markers;

#[cfg(test)]
mod tests;
