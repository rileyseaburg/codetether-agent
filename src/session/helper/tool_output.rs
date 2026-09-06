//! Build call-associated tool messages while keeping images out of text digests.

use crate::provider::{ContentPart, Message, Role};
use crate::session::helper::evidence::digest;
use serde_json::Value;
use std::collections::HashMap;

pub(crate) fn tool_result_with_status(
    tool_call_id: String,
    tool: &str,
    success: bool,
    output: String,
) -> Message {
    tool_result_with_metadata(tool_call_id, tool, success, output, None)
}

pub(crate) fn tool_result_with_metadata(
    tool_call_id: String,
    tool: &str,
    success: bool,
    output: String,
    metadata: Option<&HashMap<String, Value>>,
) -> Message {
    let output = digest::compact_output(tool, &output);
    let mut content = vec![ContentPart::ToolResult {
        tool_call_id,
        content: crate::tool::feedback::render(tool, success, &output),
    }];
    content.extend(crate::tool::result_images::content(metadata));
    Message {
        role: Role::Tool,
        content,
    }
}

#[cfg(test)]
mod tests {
    use super::tool_result_with_status;
    use crate::provider::ContentPart;

    #[test]
    fn caps_large_tool_output_in_session_history() {
        let msg = tool_result_with_status("call-1".into(), "bash", true, "x".repeat(5000));
        let ContentPart::ToolResult { content, .. } = &msg.content[0] else {
            panic!("expected tool result");
        };
        assert!(content.len() < 5000);
        assert!(content.contains("runtime digest"));
        assert!(content.contains("- status: success"));
    }
}
