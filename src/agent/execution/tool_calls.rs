//! Tool-call replay for agent execution.
//!
//! This module applies provider-requested tool calls to the session and stores
//! the resulting tool history.
//!
//! # Examples
//!
//! ```ignore
//! agent.execute_tool_calls(&mut session, vec![]).await;
//! ```

use super::messages::PendingToolCall;
use crate::agent::{Agent, ToolUse};
use crate::provider::{ContentPart, Message, Role};
use crate::session::Session;

impl Agent {
    pub(super) async fn execute_tool_calls(
        &self,
        session: &mut Session,
        tool_calls: Vec<PendingToolCall>,
    ) {
        for (id, name, arguments) in tool_calls {
            let result = self.execute_tool(&name, &arguments).await;
            session.tool_uses.push(ToolUse {
                id: id.clone(),
                name: name.clone(),
                input: arguments.clone(),
                output: result.output.clone(),
                success: result.success,
            });
            session.add_message(Message {
                role: Role::Tool,
                content: vec![ContentPart::ToolResult {
                    tool_call_id: id,
                    content: result.output,
                }],
            });
        }
    }
}
