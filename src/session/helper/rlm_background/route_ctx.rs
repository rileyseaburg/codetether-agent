//! Routing context for background RLM decisions.

use crate::provider::Message;
use crate::rlm::RoutingContext;

pub(super) fn routing_ctx(
    tool: &str,
    call_id: &str,
    session_id: &str,
    window: usize,
    messages: &[Message],
) -> RoutingContext {
    RoutingContext {
        tool_id: tool.into(),
        session_id: session_id.into(),
        call_id: Some(call_id.into()),
        model_context_limit: window,
        current_context_tokens: Some(super::super::token::estimate_tokens_for_messages(messages)),
    }
}
