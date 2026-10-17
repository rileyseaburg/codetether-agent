use super::*;

/// Convert live LLM tool definitions into catalog schemas.
///
/// # Examples
///
/// ```rust
/// use codetether_bus_core::tool_catalog::from_definitions;
/// use codetether_provider_core::ToolDefinition;
///
/// let schemas = from_definitions(&[ToolDefinition {
///     name: "read".into(),
///     description: "Read a file".into(),
///     parameters: serde_json::json!({"type": "object"}),
/// }]);
/// assert_eq!(schemas[0].name, "read");
/// ```
pub fn from_definitions(
    definitions: &[codetether_provider_core::ToolDefinition],
) -> Vec<ToolSchema> {
    definitions
        .iter()
        .map(|d| ToolSchema {
            name: d.name.clone(),
            description: d.description.clone(),
            parameters: d.parameters.clone(),
        })
        .collect()
}
