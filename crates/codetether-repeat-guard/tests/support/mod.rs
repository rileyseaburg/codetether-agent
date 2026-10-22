use codetether_provider_core::{ContentPart, Message, Role};
pub use codetether_repeat_guard::ProgressLedger as Ledger;
pub use serde_json::json;
pub fn call(id: &str, name: &str, input: serde_json::Value) -> Message {
    Message {
        role: Role::Assistant,
        content: vec![ContentPart::ToolCall {
            id: id.into(),
            name: name.into(),
            arguments: input.to_string(),
            thought_signature: None,
        }],
    }
}
pub fn reply(id: &str, content: &str) -> Message {
    Message {
        role: Role::Tool,
        content: vec![ContentPart::ToolResult {
            tool_call_id: id.into(),
            content: content.into(),
        }],
    }
}
#[allow(dead_code)] // Shared by integration targets with different fixture needs.
pub fn text(text: &str) -> Message {
    Message {
        role: Role::User,
        content: vec![ContentPart::Text { text: text.into() }],
    }
}
pub fn outcome(
    ledger: &mut Ledger,
    index: usize,
    name: &str,
    input: serde_json::Value,
    success: bool,
    output: &str,
) {
    let id = index.to_string();
    ledger.observe_message(&call(&id, name, input), index * 2);
    ledger.observe_message(
        &reply(
            &id,
            &format!(
                "Tool call feedback\n- status: {}\n\nOutput:\n{output}",
                if success { "success" } else { "failure" }
            ),
        ),
        index * 2 + 1,
    );
}
