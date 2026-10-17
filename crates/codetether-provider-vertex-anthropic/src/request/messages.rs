//! Role dispatch and final system-content shape.
use crate::*;
impl VertexAnthropicProvider {
    pub(crate) fn convert_messages(messages: &[Message]) -> (Option<Value>, Vec<Value>) {
        let mut system_blocks: Vec<Value> = Vec::new();
        let mut api_messages: Vec<Value> = Vec::new();
        for msg in messages {
            match msg.role {
                Role::System | Role::Developer => {
                    super::system::push(msg, &mut system_blocks, &mut api_messages)
                }
                Role::User => super::user::push(msg, &mut system_blocks, &mut api_messages),
                Role::Assistant => {
                    super::assistant::push(msg, &mut system_blocks, &mut api_messages)
                }
                Role::Tool => super::tool::push(msg, &mut system_blocks, &mut api_messages),
            }
        }
        let system = if system_blocks.is_empty() {
            None
        } else if system_blocks.len() == 1 {
            // Single block: return as string for simplicity
            system_blocks
                .first()
                .and_then(|b| b.get("text"))
                .and_then(Value::as_str)
                .map(|s| json!(s))
        } else {
            // Multiple blocks: return as array
            Some(json!(system_blocks))
        };

        (system, api_messages)
    }
}
