//! Build the `CompletionRequest` used to ask the router model for a plan.

use crate::provider::{CompletionRequest, ContentPart, Message, Role};

use super::prompt::{ROUTER_SYSTEM, build_user_prompt};

/// Construct the provider request carrying the router prompt.
///
/// `temperature = 1.0` keeps the request compatible with the Z.AI GLM
/// family (see project AGENTS notes on GLM-5.1 temperature).
///
/// # Examples
///
/// ```rust
/// use codetether_agent::search::request::build_router_request;
/// let req = build_router_request("glm-5.1", "find fn main", 1);
/// assert_eq!(req.messages.len(), 2);
/// ```
pub fn build_router_request(model: &str, query: &str, top_n: usize) -> CompletionRequest {
    CompletionRequest {
        model: model.to_string(),
        messages: vec![
            Message {
                role: Role::System,
                content: vec![ContentPart::Text {
                    text: ROUTER_SYSTEM.to_string(),
                }],
            },
            Message {
                role: Role::User,
                content: vec![ContentPart::Text {
                    text: build_user_prompt(query, top_n),
                }],
            },
        ],
        tools: Vec::new(),
        temperature: Some(1.0),
        top_p: Some(0.9),
        max_tokens: Some(400),
        stop: Vec::new(),
    }
}
