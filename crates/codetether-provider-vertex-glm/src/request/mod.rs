//! Message roles and request settings for Vertex GLM.
use crate::*;
impl VertexGlmProvider {
    pub(super) fn convert_messages(messages: &[Message]) -> Vec<Value> {
        messages
            .iter()
            .map(|msg| {
                let role = match msg.role {
                    Role::System | Role::Developer => "system",
                    Role::User => "user",
                    Role::Assistant => "assistant",
                    Role::Tool => "tool",
                };

                match msg.role {
                    Role::Tool => tool::encode(msg, role),
                    Role::Assistant => assistant::encode(msg),
                    _ => text::encode(msg, role),
                }
            })
            .collect()
    }
    fn convert_tools(tools: &[ToolDefinition]) -> Vec<Value> {
        tools
            .iter()
            .map(|t| {
                json!({
                    "type": "function",
                    "function": {
                        "name": t.name,
                        "description": t.description,
                        "parameters": t.parameters
                    }
                })
            })
            .collect()
    }
}
mod assistant;
mod body;
mod model_id;
mod text;
mod tool;
