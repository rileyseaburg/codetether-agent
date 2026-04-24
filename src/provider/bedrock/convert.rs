//! Convert the crate's generic [`Message`] format to the Bedrock Converse
//! API's JSON schema, and back-convert tool definitions.
//!
//! Bedrock Converse requires:
//! - A separate top-level `system` array for system prompts.
//! - Strict alternation of `user` / `assistant` messages.
//! - Assistant tool-use blocks with `toolUse` objects (`input` as an object).
//! - Tool results appear in the *next user* message as `toolResult` blocks.
//!
//! # Examples
//!
//! ```rust
//! use codetether_agent::provider::bedrock::{convert_messages, convert_tools};
//! use codetether_agent::provider::{ContentPart, Message, Role, ToolDefinition};
//! use serde_json::json;
//!
//! let msgs = vec![
//!     Message {
//!         role: Role::System,
//!         content: vec![ContentPart::Text { text: "You are helpful.".into() }],
//!     },
//!     Message {
//!         role: Role::User,
//!         content: vec![ContentPart::Text { text: "hi".into() }],
//!     },
//! ];
//! let (system, api_msgs) = convert_messages(&msgs);
//! assert_eq!(system.len(), 1);
//! assert_eq!(api_msgs.len(), 1);
//! assert_eq!(api_msgs[0]["role"], "user");
//!
//! let tools = vec![ToolDefinition {
//!     name: "echo".into(),
//!     description: "Echo text".into(),
//!     parameters: json!({"type":"object"}),
//! }];
//! let converted = convert_tools(&tools);
//! assert_eq!(converted[0]["toolSpec"]["name"], "echo");
//! ```

use crate::provider::{ContentPart, Message, Role, ToolDefinition};
use serde_json::{Value, json};

/// Convert generic [`Message`]s to Bedrock Converse API format.
///
/// IMPORTANT: Bedrock requires strict role alternation (user/assistant).
/// Consecutive [`Role::Tool`] messages are merged into a single `"user"`
/// message so all `toolResult` blocks for a given assistant turn appear
/// together. Consecutive same-role messages are also merged to prevent
/// validation errors.
///
/// # Arguments
///
/// * `messages` — The crate-internal chat transcript to send.
///
/// # Returns
///
/// A tuple `(system_parts, api_messages)`:
/// - `system_parts`: objects suitable for the top-level `"system"` array.
/// - `api_messages`: objects suitable for the top-level `"messages"` array.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::provider::bedrock::convert_messages;
/// use codetether_agent::provider::{ContentPart, Message, Role};
///
/// let msgs = vec![Message {
///     role: Role::User,
///     content: vec![ContentPart::Text { text: "hello".into() }],
/// }];
/// let (system, api_msgs) = convert_messages(&msgs);
/// assert!(system.is_empty());
/// assert_eq!(api_msgs.len(), 1);
/// assert_eq!(api_msgs[0]["content"][0]["text"], "hello");
/// ```
pub fn convert_messages(messages: &[Message]) -> (Vec<Value>, Vec<Value>) {
    let mut system_parts: Vec<Value> = Vec::new();
    let mut api_messages: Vec<Value> = Vec::new();

    for msg in messages {
        match msg.role {
            Role::System => append_system(msg, &mut system_parts),
            Role::User => append_user(msg, &mut api_messages),
            Role::Assistant => append_assistant(msg, &mut api_messages),
            Role::Tool => append_tool(msg, &mut api_messages),
        }
    }

    repair_orphan_tool_uses(&mut api_messages);
    (system_parts, api_messages)
}

/// Convert crate-internal [`ToolDefinition`]s into Bedrock `toolConfig.tools`
/// entries.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::provider::bedrock::convert_tools;
/// use codetether_agent::provider::ToolDefinition;
/// use serde_json::json;
///
/// let t = vec![ToolDefinition {
///     name: "ls".into(),
///     description: "List files".into(),
///     parameters: json!({"type":"object"}),
/// }];
/// let out = convert_tools(&t);
/// assert_eq!(out[0]["toolSpec"]["description"], "List files");
/// ```
pub fn convert_tools(tools: &[ToolDefinition]) -> Vec<Value> {
    tools
        .iter()
        .map(|t| {
            json!({
                "toolSpec": {
                    "name": t.name,
                    "description": t.description,
                    "inputSchema": {
                        "json": t.parameters
                    }
                }
            })
        })
        .collect()
}

fn append_system(msg: &Message, system_parts: &mut Vec<Value>) {
    let text: String = msg
        .content
        .iter()
        .filter_map(|p| match p {
            ContentPart::Text { text } => Some(text.clone()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("\n");
    if !text.trim().is_empty() {
        system_parts.push(json!({"text": text}));
    }
}

fn append_user(msg: &Message, api_messages: &mut Vec<Value>) {
    let mut content_parts: Vec<Value> = Vec::new();
    for part in &msg.content {
        if let ContentPart::Text { text } = part
            && !text.trim().is_empty()
        {
            content_parts.push(json!({"text": text}));
        }
    }
    if content_parts.is_empty() {
        return;
    }
    if let Some(last) = api_messages.last_mut()
        && last.get("role").and_then(|r| r.as_str()) == Some("user")
        && let Some(arr) = last.get_mut("content").and_then(|c| c.as_array_mut())
    {
        arr.extend(content_parts);
        return;
    }
    api_messages.push(json!({
        "role": "user",
        "content": content_parts
    }));
}

fn append_assistant(msg: &Message, api_messages: &mut Vec<Value>) {
    let mut content_parts: Vec<Value> = Vec::new();
    for part in &msg.content {
        match part {
            ContentPart::Text { text } => {
                if !text.trim().is_empty() {
                    content_parts.push(json!({"text": text}));
                }
            }
            ContentPart::ToolCall {
                id,
                name,
                arguments,
                ..
            } => {
                let input: Value =
                    serde_json::from_str(arguments).unwrap_or_else(|_| json!({"raw": arguments}));
                content_parts.push(json!({
                    "toolUse": {
                        "toolUseId": id,
                        "name": name,
                        "input": input
                    }
                }));
            }
            _ => {}
        }
    }
    // Bedrock rejects whitespace-only text blocks; drop empty assistant turns.
    if content_parts.is_empty() {
        return;
    }
    if let Some(last) = api_messages.last_mut()
        && last.get("role").and_then(|r| r.as_str()) == Some("assistant")
        && let Some(arr) = last.get_mut("content").and_then(|c| c.as_array_mut())
    {
        arr.extend(content_parts);
        return;
    }
    api_messages.push(json!({
        "role": "assistant",
        "content": content_parts
    }));
}

fn append_tool(msg: &Message, api_messages: &mut Vec<Value>) {
    let mut content_parts: Vec<Value> = Vec::new();
    for part in &msg.content {
        if let ContentPart::ToolResult {
            tool_call_id,
            content,
        } = part
        {
            let content = if content.trim().is_empty() {
                "(empty tool result)".to_string()
            } else {
                content.clone()
            };
            content_parts.push(json!({
                "toolResult": {
                    "toolUseId": tool_call_id,
                    "content": [{"text": content}],
                    "status": "success"
                }
            }));
        }
    }
    if content_parts.is_empty() {
        return;
    }
    if let Some(last) = api_messages.last_mut()
        && last.get("role").and_then(|r| r.as_str()) == Some("user")
        && let Some(arr) = last.get_mut("content").and_then(|c| c.as_array_mut())
    {
        arr.extend(content_parts);
        return;
    }
    api_messages.push(json!({
        "role": "user",
        "content": content_parts
    }));
}

/// Synthesize `toolResult` blocks for every assistant `toolUse` whose
/// `toolUseId` is not matched in the following user turn.
///
/// Bedrock strictly requires that every assistant `toolUse` be paired
/// with a `toolResult` (same `toolUseId`) in the immediately following
/// user message. If the session was interrupted (stream error, crash,
/// network drop, `continue`-after-failure), the persisted transcript
/// can contain orphan `toolUse` blocks that make every subsequent
/// request fail with:
///
/// ```text
/// Expected toolResult blocks at messages.X.content for the following Ids: ...
/// ```
///
/// This repair pass scans each assistant message, collects the
/// `toolUseId`s declared in it, inspects the next message (or inserts
/// a synthetic `user` message if the assistant was last), and appends
/// a `toolResult` with `status: "error"` and a short text payload for
/// each missing id. This keeps the model moving forward instead of
/// hard-failing on every retry.
fn repair_orphan_tool_uses(api_messages: &mut Vec<Value>) {
    let mut i = 0;
    while i < api_messages.len() {
        if api_messages[i].get("role").and_then(Value::as_str) != Some("assistant") {
            i += 1;
            continue;
        }
        let declared_ids: Vec<String> = api_messages[i]
            .get("content")
            .and_then(Value::as_array)
            .map(|arr| {
                arr.iter()
                    .filter_map(|p| {
                        p.get("toolUse")
                            .and_then(|tu| tu.get("toolUseId"))
                            .and_then(Value::as_str)
                            .map(String::from)
                    })
                    .collect()
            })
            .unwrap_or_default();
        if declared_ids.is_empty() {
            i += 1;
            continue;
        }

        let satisfied_ids: Vec<String> = api_messages
            .get(i + 1)
            .filter(|m| m.get("role").and_then(Value::as_str) == Some("user"))
            .and_then(|m| m.get("content").and_then(Value::as_array))
            .map(|arr| {
                arr.iter()
                    .filter_map(|p| {
                        p.get("toolResult")
                            .and_then(|tr| tr.get("toolUseId"))
                            .and_then(Value::as_str)
                            .map(String::from)
                    })
                    .collect()
            })
            .unwrap_or_default();

        let missing: Vec<String> = declared_ids
            .into_iter()
            .filter(|id| !satisfied_ids.contains(id))
            .collect();
        if missing.is_empty() {
            i += 1;
            continue;
        }

        let synthetic: Vec<Value> = missing
            .iter()
            .map(|id| {
                json!({
                    "toolResult": {
                        "toolUseId": id,
                        "content": [{"text": "(tool call interrupted; no result recorded)"}],
                        "status": "error"
                    }
                })
            })
            .collect();

        let next_is_user = api_messages
            .get(i + 1)
            .and_then(|m| m.get("role").and_then(Value::as_str))
            == Some("user");

        if next_is_user {
            if let Some(arr) = api_messages[i + 1]
                .get_mut("content")
                .and_then(Value::as_array_mut)
            {
                let mut merged = synthetic;
                merged.extend(arr.drain(..));
                *arr = merged;
            }
        } else {
            api_messages.insert(
                i + 1,
                json!({
                    "role": "user",
                    "content": synthetic
                }),
            );
        }
        i += 1;
    }
}

#[cfg(test)]
mod repair_tests {
    use super::*;

    #[test]
    fn synthesizes_missing_tool_result_when_assistant_is_last() {
        let mut msgs = vec![json!({
            "role": "assistant",
            "content": [{"toolUse": {"toolUseId": "call_x", "name": "t", "input": {}}}]
        })];
        repair_orphan_tool_uses(&mut msgs);
        assert_eq!(msgs.len(), 2);
        assert_eq!(msgs[1]["role"], "user");
        assert_eq!(msgs[1]["content"][0]["toolResult"]["toolUseId"], "call_x");
        assert_eq!(msgs[1]["content"][0]["toolResult"]["status"], "error");
    }

    #[test]
    fn prepends_missing_result_into_existing_user_turn() {
        let mut msgs = vec![
            json!({
                "role": "assistant",
                "content": [{"toolUse": {"toolUseId": "call_a", "name": "t", "input": {}}}]
            }),
            json!({
                "role": "user",
                "content": [{"text": "continue"}]
            }),
        ];
        repair_orphan_tool_uses(&mut msgs);
        assert_eq!(msgs.len(), 2);
        assert_eq!(msgs[1]["content"][0]["toolResult"]["toolUseId"], "call_a");
        assert_eq!(msgs[1]["content"][1]["text"], "continue");
    }

    #[test]
    fn leaves_already_paired_tool_uses_alone() {
        let before = vec![
            json!({
                "role": "assistant",
                "content": [{"toolUse": {"toolUseId": "call_ok", "name": "t", "input": {}}}]
            }),
            json!({
                "role": "user",
                "content": [{"toolResult": {"toolUseId": "call_ok", "content": [{"text": "ok"}], "status": "success"}}]
            }),
        ];
        let mut after = before.clone();
        repair_orphan_tool_uses(&mut after);
        assert_eq!(before, after);
    }

    #[test]
    fn handles_multiple_missing_ids_in_one_assistant_turn() {
        let mut msgs = vec![json!({
            "role": "assistant",
            "content": [
                {"toolUse": {"toolUseId": "call_1", "name": "t", "input": {}}},
                {"toolUse": {"toolUseId": "call_2", "name": "t", "input": {}}}
            ]
        })];
        repair_orphan_tool_uses(&mut msgs);
        assert_eq!(msgs[1]["content"].as_array().unwrap().len(), 2);
        assert_eq!(msgs[1]["content"][0]["toolResult"]["toolUseId"], "call_1");
        assert_eq!(msgs[1]["content"][1]["toolResult"]["toolUseId"], "call_2");
    }
}
