//! Drain Ralph learning messages and render them in received order.
use super::*;
/// Collect formatted learning and handoff lines for a project.
///
/// # Arguments
/// `bus` supplies the existing shared bus; `prd_id` identifies the project.
/// # Returns
/// Ordered lines from the collector drain, ignoring unrelated message variants.
/// # Examples
/// ```rust,no_run
/// fn example(bus: &std::sync::Arc<codetether_bus_core::AgentBus>) {
/// let learnings = codetether_ralph_core::bus::collect(bus, "project");
/// }
/// ```
pub fn collect(bus: &Arc<AgentBus>, prd_id: &str) -> Vec<String> {
    let mut handle = bus.handle("ralph-collector");
    let envelopes = handle.drain_ralph_learnings(prd_id);
    let mut learnings = Vec::new();
    for env in envelopes {
        match env.message {
            BusMessage::RalphLearning {
                story_id,
                iteration,
                learnings: items,
                ..
            } => {
                for l in items {
                    learnings.push(format!("[{story_id} iter {iteration}] {l}"));
                }
            }
            BusMessage::RalphHandoff {
                from_story,
                progress_summary,
                ..
            } => {
                learnings.push(format!("[handoff from {from_story}] {progress_summary}"));
            }
            _ => {}
        }
    }
    learnings
}
