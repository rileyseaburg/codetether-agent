//! Delegate relay normalization and handoff rendering.
use super::*;
impl RalphLoop {
    /// Normalize text for convergence detection in relay loops.
    pub(in crate::ralph::ralph_loop) fn normalize_for_relay(text: &str) -> String {
        codetether_ralph_core::presentation::normalize_for_relay(text)
    }

    /// Format relay handoff text between agents.
    pub(in crate::ralph::ralph_loop) fn prepare_relay_handoff(
        from_agent: &str,
        output: &str,
        original_task: &str,
    ) -> String {
        codetether_ralph_core::presentation::prepare_relay_handoff(
            from_agent,
            output,
            original_task,
        )
    }
}
