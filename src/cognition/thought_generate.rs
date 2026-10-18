//! Native compatibility adapter for shared cognition generation.
use super::{ThinkerClient, ThoughtEvent, ThoughtResult, ThoughtWorkItem};

/// Generate one thought for `work`, using the thinker when it is configured.
///
/// Uses deterministic phase text when no thinker exists, the thinker errors, or
/// it returns empty output.
pub(super) async fn generate_phase_thought(
    thinker: Option<&ThinkerClient>,
    work: &ThoughtWorkItem,
    context: &[ThoughtEvent],
) -> ThoughtResult {
    codetether_cognition_core::thought::generate::generate_phase_thought(thinker, work, context)
        .await
}
