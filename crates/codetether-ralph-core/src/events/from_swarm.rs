//! Translate shared swarm activity into Ralph story events.
use super::RalphEvent;
use codetether_swarm_core::SwarmEvent;
/// Project a supported swarm event into its story-specific Ralph event.
///
/// # Arguments
/// `story_id` identifies the story; `event` supplies owned activity data.
/// # Returns
/// A translated event, or None for swarm-specific activity.
/// # Examples
/// ```rust,no_run
/// fn example(event: codetether_swarm_core::SwarmEvent) {
/// let mapped = codetether_ralph_core::events::from_swarm_event("US-1", event);
/// }
/// ```
pub fn from_swarm_event(story_id: &str, event: SwarmEvent) -> Option<RalphEvent> {
    Some(match event {
        SwarmEvent::AgentToolCall { tool_name, .. } => RalphEvent::StoryToolCall {
            story_id: story_id.to_string(),
            tool_name,
        },
        SwarmEvent::AgentToolCallDetail { detail, .. } => RalphEvent::StoryToolCallDetail {
            story_id: story_id.to_string(),
            detail,
        },
        SwarmEvent::AgentMessage { entry, .. } => RalphEvent::StoryMessage {
            story_id: story_id.to_string(),
            entry,
        },
        SwarmEvent::AgentOutput { output, .. } => RalphEvent::StoryOutput {
            story_id: story_id.to_string(),
            output,
        },
        SwarmEvent::AgentError { error, .. } => RalphEvent::StoryError {
            story_id: story_id.to_string(),
            error,
        },
        _ => return None, // Skip swarm-specific events
    })
}
