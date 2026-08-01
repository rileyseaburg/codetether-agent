//! OpenRouter conversation-shape normalization.
//!
//! Some endpoints require strict user/assistant alternation and reject adjacent
//! `system` messages. CodeTether may add retrieved context as a second system
//! message, so compatible plain-text neighbors are coalesced before sending.

use serde_json::Value;

pub(super) fn coalesce(messages: Vec<Value>) -> Vec<Value> {
    let mut output: Vec<Value> = Vec::with_capacity(messages.len());
    for message in messages {
        if let Some(previous) = output.last_mut()
            && mergeable(previous, &message)
        {
            merge(previous, &message);
        } else {
            output.push(message);
        }
    }
    output
}

fn mergeable(left: &Value, right: &Value) -> bool {
    let role = left.get("role").and_then(Value::as_str);
    role == right.get("role").and_then(Value::as_str)
        && matches!(role, Some("system" | "user"))
        && left.get("content").is_some_and(Value::is_string)
        && right.get("content").is_some_and(Value::is_string)
}

fn merge(left: &mut Value, right: &Value) {
    let Some(existing) = left.get("content").and_then(Value::as_str) else { return };
    let Some(next) = right.get("content").and_then(Value::as_str) else { return };
    left["content"] = Value::String(format!("{existing}\n\n{next}"));
}

#[cfg(test)]
mod tests {
    use super::coalesce;
    use serde_json::json;

    #[test]
    fn combines_adjacent_system_messages() {
        let result = coalesce(vec![
            json!({"role":"system","content":"rules"}),
            json!({"role":"system","content":"context"}),
            json!({"role":"user","content":"task"}),
        ]);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0]["content"], "rules\n\ncontext");
    }
}