//! Header visibility uses the same local, swarm and tool registries.
use super::Host;
/// Whether any child or swarm agent makes the header visible.
/// # Arguments
/// Receives the native registry source.
/// # Returns
/// True when at least one registry has an agent.
/// # Examples
/// ```
/// fn visible<H: codetether_agent_dashboard::bar::Host>(host: &H) -> bool {
///     codetether_agent_dashboard::bar::visible(host)
/// }
/// ```
pub fn visible(host: &impl Host) -> bool {
    !host.agents().is_empty() || !host.subtasks().is_empty() || !host.tool_agents().is_empty()
}
