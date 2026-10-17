//! Sanitized function tool schemas.
use crate::*;
impl OpenRouterProvider {
    pub(crate) fn convert_tools(tools: &[ToolDefinition]) -> Vec<Value> {
        tools
            .iter()
            .map(|t| {
                json!({
                    "type": "function",
                    "function": {
                        "name": t.name,
                        "description": t.description,
                        "parameters": schema::sanitize(t.parameters.clone())
                    }
                })
            })
            .collect()
    }
}
