//! Initiator and vision request flags.
use crate::*;
impl CopilotProvider {
    pub(crate) fn is_agent_initiated(messages: &[Message]) -> bool {
        messages
            .iter()
            .rev()
            .find(|msg| !matches!(msg.role, Role::System | Role::Developer))
            .map(|msg| msg.role != Role::User)
            .unwrap_or(false)
    }
    pub(crate) fn has_vision_input(messages: &[Message]) -> bool {
        messages.iter().any(|msg| {
            msg.content
                .iter()
                .any(|part| matches!(part, ContentPart::Image { .. }))
        })
    }
}
