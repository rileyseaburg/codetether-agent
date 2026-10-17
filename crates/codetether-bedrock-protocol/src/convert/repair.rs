//! Bedrock tool-use and tool-result turn normalization.

use serde_json::{Value, json};

mod blocks;
#[cfg(test)]
mod tests;

pub(super) fn tool_exchanges(messages: &mut Vec<Value>) {
    let mut index = 0;
    while index < messages.len() {
        let ids = blocks::tool_use_ids(&messages[index]);
        if ids.is_empty() {
            blocks::drop_orphan_results(&mut messages[index]);
            index += 1;
            continue;
        }
        ensure_user_turn(messages, index + 1);
        blocks::put_required_results_first(&mut messages[index + 1], &ids);
        index += 2;
    }
}

fn ensure_user_turn(messages: &mut Vec<Value>, index: usize) {
    let is_user = messages
        .get(index)
        .and_then(|message| message.get("role"))
        .and_then(Value::as_str)
        == Some("user");
    if !is_user {
        messages.insert(index, json!({"role": "user", "content": []}));
    } else if !messages[index]["content"].is_array() {
        messages[index]["content"] = json!([]);
    }
}
