//! Detail-row assembly for `/spawn`-managed children.

use ratatui::text::Line;

use crate::managed::Agent;
use codetether_provider_core::Message;

/// Build managed-child detail rows from display fields and messages.
/// # Arguments
/// * `agent` - Borrowed child identity and processing state.
/// * `messages` - Session messages to display.
/// # Returns
/// Identity metadata followed by the formatted transcript.
/// # Examples
/// ```no_run
/// fn detail(agent: &codetether_agent_dashboard::managed::Agent<'_>) {
///     assert!(!codetether_agent_dashboard::managed_detail::lines(agent, &[]).is_empty());
/// }
/// ```
pub fn lines(agent: &Agent<'_>, messages: &[Message]) -> Vec<Line<'static>> {
    let status = if agent.is_processing {
        "working"
    } else {
        "idle"
    };
    let model = agent.model_id.unwrap_or("default model");
    let parent = agent.parent.unwrap_or("main");
    let mut rows = crate::metadata::lines(agent.name, parent, status, model, agent.instructions);
    crate::messages::append(&mut rows, messages, crate::messages::Source::ManagedAgent);
    rows
}
