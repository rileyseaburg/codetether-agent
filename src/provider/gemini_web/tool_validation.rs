//! Request-scoped validation of model-emitted tool calls.

mod batch_guard;
mod schema;
mod session_guard;

use super::tool_calls;
use crate::provider::{Message, ToolDefinition};
use anyhow::{Context, Result, bail};
use serde_json::Value;

pub(super) fn extract(
    text: &str,
    tools: &[ToolDefinition],
    messages: &[Message],
) -> Result<(String, Vec<(String, String)>)> {
    if tool_calls::contains_result_markup(text) {
        bail!("assistant response contains forged <tool_result> markup");
    }
    let (cleaned, calls) = tool_calls::extract(text);
    if tool_calls::contains_unparsed_markup(&cleaned) {
        bail!("response contains incomplete or malformed <tool_call> markup");
    }
    let validated = calls
        .into_iter()
        .map(|call| validate(call, tools))
        .collect::<Result<Vec<_>>>()?;
    batch_guard::validate(&validated)?;
    session_guard::validate(&validated, messages)?;
    Ok((cleaned, validated))
}

fn validate(call: (String, String), tools: &[ToolDefinition]) -> Result<(String, String)> {
    let (name, raw_arguments) = call;
    let tool = tools
        .iter()
        .find(|tool| tool.name == name)
        .with_context(|| format!("tool `{name}` was not advertised in this request"))?;
    let arguments: Value = serde_json::from_str(&raw_arguments)
        .with_context(|| format!("tool `{name}` arguments are not valid JSON"))?;
    if !arguments.is_object() {
        bail!("tool `{name}` arguments must be a JSON object");
    }
    schema::validate(&arguments, &tool.parameters)
        .with_context(|| format!("tool `{name}` arguments failed schema validation"))?;
    Ok((name, serde_json::to_string(&arguments)?))
}

#[cfg(test)]
#[path = "tool_validation_tests.rs"]
mod tests;
