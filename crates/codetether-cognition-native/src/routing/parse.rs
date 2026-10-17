//! Application warning adapter for the shared FunctionGemma parser.
use super::parsed_call::ParsedToolCall;

pub(super) fn parse_functiongemma_response(text: &str) -> Vec<ParsedToolCall> {
    codetether_cognition_core::tool_routing::parse_functiongemma_response(text, |block| {
        tracing::warn!(target: "codetether_agent::cognition::tool_router::parse",
            block = %block,
            "FunctionGemma produced unparseable tool_call block"
        );
    })
}
