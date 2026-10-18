//! OpenAI-compatible tools conversion.
use crate::{OpenAiApiResult, openai_bad_request};
use codetether_openai_wire::OpenAiRequestTool;

/// Validate and convert function tool definitions.
/// # Arguments
/// tools are the incoming wire definitions.
/// # Returns
/// Provider tool definitions in input order.
/// # Errors
/// Returns an invalid-request error for unsupported or incomplete input.
/// # Examples
/// ```
/// assert!(codetether_openai_adapter::convert_openai_tools(&[]).unwrap().is_empty());
/// ```
pub fn convert_openai_tools(
    tools: &[OpenAiRequestTool],
) -> OpenAiApiResult<Vec<codetether_provider_core::ToolDefinition>> {
    let mut converted = Vec::with_capacity(tools.len());

    for (index, tool) in tools.iter().enumerate() {
        if tool.kind != "function" {
            return Err(openai_bad_request(format!(
                "tools[{index}].type `{}` is not supported",
                tool.kind
            )));
        }
        if tool.function.name.trim().is_empty() {
            return Err(openai_bad_request(format!(
                "tools[{index}].function.name is required"
            )));
        }

        converted.push(codetether_provider_core::ToolDefinition {
            name: tool.function.name.clone(),
            description: tool.function.description.clone(),
            parameters: tool.function.parameters.clone(),
        });
    }

    Ok(converted)
}
