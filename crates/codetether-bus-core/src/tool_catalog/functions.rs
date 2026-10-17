use super::*;

/// Render schemas as the OpenAI-style function entries used by chat templates.
///
/// # Examples
///
/// ```rust
/// use codetether_bus_core::tool_catalog::{ToolSchema, to_function_specs};
///
/// let specs = to_function_specs(&[ToolSchema {
///     name: "read".into(),
///     description: "Read a file".into(),
///     parameters: serde_json::json!({"type": "object"}),
/// }]);
/// assert_eq!(specs[0]["type"], "function");
/// assert_eq!(specs[0]["function"]["name"], "read");
/// ```
pub fn to_function_specs(schemas: &[ToolSchema]) -> Vec<serde_json::Value> {
    schemas.iter().map(function_spec).collect()
}

fn function_spec(schema: &ToolSchema) -> serde_json::Value {
    serde_json::json!({
        "type": "function",
        "function": {
            "name": schema.name,
            "description": schema.description,
            "parameters": schema.parameters,
        }
    })
}
