use crate::provider::{ContentPart, Message, Role};
use serde_json::Value;

/// Build a DeepSeek-API-shaped JSON object for the tetherscript hook.
pub fn build_ds_msg(msg: &Message) -> Value {
    let mut ds = serde_json::Map::new();
    ds.insert("role".into(), role_str(msg.role).into());
    ds.insert("content".into(), collect_text(msg).into());
    ds.insert("reasoning_content".into(), collect_reasoning(msg));
    ds.insert("tool_calls".into(), collect_tool_calls(msg));
    Value::Object(ds)
}

fn role_str(role: Role) -> &'static str {
    match role {
        Role::Assistant => "assistant",
        Role::User => "user",
        Role::Tool => "tool",
        Role::System => "system",
    }
}

fn collect_text(msg: &Message) -> String {
    msg.content.iter().filter_map(|p| match p { ContentPart::Text { text } => Some(text.as_str()), _ => None }).collect::<Vec<_>>().join("\n")
}

fn collect_reasoning(msg: &Message) -> Value {
    let s: String = msg.content.iter().filter_map(|p| match p { ContentPart::Thinking { text } => Some(text.as_str()), _ => None }).collect::<Vec<_>>().join("");
    if s.is_empty() { Value::Null } else { serde_json::json!(s) }
}

fn collect_tool_calls(msg: &Message) -> Value {
    let calls: Vec<Value> = msg.content.iter().filter_map(|p| match p {
        ContentPart::ToolCall { id, name, arguments, .. } => Some(serde_json::json!({
            "id": id, "type": "function", "function": { "name": name, "arguments": arguments }
        })),
        _ => None,
    }).collect();
    if calls.is_empty() { Value::Null } else { Value::Array(calls) }
}
