//! Sequential execution of tool calls from one provider turn.

use super::{
    state::{State, ToolCall},
    tool_one,
};
use crate::provider::{ContentPart, Message, Role};

pub(super) async fn execute(state: &mut State, calls: Vec<ToolCall>) -> bool {
    for call in calls {
        let completed = tool_one::execute(state, call).await;
        state.messages.push(Message {
            role: Role::Tool,
            content: vec![ContentPart::ToolResult {
                tool_call_id: completed.id,
                content: completed.output,
            }],
        });
        tracing::debug!(tool = %completed.name, "Recorded tool result");
        if completed.timed_out {
            return true;
        }
    }
    false
}
