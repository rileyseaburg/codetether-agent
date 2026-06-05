use crate::provider::{ContentPart, Message, Role};

pub(crate) fn tool_result(tool_call_id: String, tool: &str, output: String) -> Message {
    Message {
        role: Role::Tool,
        content: vec![ContentPart::ToolResult {
            tool_call_id,
            content: super::evidence::digest::compact_output(tool, &output),
        }],
    }
}

#[cfg(test)]
mod tests {
    use super::tool_result;
    use crate::provider::ContentPart;

    #[test]
    fn caps_large_tool_output_in_session_history() {
        let msg = tool_result("call-1".into(), "bash", "x".repeat(5000));
        let ContentPart::ToolResult { content, .. } = &msg.content[0] else {
            panic!("expected tool result");
        };
        assert!(content.len() < 5000);
        assert!(content.contains("runtime digest"));
    }
}
