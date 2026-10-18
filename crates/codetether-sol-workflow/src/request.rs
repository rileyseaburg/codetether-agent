//! Tool-free planner request construction and text extraction.
use codetether_provider_core::{CompletionRequest, ContentPart, Message, Role};

/// Build a tool-free planner completion request.
/// # Arguments
/// * `model` - Resolved provider model.
/// * `system` - Planning or review contract.
/// * `user` - Task and supplied context.
/// # Returns
/// A two-message request with no tools and a 4,000-token response limit.
/// # Examples
/// ```
/// let request = codetether_sol_workflow::request("model".into(), "plan".into(), "task".into());
/// assert!(request.tools.is_empty());
/// assert_eq!(request.max_tokens, Some(4_000));
/// ```
pub fn request(model: String, system: String, user: String) -> CompletionRequest {
    CompletionRequest {
        model,
        messages: vec![message(Role::System, system), message(Role::User, user)],
        tools: Vec::new(),
        temperature: None,
        top_p: None,
        max_tokens: Some(4_000),
        stop: Vec::new(),
    }
}

fn message(role: Role, text: String) -> Message {
    Message {
        role,
        content: vec![ContentPart::Text { text }],
    }
}

/// Join text parts of a planner response with newlines.
/// # Arguments
/// * `parts` - Response content, including any non-text parts to skip.
/// # Returns
/// Text in response order, or an empty string when no text is present.
/// # Examples
/// ```
/// assert_eq!(codetether_sol_workflow::text(&[]), "");
/// ```
pub fn text(parts: &[ContentPart]) -> String {
    parts
        .iter()
        .filter_map(|part| match part {
            ContentPart::Text { text } => Some(text.as_str()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    #[test]
    fn planner_never_advertises_tools() {
        assert!(
            super::request("model".into(), "system".into(), "user".into())
                .tools
                .is_empty()
        );
    }
}
