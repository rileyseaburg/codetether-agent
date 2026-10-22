//! Remove an older complete exchange after the active instruction, never its newest result.
use super::incremental_types::MessageOrigin;
use crate::pages::ResidencyLevel;
use codetether_provider_core::{ContentPart, Message, Role};

pub(super) fn drop_oldest_exchange(
    messages: &mut Vec<Message>,
    resolutions: &mut Vec<ResidencyLevel>,
    origins: &mut Vec<MessageOrigin>,
) -> bool {
    let Some(active) = crate::selection::active_tail::active_user_tail_start(messages, 0) else {
        return false;
    };
    for start in active + 1..messages.len().saturating_sub(1) {
        if messages[start].role != Role::Assistant {
            continue;
        }
        let mut end = start + 1;
        while end < messages.len() && messages[end].role == Role::Tool {
            end += 1;
        }
        // The latest exchange must remain available, including all its tool results.
        if end == messages.len() || !complete_exchange(&messages[start..end]) {
            continue;
        }
        messages.drain(start..end);
        resolutions.drain(start..end);
        origins.drain(start..end);
        return true;
    }
    false
}

fn complete_exchange(messages: &[Message]) -> bool {
    let calls: Vec<_> = messages[0]
        .content
        .iter()
        .filter_map(|part| match part {
            ContentPart::ToolCall { id, .. } => Some(id.as_str()),
            _ => None,
        })
        .collect();
    let results: Vec<_> = messages[1..]
        .iter()
        .flat_map(|message| &message.content)
        .filter_map(|part| match part {
            ContentPart::ToolResult { tool_call_id, .. } => Some(tool_call_id.as_str()),
            _ => None,
        })
        .collect();
    calls.iter().all(|id| results.contains(id)) && results.iter().all(|id| calls.contains(id))
}

#[cfg(test)]
#[path = "incremental_tail_clamp_tests.rs"]
mod tests;
